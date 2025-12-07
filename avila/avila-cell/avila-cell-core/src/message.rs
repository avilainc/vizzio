//! Message types and traits

use crate::Result;

#[cfg(not(feature = "std"))]
use alloc::{string::String, vec::Vec, format};

#[cfg(feature = "std")]
use std::{string::String, vec::Vec};

/// Trait for messages that can be passed between cells
pub trait MessageTrait: core::fmt::Debug {
    /// Get message unique identifier
    fn id(&self) -> String;

    /// Get message type
    fn message_type(&self) -> &str;

    /// Get sender ID
    fn sender(&self) -> Option<String>;

    /// Get recipient ID
    fn recipient(&self) -> Option<String>;

    /// Serialize message to bytes
    fn to_bytes(&self) -> Result<Vec<u8>>;
}

/// Basic message structure
#[derive(Debug, Clone)]
pub struct Message {
    /// Unique identifier
    pub id: String,
    /// Message type
    pub message_type: String,
    /// Sender ID
    pub sender: Option<String>,
    /// Recipient ID
    pub recipient: Option<String>,
    /// Message payload
    pub payload: Vec<u8>,
}

impl Message {
    /// Create new message
    pub fn new(message_type: impl Into<String>) -> Self {
        Self {
            id: format!("msg-{}", uuid::Uuid::new_v4()),
            message_type: message_type.into(),
            sender: None,
            recipient: None,
            payload: Vec::new(),
        }
    }

    /// Set sender
    pub fn with_sender(mut self, sender: impl Into<String>) -> Self {
        self.sender = Some(sender.into());
        self
    }

    /// Set recipient
    pub fn with_recipient(mut self, recipient: impl Into<String>) -> Self {
        self.recipient = Some(recipient.into());
        self
    }

    /// Set payload
    pub fn with_payload(mut self, payload: Vec<u8>) -> Self {
        self.payload = payload;
        self
    }
}

impl MessageTrait for Message {
    fn id(&self) -> String {
        self.id.clone()
    }

    fn message_type(&self) -> &str {
        &self.message_type
    }

    fn sender(&self) -> Option<String> {
        self.sender.clone()
    }

    fn recipient(&self) -> Option<String> {
        self.recipient.clone()
    }

    fn to_bytes(&self) -> Result<Vec<u8>> {
        Ok(self.payload.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_creation() {
        let msg = Message::new("test-message");
        assert_eq!(msg.message_type(), "test-message");
    }

    #[test]
    fn test_message_builder() {
        let sender_id = Id::new();
        let recipient_id = Id::new();
        let msg = Message::new("test")
            .with_sender(sender_id)
            .with_recipient(recipient_id)
            .with_payload(vec![1, 2, 3]);

        assert_eq!(msg.sender(), Some(sender_id));
        assert_eq!(msg.recipient(), Some(recipient_id));
        assert_eq!(msg.payload, vec![1, 2, 3]);
    }
}
