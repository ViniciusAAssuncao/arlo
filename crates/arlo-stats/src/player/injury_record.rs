use crate::aggregator::StatAggregator;
use crate::officiating::keyed_registry::{KeyedStat, KeyedStatRegistry};
use crate::snapshot::{IntoSnapshot, PlayerInjurySnapshot};
use arlo_domain::{BodyRegion, InjuryMechanism, InjurySeverityGrade};
use arlo_events::MatchEvent;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct PlayerInjuryStats {
    pub player_id: Uuid,
    pub total_injuries: u32,
    pub contact_injuries: u32,
    pub non_contact_injuries: u32,
    pub grade_1_injuries: u32,
    pub grade_2_injuries: u32,
    pub grade_3_injuries: u32,
    pub by_mechanism: HashMap<InjuryMechanism, u32>,
    pub by_body_region: HashMap<BodyRegion, u32>,
    pub by_severity_grade: HashMap<InjurySeverityGrade, u32>,
}

impl PlayerInjuryStats {
    pub fn new(player_id: Uuid) -> Self {
        Self {
            player_id,
            total_injuries: 0,
            contact_injuries: 0,
            non_contact_injuries: 0,
            grade_1_injuries: 0,
            grade_2_injuries: 0,
            grade_3_injuries: 0,
            by_mechanism: HashMap::new(),
            by_body_region: HashMap::new(),
            by_severity_grade: HashMap::new(),
        }
    }

    pub fn player_id(&self) -> Uuid {
        self.player_id
    }

    pub fn total_injuries(&self) -> u32 {
        self.total_injuries
    }

    pub fn contact_injuries(&self) -> u32 {
        self.contact_injuries
    }

    pub fn non_contact_injuries(&self) -> u32 {
        self.non_contact_injuries
    }

    pub fn grade_1_injuries(&self) -> u32 {
        self.grade_1_injuries
    }

    pub fn grade_2_injuries(&self) -> u32 {
        self.grade_2_injuries
    }

    pub fn grade_3_injuries(&self) -> u32 {
        self.grade_3_injuries
    }

    pub fn by_mechanism(&self) -> &HashMap<InjuryMechanism, u32> {
        &self.by_mechanism
    }

    pub fn by_body_region(&self) -> &HashMap<BodyRegion, u32> {
        &self.by_body_region
    }

    pub fn by_severity_grade(&self) -> &HashMap<InjurySeverityGrade, u32> {
        &self.by_severity_grade
    }

    pub fn count_for_mechanism(&self, mechanism: InjuryMechanism) -> u32 {
        self.by_mechanism.get(&mechanism).copied().unwrap_or(0)
    }

    pub fn count_for_body_region(&self, body_region: BodyRegion) -> u32 {
        self.by_body_region.get(&body_region).copied().unwrap_or(0)
    }

    pub fn count_for_severity_grade(&self, grade: InjurySeverityGrade) -> u32 {
        self.by_severity_grade.get(&grade).copied().unwrap_or(0)
    }

    pub fn record_injury(
        &mut self,
        mechanism: InjuryMechanism,
        body_region: BodyRegion,
        grade: InjurySeverityGrade,
    ) {
        self.total_injuries += 1;
        match mechanism {
            InjuryMechanism::Contact => self.contact_injuries += 1,
            InjuryMechanism::NonContact => self.non_contact_injuries += 1,
        }
        match grade {
            InjurySeverityGrade::Grade1 => self.grade_1_injuries += 1,
            InjurySeverityGrade::Grade2 => self.grade_2_injuries += 1,
            InjurySeverityGrade::Grade3 => self.grade_3_injuries += 1,
        }
        *self.by_mechanism.entry(mechanism).or_insert(0) += 1;
        *self.by_body_region.entry(body_region).or_insert(0) += 1;
        *self.by_severity_grade.entry(grade).or_insert(0) += 1;
    }
}

impl KeyedStat for PlayerInjuryStats {
    fn new_for(player_id: Uuid) -> Self {
        Self::new(player_id)
    }
}

impl IntoSnapshot for PlayerInjuryStats {
    type Snapshot = PlayerInjurySnapshot;

    fn into_snapshot(&self) -> Self::Snapshot {
        PlayerInjurySnapshot {
            player_id: self.player_id,
            total_injuries: self.total_injuries,
            contact_injuries: self.contact_injuries,
            non_contact_injuries: self.non_contact_injuries,
            grade_1_injuries: self.grade_1_injuries,
            grade_2_injuries: self.grade_2_injuries,
            grade_3_injuries: self.grade_3_injuries,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct PlayerInjuryAggregator {
    registry: KeyedStatRegistry<PlayerInjuryStats>,
}

impl PlayerInjuryAggregator {
    pub fn new() -> Self {
        Self {
            registry: KeyedStatRegistry::new(),
        }
    }

    pub fn get(&self, player_id: &Uuid) -> Option<&PlayerInjuryStats> {
        self.registry.get(player_id)
    }

    pub fn get_or_default(&self, player_id: &Uuid) -> PlayerInjuryStats {
        self.registry.get_or_default(player_id)
    }

    pub fn all_stats(&self) -> &HashMap<Uuid, PlayerInjuryStats> {
        self.registry.all_stats()
    }

    pub fn record_injury(
        &mut self,
        player_id: Uuid,
        mechanism: InjuryMechanism,
        body_region: BodyRegion,
        grade: InjurySeverityGrade,
    ) {
        let stats = self.registry.entry_or_default(player_id);
        stats.record_injury(mechanism, body_region, grade);
    }
}

impl IntoSnapshot for PlayerInjuryAggregator {
    type Snapshot = HashMap<Uuid, PlayerInjurySnapshot>;

    fn into_snapshot(&self) -> Self::Snapshot {
        self.registry
            .all_stats()
            .iter()
            .map(|(&id, stats)| (id, stats.into_snapshot()))
            .collect()
    }
}

impl StatAggregator for PlayerInjuryAggregator {
    fn handle_event(&mut self, event: &MatchEvent) {
        if let MatchEvent::InjuryIncidentRecorded(e) = event {
            self.record_injury(
                e.player_id(),
                e.mechanism(),
                e.body_region(),
                e.severity_grade(),
            );
        }
    }

    fn reset(&mut self) {
        self.registry.clear();
    }
}