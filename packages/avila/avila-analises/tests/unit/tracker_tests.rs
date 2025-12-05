//! Unit tests for event tracker

use avila_analises::models::*;
use avila_analises::tracker::*;
use chrono::{Duration, Utc};
use std::collections::HashMap;
use uuid::Uuid;

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to create a test event
    fn create_test_event(user_id: &str, event_type: EventType) -> BehaviorEvent {
        BehaviorEvent {
            event_id: Uuid::new_v4().to_string(),
            user_id: user_id.to_string(),
            session_id: Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            event_type,
            metadata: HashMap::new(),
            context: EventContext {
                user_agent: "test-agent".to_string(),
                ip: "127.0.0.1".to_string(),
                country: Some("US".to_string()),
                device_type: DeviceType::Desktop,
                referrer: None,
                utm_source: None,
                utm_medium: None,
                utm_campaign: None,
            },
        }
    }

    #[tokio::test]
    async fn test_event_store_basic() {
        let store = EventStore::new();
        let event = create_test_event("user1", EventType::PageView {
            url: "/test".to_string(),
            title: "Test Page".to_string(),
            duration_ms: 1000,
        });

        let result = store.store(event.clone()).await;
        assert!(result.is_ok());

        let user_events = store.get_user_events("user1");
        assert_eq!(user_events.len(), 1);
        assert_eq!(user_events[0].event_id, event.event_id);
    }

    #[tokio::test]
    async fn test_event_validation() {
        let store = EventStore::new();

        // Test valid event
        let valid_event = create_test_event("user1", EventType::Click {
            element_id: "button1".to_string(),
            element_class: "btn-primary".to_string(),
            x: 100,
            y: 200,
        });

        assert!(store.store(valid_event).await.is_ok());

        // Test multiple events for same user
        let event2 = create_test_event("user1", EventType::Search {
            query: "test query".to_string(),
            results_count: 10,
        });

        assert!(store.store(event2).await.is_ok());

        let user_events = store.get_user_events("user1");
        assert_eq!(user_events.len(), 2);
    }

    #[tokio::test]
    async fn test_batch_processing() {
        let store = EventStore::new();

        // Create batch of events
        let events: Vec<BehaviorEvent> = (0..100)
            .map(|i| create_test_event(&format!("user{}", i % 10), EventType::PageView {
                url: format!("/page{}", i),
                title: format!("Page {}", i),
                duration_ms: 1000 + i as u64,
            }))
            .collect();

        // Store all events
        for event in events.iter() {
            store.store(event.clone()).await.unwrap();
        }

        // Verify all events stored
        let all_events = store.get_all_events();
        assert_eq!(all_events.len(), 100);

        // Verify events grouped by user
        for i in 0..10 {
            let user_events = store.get_user_events(&format!("user{}", i));
            assert_eq!(user_events.len(), 10); // 10 events per user
        }
    }

    #[tokio::test]
    async fn test_event_range_query() {
        let store = EventStore::new();
        let now = Utc::now();

        // Create events with different timestamps
        let mut event1 = create_test_event("user1", EventType::PageView {
            url: "/page1".to_string(),
            title: "Page 1".to_string(),
            duration_ms: 1000,
        });
        event1.timestamp = now - Duration::hours(2);

        let mut event2 = create_test_event("user1", EventType::PageView {
            url: "/page2".to_string(),
            title: "Page 2".to_string(),
            duration_ms: 2000,
        });
        event2.timestamp = now - Duration::hours(1);

        let mut event3 = create_test_event("user1", EventType::PageView {
            url: "/page3".to_string(),
            title: "Page 3".to_string(),
            duration_ms: 3000,
        });
        event3.timestamp = now;

        store.store(event1).await.unwrap();
        store.store(event2.clone()).await.unwrap();
        store.store(event3).await.unwrap();

        // Query events in last 90 minutes
        let start = now - Duration::minutes(90);
        let end = now + Duration::minutes(10);
        let range_events = store.get_events_in_range(start, end);

        assert_eq!(range_events.len(), 2); // event2 and event3
        assert!(range_events.iter().any(|e| e.event_id == event2.event_id));
    }

    #[tokio::test]
    async fn test_session_creation() {
        let session_manager = SessionManager::new(30);
        let event = create_test_event("user1", EventType::PageView {
            url: "/home".to_string(),
            title: "Home".to_string(),
            duration_ms: 1000,
        });

        session_manager.update_session(&event).await.unwrap();

        let session = session_manager.get_session(&event.session_id);
        assert!(session.is_some());

        let session = session.unwrap();
        assert_eq!(session.user_id, "user1");
        assert_eq!(session.events.len(), 1);
        assert_eq!(session.page_sequence.len(), 1);
    }

    #[tokio::test]
    async fn test_session_updates() {
        let session_manager = SessionManager::new(30);
        let session_id = Uuid::new_v4().to_string();

        // First event
        let mut event1 = create_test_event("user1", EventType::PageView {
            url: "/page1".to_string(),
            title: "Page 1".to_string(),
            duration_ms: 1000,
        });
        event1.session_id = session_id.clone();

        session_manager.update_session(&event1).await.unwrap();

        // Second event in same session
        let mut event2 = create_test_event("user1", EventType::Click {
            element_id: "btn1".to_string(),
            element_class: "button".to_string(),
            x: 100,
            y: 100,
        });
        event2.session_id = session_id.clone();
        event2.timestamp = Utc::now() + Duration::seconds(5);

        session_manager.update_session(&event2).await.unwrap();

        let session = session_manager.get_session(&session_id).unwrap();
        assert_eq!(session.events.len(), 2);
        assert_eq!(session.page_sequence.len(), 1); // Only PageView adds to sequence
    }

    #[tokio::test]
    async fn test_session_metrics() {
        let session_manager = SessionManager::new(30);
        let session_id = Uuid::new_v4().to_string();

        // Create a session with multiple events
        let mut events = vec![
            create_test_event("user1", EventType::PageView {
                url: "/home".to_string(),
                title: "Home".to_string(),
                duration_ms: 1000,
            }),
            create_test_event("user1", EventType::PageView {
                url: "/products".to_string(),
                title: "Products".to_string(),
                duration_ms: 2000,
            }),
            create_test_event("user1", EventType::Purchase {
                product_id: "prod1".to_string(),
                amount: 99.99,
                currency: "USD".to_string(),
            }),
        ];

        for event in &mut events {
            event.session_id = session_id.clone();
            session_manager.update_session(event).await.unwrap();
        }

        let session = session_manager.get_session(&session_id).unwrap();
        let metrics = session_manager.calculate_session_metrics(&session);

        assert_eq!(metrics.page_views, 2);
        assert!(!metrics.bounce); // More than 1 page view
        assert!(metrics.converted); // Has purchase event
    }

    #[tokio::test]
    async fn test_session_expiration() {
        let session_manager = SessionManager::new(30);

        // Create old session
        let mut event = create_test_event("user1", EventType::PageView {
            url: "/test".to_string(),
            title: "Test".to_string(),
            duration_ms: 1000,
        });
        event.timestamp = Utc::now() - Duration::hours(2);

        session_manager.update_session(&event).await.unwrap();

        let session = session_manager.get_session(&event.session_id).unwrap();
        assert!(session_manager.is_session_expired(&session));
    }

    #[tokio::test]
    async fn test_session_cleanup() {
        let session_manager = SessionManager::new(1); // 1 minute timeout

        // Create multiple sessions
        for i in 0..5 {
            let mut event = create_test_event(&format!("user{}", i), EventType::PageView {
                url: format!("/page{}", i),
                title: format!("Page {}", i),
                duration_ms: 1000,
            });

            // Make some sessions old
            if i < 3 {
                event.timestamp = Utc::now() - Duration::hours(1);
            }

            session_manager.update_session(&event).await.unwrap();
        }

        let removed = session_manager.cleanup_expired_sessions().await;
        assert_eq!(removed, 3); // 3 expired sessions removed
    }

    #[tokio::test]
    async fn test_concurrent_event_storage() {
        let store = EventStore::new();
        let store_clone = store.clone();

        let mut handles = vec![];

        // Spawn concurrent tasks
        for i in 0..10 {
            let store = store_clone.clone();
            let handle = tokio::spawn(async move {
                for j in 0..10 {
                    let event = create_test_event(
                        &format!("user{}", i),
                        EventType::PageView {
                            url: format!("/page{}", j),
                            title: format!("Page {}", j),
                            duration_ms: 1000,
                        },
                    );
                    store.store(event).await.unwrap();
                }
            });
            handles.push(handle);
        }

        // Wait for all tasks
        for handle in handles {
            handle.await.unwrap();
        }

        // Verify all events stored
        let all_events = store.get_all_events();
        assert_eq!(all_events.len(), 100); // 10 users * 10 events
    }
}

