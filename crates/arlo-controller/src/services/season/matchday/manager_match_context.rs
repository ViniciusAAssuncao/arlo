use arlo_domain::{AttributeDefinition, AttributeKey, Manager, Player, Team};
use std::collections::HashMap;
use sqlx::{Row, SqlitePool};
use uuid::Uuid;

pub struct ManagerMatchContext {
    pub relative_strength: f64,
    pub importance: f64,
    pub rotation_opportunity: f64,
    pub selection_seed: u128,
    pub recent_starts: HashMap<Uuid, u8>,
}

pub async fn load_recent_starts(pool: &SqlitePool, team_id: Uuid) -> Result<HashMap<Uuid, u8>, sqlx::Error> {
    let rows = sqlx::query(
        "SELECT s.player_id, SUM(CASE WHEN s.was_starter THEN 1 ELSE 0 END) AS starts \
         FROM (SELECT m.id FROM matches m JOIN fixtures f ON f.id = m.fixture_id \
         WHERE f.home_team_id = ? OR f.away_team_id = ? \
         ORDER BY m.completed_at_unix_seconds DESC LIMIT 3) recent \
         JOIN match_squad_selections s ON s.match_id = recent.id \
         WHERE s.team_id = ? GROUP BY s.player_id",
    ).bind(team_id.to_string()).bind(team_id.to_string()).bind(team_id.to_string())
        .fetch_all(pool).await?;
    let mut starts = HashMap::with_capacity(rows.len());
    for row in rows {
        let id: String = row.try_get("player_id")?;
        let count: i64 = row.try_get("starts")?;
        if let Ok(id) = Uuid::parse_str(&id) {
            starts.insert(id, count.clamp(0, 3) as u8);
        }
    }
    Ok(starts)
}

pub fn assess(
    manager: &Manager,
    own_team: &Team,
    opponent_team: &Team,
    own_players: &[Player],
    opponent_players: &[Player],
    keys: &HashMap<Uuid, AttributeKey>,
    definitions: &HashMap<Uuid, AttributeDefinition>,
    fixture_id: Uuid,
    knockout: bool,
    round_index: i32,
    recent_starts: HashMap<Uuid, u8>,
) -> ManagerMatchContext {
    let own = squad_quality(own_players, keys) + own_team.prestige() as f64 * 0.025;
    let opponent = squad_quality(opponent_players, keys) + opponent_team.prestige() as f64 * 0.025;
    let knowledge = manager.attributes().iter()
        .find(|entry| definitions.get(&entry.attribute_definition_id())
            .is_some_and(|definition| definition.key() == AttributeKey::TacticalKnowledge))
        .map_or(10.0, |entry| f64::from(entry.value()));
    let seed = fixture_id.as_u128() ^ manager.id().as_u128().rotate_left(17);
    let uncertainty = ((seed >> 32) as u64 as f64 / u64::MAX as f64 - 0.5)
        * (0.32 - knowledge.clamp(1.0, 20.0) * 0.011);
    let relative_strength = ((own - opponent) / 7.0 + uncertainty).clamp(-1.0, 1.0);
    let importance = if knockout { 1.0 } else {
        (0.48 + (1.0 - relative_strength.abs()) * 0.12
            + (f64::from(round_index.max(0)) / 20.0).min(1.0) * 0.12).clamp(0.0, 1.0)
    };
    let rotation_opportunity = (relative_strength.max(0.0) * (1.0 - importance) * 2.0)
        .clamp(0.0, 1.0);
    ManagerMatchContext { relative_strength, importance, rotation_opportunity,
        selection_seed: seed, recent_starts }
}

fn squad_quality(players: &[Player], keys: &HashMap<Uuid, AttributeKey>) -> f64 {
    let mut ratings: Vec<_> = players.iter().map(|player| {
        let relevant = [AttributeKey::Finishing, AttributeKey::Passing,
            AttributeKey::DefensiveContainment, AttributeKey::Decisions,
            AttributeKey::WorkRate, AttributeKey::Stamina];
        let total = relevant.iter().map(|key| player.attributes().iter()
            .find(|entry| keys.get(&entry.attribute_definition_id()) == Some(key))
            .map_or(10.0, |entry| f64::from(entry.value()))).sum::<f64>();
        let proficiency = player.positions().iter().map(|entry| entry.proficiency())
            .max().unwrap_or(0) as f64;
        total / relevant.len() as f64 + proficiency * 0.18
    }).collect();
    ratings.sort_by(|a, b| b.total_cmp(a));
    let count = ratings.len().min(14);
    if count == 0 { 10.0 } else { ratings[..count].iter().sum::<f64>() / count as f64 }
}
