use crate::aggregator::StatAggregator;
use crate::snapshot::{IntoSnapshot, PlayerArtrineDecisionSnapshot};
use arlo_domain::ArtrineDecisionKind;
use arlo_events::MatchEvent;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub struct DecisionKindStats {
    pub total: u32,
    pub successful: u32,
    pub failed: u32,
    pub mirins_advanced: f64,
    pub points_generated: u32,
}

impl DecisionKindStats {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn total(&self) -> u32 {
        self.total
    }

    pub fn successful(&self) -> u32 {
        self.successful
    }

    pub fn failed(&self) -> u32 {
        self.failed
    }

    pub fn mirins_advanced(&self) -> f64 {
        self.mirins_advanced
    }

    pub fn points_generated(&self) -> u32 {
        self.points_generated
    }

    pub fn success_rate(&self) -> f64 {
        if self.total == 0 {
            0.0
        } else {
            (self.successful as f64) / (self.total as f64)
        }
    }

    pub fn average_mirins_advanced(&self) -> f64 {
        if self.total == 0 {
            0.0
        } else {
            self.mirins_advanced / (self.total as f64)
        }
    }

    pub fn average_points_generated(&self) -> f64 {
        if self.total == 0 {
            0.0
        } else {
            (self.points_generated as f64) / (self.total as f64)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlayerArtrineDecisionStats {
    pub player_id: Uuid,
    pub total_decisions: u32,
    pub total_successful_decisions: u32,
    pub total_failed_decisions: u32,
    pub total_mirins_advanced: f64,
    pub total_points_generated: u32,
    pub goal_points_generated: u32,
    pub field_points_generated: u32,
    pub field_goals_generated: u32,
    pub by_kind: HashMap<ArtrineDecisionKind, DecisionKindStats>,
}

impl PlayerArtrineDecisionStats {
    pub fn new(player_id: Uuid) -> Self {
        Self {
            player_id,
            total_decisions: 0,
            total_successful_decisions: 0,
            total_failed_decisions: 0,
            total_mirins_advanced: 0.0,
            total_points_generated: 0,
            goal_points_generated: 0,
            field_points_generated: 0,
            field_goals_generated: 0,
            by_kind: HashMap::new(),
        }
    }

    pub fn player_id(&self) -> Uuid {
        self.player_id
    }

    pub fn total_decisions(&self) -> u32 {
        self.total_decisions
    }

    pub fn total_successful_decisions(&self) -> u32 {
        self.total_successful_decisions
    }

    pub fn total_failed_decisions(&self) -> u32 {
        self.total_failed_decisions
    }

    pub fn total_mirins_advanced(&self) -> f64 {
        self.total_mirins_advanced
    }

    pub fn total_points_generated(&self) -> u32 {
        self.total_points_generated
    }

    pub fn goal_points_generated(&self) -> u32 {
        self.goal_points_generated
    }

    pub fn field_points_generated(&self) -> u32 {
        self.field_points_generated
    }

    pub fn field_goals_generated(&self) -> u32 {
        self.field_goals_generated
    }

    pub fn by_kind(&self) -> &HashMap<ArtrineDecisionKind, DecisionKindStats> {
        &self.by_kind
    }

    pub fn stats_for_kind(&self, kind: ArtrineDecisionKind) -> Option<&DecisionKindStats> {
        self.by_kind.get(&kind)
    }

    pub fn count_for_kind(&self, kind: ArtrineDecisionKind) -> u32 {
        self.by_kind.get(&kind).map(|s| s.total).unwrap_or(0)
    }

    pub fn success_rate(&self) -> f64 {
        if self.total_decisions == 0 {
            0.0
        } else {
            (self.total_successful_decisions as f64) / (self.total_decisions as f64)
        }
    }

    pub fn average_mirins_per_decision(&self) -> f64 {
        if self.total_decisions == 0 {
            0.0
        } else {
            self.total_mirins_advanced / (self.total_decisions as f64)
        }
    }

    pub fn average_points_per_decision(&self) -> f64 {
        if self.total_decisions == 0 {
            0.0
        } else {
            (self.total_points_generated as f64) / (self.total_decisions as f64)
        }
    }
}

impl IntoSnapshot for PlayerArtrineDecisionStats {
    type Snapshot = PlayerArtrineDecisionSnapshot;

    fn into_snapshot(&self) -> Self::Snapshot {
        PlayerArtrineDecisionSnapshot {
            player_id: self.player_id,
            total_decisions: self.total_decisions,
            total_successful_decisions: self.total_successful_decisions,
            total_failed_decisions: self.total_failed_decisions,
            total_mirins_advanced: self.total_mirins_advanced,
            total_points_generated: self.total_points_generated,
            goal_points_generated: self.goal_points_generated,
            field_points_generated: self.field_points_generated,
            field_goals_generated: self.field_goals_generated,
            success_rate: self.success_rate(),
            average_mirins_per_decision: self.average_mirins_per_decision(),
            average_points_per_decision: self.average_points_per_decision(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct PendingDecisionPlay {
    artrine_id: Uuid,
    decision_kind: ArtrineDecisionKind,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct PlayerArtrineDecisionAggregator {
    stats: HashMap<Uuid, PlayerArtrineDecisionStats>,
    current_play: Option<PendingDecisionPlay>,
    scoring_carrier_id: Option<Uuid>,
}

impl PlayerArtrineDecisionAggregator {
    pub fn new() -> Self {
        Self {
            stats: HashMap::new(),
            current_play: None,
            scoring_carrier_id: None,
        }
    }

    pub fn get(&self, player_id: &Uuid) -> Option<&PlayerArtrineDecisionStats> {
        self.stats.get(player_id)
    }

    pub fn get_or_default(&self, player_id: &Uuid) -> PlayerArtrineDecisionStats {
        self.stats
            .get(player_id)
            .cloned()
            .unwrap_or_else(|| PlayerArtrineDecisionStats::new(*player_id))
    }

    pub fn all_stats(&self) -> &HashMap<Uuid, PlayerArtrineDecisionStats> {
        &self.stats
    }

    fn get_mut_or_create(&mut self, player_id: Uuid) -> &mut PlayerArtrineDecisionStats {
        self.stats
            .entry(player_id)
            .or_insert_with(|| PlayerArtrineDecisionStats::new(player_id))
    }

    fn finalize_pending_play(&mut self) {
        if let Some(play) = self.current_play.take() {
            let stats = self.get_mut_or_create(play.artrine_id);
            let kind_stats = stats.by_kind.entry(play.decision_kind).or_default();
            stats.total_failed_decisions += 1;
            kind_stats.failed += 1;
        }
    }

    fn record_carry(&mut self, carrier_id: Uuid, gain_mirim: f64) {
        let Some(play) = self.current_play.take() else {
            if self.scoring_carrier_id != Some(carrier_id) {
                self.scoring_carrier_id = None;
            }
            return;
        };
        if play.artrine_id != carrier_id || play.decision_kind != ArtrineDecisionKind::SelfCarry {
            self.current_play = Some(play);
            self.scoring_carrier_id = None;
            return;
        }
        let advance = gain_mirim.max(0.0);
        let stats = self.get_mut_or_create(carrier_id);
        let kind_stats = stats.by_kind.entry(play.decision_kind).or_default();
        stats.total_mirins_advanced += advance;
        kind_stats.mirins_advanced += advance;
        if advance > 0.0 {
            stats.total_successful_decisions += 1;
            kind_stats.successful += 1;
            self.scoring_carrier_id = Some(carrier_id);
        } else {
            stats.total_failed_decisions += 1;
            kind_stats.failed += 1;
            self.scoring_carrier_id = None;
        }
    }

    fn record_score(&mut self, scorer_id: Uuid, points: u32, score_kind: ScoreKind) {
        let scorer_is_carrier = self.scoring_carrier_id.take() == Some(scorer_id);
        if !scorer_is_carrier {
            return;
        }
        let stats = self.get_mut_or_create(scorer_id);
        stats.total_points_generated += points;
        stats.by_kind
            .entry(ArtrineDecisionKind::SelfCarry)
            .or_default()
            .points_generated += points;
        match score_kind {
            ScoreKind::GoalPoint => stats.goal_points_generated += 1,
            ScoreKind::FieldPoint => stats.field_points_generated += 1,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum ScoreKind {
    GoalPoint,
    FieldPoint,
}

impl IntoSnapshot for PlayerArtrineDecisionAggregator {
    type Snapshot = HashMap<Uuid, PlayerArtrineDecisionSnapshot>;

    fn into_snapshot(&self) -> Self::Snapshot {
        self.stats
            .iter()
            .map(|(&id, stats)| (id, stats.into_snapshot()))
            .collect()
    }
}

impl StatAggregator for PlayerArtrineDecisionAggregator {
    fn handle_event(&mut self, event: &MatchEvent) {
        match event {
            MatchEvent::CallToActionStarted(_) => {
                self.finalize_pending_play();
                self.scoring_carrier_id = None;
            }
            MatchEvent::ArtrineDecisionMade(e) => {
                self.finalize_pending_play();
                self.scoring_carrier_id = None;

                let stats = self.get_mut_or_create(e.artrine_id());
                stats.total_decisions += 1;
                let kind_stats = stats.by_kind.entry(e.decision_kind()).or_default();
                kind_stats.total += 1;

                self.current_play = Some(PendingDecisionPlay {
                    artrine_id: e.artrine_id(),
                    decision_kind: e.decision_kind(),
                });
            }
            MatchEvent::CarryResolved(e) => self.record_carry(e.carrier_id(), e.gain_mirim()),
            MatchEvent::GoalPoint(e) => {
                self.record_score(e.scorer_id(), e.points(), ScoreKind::GoalPoint);
            }
            MatchEvent::FieldPoint(e) => {
                self.record_score(e.scorer_id(), e.points(), ScoreKind::FieldPoint);
            }
            MatchEvent::PassCompleted(_)
            | MatchEvent::DistributionCompleted(_)
            | MatchEvent::Turnover(_)
            | MatchEvent::OutOfBounds(_)
            | MatchEvent::ScoringAttemptMissed(_)
            | MatchEvent::TimeCallUsed(_)
            | MatchEvent::KickFoulAwarded(_)
            | MatchEvent::FieldGoal(_) => self.scoring_carrier_id = None,
            _ => {}
        }
    }

    fn reset(&mut self) {
        self.stats.clear();
        self.current_play = None;
        self.scoring_carrier_id = None;
    }
}
