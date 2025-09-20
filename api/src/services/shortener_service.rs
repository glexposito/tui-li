use crate::models::url_item::UrlItem;
use crate::stores::url_store::UrlStore;
use anyhow::Result;
use chrono::Utc;
use rand::{Rng, distr::Alphanumeric};

pub struct ShortenerService {
    store: UrlStore,
}

impl ShortenerService {
    pub fn new(store: UrlStore) -> Self {
        Self { store }
    }

    /// Shorten a URL, persist it, and return the short ID.
    pub async fn shorten(&self, long_url: &str) -> Result<String> {
        let hash = Self::hash_url(long_url);
        let reverse_pk = format!("URL#{hash}");

        if let Some(existing) = self.store.get_by_pk(&reverse_pk).await? {
            return Ok(existing.id);
        }

        // TODO: Ensure the ID is unique; if it's not, append an extra character and retry.
        let id: String = rand::rng()
            .sample_iter(&Alphanumeric)
            .take(6)
            .map(char::from)
            .collect();

        let created_at = Utc::now().to_rfc3339();

        let canonical = UrlItem {
            pk: format!("ID#{id}"),
            id: id.clone(),
            long_url: long_url.to_string(),
            created_at: created_at.clone(),
            ttl: None,
        };

        let reverse = UrlItem {
            pk: format!("URL#{}", Self::hash_url(long_url)),
            id: id.clone(),
            long_url: long_url.to_string(),
            created_at: created_at.clone(),
            ttl: None,
        };

        let items = vec![canonical, reverse];

        // save both items in a transaction
        self.store.create_many(items).await?;

        Ok(id)
    }

    /// Resolve a short ID back to the original URL.
    pub async fn resolve(&self, id: &str) -> Result<Option<String>> {
        let pk = format!("ID#{id}");

        match self.store.get_by_pk(&pk).await? {
            Some(item) => Ok(Some(item.long_url)),
            None => Ok(None),
        }
    }

    fn hash_url(url: &str) -> String {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(url.as_bytes());
        hex::encode(hasher.finalize()) // full 64 hex chars (256 bits)
    }
}
