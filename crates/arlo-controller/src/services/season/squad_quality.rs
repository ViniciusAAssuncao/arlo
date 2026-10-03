use arlo_domain::{AttributeKey, Player};
use std::collections::HashMap;
use uuid::Uuid;

pub(crate) fn squad_quality<'a>(
    players: impl IntoIterator<Item = &'a Player>,
    keys: &HashMap<Uuid, AttributeKey>,
) -> f64 {
    let relevant = [
        AttributeKey::Finishing,
        AttributeKey::Passing,
        AttributeKey::DefensiveContainment,
        AttributeKey::Decisions,
        AttributeKey::WorkRate,
        AttributeKey::Stamina,
    ];
    let mut ratings: Vec<f64> = players
        .into_iter()
        .map(|player| {
            let total = relevant
                .iter()
                .map(|key| {
                    player
                        .attributes()
                        .iter()
                        .find(|entry| keys.get(&entry.attribute_definition_id()) == Some(key))
                        .map_or(10.0, |entry| f64::from(entry.value()))
                })
                .sum::<f64>();
            let proficiency = player
                .positions()
                .iter()
                .map(|entry| entry.proficiency())
                .max()
                .unwrap_or(0) as f64;
            total / relevant.len() as f64 + proficiency * 0.18
        })
        .collect();
    ratings.sort_by(|a, b| b.total_cmp(a));
    let count = ratings.len().min(14);
    if count == 0 {
        10.0
    } else {
        ratings[..count].iter().sum::<f64>() / count as f64
    }
}
