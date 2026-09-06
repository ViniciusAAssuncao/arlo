use crate::match_decision::event_translation::{
    create_envelope, translate_call_to_action_started, translate_duel_resolved,
    translate_pass_completed,
};
use crate::resolution::context::DuelContext;
use crate::resolution::duel_kind::DuelKind;
use crate::resolution::duel_timing::derive_duel_duration;
use crate::resolution::resolver::resolve_duel_for_participants;
use crate::resolution::AttributedDuelOutcome;
use crate::rng::RngStream;
use crate::spatial::ball_kinematics::{ball_flight_duration, calculate_pass_speed};
use crate::spatial::decision_vector::calculate_player_speed;
use crate::spatial::proximity::calculate_distance_mirim;
use crate::time::{DurationComponentKind, DurationLedger};
use crate::world_state::match_state::MatchState;
use arlo_domain::{Player, Position as DomainPosition};
use arlo_events::EventSink;
use arlo_math::units::{Position as VectorPosition, MIRIM_TO_METERS};
use uuid::Uuid;

pub struct PassPhaseResult<'a> {
    pub passer: &'a Player,
    pub artrine: &'a Player,
    pub pass_rusher: &'a Player,
    pub goalguard: &'a Player,
    pub pass_duel_outcome: AttributedDuelOutcome,
    pub pass_completed: bool,
    pub is_aerial: bool,
    pub reception_point: VectorPosition,
    pub down_number: u32,
    pub scrimmage_point: VectorPosition,
    pub scrimmage_x_mirim: f64,
    pub duration_ledger: DurationLedger,
}

pub fn find_player_by_position<'a>(
    players: &[&'a Player],
    target: DomainPosition,
) -> Option<&'a Player> {
    players.iter().copied().find(|p| {
        p.positions()
            .iter()
            .any(|pos| pos.position() == target && pos.proficiency() > 0)
    })
}

pub fn resolve_pass_phase<'a>(
    state: &mut MatchState,
    offense_players: &[&'a Player],
    defense_players: &[&'a Player],
    is_home_offense: bool,
    offense_team_id: Uuid,
    defense_team_id: Uuid,
    sink: &mut impl EventSink,
) -> PassPhaseResult<'a> {
    let passer = find_player_by_position(offense_players, DomainPosition::Passer)
        .unwrap_or(offense_players[0]);
    let artrine = find_player_by_position(offense_players, DomainPosition::Artrine)
        .unwrap_or_else(|| {
            if offense_players.len() > 1 {
                offense_players[1]
            } else {
                offense_players[0]
            }
        });
    let pass_rusher = find_player_by_position(defense_players, DomainPosition::PassRusher)
        .unwrap_or(defense_players[0]);
    let goalguard = find_player_by_position(defense_players, DomainPosition::Goalguard)
        .unwrap_or_else(|| defense_players[defense_players.len() - 1]);

    let down_number = state.possession().down() as u32;
    let scrimmage_point = state.possession().scrimmage_point();
    let scrimmage_x_mirim = scrimmage_point.raw().0 / MIRIM_TO_METERS;
    let target_advance_mirim = state
        .possession()
        .series_state()
        .remaining_mirins_to_target();

    let passer_pos = state
        .spatial_map()
        .get_position(&passer.id())
        .unwrap_or(scrimmage_point);
    let artrine_pos = state
        .spatial_map()
        .get_position(&artrine.id())
        .unwrap_or(scrimmage_point);
    let pass_rusher_pos = state
        .spatial_map()
        .get_position(&pass_rusher.id())
        .unwrap_or(scrimmage_point);

    let cta_event = translate_call_to_action_started(
        offense_team_id,
        defense_team_id,
        passer.id(),
        artrine.id(),
        down_number,
        scrimmage_x_mirim,
        target_advance_mirim,
    );
    let seq = state.next_sequence();
    let clock_inst = state.clock().to_instant();
    sink.record(create_envelope(seq, clock_inst, cta_event));

    let context = if is_home_offense {
        DuelContext::attacker_home()
    } else {
        DuelContext::defender_home()
    };

    let mut duel_rng = state
        .rng_provider()
        .indexed_rng_for(RngStream::DuelResolution, seq);
    let pass_blockers = vec![
        (passer, DomainPosition::Passer),
        (artrine, DomainPosition::Artrine),
    ];
    let pass_rushers = vec![
        (pass_rusher, DomainPosition::PassRusher),
    ];
    let raw_pass_duel = resolve_duel_for_participants(
        DuelKind::PassProtection,
        passer,
        &pass_blockers,
        pass_rusher,
        &pass_rushers,
        state.attribute_keys(),
        &context,
        &mut duel_rng,
    );

    let attacker_ids = vec![passer.id(), artrine.id()];
    let defender_ids = vec![pass_rusher.id()];

    let pass_duel_event = translate_duel_resolved(
        &raw_pass_duel,
        attacker_ids.clone(),
        defender_ids.clone(),
    );
    let seq = state.next_sequence();
    let clock_inst = state.clock().to_instant();
    sink.record(create_envelope(seq, clock_inst, pass_duel_event));

    let pass_duel_outcome = AttributedDuelOutcome::new(
        raw_pass_duel,
        attacker_ids,
        defender_ids,
    );

    let passer_mult = state.player_fatigue_multiplier(passer);
    let pass_rusher_mult = state.player_fatigue_multiplier(pass_rusher);
    let passer_speed = calculate_player_speed(passer, state.attribute_keys(), passer_mult);
    let pass_rusher_speed = calculate_player_speed(pass_rusher, state.attribute_keys(), pass_rusher_mult);
    let pass_protection_duration = derive_duel_duration(
        passer_pos,
        passer_speed,
        pass_rusher_pos,
        pass_rusher_speed,
    );

    let pass_completed = pass_duel_outcome.outcome().attacker_won();
    let is_aerial = false;
    let reception_point = artrine_pos;
    let pass_distance_mirim = calculate_distance_mirim(passer_pos, artrine_pos);

    let mut duration_ledger = DurationLedger::new();
    duration_ledger.record_live(
        DurationComponentKind::PassProtectionEngagement,
        pass_protection_duration,
    );

    if pass_completed {
        let pass_speed = calculate_pass_speed(passer, state.attribute_keys());
        let flight_duration = ball_flight_duration(pass_distance_mirim, pass_speed);
        duration_ledger.record_live(
            DurationComponentKind::InitialHandoffFlight,
            flight_duration,
        );

        let pass_event = translate_pass_completed(
            passer.id(),
            artrine.id(),
            is_aerial,
            reception_point,
            pass_distance_mirim,
        );
        let seq = state.next_sequence();
        let clock_inst = state.clock().to_instant();
        sink.record(create_envelope(seq, clock_inst, pass_event));
    }

    PassPhaseResult {
        passer,
        artrine,
        pass_rusher,
        goalguard,
        pass_duel_outcome,
        pass_completed,
        is_aerial,
        reception_point,
        down_number,
        scrimmage_point,
        scrimmage_x_mirim,
        duration_ledger,
    }
}