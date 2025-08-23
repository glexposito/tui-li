use std::collections::HashMap;
use uuid::Uuid;
use crate::models::url::UrlMapping;

pub struct UrlStore {
    urls: HashMap<String, String>, // short -> long
}

impl UrlStore {
    pub fn new() -> Self {
        UrlStore { urls: HashMap::new() }
    }

    pub fn add_url(&mut self, long_url: String) -> UrlMapping {
        let id = Uuid::new_v4().to_string()[..6].to_string();
        self.urls.insert(id.clone(), long_url.clone());

        UrlMapping {
            id,
            long_url,
        }
    }

    pub fn get_url(&self, id: &str) -> Option<String> {
        self.urls.get(id).cloned()
    }
}
