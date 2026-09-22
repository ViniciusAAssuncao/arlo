use crate::artrine::execution::drive_award::award_drives_with_profile;
use crate::possession::locate_zone;
use crate::resolution::outcome_distribution::{sample_action_progression, ActionProgressionKind};
use crate::team_identity::{
    effort_multiplier, long_launch_advance_multiplier, short_pass_advance_multiplier,
};
use crate::world_state::match_state::MatchState;
use crate::world_state::step::down_resolution::contest_stage::ActionContestOutcome;
use crate::world_state::step::down_resolution::context::{DownStaticContext, TouchDynamicContext};
use arlo_domain::sport_constants::AWC_DEFAULT_SECOND_ZONE_DEPTH_MIRIM;
use arlo_domain::{ArtrineDecisionKind, PitchZone, Player};
use arlo_math::units::Duration;
use rand::Rng;

pub struct ActionProgressionOutcome {
    pub mirins_advanced: f64,
    pub drives_recorded: u32,
    pub new_normalized_proximity: f64,
    pub new_zone: PitchZone,
    pub live_duration: Duration,
}

pub fn resolve_progression<R: Rng + ?Sized>(
    static_ctx: &DownStaticContext<'_>,
    touch_ctx: &TouchDynamicContext<'_>,
    carrier: &Player,
    decision: ArtrineDecisionKind,
    contest: &ActionContestOutcome<'_>,
    _state: &MatchState,
    rng: &mut R,
) -> ActionProgressionOutcome {
    let (prog_kind, mult) = match decision {
        ArtrineDecisionKind::SelfCarry => (ActionProgressionKind::Carry, 1.0),
        ArtrineDecisionKind::ShortPass => (
            ActionProgressionKind::ShortPass,
            short_pass_advance_multiplier(static_ctx.passing_range),
        ),
        ArtrineDecisionKind::LongLaunch => (
            ActionProgressionKind::LongLaunch,
            long_launch_advance_multiplier(static_ctx.passing_range),
        ),
        ArtrineDecisionKind::Cross => (ActionProgressionKind::Cross, 1.0),
        ArtrineDecisionKind::SelfFinish => (ActionProgressionKind::Carry, 0.1),
    };

    let effective_mult = if contest.attacker_won {
        mult
    } else {
        mult * 0.15
    };

    let mirins_advanced =
        sample_action_progression(prog_kind, contest.net_advantage, effective_mult, rng);

    let drives_recorded = award_drives_with_profile(
        touch_ctx.is_true_artrine && carrier.id() == touch_ctx.carrier.id(),
        decision,
        contest.attacker_won,
        contest.net_advantage,
        &touch_ctx.carrier_table,
        mirins_advanced,
        &static_ctx.drive_award_profile,
        rng,
    );

    let delta_norm = mirins_advanced / static_ctx.pitch_length_mirim.max(1.0);
    let new_normalized_proximity = (touch_ctx.normalized_proximity + delta_norm).clamp(0.0, 1.0);

    let new_zone = locate_zone(
        new_normalized_proximity,
        static_ctx.pitch_length_mirim,
        AWC_DEFAULT_SECOND_ZONE_DEPTH_MIRIM,
    );

    let tempo_mult = effort_multiplier(static_ctx.offense_tempo);
    let base_seconds =
        (8.0 + (mirins_advanced * 0.4).clamp(0.0, 15.0) + contest.net_advantage * 0.1) / tempo_mult;
    let live_duration = Duration::new(base_seconds.clamp(4.0, 25.0));

    ActionProgressionOutcome {
        mirins_advanced,
        drives_recorded,
        new_normalized_proximity,
        new_zone,
        live_duration,
    }
}