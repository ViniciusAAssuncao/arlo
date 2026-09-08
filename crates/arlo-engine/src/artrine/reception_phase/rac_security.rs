use crate::artrine::execution::security::resolve_ball_security;
use crate::artrine::reception_phase::rac_context::RacContext;
use crate::physical::systems::degradation::calculate_effective_player_speed;
use crate::physical::FatigueState;
use crate::resolution::duel_timing::derive_duel_duration;
use crate::resolution::{AttributedDuelOutcome, DuelKind};
use crate::spatial::positioning_drift::get_drifted_defender_position;
use crate::spatial::proximity::calculate_distance_mirim;
use arlo_domain::sport_constants::PROXIMITY_CONTEST_RADIUS_MIRIM;
use arlo_domain::Player;
use arlo_math::units::{Duration, Position as VectorPosition, Speed};
use rand::Rng;
use uuid::Uuid;

pub struct RacSecurityResult {
    pub duel: AttributedDuelOutcome,
    pub turnover: Option<Uuid>,
    pub recovering_player_id: Option<Uuid>,
    pub duration: Duration,
}

pub fn resolve_rac_security<F, R>(
    ctx: &RacContext<'_, F>,
    receiver_pos_vec: VectorPosition,
    rec_spd: Speed,
    rng: &mut R,
) -> RacSecurityResult
where
    F: Fn(&Uuid) -> FatigueState,
    R: Rng + ?Sized,
{
    let close_defenders: Vec<&Player> = ctx
        .defenders
        .iter()
        .copied()
        .filter(|cand| {
            get_drifted_defender_position(cand, ctx.spatial_map, ctx.attribute_keys, rng)
                .map(|p| {
                    calculate_distance_mirim(receiver_pos_vec, p) <= PROXIMITY_CONTEST_RADIUS_MIRIM
                })
                .unwrap_or(false)
        })
        .collect();

    let (sec_defenders, sec_lead) = if !close_defenders.is_empty() {
        (close_defenders.as_slice(), close_defenders[0])
    } else {
        (&ctx.defenders[..1], ctx.defenders[0])
    };

    let sec_context = ctx.duel_context.for_duel_kind(DuelKind::BallSecurityCarry);
    let sec_result = resolve_ball_security(
        DuelKind::BallSecurityCarry,
        ctx.receiver,
        ctx.receiver_pos_domain,
        sec_defenders,
        ctx.defense_position_index,
        ctx.attribute_keys,
        ctx.defense_team_id,
        &sec_context,
        ctx.fatigue_for,
        rng,
    );

    let sec_lead_state = ctx.fatigue(&sec_lead.id());
    let sec_def_pos =
        get_drifted_defender_position(sec_lead, ctx.spatial_map, ctx.attribute_keys, rng)
            .unwrap_or(receiver_pos_vec);
    let sec_def_spd =
        calculate_effective_player_speed(sec_lead, ctx.attribute_keys, &sec_lead_state);

    let duration = derive_duel_duration(receiver_pos_vec, rec_spd, sec_def_pos, sec_def_spd);

    RacSecurityResult {
        duel: sec_result.duel_outcome,
        turnover: sec_result.turnover_team_id,
        recovering_player_id: sec_result.recovering_player_id,
        duration,
    }
}
