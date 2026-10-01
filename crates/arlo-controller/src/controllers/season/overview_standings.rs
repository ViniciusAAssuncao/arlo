use crate::domain::season::StandingsEntry;
use crate::dto::season::StandingsEntryDto;
use std::collections::HashMap;
use uuid::Uuid;

pub fn build_overview_standings(
    standings_entries: &[StandingsEntry],
    team_name_map: &HashMap<Uuid, String>,
) -> Vec<StandingsEntryDto> {
    standings_entries
        .iter()
        .map(|entry| {
            let team_name = team_name_map
                .get(&entry.team_id())
                .cloned()
                .unwrap_or_else(|| "Time Desconhecido".to_string());

            StandingsEntryDto {
                team_id: entry.team_id().to_string(),
                team_name,
                played: entry.played(),
                won: entry.won(),
                drawn: entry.drawn(),
                lost: entry.lost(),
                goal_points_for: entry.goal_points_for(),
                goal_points_against: entry.goal_points_against(),
                field_goals_for: entry.field_goals_for(),
                field_goals_against: entry.field_goals_against(),
                field_points_for: entry.field_points_for(),
                field_points_against: entry.field_points_against(),
                total_points_for: entry.total_points_for(),
                total_points_against: entry.total_points_against(),
                home_won: entry.home_away().home_won(),
                home_drawn: entry.home_away().home_drawn(),
                home_lost: entry.home_away().home_lost(),
                away_won: entry.home_away().away_won(),
                away_drawn: entry.home_away().away_drawn(),
                away_lost: entry.home_away().away_lost(),
                pb: entry.spa_metrics().pb(),
                feo: entry.spa_metrics().feo(),
                ispa: entry.spa_metrics().ispa(),
                qta: entry.qta(),
            }
        })
        .collect()
}
