use super::context::{ManagerDecisionContext, ManagerSkills, PlayerContext, RoleFit, SlotContext};
use super::evidence::read_evidence;
use arlo_analytics::PlayerPerformanceAggregator;
use arlo_domain::{sport_constants::effective_manager_flexibility, AttributeKey, Player};
use arlo_engine::input::RoleFitTable;
use arlo_engine::{MatchInput, MatchPhase, MatchState, TeamInput, TeamState};
use arlo_stats::AggregatorRegistry;
use uuid::Uuid;

pub(super) fn build_context(
    input: &MatchInput,
    state: &MatchState,
    registry: Option<&AggregatorRegistry>,
    team_id: Uuid,
) -> Option<ManagerDecisionContext> {
    if state.phase() != MatchPhase::Stopped
        || state.clock().seconds_in_period() >= state.clock().period_limit_seconds()
    {
        return None;
    }
    let (team, team_state, opponent) = if team_id == input.home().team_id() {
        (input.home(), state.home(), state.away())
    } else if team_id == input.away().team_id() {
        (input.away(), state.away(), state.home())
    } else {
        return None;
    };
    if team.manager().is_human_controlled() {
        return None;
    }
    let elapsed = state.clock().total_elapsed_seconds();
    let skills = manager_skills(input, team);
    let analytics = registry.and_then(|registry| registry.get::<PlayerPerformanceAggregator>());
    let slots: Vec<_> = team_state
        .lineup(team)
        .assignments()
        .iter()
        .filter_map(|assignment| {
            let player_id = team_state.slot_player_id(assignment.player_id());
            if !team_state.active_player_ids().contains(&player_id) {
                return None;
            }
            let slot = team_state
                .formation(team)
                .slots()
                .get(assignment.formation_slot_index())?;
            Some(SlotContext {
                player_id,
                offensive_position: slot.offensive_position(),
                defensive_position: slot.defensive_position(),
                role: assignment.slot_role(),
            })
        })
        .collect();
    let targets: Vec<_> = slots
        .iter()
        .map(|slot| (slot.offensive_position, slot.defensive_position, slot.role))
        .collect();
    let role_fits = input.player_role_fits(team_id, state.team_instructions(team), &targets)?;
    let mut players: Vec<_> = team
        .roster()
        .iter()
        .map(|player| {
            player_context(
                input, state, team, team_state, analytics, &role_fits, player,
            )
        })
        .collect();
    players.sort_by_key(|player| player.id);
    let regulation = f64::from(input.format().regulation_periods())
        * f64::from(input.format().regulation_period_duration_minutes())
        * 60.0;
    Some(ManagerDecisionContext {
        seed: input.seed(),
        sequence: state.next_event_sequence(),
        team_id,
        manager_id: team.manager().id(),
        elapsed,
        progress: (elapsed / regulation.max(1.0)).clamp(0.0, 1.0),
        deficit: f64::from(opponent.score().total_points())
            - f64::from(team_state.score().total_points()),
        last_substitution_at: team_state.last_voluntary_substitution_at(),
        last_tactical_switch_at: team_state.last_tactical_switch_at(),
        last_realignment_at: team_state.last_tactical_realignment_at(),
        last_plan_activation_at: team_state.last_plan_activation_at(),
        current_effect: super::plan_adapter::effect(
            &state.team_instructions(team),
            team_state.formation(team),
        ),
        plans: if super::cooldowns::DecisionReadiness::from_state(team_state, elapsed).plan {
            super::plan_adapter::build(input, state, team, team_state, &slots)
        } else {
            Vec::new()
        },
        skills,
        slots,
        players,
    })
}

fn player_context(
    input: &MatchInput,
    state: &MatchState,
    team: &TeamInput,
    team_state: &TeamState,
    analytics: Option<&PlayerPerformanceAggregator>,
    role_fits: &RoleFitTable,
    player: &Player,
) -> PlayerContext {
    let values = input.player_attributes(player.id());
    let attribute = |key: AttributeKey| {
        values
            .and_then(|values| values[key.index()])
            .unwrap_or(10.0)
    };
    let average = |keys: &[AttributeKey]| {
        keys.iter().map(|key| attribute(*key)).sum::<f64>() / keys.len() as f64 / 20.0
    };
    let fits = role_fits[&player.id()]
        .iter()
        .map(|&(quality, proficiency)| RoleFit {
            quality,
            proficiency,
        })
        .collect();
    let history = analytics
        .and_then(|analytics| analytics.context())
        .map(|context| context.substitution_history())
        .unwrap_or(&[]);
    PlayerContext {
        id: player.id(),
        active: team_state.active_player_ids().contains(&player.id()),
        available: team_state.is_available_reserve(player.id()),
        injured: team_state.injured_player_ids().contains(&player.id()),
        starter: team
            .lineup()
            .assignments()
            .iter()
            .any(|assignment| assignment.player_id() == player.id()),
        captain: player.captaincy_role().is_some(),
        energy: state.player_energy(player.id()),
        morale: state.player_morale(player.id()),
        settling: state.player_settling_factor(player.id()),
        entered_at: state.player_last_entered_at(player.id()),
        exited_at: history
            .iter()
            .rev()
            .find(|record| record.player_out_id() == player.id())
            .map(|record| record.clock_seconds()),
        fits,
        attack: average(&[
            AttributeKey::Finishing,
            AttributeKey::Pace,
            AttributeKey::Anticipation,
        ]),
        defense: average(&[
            AttributeKey::DefensiveContainment,
            AttributeKey::Positioning,
            AttributeKey::Concentration,
        ]),
        control: average(&[
            AttributeKey::Passing,
            AttributeKey::Vision,
            AttributeKey::DriveTechnique,
        ]),
        security: average(&[
            AttributeKey::ArloControl,
            AttributeKey::HandsReception,
            AttributeKey::Decisions,
        ]),
        discipline: average(&[
            AttributeKey::Concentration,
            AttributeKey::Composure,
            AttributeKey::Decisions,
        ]),
        evidence: read_evidence(analytics, player.id()),
    }
}

fn manager_skills(input: &MatchInput, team: &TeamInput) -> ManagerSkills {
    let raw = |key| {
        team.manager()
            .attributes()
            .iter()
            .find(|entry| {
                input
                    .manager_attribute_keys()
                    .get(&entry.attribute_definition_id())
                    == Some(&key)
            })
            .map_or(10.0, |entry| f64::from(entry.value()))
    };
    let value = |key| ((raw(key) - 1.0) / 19.0).clamp(0.0, 1.0);
    let tendency = team
        .manager()
        .tactical_profile()
        .map_or(value(AttributeKey::Adaptability), |profile| {
            profile.flexibility_tendency()
        });
    ManagerSkills {
        judging: value(AttributeKey::JudgingAbility),
        adjustments: value(AttributeKey::InGameAdjustments),
        knowledge: value(AttributeKey::TacticalKnowledge),
        adaptability: value(AttributeKey::Adaptability),
        offense: value(AttributeKey::OffensePlanning),
        defense: value(AttributeKey::DefenseOrganization),
        artro: value(AttributeKey::ArtroStrategy),
        load: value(AttributeKey::LoadManagement),
        composure: value(AttributeKey::Composure),
        patience: value(AttributeKey::Discipline),
        flexibility: effective_manager_flexibility(raw(AttributeKey::Adaptability), tendency),
    }
}
