use serde::Serialize;

#[derive(Serialize)]
pub struct UrlMapping {
    pub id: String,
    pub long_url: String,
}
