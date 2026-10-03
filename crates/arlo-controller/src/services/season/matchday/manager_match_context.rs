use crate::services::season::squad_quality::squad_quality;
use arlo_domain::{AttributeDefinition, AttributeKey, Manager, Player, Position, Team};
use std::collections::HashMap;
use sqlx::{Row, SqlitePool};
use uuid::Uuid;

pub struct ManagerMatchContext {
    pub relative_strength: f64,
    pub importance: f64,
    pub rotation_opportunity: f64,
    pub selection_seed: u128,
    pub recent_starts: HashMap<Uuid, u8>,
    pub core_artrine_id: Option<Uuid>,
    pub core_artrine_form: f64,
}

pub struct RecentTeamHistory {
    starts: HashMap<Uuid, u8>,
    core_artrine_id: Option<Uuid>,
    core_artrine_form: f64,
    core_artrine_starts: u8,
    setbacks: f64,
}

pub async fn load_recent_history(pool: &SqlitePool, team_id: Uuid) -> Result<RecentTeamHistory, sqlx::Error> {
    let rows = sqlx::query(
        "SELECT recent.rn, f.home_team_id, f.home_score, f.away_score, s.player_id, \
         s.was_starter, fs.position, d.total_duels, d.total_wins \
         FROM (SELECT m.id, m.fixture_id, ROW_NUMBER() OVER \
         (ORDER BY f.scheduled_year DESC, f.scheduled_day_of_year DESC, m.id DESC) rn \
         FROM matches m JOIN fixtures f ON f.id = m.fixture_id \
         WHERE (f.home_team_id = ? OR f.away_team_id = ?) AND f.status = 'Completed') recent \
         JOIN fixtures f ON f.id = recent.fixture_id \
         JOIN match_team_lineup_usage u ON u.match_id = recent.id AND u.team_id = ? \
         JOIN match_squad_selections s ON s.match_id = recent.id AND s.team_id = ? \
         LEFT JOIN formation_slots fs ON fs.formation_id = u.formation_id \
         AND fs.slot_index = s.formation_slot_index \
         LEFT JOIN match_player_duels d ON d.match_id = recent.id AND d.player_id = s.player_id \
         WHERE recent.rn <= 10 AND s.was_starter = 1",
    ).bind(team_id.to_string()).bind(team_id.to_string())
        .bind(team_id.to_string()).bind(team_id.to_string())
        .fetch_all(pool).await?;
    let mut starts = HashMap::with_capacity(rows.len());
    let mut artrines: HashMap<Uuid, (u8, i64, i64)> = HashMap::new();
    let mut setbacks = 0.0;
    let mut seen_games = std::collections::HashSet::new();
    for row in rows {
        let id: String = row.try_get("player_id")?;
        let Ok(id) = Uuid::parse_str(&id) else { continue };
        let rn: i64 = row.try_get("rn")?;
        let starter: bool = row.try_get("was_starter")?;
        if rn <= 3 && starter {
            *starts.entry(id).or_insert(0) += 1;
        }
        if rn <= 3 && seen_games.insert(rn) {
            let home: String = row.try_get("home_team_id")?;
            let home_score: Option<i64> = row.try_get("home_score")?;
            let away_score: Option<i64> = row.try_get("away_score")?;
            if let (Some(home_score), Some(away_score)) = (home_score, away_score) {
                let margin = if home == team_id.to_string() {
                    home_score - away_score
                } else { away_score - home_score };
                setbacks += if margin < 0 { 0.5 } else if margin == 0 { 0.3 } else { 0.0 };
            }
        }
        let position: Option<String> = row.try_get("position")?;
        if starter && position.as_deref() == Some("A") {
            let entry = artrines.entry(id).or_insert((0, 0, 0));
            entry.0 += 1;
            entry.1 += row.try_get::<Option<i64>, _>("total_wins")?.unwrap_or(0);
            entry.2 += row.try_get::<Option<i64>, _>("total_duels")?.unwrap_or(0);
        }
    }
    let core = artrines.into_iter().max_by(|(left_id, left), (right_id, right)| {
        left.0.cmp(&right.0).then_with(|| left.1.cmp(&right.1))
            .then_with(|| left_id.cmp(right_id))
    });
    let (core_artrine_id, core_artrine_form, core_artrine_starts) = core.map_or((None, 0.5, 0), |(id, (starts, wins, duels))| {
        (Some(id), if duels >= 100 { wins as f64 / duels as f64 } else { 0.5 }, starts)
    });
    Ok(RecentTeamHistory { starts, core_artrine_id, core_artrine_form, core_artrine_starts, setbacks })
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
    history: RecentTeamHistory,
) -> ManagerMatchContext {
    let own = squad_quality(own_players.iter(), keys) + own_team.prestige() as f64 * 0.025;
    let opponent = squad_quality(opponent_players.iter(), keys) + opponent_team.prestige() as f64 * 0.025;
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
        .clamp(0.0, 1.0) * (1.0 - history.setbacks.min(0.9));
    let rigor = manager.attributes().iter()
        .find(|entry| definitions.get(&entry.attribute_definition_id())
            .is_some_and(|definition| definition.key() == AttributeKey::Rigor))
        .map_or(10.0, |entry| f64::from(entry.value()));
    let required_starts = (11.0 - rigor / 4.0).round().clamp(6.0, 10.0) as u8;
    let core_artrine_id = own_players.iter()
        .filter(|player| player.positions().iter().any(|entry| entry.position() == Position::Artrine))
        .max_by(|left, right| artrine_strength(left, keys).total_cmp(&artrine_strength(right, keys)))
        .map(Player::id);
    let core_artrine_form = if history.core_artrine_id == core_artrine_id
        && history.core_artrine_starts >= required_starts {
        history.core_artrine_form
    } else { 0.5 };
    ManagerMatchContext { relative_strength, importance, rotation_opportunity,
        selection_seed: seed, recent_starts: history.starts,
        core_artrine_id,
        core_artrine_form }
}

fn artrine_strength(player: &Player, keys: &HashMap<Uuid, AttributeKey>) -> f64 {
    let proficiency = player.positions().iter().filter(|entry| entry.position() == Position::Artrine)
        .map(|entry| entry.proficiency()).max().unwrap_or(0) as f64;
    let relevant = [AttributeKey::ArloControl, AttributeKey::Decisions,
        AttributeKey::Passing, AttributeKey::DriveTechnique, AttributeKey::Leadership];
    let skill = relevant.iter().map(|key| player.attributes().iter()
        .find(|entry| keys.get(&entry.attribute_definition_id()) == Some(key))
        .map_or(10.0, |entry| f64::from(entry.value()))).sum::<f64>() / relevant.len() as f64;
    proficiency * 5.0 + skill * 1.8
}
