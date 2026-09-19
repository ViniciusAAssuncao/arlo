pub use crate::attributes::profiles::default_impulse_baseline_profile;
use crate::attributes::profiles::AttributeProfile;
use crate::attributes::PlayerAttributeTable;
use crate::caching::impulse_baseline_profile;
use crate::home_advantage::HomeAdvantageProfile;
use arlo_domain::sport_constants::MAX_CAPTAINCY_BASELINE_BOOST;
use arlo_domain::{AttributeKey, CaptaincyRole, Player};

pub type ImpulseBaselineProfile = AttributeProfile;

pub fn calculate_player_impulse_baseline(
    table: &PlayerAttributeTable,
    profile: &ImpulseBaselineProfile,
) -> f64 {
    let avg = profile.evaluate_saturated_average(|key| table.get(key));
    let norm = (avg - 10.0) / 10.0;
    let mapped = 100.0 / (1.0 + (-1.8 * norm).exp());
    mapped.clamp(0.0, 100.0)
}

pub fn calculate_captaincy_influence(table: &PlayerAttributeTable) -> f64 {
    let leadership = table.get(AttributeKey::Leadership);
    let communication = table.get(AttributeKey::Communication);
    let determination = table.get(AttributeKey::Determination);
    let teamwork = table.get(AttributeKey::Teamwork);

    let composite =
        (leadership * 0.40 + communication * 0.25 + determination * 0.20 + teamwork * 0.15) / 20.0;

    let delta = (composite.clamp(0.0, 1.0) - 0.50) / 0.50;
    delta.clamp(-1.0, 1.0)
}

pub fn calculate_player_contextual_baseline(
    table: &PlayerAttributeTable,
    captain_influence: f64,
    is_captain: bool,
    is_home: bool,
    ha_profile: &HomeAdvantageProfile,
) -> f64 {
    let profile = impulse_baseline_profile();
    let base = calculate_player_impulse_baseline(table, profile);

    let captain_boost = if is_captain {
        captain_influence * MAX_CAPTAINCY_BASELINE_BOOST * 0.50
    } else {
        captain_influence * MAX_CAPTAINCY_BASELINE_BOOST
    };

    let home_boost = if is_home {
        ha_profile.impulse_baseline_boost()
    } else {
        0.0
    };

    (base + captain_boost + home_boost).clamp(5.0, 98.0)
}

pub fn find_active_captain<'a>(players: &[&'a Player]) -> Option<&'a Player> {
    if players.is_empty() {
        return None;
    }

    if let Some(&captain) = players
        .iter()
        .find(|p| p.captaincy_role() == Some(CaptaincyRole::Captain))
    {
        return Some(captain);
    }

    if let Some(&vice_captain) = players
        .iter()
        .find(|p| p.captaincy_role() == Some(CaptaincyRole::ViceCaptain))
    {
        return Some(vice_captain);
    }

    players.first().copied()
}
