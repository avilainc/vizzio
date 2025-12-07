//! # Builder - Builder pattern for advanced coordinator configuration
extern crate alloc;
use alloc::boxed::Box;
use crate::coordinator::Coordinator;
use crate::events::{EventBus, EventHandler};
use crate::retry::RetryManager;
use crate::validation::IdValidator;

/// Builder for creating a Coordinator with advanced features
pub struct CoordinatorBuilder {
    event_bus: Option<EventBus>,
    retry_manager: Option<RetryManager>,
    id_validator: Option<IdValidator>,
}

impl CoordinatorBuilder {
    pub fn new() -> Self {
        Self {
            event_bus: None,
            retry_manager: None,
            id_validator: None,
        }
    }

    pub fn with_events(mut self) -> Self {
        self.event_bus = Some(EventBus::new());
        self
    }

    pub fn with_event_handler(mut self, handler: Box<dyn EventHandler>) -> Self {
        if self.event_bus.is_none() {
            self.event_bus = Some(EventBus::new());
        }
        if let Some(bus) = &mut self.event_bus {
            bus.subscribe(handler);
        }
        self
    }

    pub fn with_retry_management(mut self) -> Self {
        self.retry_manager = Some(RetryManager::new());
        self
    }

    pub fn with_id_validation(mut self) -> Self {
        self.id_validator = Some(IdValidator::new());
        self
    }

    pub fn build(self) -> AdvancedCoordinator {
        AdvancedCoordinator {
            coordinator: Coordinator::new(),
            event_bus: self.event_bus,
            retry_manager: self.retry_manager,
            id_validator: self.id_validator,
        }
    }
}

impl Default for CoordinatorBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Advanced coordinator with optional features
pub struct AdvancedCoordinator {
    pub coordinator: Coordinator,
    pub event_bus: Option<EventBus>,
    pub retry_manager: Option<RetryManager>,
    pub id_validator: Option<IdValidator>,
}

impl AdvancedCoordinator {
    pub fn builder() -> CoordinatorBuilder {
        CoordinatorBuilder::new()
    }

    pub fn new() -> Self {
        Self {
            coordinator: Coordinator::new(),
            event_bus: None,
            retry_manager: None,
            id_validator: None,
        }
    }
}

impl Default for AdvancedCoordinator {
    fn default() -> Self {
        Self::new()
    }
}
