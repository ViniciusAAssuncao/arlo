use crate::psychology::systems::events::ImpulseEvent;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DispatchedImpulseEvent {
    pub target_id: Uuid,
    pub event: ImpulseEvent,
}

impl DispatchedImpulseEvent {
    pub fn new(target_id: Uuid, event: ImpulseEvent) -> Self {
        Self { target_id, event }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ImpulseEventBus {
    events: Vec<DispatchedImpulseEvent>,
}

impl ImpulseEventBus {
    pub fn new() -> Self {
        Self { events: Vec::new() }
    }

    pub fn publish(&mut self, target_id: Uuid, event: ImpulseEvent) {
        self.events.push(DispatchedImpulseEvent::new(target_id, event));
    }

    pub fn publish_events(&mut self, events: impl IntoIterator<Item = DispatchedImpulseEvent>) {
        self.events.extend(events);
    }

    pub fn events(&self) -> &[DispatchedImpulseEvent] {
        &self.events
    }

    pub fn drain_events(&mut self) -> Vec<DispatchedImpulseEvent> {
        std::mem::take(&mut self.events)
    }

    pub fn clear(&mut self) {
        self.events.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }

    pub fn len(&self) -> usize {
        self.events.len()
    }
}
