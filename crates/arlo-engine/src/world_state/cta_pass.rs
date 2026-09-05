use crate::match_decision::event_translation::{
    create_envelope, translate_call_to_action_started, translate_duel_resolved,
    translate_pass_completed,
};
use crate::resolution::context::DuelContext;
use crate::resolution::duel_kind::DuelKind;
use crate::resolution::outcome::DuelOutcome;
use crate::resolution::resolver::resolve_duel_for_participants;
use crate::rng::RngStream;
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
    pub pass_duel_outcome: DuelOutcome,
    pub pass_completed: bool,
    pub is_aerial: bool,
    pub reception_point: VectorPosition,
    pub down_number: u32,
    pub scrimmage_point: VectorPosition,
    pub scrimmage_x_mirim: f64,
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
    let pass_blockers = vec![passer, artrine];
    let pass_rushers = vec![pass_rusher];
    let pass_duel_outcome = resolve_duel_for_participants(
        DuelKind::PassProtection,
        &pass_blockers,
        &pass_rushers,
        state.attribute_keys(),
        &context,
        &mut duel_rng,
    );

    let pass_duel_event = translate_duel_resolved(
        &pass_duel_outcome,
        vec![passer.id(), artrine.id()],
        vec![pass_rusher.id()],
    );
    let seq = state.next_sequence();
    let clock_inst = state.clock().to_instant();
    sink.record(create_envelope(seq, clock_inst, pass_duel_event));

    let pass_completed = pass_duel_outcome.attacker_won();
    let is_aerial = false;
    let reception_point = scrimmage_point;

    if pass_completed {
        let pass_event = translate_pass_completed(
            passer.id(),
            artrine.id(),
            is_aerial,
            reception_point,
            3.0,
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
    }
}