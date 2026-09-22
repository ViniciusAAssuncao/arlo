use arlo_domain::sport_constants::{DEFENSE_RX_MAX, DEFENSE_RX_MIN};
use arlo_domain::PitchZone;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LineFaultDepthGate {
    pub is_plausibly_beyond: bool,
    pub depth_margin: f64,
    pub defensive_line_proximity: f64,
}

pub fn defensive_line_to_proximity(defensive_line_height: f64, zone: PitchZone) -> f64 {
    let h = defensive_line_height.clamp(0.0, 1.0);
    let base_depth = 1.0 - (DEFENSE_RX_MIN + h * (DEFENSE_RX_MAX - DEFENSE_RX_MIN));
    match zone {
        PitchZone::FirstZone => base_depth.max(0.93),
        PitchZone::SecondZone => base_depth.max(0.85),
        PitchZone::OpenField => base_depth.min(0.88),
    }
}

pub fn evaluate_reception_depth_gate(
    normalized_proximity: f64,
    defensive_line_height: f64,
    zone: PitchZone,
) -> LineFaultDepthGate {
    let defensive_line_proximity = defensive_line_to_proximity(defensive_line_height, zone);
    let depth_margin = normalized_proximity - defensive_line_proximity;
    let is_plausibly_beyond = depth_margin > 0.0;

    LineFaultDepthGate {
        is_plausibly_beyond,
        depth_margin: depth_margin.max(0.0),
        defensive_line_proximity,
    }
}

pub fn scale_line_fault_probability(base_probability: f64, depth_margin: f64) -> f64 {
    if depth_margin <= 0.0 {
        0.0
    } else {
        let normalized_margin = (depth_margin / 0.06).clamp(0.0, 2.5);
        let margin_factor = normalized_margin.powf(1.5);
        (base_probability * margin_factor).clamp(0.0, 0.95)
    }
}