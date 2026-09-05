use crate::match_decision::event_translation::{
    create_envelope, translate_drive_recorded, translate_duel_resolved,
};
use crate::resolution::aggregate_progression::AggregateProgressionStrategy;
use crate::resolution::context::DuelContext;
use crate::resolution::duel_kind::DuelKind;
use crate::resolution::outcome::DuelOutcome;
use crate::resolution::progression_strategy::ProgressionResolutionStrategy;
use crate::resolution::resolver::resolve_duel_for_participants;
use crate::rng::RngStream;
use crate::world_state::cta_pass::PassPhaseResult;
use crate::world_state::match_state::MatchState;
use arlo_domain::pitch::artro_rows_for_pitch;
use arlo_domain::sport_constants::SPATIAL_TICK_DURATION_SECONDS;
use arlo_domain::Player;
use arlo_events::{EventArtroPlacement, EventSink};
use arlo_math::units::{Duration, Position as VectorPosition, MIRIM_TO_METERS};

pub struct ProgressionPhaseResult {
    pub artro_duel_outcome: DuelOutcome,
    pub mirins_advanced: f64,
    pub drives_recorded_count: u32,
    pub start_x_mirim: f64,
    pub end_x_mirim: f64,
    pub end_position: VectorPosition,
}

pub fn resolve_progression_phase(
    state: &mut MatchState,
    pass_phase: &PassPhaseResult<'_>,
    offense_players: &[&Player],
    defense_players: &[&Player],
    is_home_offense: bool,
    sink: &mut impl EventSink,
) -> ProgressionPhaseResult {
    let context = if is_home_offense {
        DuelContext::attacker_home()
    } else {
        DuelContext::defender_home()
    };

    let duel_seq = state.event_sequence();
    let mut duel_rng = state
        .rng_provider()
        .indexed_rng_for(RngStream::DuelResolution, duel_seq);
    let artro_duel_outcome = resolve_duel_for_participants(
        DuelKind::ArtroBreakthrough,
        offense_players,
        defense_players,
        state.attribute_keys(),
        &context,
        &mut duel_rng,
    );

    let artro_duel_event = translate_duel_resolved(
        &artro_duel_outcome,
        offense_players.iter().map(|p| p.id()).collect(),
        defense_players.iter().map(|p| p.id()).collect(),
    );
    let seq = state.next_sequence();
    let clock_inst = state.clock().to_instant();
    sink.record(create_envelope(seq, clock_inst, artro_duel_event));

    let mut prog_rng = state
        .rng_provider()
        .indexed_rng_for(RngStream::ProgressionDistribution, seq);
    let progression_strategy = AggregateProgressionStrategy::default();
    let raw_mirins_advanced =
        progression_strategy.resolve_progression(&artro_duel_outcome, &mut prog_rng);
    let mirins_advanced = if pass_phase.pass_completed {
        raw_mirins_advanced
    } else {
        0.0
    };

    let pitch_length_mirim = state.pitch().length_mirim();
    let start_x_mirim = pass_phase.scrimmage_x_mirim;
    let end_x_mirim = if is_home_offense {
        (start_x_mirim + mirins_advanced).min(pitch_length_mirim)
    } else {
        (start_x_mirim - mirins_advanced).max(0.0)
    };

    let all_artro_rows = artro_rows_for_pitch(state.pitch());
    let mut drives_recorded_count = 0;

    if pass_phase.pass_completed && !pass_phase.is_aerial {
        let min_x = start_x_mirim.min(end_x_mirim);
        let max_x = start_x_mirim.max(end_x_mirim);

        for row in &all_artro_rows {
            let rx = row.x_mirim();
            if rx >= min_x && rx <= max_x {
                state.increment_drives();
                drives_recorded_count += 1;

                let drive_event = translate_drive_recorded(
                    pass_phase.artrine.id(),
                    row.row_index(),
                    EventArtroPlacement::Central,
                    state.drives_in_current_series(),
                    rx,
                );
                let seq = state.next_sequence();
                let clock_inst = state.clock().to_instant();
                sink.record(create_envelope(seq, clock_inst, drive_event));
            }
        }
    }

    let tick_duration = Duration::new(SPATIAL_TICK_DURATION_SECONDS * 5.0);
    state.spatial_map_mut().tick(tick_duration);

    let end_position = VectorPosition::from_components(
        end_x_mirim * MIRIM_TO_METERS,
        pass_phase.scrimmage_point.raw().1,
        0.0,
    );

    ProgressionPhaseResult {
        artro_duel_outcome,
        mirins_advanced,
        drives_recorded_count,
        start_x_mirim,
        end_x_mirim,
        end_position,
    }
}