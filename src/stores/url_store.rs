use crate::models::url_item::UrlItem;
use toasty::db::Db as ToastyDb;

pub struct UrlStore {
    db: ToastyDb,
}

impl UrlStore {
    pub async fn get_by_id(&self, id: &str) -> anyhow::Result<UrlItem> {
        let pk = format!("ID#{id}");
        UrlItem::get_by_pk(&self.db, &pk).await
    }

    pub async fn get_by_url_hash(&self, hash: &str) -> anyhow::Result<UrlItem> {
        let pk = format!("URL#{hash}");
        UrlItem::get_by_pk(&self.db, &pk).await
    }

    pub async fn create(&self, id: &str, long_url: &str, hash: &str) -> anyhow::Result<()> {
        let created_at = chrono::Utc::now().to_rfc3339();

        // two rows: ID#... (canonical) and URL#... (reverse pointer)
        let canonical_row = UrlItem::create()
            .pk(format!("ID#{id}"))
            .id(id.to_string())
            .long_url(long_url.to_string())
            .created_at(created_at.clone())
            .ttl(None);

        let reverse_pointer = UrlItem::create()
            .pk(format!("URL#{hash}"))
            .id(id.to_string())
            .long_url(long_url.to_string())
            .created_at(created_at)
            .ttl(None);

        // pure-Toasty path (note: not atomic across the two writes)
        UrlItem::create_many()
            .item(canonical_row)
            .item(reverse_pointer)
            .exec(&self.db)
            .await?;

        Ok(())
    }
}
