use crate::models::url::UrlMapping;
use rand::{Rng, distr::Alphanumeric};
use std::collections::HashMap;

pub struct UrlStore {
    urls: HashMap<String, String>,
}

impl UrlStore {
    pub fn new() -> Self {
        UrlStore {
            urls: HashMap::new(),
        }
    }

    fn gen_id(len: usize) -> String {
        rand::rng()
            .sample_iter(&Alphanumeric)
            .take(len)
            .map(char::from)
            .collect()
    }

    pub fn add_url(&mut self, long_url: String) -> UrlMapping {
        let mut len = 5;
        loop {
            let id = Self::gen_id(len);
            if !self.urls.contains_key(&id) {
                self.urls.insert(id.clone(), long_url.clone());
                return UrlMapping { id, long_url };
            }

            len += 1;
        }
    }

    pub fn get_url(&self, id: &str) -> Option<String> {
        self.urls.get(id).cloned()
    }
}
