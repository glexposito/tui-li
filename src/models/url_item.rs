use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UrlItem {
    pub pk: String,
    pub id: String,
    pub long_url: String,
    pub created_at: String,
    pub ttl: Option<i64>,
}
