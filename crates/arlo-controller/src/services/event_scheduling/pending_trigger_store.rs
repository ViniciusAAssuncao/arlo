use crate::domain::calendar::{CalendarDate, CalendarSystem};
use crate::domain::event_scheduling::PendingTrigger;
use crate::error::ControllerResult;
use crate::services::event_scheduling::trigger_rehydration::rehydrate_all_triggers;
use sqlx::SqlitePool;
use std::collections::BTreeMap;
use tokio::sync::RwLock;

#[derive(Debug, Default)]
pub struct PendingTriggerStore {
    triggers: RwLock<BTreeMap<(i64, u32), Vec<PendingTrigger>>>,
}

impl PendingTriggerStore {
    pub fn new() -> Self {
        Self {
            triggers: RwLock::new(BTreeMap::new()),
        }
    }

    pub fn from_index(index: BTreeMap<(i64, u32), Vec<PendingTrigger>>) -> Self {
        Self {
            triggers: RwLock::new(index),
        }
    }

    pub async fn rebuild(
        &self,
        pool: &SqlitePool,
        calendar: &CalendarSystem,
        reference_year: i64,
    ) -> ControllerResult<()> {
        let new_index = rehydrate_all_triggers(pool, calendar, reference_year).await?;
        let mut guard = self.triggers.write().await;
        *guard = new_index;
        Ok(())
    }

    pub async fn insert(&self, trigger: PendingTrigger) {
        let date = trigger.trigger_date();
        let key = (date.year(), date.day_of_year());
        let mut guard = self.triggers.write().await;
        guard.entry(key).or_default().push(trigger);
    }

    pub async fn pop_due(&self, current_date: &CalendarDate) -> Vec<PendingTrigger> {
        let current_key = (current_date.year(), current_date.day_of_year());
        let mut guard = self.triggers.write().await;

        let due_keys: Vec<(i64, u32)> = guard
            .range(..=current_key)
            .map(|(k, _)| *k)
            .collect();

        let mut due_triggers = Vec::new();
        for key in due_keys {
            if let Some(triggers) = guard.remove(&key) {
                due_triggers.extend(triggers);
            }
        }

        due_triggers
    }

    pub async fn len(&self) -> usize {
        let guard = self.triggers.read().await;
        guard.values().map(|v| v.len()).sum()
    }

    pub async fn is_empty(&self) -> bool {
        let guard = self.triggers.read().await;
        guard.is_empty()
    }
}