use crate::error::{EngineError, EngineResult};
use crate::match_decision::event_translation::{
    create_envelope, translate_call_to_action_started, translate_duel_resolved,
    translate_pass_completed,
};
use crate::physical::systems::degradation::calculate_effective_player_speed;
use crate::resolution::context::DuelContext;
use crate::resolution::duel_kind::DuelKind;
use crate::resolution::duel_timing::derive_duel_duration;
use crate::resolution::resolver::resolve_duel_for_participants_with_fatigue;
use crate::resolution::AttributedDuelOutcome;
use crate::rng::RngStream;
use crate::spatial::ball_kinematics::{ball_flight_duration, calculate_pass_speed_with_state};
use crate::spatial::proximity::calculate_distance_mirim;
use crate::time::{DurationComponentKind, DurationLedger};
use crate::world_state::match_state::MatchState;
use arlo_domain::{Player, Position as DomainPosition, SlotRole};
use arlo_events::EventSink;
use arlo_math::units::{Position as VectorPosition, MIRIM_TO_METERS};
use std::collections::HashMap;
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
) -> EngineResult<&'a Player> {
    players
        .iter()
        .copied()
        .find(|p| {
            p.positions()
                .iter()
                .any(|pos| pos.position() == target && pos.proficiency() > 0)
        })
        .ok_or_else(|| EngineError::MissingRequiredPosition(format!("{target:?}")))
}

pub fn resolve_pass_phase<'a>(
    state: &mut MatchState,
    offense_players: &[&'a Player],
    offense_pos_index: &HashMap<Uuid, DomainPosition>,
    offense_role_index: &HashMap<Uuid, SlotRole>,
    defense_players: &[&'a Player],
    is_home_offense: bool,
    offense_team_id: Uuid,
    defense_team_id: Uuid,
    sink: &mut impl EventSink,
) -> EngineResult<PassPhaseResult<'a>> {
    let passer = find_player_by_position(offense_players, DomainPosition::Passer)?;
    let artrine = find_player_by_position(offense_players, DomainPosition::Artrine)?;
    let pass_rusher = find_player_by_position(defense_players, DomainPosition::PassRusher)?;
    let goalguard = find_player_by_position(defense_players, DomainPosition::Goalguard)?;

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

    let mut pass_blockers = vec![
        (passer, DomainPosition::Passer),
        (artrine, DomainPosition::Artrine),
    ];
    let mut attacker_ids = vec![passer.id(), artrine.id()];

    for &player in offense_players {
        if offense_role_index.get(&player.id()) == Some(&SlotRole::Safeguard) {
            if !attacker_ids.contains(&player.id()) {
                let pos = offense_pos_index
                    .get(&player.id())
                    .copied()
                    .unwrap_or_else(|| {
                        player
                            .positions()
                            .first()
                            .map(|pp| pp.position())
                            .unwrap_or(DomainPosition::Fullback)
                    });
                pass_blockers.push((player, pos));
                attacker_ids.push(player.id());
            }
        }
    }

    let pass_rushers = vec![
        (pass_rusher, DomainPosition::PassRusher),
    ];

    let home_fatigue = state.home_fatigue().clone();
    let away_fatigue = state.away_fatigue().clone();
    let fatigue_lookup = move |id: &Uuid| {
        home_fatigue
            .get(id)
            .or_else(|| away_fatigue.get(id))
            .copied()
            .unwrap_or_default()
    };

    let raw_pass_duel = resolve_duel_for_participants_with_fatigue(
        DuelKind::PassProtection,
        passer,
        &pass_blockers,
        pass_rusher,
        &pass_rushers,
        state.attribute_keys(),
        &context,
        &fatigue_lookup,
        &mut duel_rng,
    );

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

    let passer_state = state.fatigue_for(&passer.id());
    let pass_rusher_state = state.fatigue_for(&pass_rusher.id());
    let passer_speed = calculate_effective_player_speed(passer, state.attribute_keys(), &passer_state);
    let pass_rusher_speed = calculate_effective_player_speed(pass_rusher, state.attribute_keys(), &pass_rusher_state);
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
        let pass_speed = calculate_pass_speed_with_state(passer, state.attribute_keys(), &passer_state);
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

    Ok(PassPhaseResult {
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
    })
}
