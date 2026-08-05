use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct HistorySummary {
    pub total_count: u64,
    pub favorite_count: u64,
}

impl HistorySummary {
    pub const fn empty() -> Self {
        Self {
            total_count: 0,
            favorite_count: 0,
        }
    }
}
