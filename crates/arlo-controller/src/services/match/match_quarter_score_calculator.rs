use arlo_persistence::models::incidents::{MatchAddedTimeRow, MatchScoringPlayRow};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuarterScoreSummary {
    pub period: u32,
    pub home_points: u32,
    pub away_points: u32,
    pub home_goal_points: u32,
    pub away_goal_points: u32,
    pub home_field_points: u32,
    pub away_field_points: u32,
    pub home_field_goals: u32,
    pub away_field_goals: u32,
    pub added_time_seconds: f64,
    pub has_added_time: bool,
}

pub fn calculate_quarter_scores(
    home_team_id: &str,
    away_team_id: &str,
    scoring_plays: &[MatchScoringPlayRow],
    added_time_records: &[MatchAddedTimeRow],
    final_period: u32,
) -> Vec<QuarterScoreSummary> {
    let mut added_time_by_period: HashMap<u32, f64> = HashMap::new();
    for record in added_time_records {
        let p = record.period.max(1) as u32;
        let entry = added_time_by_period.entry(p).or_insert(0.0);
        if record.added_time_seconds > *entry {
            *entry = record.added_time_seconds;
        }
    }

    let max_period = scoring_plays
        .iter()
        .map(|s| s.period.max(1) as u32)
        .chain(added_time_records.iter().map(|a| a.period.max(1) as u32))
        .chain(std::iter::once(final_period.max(1)))
        .max()
        .unwrap_or(1);

    let mut quarter_scores = Vec::with_capacity(max_period as usize);

    for period in 1..=max_period {
        let mut home_points = 0u32;
        let mut away_points = 0u32;
        let mut home_goal_points = 0u32;
        let mut away_goal_points = 0u32;
        let mut home_field_points = 0u32;
        let mut away_field_points = 0u32;
        let mut home_field_goals = 0u32;
        let mut away_field_goals = 0u32;

        for play in scoring_plays {
            if play.period.max(1) as u32 != period {
                continue;
            }

            let pts = play.points.max(0) as u32;
            let is_home = play.team_id == home_team_id;
            let is_away = play.team_id == away_team_id;

            if !is_home && !is_away {
                continue;
            }

            if is_home {
                home_points += pts;
                match play.play_type.as_str() {
                    "GoalPoint" => home_goal_points += pts,
                    "FieldPoint" => home_field_points += pts,
                    "FieldGoal" => home_field_goals += pts,
                    _ => {}
                }
            } else {
                away_points += pts;
                match play.play_type.as_str() {
                    "GoalPoint" => away_goal_points += pts,
                    "FieldPoint" => away_field_points += pts,
                    "FieldGoal" => away_field_goals += pts,
                    _ => {}
                }
            }
        }

        let added_time_seconds = added_time_by_period.get(&period).copied().unwrap_or(0.0);
        let has_added_time = added_time_seconds > 0.0;

        quarter_scores.push(QuarterScoreSummary {
            period,
            home_points,
            away_points,
            home_goal_points,
            away_goal_points,
            home_field_points,
            away_field_points,
            home_field_goals,
            away_field_goals,
            added_time_seconds,
            has_added_time,
        });
    }

    quarter_scores
}
