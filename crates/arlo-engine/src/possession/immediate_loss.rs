use arlo_domain::sport_constants::IMMEDIATE_POSSESSION_CONTROL_SECONDS;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PossessionLossClassification {
    Immediate,
    Established,
}

pub fn is_immediate_loss(control_duration_seconds: f64) -> bool {
    control_duration_seconds < IMMEDIATE_POSSESSION_CONTROL_SECONDS as f64
}

pub fn is_established_possession(control_duration_seconds: f64) -> bool {
    !is_immediate_loss(control_duration_seconds)
}

pub fn classify_possession_loss(control_duration_seconds: f64) -> PossessionLossClassification {
    if is_immediate_loss(control_duration_seconds) {
        PossessionLossClassification::Immediate
    } else {
        PossessionLossClassification::Established
    }
}