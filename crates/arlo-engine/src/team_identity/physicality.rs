use arlo_domain::sport_constants::TACTICAL_STYLE_LOGIT_SCALE;
use arlo_tactics::Physicality;

pub fn offensive_contact_logit_offset(physicality: Physicality) -> f64 {
    physicality.value() * TACTICAL_STYLE_LOGIT_SCALE
}
