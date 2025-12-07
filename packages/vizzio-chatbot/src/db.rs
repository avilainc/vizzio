use avila_db::{Database as AvilaDb, Collection, Query};
use avila_serde::{Serialize, Deserialize};
use avila_error::Result;
use avila_log::info;
use crate::models::ChatMessage;

#[derive(Clone)]
pub struct Database {
    db: AvilaDb,
}

impl Database {
    pub fn new(db: AvilaDb) -> Self {
        Self { db }
    }

    pub async fn init(&self) -> Result<()> {
        let messages = self.db.collection("messages");

        // Criar índices usando avila-db
        messages.create_index("timestamp", true).await?;
        messages.create_index("sender", false).await?;
        messages.create_index("read", false).await?;

        info!("✅ Database initialized");
        Ok(())
    }

    pub async fn save_message(&self, message: &ChatMessage) -> Result<()> {
        let collection = self.db.collection("messages");
        collection.insert_one(message).await?;
        Ok(())
    }

    pub async fn get_messages(&self, limit: usize) -> Result<Vec<ChatMessage>> {
        let collection = self.db.collection("messages");
        let messages: Vec<ChatMessage> = collection
            .find(Query::empty())
            .sort("timestamp", -1)
            .limit(limit)
            .execute()
            .await?;

        Ok(messages.into_iter().rev().collect())
    }

    pub async fn get_unread_count(&self) -> Result<u64> {
        let collection = self.db.collection("messages");
        let count = collection
            .count(Query::field("read").eq(false))
            .await?;
        Ok(count)
    }

    pub async fn mark_as_read(&self, message_id: &str) -> Result<()> {
        let collection = self.db.collection("messages");
        collection
            .update_one(
                Query::field("id").eq(message_id),
                Query::set("read", true).set("status", "read"),
            )
            .await?;
        Ok(())
    }
}
