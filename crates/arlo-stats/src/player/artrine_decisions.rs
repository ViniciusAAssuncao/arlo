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
    down_number: u32,
    all_duels_won: bool,
    had_duel: bool,
    turnover: bool,
    points_in_play: u32,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct PlayerArtrineDecisionAggregator {
    stats: HashMap<Uuid, PlayerArtrineDecisionStats>,
    current_play: Option<PendingDecisionPlay>,
}

impl PlayerArtrineDecisionAggregator {
    pub fn new() -> Self {
        Self {
            stats: HashMap::new(),
            current_play: None,
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
            let successful = !play.turnover
                && (play.points_in_play > 0 || (play.had_duel && play.all_duels_won));
            let stats = self.get_mut_or_create(play.artrine_id);
            let kind_stats = stats.by_kind.entry(play.decision_kind).or_default();
            if successful {
                stats.total_successful_decisions += 1;
                kind_stats.successful += 1;
            } else {
                stats.total_failed_decisions += 1;
                kind_stats.failed += 1;
            }
        }
    }
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
            }
            MatchEvent::ArtrineDecisionMade(e) => {
                self.finalize_pending_play();

                let stats = self.get_mut_or_create(e.artrine_id());
                stats.total_decisions += 1;
                let kind_stats = stats.by_kind.entry(e.decision_kind()).or_default();
                kind_stats.total += 1;

                self.current_play = Some(PendingDecisionPlay {
                    artrine_id: e.artrine_id(),
                    decision_kind: e.decision_kind(),
                    down_number: e.down_number(),
                    all_duels_won: true,
                    had_duel: false,
                    turnover: false,
                    points_in_play: 0,
                });
            }
            MatchEvent::DuelResolved(e) => {
                if let Some(play) = &mut self.current_play {
                    play.had_duel = true;
                    if !e.attacker_won() {
                        play.all_duels_won = false;
                    }
                }
            }
            MatchEvent::GoalPoint(e) => {
                let maybe_info = if let Some(play) = &mut self.current_play {
                    play.points_in_play += e.points();
                    Some((play.artrine_id, play.decision_kind))
                } else {
                    None
                };
                if let Some((artrine_id, decision_kind)) = maybe_info {
                    let stats = self.get_mut_or_create(artrine_id);
                    stats.goal_points_generated += 1;
                    stats.total_points_generated += e.points();
                    let kind_stats = stats.by_kind.entry(decision_kind).or_default();
                    kind_stats.points_generated += e.points();
                }
            }
            MatchEvent::FieldPoint(e) => {
                let maybe_info = if let Some(play) = &mut self.current_play {
                    play.points_in_play += e.points();
                    Some((play.artrine_id, play.decision_kind))
                } else {
                    None
                };
                if let Some((artrine_id, decision_kind)) = maybe_info {
                    let stats = self.get_mut_or_create(artrine_id);
                    stats.field_points_generated += 1;
                    stats.total_points_generated += e.points();
                    let kind_stats = stats.by_kind.entry(decision_kind).or_default();
                    kind_stats.points_generated += e.points();
                }
            }
            MatchEvent::FieldGoal(e) => {
                let maybe_info = if let Some(play) = &mut self.current_play {
                    play.points_in_play += e.points();
                    Some((play.artrine_id, play.decision_kind))
                } else {
                    None
                };
                if let Some((artrine_id, decision_kind)) = maybe_info {
                    let stats = self.get_mut_or_create(artrine_id);
                    stats.field_goals_generated += 1;
                    stats.total_points_generated += e.points();
                    let kind_stats = stats.by_kind.entry(decision_kind).or_default();
                    kind_stats.points_generated += e.points();
                }
            }
            MatchEvent::Turnover(_) => {
                if let Some(play) = &mut self.current_play {
                    play.turnover = true;
                }
            }
            MatchEvent::DownAdvanced(e) => {
                if let Some(play) = self.current_play.take() {
                    let mirins = e.mirins_advanced_this_down();
                    let successful = !play.turnover
                        && (play.points_in_play > 0
                            || (play.all_duels_won && (mirins > 0.0 || e.first_down_achieved())));

                    let stats = self.get_mut_or_create(play.artrine_id);
                    stats.total_mirins_advanced += mirins;
                    let kind_stats = stats.by_kind.entry(play.decision_kind).or_default();
                    kind_stats.mirins_advanced += mirins;

                    if successful {
                        stats.total_successful_decisions += 1;
                        kind_stats.successful += 1;
                    } else {
                        stats.total_failed_decisions += 1;
                        kind_stats.failed += 1;
                    }
                }
            }
            _ => {}
        }
    }

    fn reset(&mut self) {
        self.stats.clear();
        self.current_play = None;
    }
}
