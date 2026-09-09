use crate::aggregator::StatAggregator;
use crate::manager::ManagerDecisionAggregator;
use crate::player::{
    PlayerArtrineDecisionAggregator, PlayerAssistsAggregator, PlayerDrivesAggregator,
    PlayerDuelAggregator, PlayerImpulseAggregator, PlayerPhysicalAggregator,
    PlayerReceivingAggregator, PlayerScoringAttemptsAggregator, PlayerTouchesAggregator,
};
use crate::snapshot::{
    IntoSnapshot, PeriodicMatchSnapshot, PlayerMatchSnapshot, TeamMatchSnapshot,
};
use crate::team::TeamPossessionAggregator;
use arlo_events::{MatchClockInstant, MatchEvent, MatchEventEnvelope};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

#[derive(Default)]
pub struct AggregatorRegistry {
    aggregators: Vec<Box<dyn StatAggregator>>,
}

impl AggregatorRegistry {
    pub fn new() -> Self {
        Self {
            aggregators: Vec::new(),
        }
    }

    pub fn with_default_aggregators() -> Self {
        let mut registry = Self::new();
        registry.register_aggregator(PlayerArtrineDecisionAggregator::new());
        registry.register_aggregator(PlayerDrivesAggregator::new());
        registry.register_aggregator(PlayerDuelAggregator::new());
        registry.register_aggregator(PlayerReceivingAggregator::new());
        registry.register_aggregator(PlayerTouchesAggregator::new());
        registry.register_aggregator(PlayerScoringAttemptsAggregator::new());
        registry.register_aggregator(PlayerPhysicalAggregator::new());
        registry.register_aggregator(PlayerImpulseAggregator::new());
        registry.register_aggregator(PlayerAssistsAggregator::new());
        registry.register_aggregator(TeamPossessionAggregator::new());
        registry.register_aggregator(ManagerDecisionAggregator::new());
        registry
    }

    pub fn register(&mut self, aggregator: Box<dyn StatAggregator>) {
        self.aggregators.push(aggregator);
    }

    pub fn register_aggregator<T: StatAggregator + 'static>(&mut self, aggregator: T) {
        self.aggregators.push(Box::new(aggregator));
    }

    pub fn get<T: StatAggregator + 'static>(&self) -> Option<&T> {
        for agg in &self.aggregators {
            if let Some(downcasted) = agg.as_any().downcast_ref::<T>() {
                return Some(downcasted);
            }
        }
        None
    }

    pub fn get_mut<T: StatAggregator + 'static>(&mut self) -> Option<&mut T> {
        for agg in &mut self.aggregators {
            if let Some(downcasted) = agg.as_any_mut().downcast_mut::<T>() {
                return Some(downcasted);
            }
        }
        None
    }

    pub fn handle_envelope(&mut self, envelope: &MatchEventEnvelope) {
        for aggregator in &mut self.aggregators {
            aggregator.handle_envelope(envelope);
        }
    }

    pub fn handle_event(&mut self, event: &MatchEvent) {
        for aggregator in &mut self.aggregators {
            aggregator.handle_event(event);
        }
    }

    pub fn handle_envelopes<'a>(
        &mut self,
        envelopes: impl IntoIterator<Item = &'a MatchEventEnvelope>,
    ) {
        for envelope in envelopes {
            self.handle_envelope(envelope);
        }
    }

    pub fn reset_all(&mut self) {
        for aggregator in &mut self.aggregators {
            aggregator.reset();
        }
    }

    pub fn aggregators(&self) -> &[Box<dyn StatAggregator>] {
        &self.aggregators
    }

    pub fn len(&self) -> usize {
        self.aggregators.len()
    }

    pub fn is_empty(&self) -> bool {
        self.aggregators.is_empty()
    }

    pub fn all_player_ids(&self) -> HashSet<Uuid> {
        let mut ids = HashSet::new();
        if let Some(agg) = self.get::<PlayerTouchesAggregator>() {
            ids.extend(agg.all_stats().keys().copied());
        }
        if let Some(agg) = self.get::<PlayerDuelAggregator>() {
            ids.extend(agg.all_stats().keys().copied());
        }
        if let Some(agg) = self.get::<PlayerDrivesAggregator>() {
            ids.extend(agg.all_stats().keys().copied());
        }
        if let Some(agg) = self.get::<PlayerReceivingAggregator>() {
            ids.extend(agg.all_stats().keys().copied());
        }
        if let Some(agg) = self.get::<PlayerScoringAttemptsAggregator>() {
            ids.extend(agg.all_stats().keys().copied());
        }
        if let Some(agg) = self.get::<PlayerArtrineDecisionAggregator>() {
            ids.extend(agg.all_stats().keys().copied());
        }
        if let Some(agg) = self.get::<PlayerPhysicalAggregator>() {
            ids.extend(agg.all_stats().keys().copied());
        }
        if let Some(agg) = self.get::<PlayerImpulseAggregator>() {
            ids.extend(agg.all_player_stats().keys().copied());
        }
        if let Some(agg) = self.get::<PlayerAssistsAggregator>() {
            ids.extend(agg.all_stats().keys().copied());
        }
        ids
    }

    pub fn all_team_ids(&self) -> HashSet<Uuid> {
        let mut ids = HashSet::new();
        if let Some(agg) = self.get::<TeamPossessionAggregator>() {
            ids.extend(agg.all_stats().keys().copied());
        }
        if let Some(agg) = self.get::<PlayerImpulseAggregator>() {
            ids.extend(agg.all_team_stats().keys().copied());
        }
        if let Some(agg) = self.get::<ManagerDecisionAggregator>() {
            ids.extend(agg.all_logs().keys().copied());
        }
        ids
    }

    pub fn create_team_snapshot(&self, team_id: &Uuid) -> TeamMatchSnapshot {
        let mut snap = TeamMatchSnapshot::new(*team_id, 0.0);

        if let Some(agg) = self.get::<TeamPossessionAggregator>() {
            let p = agg.get_or_default(team_id);
            snap.total_possession_seconds = p.total_possession_seconds;
        }

        snap
    }

    pub fn create_player_snapshot(&self, player_id: &Uuid) -> PlayerMatchSnapshot {
        let mut snap = PlayerMatchSnapshot::new(*player_id);

        if let Some(agg) = self.get::<PlayerTouchesAggregator>() {
            let t = agg.get_or_default(player_id);
            snap.total_touches = t.total_touches;
            snap.passes_attempted = t.passes_attempted;
            snap.passes_received = t.passes_received;
            snap.recoveries = t.recoveries;
            snap.turnovers_conceded = t.turnovers_conceded;
        }

        if let Some(agg) = self.get::<PlayerDrivesAggregator>() {
            let d = agg.get_or_default(player_id);
            snap.total_drives = d.total_drives;
            snap.central_drives = d.central_drives;
            snap.left_lateral_drives = d.left_lateral_drives;
            snap.right_lateral_drives = d.right_lateral_drives;
            snap.lateral_drives = d.lateral_drives();
            snap.max_drives_in_series = d.max_drives_in_series;
        }

        if let Some(agg) = self.get::<PlayerDuelAggregator>() {
            let du = agg.get_or_default(player_id);
            snap.total_duels = du.total_duels;
            snap.total_duel_wins = du.total_wins;
            snap.total_duel_losses = du.total_losses;
            snap.duel_win_rate = du.win_rate();
            snap.attacker_duels = du.attacker_duels;
            snap.attacker_duel_wins = du.attacker_wins;
            snap.attacker_duel_losses = du.attacker_losses;
            snap.attacker_duel_win_rate = du.attacker_win_rate();
            snap.defender_duels = du.defender_duels;
            snap.defender_duel_wins = du.defender_wins;
            snap.defender_duel_losses = du.defender_losses;
            snap.defender_duel_win_rate = du.defender_win_rate();
        }

        if let Some(agg) = self.get::<PlayerReceivingAggregator>() {
            let r = agg.get_or_default(player_id);
            snap.targets = r.targets;
            snap.receptions = r.receptions;
            snap.drops = r.drops;
            snap.catch_rate = r.catch_rate();
            snap.drop_rate = r.drop_rate();
            snap.receiving_mirins = r.receiving_mirins;
            snap.run_after_catch_mirins = r.run_after_catch_mirins;
            snap.longest_reception_mirim = r.longest_reception_mirim;
            snap.average_mirins_per_reception = r.average_mirins_per_reception();
        }

        if let Some(agg) = self.get::<PlayerScoringAttemptsAggregator>() {
            let sc = agg.get_or_default(player_id);
            snap.scoring_attempts = sc.attempts;
            snap.scoring_conversions = sc.converted;
            snap.scoring_misses = sc.missed;
            snap.scoring_conversion_rate = sc.conversion_rate();
            snap.goal_points_scored = sc.goal_points_scored;
            snap.field_points_scored = sc.field_points_scored;
            snap.field_goals_scored = sc.field_goals_scored;
            snap.total_points_scored = sc.total_points_scored;
        }

        if let Some(agg) = self.get::<PlayerAssistsAggregator>() {
            let a = agg.get_or_default(player_id);
            snap.goalpoint_assists = a.goalpoint_assists;
        }

        if let Some(agg) = self.get::<PlayerArtrineDecisionAggregator>() {
            let ad = agg.get_or_default(player_id);
            snap.artrine_decisions_total = ad.total_decisions;
            snap.artrine_decisions_successful = ad.total_successful_decisions;
            snap.artrine_decisions_failed = ad.total_failed_decisions;
            snap.artrine_success_rate = ad.success_rate();
            snap.artrine_mirins_advanced = ad.total_mirins_advanced;
            snap.artrine_points_generated = ad.total_points_generated;
            snap.artrine_goal_points_generated = ad.goal_points_generated;
            snap.artrine_field_points_generated = ad.field_points_generated;
            snap.artrine_field_goals_generated = ad.field_goals_generated;
        }

        if let Some(agg) = self.get::<PlayerPhysicalAggregator>() {
            let p = agg.get_or_default(player_id);
            snap.end_energy_level = p.end_energy_level;
            snap.peak_anaerobic_depletion = p.peak_anaerobic_depletion;
            snap.total_distance_covered = p.total_distance_covered;
            snap.high_intensity_distance = p.high_intensity_distance;
            snap.low_intensity_distance = p.low_intensity_distance;
            snap.metabolic_energy_joules = p.metabolic_energy_joules;
            snap.peak_speed_meters_per_sec = p.peak_speed_meters_per_sec;
            snap.intra_match_recovery_amount = p.intra_match_recovery_amount;
            snap.distance_first_zone = p.distance_first_zone;
            snap.distance_second_zone = p.distance_second_zone;
            snap.distance_corridors = p.distance_corridors;
            snap.distance_central = p.distance_central;
        }

        if let Some(agg) = self.get::<PlayerImpulseAggregator>() {
            let imp = agg.get_or_default(player_id);
            snap.impulse_baseline = imp.baseline;
            snap.impulse_current = imp.current_value;
            snap.impulse_min = imp.min_value;
            snap.impulse_max = imp.max_value;
            snap.impulse_average = imp.average_value();
            snap.impulse_shifts_total = imp.shifts_count;
            snap.impulse_time_below_baseline_seconds = imp.time_below_baseline_seconds;
            snap.impulse_runs_count = imp.runs_count();
            snap.impulse_longest_run_seconds = imp.longest_run_duration_seconds();
            snap.impulse_peak_run_value = imp.peak_run_value();
            snap.impulse_total_run_intensity = imp.total_integrated_run_intensity();
        }

        snap
    }

    pub fn create_all_player_snapshots(&self) -> HashMap<Uuid, PlayerMatchSnapshot> {
        let ids = self.all_player_ids();
        ids.into_iter()
            .map(|id| (id, self.create_player_snapshot(&id)))
            .collect()
    }

    pub fn create_all_team_snapshots(&self) -> HashMap<Uuid, TeamMatchSnapshot> {
        let ids = self.all_team_ids();
        ids.into_iter()
            .map(|id| (id, self.create_team_snapshot(&id)))
            .collect()
    }

    pub fn player_snapshots_vec(&self) -> Vec<PlayerMatchSnapshot> {
        let mut ids: Vec<Uuid> = self.all_player_ids().into_iter().collect();
        ids.sort();
        ids.into_iter()
            .map(|id| self.create_player_snapshot(&id))
            .collect()
    }

    pub fn team_snapshots_vec(&self) -> Vec<TeamMatchSnapshot> {
        let mut ids: Vec<Uuid> = self.all_team_ids().into_iter().collect();
        ids.sort();
        ids.into_iter()
            .map(|id| self.create_team_snapshot(&id))
            .collect()
    }

    pub fn capture_periodic_snapshot(
        &self,
        sequence_number: u64,
        clock: MatchClockInstant,
    ) -> PeriodicMatchSnapshot {
        PeriodicMatchSnapshot::new(
            sequence_number,
            clock,
            self.player_snapshots_vec(),
            self.team_snapshots_vec(),
        )
    }
}

impl IntoSnapshot for AggregatorRegistry {
    type Snapshot = Vec<PlayerMatchSnapshot>;

    fn into_snapshot(&self) -> Self::Snapshot {
        self.player_snapshots_vec()
    }
}