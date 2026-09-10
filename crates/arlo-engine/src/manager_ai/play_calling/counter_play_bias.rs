use arlo_domain::sport_constants::managerial::IN_GAME_ADJUSTMENT_MAX_IMPACT;
use arlo_tactics::PlayCall;
use uuid::Uuid;

pub fn apply_counter_bias(
    ranked: &mut Vec<(usize, f64)>,
    playbook: &[PlayCall],
    last_play_call_id: Option<Uuid>,
    last_play_failed: bool,
    in_game_adjustments: f64,
) {
    if !last_play_failed {
        return;
    }
    let Some(last_id) = last_play_call_id else {
        return;
    };

    let norm_iga = in_game_adjustments.clamp(0.0, 20.0) / 20.0;
    let multiplier = 1.0 + (norm_iga * 0.5);

    for (idx, score) in ranked.iter_mut() {
        if let Some(candidate) = playbook.get(*idx) {
            if candidate.counter_play_id() == Some(last_id) {
                *score = (*score * multiplier + norm_iga * IN_GAME_ADJUSTMENT_MAX_IMPACT).max(0.0);
            }
        }
    }
}
