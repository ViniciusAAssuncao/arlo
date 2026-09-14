use crate::domain::season::{Fixture, FixtureStatus, HomeAwayRecord, SpaMetrics, StandingsEntry};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Default, Clone)]
struct TeamRecord {
    played: u32,
    won: u32,
    drawn: u32,
    lost: u32,
    goal_points_for: u32,
    goal_points_against: u32,
    home_won: u32,
    home_drawn: u32,
    home_lost: u32,
    away_won: u32,
    away_drawn: u32,
    away_lost: u32,
}

pub fn calculate_standings(
    team_ids: &[Uuid],
    fixtures: &[Fixture],
) -> Vec<StandingsEntry> {
    let mut records: HashMap<Uuid, TeamRecord> = HashMap::with_capacity(team_ids.len());
    for &team_id in team_ids {
        records.insert(team_id, TeamRecord::default());
    }

    for fixture in fixtures {
        if fixture.status() == FixtureStatus::Completed || fixture.result().is_some() {
            if let Some(result) = fixture.result() {
                let home_id = fixture.home_team_id();
                let away_id = fixture.away_team_id();
                let home_gp = result.home_goal_points();
                let away_gp = result.away_goal_points();

                let is_home_win = if let Some(winner_id) = result.winner_team_id() {
                    if winner_id == home_id {
                        Some(true)
                    } else if winner_id == away_id {
                        Some(false)
                    } else {
                        None
                    }
                } else if result.home_score() > result.away_score() {
                    Some(true)
                } else if result.away_score() > result.home_score() {
                    Some(false)
                } else {
                    None
                };

                let home_rec = records.entry(home_id).or_default();
                home_rec.played += 1;
                home_rec.goal_points_for += home_gp;
                home_rec.goal_points_against += away_gp;
                match is_home_win {
                    Some(true) => {
                        home_rec.won += 1;
                        home_rec.home_won += 1;
                    }
                    Some(false) => {
                        home_rec.lost += 1;
                        home_rec.home_lost += 1;
                    }
                    None => {
                        home_rec.drawn += 1;
                        home_rec.home_drawn += 1;
                    }
                }

                let away_rec = records.entry(away_id).or_default();
                away_rec.played += 1;
                away_rec.goal_points_for += away_gp;
                away_rec.goal_points_against += home_gp;
                match is_home_win {
                    Some(true) => {
                        away_rec.lost += 1;
                        away_rec.away_lost += 1;
                    }
                    Some(false) => {
                        away_rec.won += 1;
                        away_rec.away_won += 1;
                    }
                    None => {
                        away_rec.drawn += 1;
                        away_rec.away_drawn += 1;
                    }
                }
            }
        }
    }

    team_ids
        .iter()
        .map(|team_id| {
            let record = records.get(team_id).cloned().unwrap_or_default();
            let home_away = HomeAwayRecord::new(
                record.home_won,
                record.home_drawn,
                record.home_lost,
                record.away_won,
                record.away_drawn,
                record.away_lost,
            );
            StandingsEntry::new(
                *team_id,
                record.played,
                record.won,
                record.drawn,
                record.lost,
                record.goal_points_for,
                record.goal_points_against,
                home_away,
                SpaMetrics::default(),
                0.0,
            )
        })
        .collect()
}
