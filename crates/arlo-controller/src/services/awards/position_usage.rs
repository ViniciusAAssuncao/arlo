use crate::domain::calendar::CalendarDate;
use crate::error::ControllerResult;
use arlo_domain::AwardPositionUsage;
use sqlx::{FromRow, SqlitePool};
use std::collections::BTreeMap;
use uuid::Uuid;

#[derive(FromRow)]
struct UsageRow {
    player_id: String,
    position_code: String,
    seconds_played: f64,
    proficiency: Option<i64>,
}

pub(super) async fn load_position_usage(
    pool: &SqlitePool,
    season_id: Uuid,
    start: CalendarDate,
    end: CalendarDate,
) -> ControllerResult<BTreeMap<Uuid, Vec<AwardPositionUsage>>> {
    let rows = sqlx::query_as::<_, UsageRow>(
        "SELECT usage.player_id, usage.position_code, SUM(usage.seconds_played) AS seconds_played, (SELECT MAX(pp.proficiency) FROM player_positions pp JOIN positions pos ON pos.id = pp.position_id WHERE pp.player_id = usage.player_id AND pos.code = usage.position_code) AS proficiency FROM match_player_position_seconds usage JOIN matches m ON m.id = usage.match_id JOIN fixtures f ON f.id = m.fixture_id JOIN season_stages ss ON ss.id = f.season_stage_id WHERE ss.season_instance_id = ? AND (f.scheduled_year > ? OR (f.scheduled_year = ? AND f.scheduled_day_of_year >= ?)) AND (f.scheduled_year < ? OR (f.scheduled_year = ? AND f.scheduled_day_of_year <= ?)) GROUP BY usage.player_id, usage.position_code ORDER BY usage.player_id, usage.position_code",
    )
    .bind(season_id.to_string())
    .bind(start.year())
    .bind(start.year())
    .bind(i64::from(start.day_of_year()))
    .bind(end.year())
    .bind(end.year())
    .bind(i64::from(end.day_of_year()))
    .fetch_all(pool)
    .await?;
    let mut usage = BTreeMap::<Uuid, Vec<AwardPositionUsage>>::new();
    for row in rows {
        usage
            .entry(Uuid::parse_str(&row.player_id)?)
            .or_default()
            .push(AwardPositionUsage {
                position_code: row.position_code,
                seconds_played: row.seconds_played,
                proficiency: row.proficiency.and_then(|value| u32::try_from(value).ok()),
            });
    }
    Ok(usage)
}
