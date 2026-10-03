use crate::error::{ControllerError, ControllerResult};
use crate::services::season::squad_quality::squad_quality;
use arlo_analytics::{PowerRankingConfig, PowerRating, TeamPowerSeed};
use arlo_domain::{AttributeKey, Player};
use std::collections::{BTreeSet, HashMap};
use uuid::Uuid;

pub fn calculate_preseason_seeds(
    team_ids: &[Uuid],
    players: &[Player],
    keys: &HashMap<Uuid, AttributeKey>,
    previous_ratings: &HashMap<Uuid, PowerRating>,
    config: &PowerRankingConfig,
) -> ControllerResult<Vec<TeamPowerSeed>> {
    config.validate()?;
    let teams: BTreeSet<Uuid> = team_ids.iter().copied().collect();
    if teams.is_empty() || teams.len() != team_ids.len() {
        return Err(ControllerError::InvalidData(
            "power ranking season teams must be nonempty and unique".into(),
        ));
    }
    let qualities: Vec<(Uuid, f64)> = teams
        .into_iter()
        .map(|team_id| {
            let players = players
                .iter()
                .filter(|player| player.team_id() == Some(team_id));
            (team_id, squad_quality(players, keys))
        })
        .collect();
    let count = qualities.len() as f64;
    let mean = qualities.iter().map(|(_, quality)| quality).sum::<f64>() / count;
    let variance = qualities
        .iter()
        .map(|(_, quality)| (quality - mean).powi(2))
        .sum::<f64>()
        / count;
    let deviation = variance.sqrt();
    qualities
        .into_iter()
        .map(|(team_id, quality)| {
            let rating = if let Some(previous) = previous_ratings.get(&team_id) {
                config.carry_over(*previous)?
            } else if deviation <= f64::EPSILON {
                config.center()?
            } else {
                let z =
                    ((quality - mean) / deviation).clamp(-config.prior_z_cap, config.prior_z_cap);
                PowerRating::new(config.center_rating + z * config.prior_spread)?
            };
            Ok(TeamPowerSeed::new(team_id, rating))
        })
        .collect()
}
