use crate::attributes::{
    AttributeKeyIndex, ManagerAttributeTable, PlayerAttributeTable, RefereeAttributeTable,
};
use crate::compatibility::score::{ScoreState, TeamScore};
use crate::compatibility::seed_view::{MatchSeed, RngProvider};
use crate::compatibility::setup::MatchSetupParams;
use crate::error::{EngineError, EngineResult};
use crate::physical::state::PhysicalState;
use crate::psychology::state::ImpulseState;
use crate::simulation::state::availability::PlayerAvailabilityTracker;
use crate::simulation::state::clock::MatchClock;
use crate::simulation::state::possession_state::PossessionState;
use crate::simulation::state::series_state::SeriesState;
use arlo_domain::{
    FaultCatalog, Formation, InjuryCatalog, Manager, ManagerControlMode, MatchFormatRules, Pitch,
    Player, Referee, SlotRole,
};
use arlo_tactics::{PlayCall, TacticalLineup, TeamTacticalProfile};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

pub use crate::simulation::state::availability::AvailabilityState;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LineupAssignmentView {
    player: Player,
    slot_role: Option<SlotRole>,
}

impl LineupAssignmentView {
    pub fn new(player: Player, slot_role: Option<SlotRole>) -> Self {
        Self { player, slot_role }
    }

    pub fn player(&self) -> &Player {
        &self.player
    }

    pub fn slot_role(&self) -> Option<SlotRole> {
        self.slot_role
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LineupView {
    team_id: Uuid,
    assignments: Vec<LineupAssignmentView>,
}

impl LineupView {
    pub fn new(team_id: Uuid, assignments: Vec<LineupAssignmentView>) -> Self {
        Self { team_id, assignments }
    }

    pub fn assignments(&self) -> &[LineupAssignmentView] {
        &self.assignments
    }

    pub fn players(&self) -> Vec<&Player> {
        self.assignments.iter().map(|a| &a.player).collect()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MatchdaySquadView {
    team_id: Uuid,
    starters: Vec<Player>,
    bench: Vec<Player>,
}

impl MatchdaySquadView {
    pub fn new(team_id: Uuid, starters: Vec<Player>, bench: Vec<Player>) -> Self {
        Self {
            team_id,
            starters,
            bench,
        }
    }

    pub fn starters(&self) -> &[Player] {
        &self.starters
    }

    pub fn bench(&self) -> &[Player] {
        &self.bench
    }

    pub fn all_players(&self) -> Vec<&Player> {
        self.starters.iter().chain(self.bench.iter()).collect()
    }

    pub fn available_replacements(&self) -> impl Iterator<Item = &Player> {
        self.bench.iter()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TeamRuntimeData {
    pub team_id: Uuid,
    pub manager: Manager,
    pub manager_table: ManagerAttributeTable,
    pub formation: Formation,
    pub tactical_lineup: TacticalLineup,
    pub tactical_profile: TeamTacticalProfile,
    pub available_profiles: Vec<TeamTacticalProfile>,
    pub playbook: Vec<PlayCall>,
    pub squad: MatchdaySquadView,
    pub lineup_view: LineupView,
    pub player_tables: HashMap<Uuid, PlayerAttributeTable>,
}

#[derive(Debug, Clone)]
pub struct MatchState {
    home_team_id: Uuid,
    away_team_id: Uuid,
    home_data: TeamRuntimeData,
    away_data: TeamRuntimeData,
    head_referee: Referee,
    peace_referee: Referee,
    head_referee_table: RefereeAttributeTable,
    peace_referee_table: RefereeAttributeTable,
    pitch: Pitch,
    format_rules: MatchFormatRules,
    fault_catalog: Arc<FaultCatalog>,
    injury_catalog: Arc<InjuryCatalog>,
    seed: MatchSeed,
    rng_provider: RngProvider,
    clock: MatchClock,
    possession: PossessionState,
    series: SeriesState,
    score: ScoreState,
    availability: PlayerAvailabilityTracker,
    player_fatigue: HashMap<Uuid, PhysicalState>,
    player_impulse: HashMap<Uuid, ImpulseState>,
    event_sequence: u64,
    last_action_score: bool,
    last_scoring_team: Option<Uuid>,
}

impl MatchState {
    pub fn new(params: MatchSetupParams) -> EngineResult<Self> {
        let key_index = AttributeKeyIndex::from_map(&params.attribute_keys);

        let build_team_data = |setup: &crate::compatibility::setup::TeamSetupParams| -> EngineResult<TeamRuntimeData> {
            let manager_table = ManagerAttributeTable::from_manager_with_index(&setup.manager, &key_index);

            let mut player_tables = HashMap::with_capacity(setup.players.len());
            for p in &setup.players {
                player_tables.insert(p.id(), PlayerAttributeTable::from_player_with_index(p, &key_index));
            }

            let starter_ids: Vec<Uuid> = setup.lineup.assignments().iter().map(|a| a.player_id()).collect();
            let mut starters = Vec::with_capacity(starter_ids.len());
            let mut bench = Vec::new();

            for p in &setup.players {
                if starter_ids.contains(&p.id()) {
                    starters.push(p.clone());
                } else {
                    bench.push(p.clone());
                }
            }

            let mut assignment_views = Vec::with_capacity(setup.lineup.assignments().len());
            for assignment in setup.lineup.assignments() {
                if let Some(player) = setup.players.iter().find(|p| p.id() == assignment.player_id()).cloned() {
                    assignment_views.push(LineupAssignmentView::new(player, Some(assignment.slot_role())));
                }
            }

            let squad = MatchdaySquadView::new(setup.team_id, starters, bench);
            let lineup_view = LineupView::new(setup.team_id, assignment_views);

            Ok(TeamRuntimeData {
                team_id: setup.team_id,
                manager: setup.manager.clone(),
                manager_table,
                formation: setup.formation.clone(),
                tactical_lineup: setup.lineup.clone(),
                tactical_profile: setup.profile.clone(),
                available_profiles: setup.available_profiles.clone(),
                playbook: setup.playbook.clone(),
                squad,
                lineup_view,
                player_tables,
            })
        };

        let home_data = build_team_data(&params.home_team)?;
        let away_data = build_team_data(&params.away_team)?;

        let head_referee_table = RefereeAttributeTable::from_referee_with_index(&params.head_referee, &key_index);
        let peace_referee_table = RefereeAttributeTable::from_referee_with_index(&params.peace_referee, &key_index);

        let rng_provider = RngProvider::new(params.seed);
        let clock = MatchClock::new(&params.format_rules);
        let possession = PossessionState::new(params.home_team.team_id, params.away_team.team_id);
        let series = SeriesState::default_ruleset();
        let score = ScoreState::new();
        let availability = PlayerAvailabilityTracker::new();

        let mut player_fatigue = HashMap::new();
        let mut player_impulse = HashMap::new();

        for p in params.home_team.players.iter().chain(params.away_team.players.iter()) {
            player_fatigue.insert(p.id(), PhysicalState::initial());
            player_impulse.insert(p.id(), ImpulseState::initial());
        }

        Ok(Self {
            home_team_id: params.home_team.team_id,
            away_team_id: params.away_team.team_id,
            home_data,
            away_data,
            head_referee: params.head_referee,
            peace_referee: params.peace_referee,
            head_referee_table,
            peace_referee_table,
            pitch: params.pitch,
            format_rules: params.format_rules,
            fault_catalog: params.fault_catalog,
            injury_catalog: params.injury_catalog,
            seed: params.seed,
            rng_provider,
            clock,
            possession,
            series,
            score,
            availability,
            player_fatigue,
            player_impulse,
            event_sequence: 0,
            last_action_score: false,
            last_scoring_team: None,
        })
    }

    pub fn home_team_id(&self) -> Uuid {
        self.home_team_id
    }

    pub fn away_team_id(&self) -> Uuid {
        self.away_team_id
    }

    pub fn home_manager(&self) -> &Manager {
        &self.home_data.manager
    }

    pub fn away_manager(&self) -> &Manager {
        &self.away_data.manager
    }

    pub fn manager_for_team(&self, team_id: Uuid) -> &Manager {
        if team_id == self.home_team_id {
            &self.home_data.manager
        } else {
            &self.away_data.manager
        }
    }

    pub fn manager_attribute_table_for(&self, team_id: Uuid) -> &ManagerAttributeTable {
        if team_id == self.home_team_id {
            &self.home_data.manager_table
        } else {
            &self.away_data.manager_table
        }
    }

    pub fn home_lineup(&self) -> &LineupView {
        &self.home_data.lineup_view
    }

    pub fn away_lineup(&self) -> &LineupView {
        &self.away_data.lineup_view
    }

    pub fn home_squad(&self) -> &MatchdaySquadView {
        &self.home_data.squad
    }

    pub fn away_squad(&self) -> &MatchdaySquadView {
        &self.away_data.squad
    }

    pub fn squad_for_team(&self, team_id: Uuid) -> &MatchdaySquadView {
        if team_id == self.home_team_id {
            &self.home_data.squad
        } else {
            &self.away_data.squad
        }
    }

    pub fn pitch(&self) -> &Pitch {
        &self.pitch
    }

    pub fn clock(&self) -> &MatchClock {
        &self.clock
    }

    pub fn clock_mut(&mut self) -> &mut MatchClock {
        &mut self.clock
    }

    pub fn possession(&self) -> &PossessionState {
        &self.possession
    }

    pub fn possession_mut(&mut self) -> &mut PossessionState {
        &mut self.possession
    }

    pub fn series(&self) -> &SeriesState {
        &self.series
    }

    pub fn series_mut(&mut self) -> &mut SeriesState {
        &mut self.series
    }

    pub fn score(&self) -> &ScoreState {
        &self.score
    }

    pub fn score_mut(&mut self) -> &mut ScoreState {
        &mut self.score
    }

    pub fn availability(&self) -> &PlayerAvailabilityTracker {
        &self.availability
    }

    pub fn availability_mut(&mut self) -> &mut PlayerAvailabilityTracker {
        &mut self.availability
    }

    pub fn head_referee(&self) -> &Referee {
        &self.head_referee
    }

    pub fn peace_referee(&self) -> &Referee {
        &self.peace_referee
    }

    pub fn head_referee_table(&self) -> &RefereeAttributeTable {
        &self.head_referee_table
    }

    pub fn peace_referee_table(&self) -> &RefereeAttributeTable {
        &self.peace_referee_table
    }

    pub fn format_rules(&self) -> &MatchFormatRules {
        &self.format_rules
    }

    pub fn rng_provider(&self) -> &RngProvider {
        &self.rng_provider
    }

    pub fn home_score(&self) -> TeamScore {
        *self.score.home()
    }

    pub fn away_score(&self) -> TeamScore {
        *self.score.away()
    }

    pub fn is_match_finished(&self) -> bool {
        self.clock.is_finished()
    }

    pub fn role_index_for_team(&self, team_id: Uuid) -> HashMap<Uuid, SlotRole> {
        let lineup = if team_id == self.home_team_id {
            &self.home_data.tactical_lineup
        } else {
            &self.away_data.tactical_lineup
        };
        lineup
            .assignments()
            .iter()
            .map(|a| (a.player_id(), a.slot_role()))
            .collect()
    }

    pub fn availability_for(&self, player_id: &Uuid) -> AvailabilityState {
        self.availability.availability_for(player_id)
    }

    pub fn is_player_available(&self, player_id: &Uuid) -> bool {
        self.availability.is_player_available(player_id)
    }

    pub fn drives_in_current_series(&self) -> u32 {
        self.series.drives_in_series()
    }

    pub fn reverse_drive(&mut self) {
        let current = self.series.drives_in_series();
        self.series.set_drives(current.saturating_sub(1));
    }

    pub fn record_drive(&mut self) {
        self.series.record_drive();
    }

    pub fn record_play_advance(&mut self, mirins: f64) {
        self.series.record_advance(mirins);
    }

    pub fn restore_scoreboard(
        &mut self,
        home: TeamScore,
        away: TeamScore,
        drives_in_current_series: u32,
    ) {
        self.score.restore(home, away);
        self.series.set_drives(drives_in_current_series);
    }

    pub fn winner(&self) -> Option<Uuid> {
        self.score.leader(self.home_team_id, self.away_team_id)
    }

    pub fn event_sequence(&self) -> u64 {
        self.event_sequence
    }

    pub fn advance_event_sequence(&mut self) -> u64 {
        self.event_sequence += 1;
        self.event_sequence
    }

    pub fn control_mode_for_team(&self, team_id: Uuid) -> ManagerControlMode {
        if team_id == self.home_team_id {
            self.home_data.manager.control_mode()
        } else {
            self.away_data.manager.control_mode()
        }
    }

    pub fn playbook_for_team(&self, team_id: Uuid) -> &[PlayCall] {
        if team_id == self.home_team_id {
            &self.home_data.playbook
        } else {
            &self.away_data.playbook
        }
    }

    pub fn available_profiles_for_team(&self, team_id: Uuid) -> &[TeamTacticalProfile] {
        if team_id == self.home_team_id {
            &self.home_data.available_profiles
        } else {
            &self.away_data.available_profiles
        }
    }

    pub fn tactical_profile_for_team(&self, team_id: Uuid) -> &TeamTacticalProfile {
        if team_id == self.home_team_id {
            &self.home_data.tactical_profile
        } else {
            &self.away_data.tactical_profile
        }
    }

    pub fn activate_tactical_profile(&mut self, team_id: Uuid, profile: TeamTacticalProfile) {
        if team_id == self.home_team_id {
            self.home_data.tactical_profile = profile;
        } else {
            self.away_data.tactical_profile = profile;
        }
    }

    pub fn player_attribute_table(&self, player_id: &Uuid) -> Option<&PlayerAttributeTable> {
        self.home_data
            .player_tables
            .get(player_id)
            .or_else(|| self.away_data.player_tables.get(player_id))
    }

    pub fn player_fatigue_state(&self, player_id: &Uuid) -> PhysicalState {
        self.player_fatigue.get(player_id).copied().unwrap_or_default()
    }

    pub fn set_player_fatigue_state(&mut self, player_id: Uuid, state: PhysicalState) {
        self.player_fatigue.insert(player_id, state);
    }

    pub fn impulse_for(&self, player_id: &Uuid) -> ImpulseState {
        self.player_impulse.get(player_id).copied().unwrap_or_default()
    }

    pub fn set_player_impulse_state(&mut self, player_id: Uuid, state: ImpulseState) {
        self.player_impulse.insert(player_id, state);
    }

    pub fn last_action_score_occurred(&self) -> bool {
        self.last_action_score
    }

    pub fn last_scoring_team(&self) -> Option<Uuid> {
        self.last_scoring_team
    }
}