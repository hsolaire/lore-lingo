//! 翻译密钥的原生存储边界；不提供文件或内存持久化降级。

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CredentialError {
    Unavailable,
}

pub trait CredentialStore: Send + Sync {
    fn read(&self, reference: &str) -> Result<Option<String>, CredentialError>;
    fn write(&self, reference: &str, secret: &str) -> Result<(), CredentialError>;
    fn delete(&self, reference: &str) -> Result<(), CredentialError>;
}

pub struct SystemCredentialStore;

#[cfg(any(target_os = "macos", target_os = "windows"))]
fn entry(reference: &str) -> Result<keyring_core::Entry, CredentialError> {
    use std::sync::OnceLock;
    static INITIALIZED: OnceLock<Result<(), CredentialError>> = OnceLock::new();
    // 步骤 1. 只注册当前平台的真实系统存储，绝不启用模拟存储。
    (*INITIALIZED.get_or_init(|| {
        #[cfg(target_os = "macos")]
        let store = apple_native_keyring_store::keychain::Store::new();
        #[cfg(target_os = "windows")]
        let store = windows_native_keyring_store::Store::new();
        keyring_core::set_default_store(store.map_err(|_| CredentialError::Unavailable)?);
        Ok(())
    }))?;
    // 步骤 2. 将访问限制在本应用的翻译凭据命名空间。
    if !reference.starts_with("translation-") || reference.len() > 128 {
        return Err(CredentialError::Unavailable);
    }
    #[cfg(target_os = "macos")]
    let entry = keyring_core::Entry::new("com.lorelingo.app", reference);
    #[cfg(target_os = "windows")]
    let entry = keyring_core::Entry::new_with_modifiers(
        "com.lorelingo.app",
        reference,
        &std::collections::HashMap::from([("persistence", "Local")]),
    );
    entry.map_err(|_| CredentialError::Unavailable)
}

#[cfg(any(target_os = "macos", target_os = "windows"))]
impl CredentialStore for SystemCredentialStore {
    fn read(&self, reference: &str) -> Result<Option<String>, CredentialError> {
        match entry(reference)?.get_password() {
            Ok(secret) => Ok(Some(secret)),
            Err(keyring_core::Error::NoEntry) => Ok(None),
            Err(_) => Err(CredentialError::Unavailable),
        }
    }

    fn write(&self, reference: &str, secret: &str) -> Result<(), CredentialError> {
        entry(reference)?
            .set_password(secret)
            .map_err(|_| CredentialError::Unavailable)
    }

    fn delete(&self, reference: &str) -> Result<(), CredentialError> {
        match entry(reference)?.delete_credential() {
            Ok(()) | Err(keyring_core::Error::NoEntry) => Ok(()),
            Err(_) => Err(CredentialError::Unavailable),
        }
    }
}

#[cfg(not(any(target_os = "macos", target_os = "windows")))]
impl CredentialStore for SystemCredentialStore {
    fn read(&self, _: &str) -> Result<Option<String>, CredentialError> {
        Err(CredentialError::Unavailable)
    }
    fn write(&self, _: &str, _: &str) -> Result<(), CredentialError> {
        Err(CredentialError::Unavailable)
    }
    fn delete(&self, _: &str) -> Result<(), CredentialError> {
        Err(CredentialError::Unavailable)
    }
}
