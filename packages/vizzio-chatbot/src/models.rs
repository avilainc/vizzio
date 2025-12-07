use avila_serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::db::Database;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum MessageType {
    Text,
    Notification,
    Alert,
    Success,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MessageStatus {
    Sending,
    Sent,
    Delivered,
    Read,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Sender {
    Bot,
    User,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub id: String,
    pub sender: Sender,
    pub content: String,
    pub timestamp: DateTime<Utc>,
    pub read: bool,
    pub status: MessageStatus,
    #[serde(rename = "type")]
    pub msg_type: MessageType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

impl ChatMessage {
    pub fn new(sender: Sender, content: String, msg_type: MessageType) -> Self {
        Self {
            id: format!("msg-{}", Uuid::new_v4()),
            sender,
            content,
            timestamp: Utc::now(),
            read: sender == Sender::User,
            status: MessageStatus::Sent,
            msg_type,
            metadata: None,
        }
    }

    pub fn with_metadata(mut self, metadata: serde_json::Value) -> Self {
        self.metadata = Some(metadata);
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationPayload {
    pub build_id: String,
    pub workflow: String,
    pub status: String,
    pub details: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatState {
    pub db: Arc<RwLock<Database>>,
}

#[derive(Debug, Deserialize)]
pub struct BuildNotification {
    pub buildId: String,
    pub workflow: String,
    pub status: String,
    pub details: serde_json::Value,
}
