use crate::artrine::execution::detect_drive_crossings_arithmetic;
use crate::resolution::outcome_distribution::{ sample_action_progression, ActionProgressionKind };
use crate::team_identity::{ long_launch_advance_multiplier, short_pass_advance_multiplier };
use crate::world_state::match_state::MatchState;
use crate::world_state::step::down_resolution::contest_stage::ActionContestOutcome;
use crate::world_state::step::down_resolution::context::DownResolutionContext;
use arlo_domain::{ ArtrineDecisionKind, PitchZone };
use arlo_math::units::Duration;
use rand::Rng;
use smallvec::SmallVec;

pub struct ActionProgressionOutcome {
    pub mirins_advanced: f64,
    pub drives_recorded: u32,
    pub drive_row_indices: SmallVec<[usize; 4]>,
    pub new_normalized_proximity: f64,
    pub new_zone: PitchZone,
    pub live_duration: Duration,
}

pub fn resolve_progression<R: Rng + ?Sized>(
    ctx: &DownResolutionContext<'_>,
    decision: ArtrineDecisionKind,
    contest: &ActionContestOutcome<'_>,
    state: &MatchState,
    rng: &mut R
) -> ActionProgressionOutcome {
    let (prog_kind, mult) = match decision {
        ArtrineDecisionKind::SelfCarry => (ActionProgressionKind::Carry, 1.0),
        ArtrineDecisionKind::ShortPass =>
            (ActionProgressionKind::ShortPass, short_pass_advance_multiplier(ctx.passing_range)),
        ArtrineDecisionKind::LongLaunch =>
            (ActionProgressionKind::LongLaunch, long_launch_advance_multiplier(ctx.passing_range)),
        ArtrineDecisionKind::Cross => (ActionProgressionKind::Cross, 1.0),
        ArtrineDecisionKind::SelfFinish => (ActionProgressionKind::Carry, 0.1),
    };

    let effective_mult = if contest.attacker_won { mult } else { mult * 0.15 };

    let mirins_advanced = sample_action_progression(
        prog_kind,
        contest.net_advantage,
        effective_mult,
        rng
    );

    let mut drive_row_indices = SmallVec::new();
    let mut drives_recorded = 0;

    let start_x_mirim = if ctx.is_home_offense {
        ctx.normalized_proximity * ctx.pitch_length_mirim
    } else {
        (1.0 - ctx.normalized_proximity) * ctx.pitch_length_mirim
    };

    let end_x_mirim = if ctx.is_home_offense {
        start_x_mirim + mirins_advanced
    } else {
        start_x_mirim - mirins_advanced
    };

    if ctx.is_true_artrine && mirins_advanced > 0.0 {
        let crossed = detect_drive_crossings_arithmetic(
            state.pitch(),
            start_x_mirim,
            end_x_mirim,
            ctx.is_home_offense
        );
        for row in crossed {
            drive_row_indices.push(row);
            drives_recorded += 1;
        }
    }

    let delta_norm = mirins_advanced / ctx.pitch_length_mirim.max(1.0);
    let new_normalized_proximity = (ctx.normalized_proximity + delta_norm).clamp(0.0, 1.0);

    let new_zone = if new_normalized_proximity >= 0.88 {
        PitchZone::FirstZone
    } else if new_normalized_proximity >= 0.72 {
        PitchZone::SecondZone
    } else {
        PitchZone::Central
    };

    let base_seconds =
        14.0 + (mirins_advanced * 0.6).clamp(0.0, 20.0) + contest.net_advantage * 0.2;
    let live_duration = Duration::new(base_seconds.clamp(8.0, 42.0));

    ActionProgressionOutcome {
        mirins_advanced,
        drives_recorded,
        drive_row_indices,
        new_normalized_proximity,
        new_zone,
        live_duration,
    }
}
