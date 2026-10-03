use crate::error::{AnalyticsError, AnalyticsResult};
use crate::power_ranking::config::PowerRankingConfig;
use crate::power_ranking::forecast::forecast_match;
use crate::power_ranking::match_result::PowerMatchResult;
use crate::power_ranking::rating::{PowerRating, TeamPowerSeed};
use crate::power_ranking::snapshot::{PowerRankingEntry, PowerRankingSnapshot};
use std::collections::{BTreeMap, HashSet};
use uuid::Uuid;

struct TeamState {
    rating: PowerRating,
    games_rated: u32,
}

pub fn replay_power_ranking(
    seeds: &[TeamPowerSeed],
    results: &[PowerMatchResult],
    config: &PowerRankingConfig,
) -> AnalyticsResult<PowerRankingSnapshot> {
    config.validate()?;
    let mut teams = BTreeMap::new();
    for seed in seeds {
        if teams
            .insert(
                seed.team_id(),
                TeamState {
                    rating: PowerRating::new(seed.initial_rating().value())?,
                    games_rated: 0,
                },
            )
            .is_some()
        {
            return Err(AnalyticsError::InvalidData(format!(
                "duplicate power seed for team {}",
                seed.team_id()
            )));
        }
    }
    let mut ordered = results.to_vec();
    ordered.sort_by_key(PowerMatchResult::ordering_key);
    let mut seen_fixtures = HashSet::new();
    for result in ordered {
        if !seen_fixtures.insert(result.fixture_id) {
            return Err(AnalyticsError::InvalidData(format!(
                "duplicate rated fixture {}",
                result.fixture_id
            )));
        }
        apply_result(&mut teams, &result, config)?;
    }
    let mut ranked: Vec<(Uuid, TeamState)> = teams.into_iter().collect();
    ranked.sort_by(|left, right| {
        right
            .1
            .rating
            .value()
            .total_cmp(&left.1.rating.value())
            .then_with(|| left.0.cmp(&right.0))
    });
    let entries = ranked
        .into_iter()
        .enumerate()
        .map(|(index, (team_id, state))| {
            PowerRankingEntry::new(index as u32 + 1, team_id, state.rating, state.games_rated)
        })
        .collect();
    Ok(PowerRankingSnapshot::new(entries))
}

fn apply_result(
    teams: &mut BTreeMap<Uuid, TeamState>,
    result: &PowerMatchResult,
    config: &PowerRankingConfig,
) -> AnalyticsResult<()> {
    if result.home_team_id == result.away_team_id {
        return Err(AnalyticsError::InvalidData(format!(
            "fixture {} has the same home and away team",
            result.fixture_id
        )));
    }
    let home = teams.get(&result.home_team_id).ok_or_else(|| {
        AnalyticsError::InvalidData(format!(
            "fixture {} has an unknown home team {}",
            result.fixture_id, result.home_team_id
        ))
    })?;
    let away = teams.get(&result.away_team_id).ok_or_else(|| {
        AnalyticsError::InvalidData(format!(
            "fixture {} has an unknown away team {}",
            result.fixture_id, result.away_team_id
        ))
    })?;
    let forecast = forecast_match(home.rating, away.rating, result.neutral_venue, config)?;
    let margin_factor = 1.0
        + config.max_margin_bonus * ((result.score_margin() as f64) / config.margin_scale).tanh();
    let delta =
        config.k_factor * margin_factor * (result.home_result() - forecast.expected_home_score());
    let next_home = PowerRating::new(home.rating.value() + delta)?;
    let next_away = PowerRating::new(away.rating.value() - delta)?;
    let home = teams.get_mut(&result.home_team_id).unwrap();
    home.rating = next_home;
    home.games_rated = home
        .games_rated
        .checked_add(1)
        .ok_or_else(|| AnalyticsError::InvalidData("home games_rated overflow".into()))?;
    let away = teams.get_mut(&result.away_team_id).unwrap();
    away.rating = next_away;
    away.games_rated = away
        .games_rated
        .checked_add(1)
        .ok_or_else(|| AnalyticsError::InvalidData("away games_rated overflow".into()))?;
    Ok(())
}
