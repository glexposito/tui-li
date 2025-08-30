/// Canonical row: short id -> long_url && Reverse pointer: url_hash -> id
#[derive(Debug, toasty::Model)]
#[table = "url"]
pub struct UrlItem {
    #[key]
    pub pk: String,

    pub id: String,

    pub long_url: String,

    pub created_at: String,

    pub ttl: Option<i64>,
}
