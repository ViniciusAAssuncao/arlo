use crate::domain::season::{Fixture, FixtureStatus, StandingsEntry};
use arlo_domain::StandingsPointsPolicy;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Default, Clone)]
struct TeamRecord {
    played: u32,
    won: u32,
    drawn: u32,
    lost: u32,
    points: i32,
}

pub fn calculate_standings(
    team_ids: &[Uuid],
    fixtures: &[Fixture],
    points_policy: &StandingsPointsPolicy,
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

                records.entry(home_id).or_default().played += 1;
                records.entry(away_id).or_default().played += 1;

                if let Some(winner_id) = result.winner_team_id() {
                    if winner_id == home_id {
                        records.entry(home_id).or_default().won += 1;
                        records.entry(home_id).or_default().points += points_policy.points_for_win();
                        records.entry(away_id).or_default().lost += 1;
                        records.entry(away_id).or_default().points += points_policy.points_for_loss();
                    } else if winner_id == away_id {
                        records.entry(away_id).or_default().won += 1;
                        records.entry(away_id).or_default().points += points_policy.points_for_win();
                        records.entry(home_id).or_default().lost += 1;
                        records.entry(home_id).or_default().points += points_policy.points_for_loss();
                    } else {
                        records.entry(home_id).or_default().drawn += 1;
                        records.entry(home_id).or_default().points += points_policy.points_for_draw();
                        records.entry(away_id).or_default().drawn += 1;
                        records.entry(away_id).or_default().points += points_policy.points_for_draw();
                    }
                } else if result.home_score() > result.away_score() {
                    records.entry(home_id).or_default().won += 1;
                    records.entry(home_id).or_default().points += points_policy.points_for_win();
                    records.entry(away_id).or_default().lost += 1;
                    records.entry(away_id).or_default().points += points_policy.points_for_loss();
                } else if result.away_score() > result.home_score() {
                    records.entry(away_id).or_default().won += 1;
                    records.entry(away_id).or_default().points += points_policy.points_for_win();
                    records.entry(home_id).or_default().lost += 1;
                    records.entry(home_id).or_default().points += points_policy.points_for_loss();
                } else {
                    records.entry(home_id).or_default().drawn += 1;
                    records.entry(home_id).or_default().points += points_policy.points_for_draw();
                    records.entry(away_id).or_default().drawn += 1;
                    records.entry(away_id).or_default().points += points_policy.points_for_draw();
                }
            }
        }
    }

    team_ids
        .iter()
        .map(|team_id| {
            let record = records.get(team_id).cloned().unwrap_or_default();
            StandingsEntry::new(
                *team_id,
                record.played,
                record.won,
                record.drawn,
                record.lost,
                record.points,
            )
        })
        .collect()
}
