use crate::scoring_model::difficulty_curves::calculate_base_difficulty_logit;
use crate::scoring_model::margin::calculate_margin_penalty_logit;
use crate::scoring_model::rating_gap_scaling::scale_rating_gap;
use crate::scoring_model::scoring_kind::ScoringKind;
use crate::scoring_model::scoring_situation::ScoringSituation;
use crate::scoring_model::tuning::ScoringDifficultyProfile;
use arlo_math::stats::contrast::logistic;
use arlo_math::Probability;

pub fn calculate_scoring_probability(
    kind: ScoringKind,
    situation: &ScoringSituation,
    profile: &ScoringDifficultyProfile,
) -> Probability {
    let base_logit = calculate_base_difficulty_logit(kind, situation, profile);
    let rating_diff = situation.finisher_rating - situation.goalguard_rating;
    let scaled_diff = scale_rating_gap(
        rating_diff,
        profile.rating_gap_saturation_point,
        profile.rating_gap_slope,
    );
    let rating_shift = scaled_diff * profile.rating_shift_weight(kind);

    let mut context_offset = 0.0;
    if let Some(dc) = situation.duel_context {
        if dc.attacker_is_home() {
            context_offset += dc.home_advantage_duel_logit();
        }
        if dc.defender_is_home() {
            context_offset -= dc.home_advantage_duel_logit();
        }
        context_offset += dc.aggression_logit_offset();
        context_offset += dc.physicality_logit_offset();
        context_offset += dc.misdirection_logit_offset();
    }

    let margin_penalty = if let Some(mc) = &situation.margin_context {
        calculate_margin_penalty_logit(kind, mc, &profile.margin_penalty)
    } else {
        0.0
    };

    let total_logit = base_logit + rating_shift + context_offset - margin_penalty;
    Probability::new_clamped(logistic(total_logit).clamp(0.01, 0.95))
}
