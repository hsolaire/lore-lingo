use std::{collections::HashMap, time::Duration};

use reqwest::{redirect::Policy, Url};
use serde::{Deserialize, Serialize};

use crate::{
    domain::captured_layout::{
        LayoutBlock, TranslationConfig, MAX_BLOCK_CHARS, MAX_TRANSLATION_BLOCKS,
        MAX_TRANSLATION_INPUT_CHARS,
    },
    services::capture_translation::CaptureTranslationError,
};

const MAX_RESPONSE_BYTES: usize = 1024 * 1024;

#[derive(Clone)]
pub struct TranslationClient {
    client: reqwest::Client,
}

impl TranslationClient {
    pub fn new() -> Result<Self, CaptureTranslationError> {
        let client = reqwest::Client::builder()
            .connect_timeout(Duration::from_secs(10))
            .timeout(Duration::from_secs(45))
            .redirect(Policy::none())
            .build()
            .map_err(|_| CaptureTranslationError::BackendFailure)?;
        Ok(Self { client })
    }

    pub async fn translate(
        &self,
        blocks: &[LayoutBlock],
        config: &TranslationConfig,
    ) -> Result<HashMap<String, String>, CaptureTranslationError> {
        let url = validated_completion_url(config)?;
        validate_input(blocks)?;

        let requested: Vec<RequestedBlock<'_>> = blocks
            .iter()
            .map(|block| RequestedBlock {
                id: &block.id,
                text: &block.source_text,
            })
            .collect();
        let user_content = serde_json::to_string(&serde_json::json!({
            "sourceLanguage": config.source_language,
            "targetLanguage": config.target_language,
            "blocks": requested,
        }))
        .map_err(|_| CaptureTranslationError::TranslationProtocol)?;

        let body = ChatRequest {
            model: &config.model,
            temperature: 0.0,
            response_format: ResponseFormat { kind: "json_object" },
            messages: vec![
                ChatMessage {
                    role: "system",
                    content: "Translate every input block into the target language. Preserve meaning and paragraph context. Return JSON only as {\"translations\":[{\"id\":\"input-id\",\"text\":\"translation\"}]}. Return every input id exactly once and do not add ids.",
                },
                ChatMessage {
                    role: "user",
                    content: &user_content,
                },
            ],
        };

        let mut request = self.client.post(url).json(&body);
        if !config.api_key.trim().is_empty() {
            request = request.bearer_auth(config.api_key.trim());
        }
        let response = request
            .send()
            .await
            .map_err(|_| CaptureTranslationError::TranslationUnavailable)?;
        if !response.status().is_success() {
            return Err(CaptureTranslationError::TranslationUnavailable);
        }
        if response
            .content_length()
            .is_some_and(|length| length > MAX_RESPONSE_BYTES as u64)
        {
            return Err(CaptureTranslationError::TranslationProtocol);
        }

        let mut response = response;
        let mut bytes = Vec::new();
        while let Some(chunk) = response
            .chunk()
            .await
            .map_err(|_| CaptureTranslationError::TranslationUnavailable)?
        {
            if bytes.len().saturating_add(chunk.len()) > MAX_RESPONSE_BYTES {
                return Err(CaptureTranslationError::TranslationProtocol);
            }
            bytes.extend_from_slice(&chunk);
        }
        let chat: ChatResponse = serde_json::from_slice(&bytes)
            .map_err(|_| CaptureTranslationError::TranslationProtocol)?;
        let content = chat
            .choices
            .first()
            .map(|choice| choice.message.content.trim())
            .filter(|content| !content.is_empty())
            .ok_or(CaptureTranslationError::TranslationProtocol)?;
        let translated: TranslationResponse = serde_json::from_str(content)
            .map_err(|_| CaptureTranslationError::TranslationProtocol)?;

        validate_translation_map(blocks, translated.translations)
    }
}

pub(crate) fn validated_completion_url(
    config: &TranslationConfig,
) -> Result<Url, CaptureTranslationError> {
    let endpoint = config.endpoint.trim();
    let model = config.model.trim();
    if endpoint.is_empty() || model.is_empty() {
        return Err(CaptureTranslationError::TranslationNotConfigured);
    }

    let mut base =
        Url::parse(endpoint).map_err(|_| CaptureTranslationError::TranslationNotConfigured)?;
    if !base.username().is_empty()
        || base.password().is_some()
        || base.host_str().is_none()
        || base.query().is_some()
        || base.fragment().is_some()
    {
        return Err(CaptureTranslationError::TranslationNotConfigured);
    }
    let address = base.host_str().and_then(|host| {
        host.trim_matches(['[', ']'])
            .parse::<std::net::IpAddr>()
            .ok()
    });
    let is_loopback = base
        .host_str()
        .is_some_and(|host| host.eq_ignore_ascii_case("localhost"))
        || address.is_some_and(|address| address.is_loopback());
    if base.scheme() != "http" && base.scheme() != "https" {
        return Err(CaptureTranslationError::TranslationNotConfigured);
    }
    if address.is_some_and(is_disallowed_network_address) {
        return Err(CaptureTranslationError::TranslationNotConfigured);
    }
    if !is_loopback && config.api_key.trim().is_empty() {
        return Err(CaptureTranslationError::TranslationNotConfigured);
    }

    let base_path = base.path().trim_end_matches('/');
    if base_path.ends_with("/chat/completions") {
        return Ok(base);
    }
    base.set_path(&format!("{base_path}/chat/completions"));
    base.set_query(None);
    base.set_fragment(None);
    Ok(base)
}

fn is_disallowed_network_address(address: std::net::IpAddr) -> bool {
    match address {
        std::net::IpAddr::V4(address) => {
            !address.is_loopback()
                && (address.is_private()
                    || address.is_link_local()
                    || address.is_broadcast()
                    || address.is_unspecified())
        }
        std::net::IpAddr::V6(address) => {
            !address.is_loopback()
                && (address.is_unspecified()
                    || address.is_unique_local()
                    || address.is_unicast_link_local())
        }
    }
}

fn validate_input(blocks: &[LayoutBlock]) -> Result<(), CaptureTranslationError> {
    let total_chars: usize = blocks
        .iter()
        .map(|block| block.source_text.chars().count())
        .sum();
    if blocks.is_empty()
        || blocks.len() > MAX_TRANSLATION_BLOCKS
        || total_chars > MAX_TRANSLATION_INPUT_CHARS
        || blocks
            .iter()
            .any(|block| block.source_text.chars().count() > MAX_BLOCK_CHARS)
    {
        return Err(CaptureTranslationError::TranslationProtocol);
    }
    Ok(())
}

fn validate_translation_map(
    blocks: &[LayoutBlock],
    translations: Vec<TranslatedBlock>,
) -> Result<HashMap<String, String>, CaptureTranslationError> {
    if translations.len() != blocks.len() {
        return Err(CaptureTranslationError::TranslationProtocol);
    }

    let expected: std::collections::HashSet<_> =
        blocks.iter().map(|block| block.id.as_str()).collect();
    let mut mapped = HashMap::with_capacity(translations.len());
    for translation in translations {
        let text = translation.text.trim();
        if !expected.contains(translation.id.as_str())
            || text.is_empty()
            || text.chars().count() > MAX_BLOCK_CHARS * 4
            || mapped.insert(translation.id, text.to_string()).is_some()
        {
            return Err(CaptureTranslationError::TranslationProtocol);
        }
    }
    if mapped.len() != expected.len() {
        return Err(CaptureTranslationError::TranslationProtocol);
    }
    Ok(mapped)
}

#[derive(Serialize)]
struct ChatRequest<'a> {
    model: &'a str,
    temperature: f32,
    response_format: ResponseFormat<'a>,
    messages: Vec<ChatMessage<'a>>,
}

#[derive(Serialize)]
struct ResponseFormat<'a> {
    #[serde(rename = "type")]
    kind: &'a str,
}

#[derive(Serialize)]
struct ChatMessage<'a> {
    role: &'a str,
    content: &'a str,
}

#[derive(Serialize)]
struct RequestedBlock<'a> {
    id: &'a str,
    text: &'a str,
}

#[derive(Deserialize)]
struct ChatResponse {
    choices: Vec<ChatChoice>,
}

#[derive(Deserialize)]
struct ChatChoice {
    message: ChatResponseMessage,
}

#[derive(Deserialize)]
struct ChatResponseMessage {
    content: String,
}

#[derive(Deserialize)]
struct TranslationResponse {
    translations: Vec<TranslatedBlock>,
}

#[derive(Deserialize)]
struct TranslatedBlock {
    id: String,
    text: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::captured_layout::{LayoutBlockKind, NormalizedRect};

    fn blocks() -> Vec<LayoutBlock> {
        vec![
            LayoutBlock {
                id: "block-0000".into(),
                kind: LayoutBlockKind::Paragraph,
                bounds: NormalizedRect {
                    x: 0.0,
                    y: 0.0,
                    width: 0.5,
                    height: 0.1,
                },
                source_text: "Hello".into(),
                translated_text: None,
                reading_order: 0,
                confidence: 1.0,
            },
            LayoutBlock {
                id: "block-0001".into(),
                kind: LayoutBlockKind::Paragraph,
                bounds: NormalizedRect {
                    x: 0.0,
                    y: 0.2,
                    width: 0.5,
                    height: 0.1,
                },
                source_text: "World".into(),
                translated_text: None,
                reading_order: 1,
                confidence: 1.0,
            },
        ]
    }

    #[test]
    fn translation_mapping_requires_exact_ids() {
        let mapped = validate_translation_map(
            &blocks(),
            vec![
                TranslatedBlock {
                    id: "block-0001".into(),
                    text: "世界".into(),
                },
                TranslatedBlock {
                    id: "block-0000".into(),
                    text: "你好".into(),
                },
            ],
        )
        .expect("exact map");
        assert_eq!(mapped["block-0000"], "你好");

        assert!(validate_translation_map(
            &blocks(),
            vec![
                TranslatedBlock {
                    id: "block-0000".into(),
                    text: "你好".into()
                },
                TranslatedBlock {
                    id: "block-0000".into(),
                    text: "重复".into()
                },
            ],
        )
        .is_err());

        assert!(validate_translation_map(
            &blocks(),
            vec![
                TranslatedBlock {
                    id: "block-0000".into(),
                    text: "你好".into()
                },
                TranslatedBlock {
                    id: "unknown".into(),
                    text: "未知".into()
                },
            ],
        )
        .is_err());
    }

    #[test]
    fn endpoints_require_http_or_https() {
        let config = |endpoint: &str, api_key: &str| TranslationConfig {
            endpoint: endpoint.into(),
            model: "model".into(),
            api_key: api_key.into(),
            source_language: "English".into(),
            target_language: "简体中文".into(),
        };
        assert!(validated_completion_url(&config("https://api.example.com/v1", "secret")).is_ok());
        assert_eq!(
            validated_completion_url(&config("https://api.example.com/v1/", "secret"))
                .expect("补全聊天接口")
                .path(),
            "/v1/chat/completions"
        );
        assert!(validated_completion_url(&config("http://127.0.0.1:11434/v1", "")).is_ok());
        assert!(validated_completion_url(&config("http://api.example.com/v1", "secret")).is_ok());
        assert!(validated_completion_url(&config("ftp://api.example.com/v1", "secret")).is_err());
        assert!(validated_completion_url(&config("https://192.168.1.20/v1", "secret")).is_err());
        assert!(validated_completion_url(&config(
            "https://user:pass@api.example.com/v1",
            "secret"
        ))
        .is_err());
    }
}
