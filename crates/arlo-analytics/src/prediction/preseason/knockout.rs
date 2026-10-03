use super::aggregation::Aggregator;
use super::plan::PreparedPlan;
use super::simulation::MatchSampler;
use super::standings::{Outcome, PlayedFixture};
use crate::error::{AnalyticsError, AnalyticsResult};
use arlo_domain::{KnockoutLegFormat, TieBreakCriterion};
use rand::Rng;
use std::collections::HashSet;

pub(super) fn run_knockout<R: Rng>(
    plan: &PreparedPlan,
    stage: usize,
    entrants: &[usize],
    start_round: u32,
    sampler: &mut MatchSampler<'_, R>,
    aggregate: &mut Aggregator,
) -> AnalyticsResult<(Vec<usize>, usize, usize, u32)> {
    if entrants.len() < 2 || !entrants.len().is_power_of_two() {
        return Err(AnalyticsError::InvalidData(
            "knockout entrants must form a power of two".into(),
        ));
    }
    let leg_format = plan.plan.stages[stage]
        .definition
        .knockout_leg_format()
        .ok_or_else(|| AnalyticsError::InvalidData("knockout leg format missing".into()))?;
    let known = if stage == 0 {
        &plan.known_ties[..]
    } else {
        &[][..]
    };
    let mut seeds: Vec<(usize, u32)> = if known.is_empty() {
        entrants
            .iter()
            .enumerate()
            .map(|(index, &team)| (team, index as u32 + 1))
            .collect()
    } else {
        let mut teams = HashSet::new();
        let mut numbers = HashSet::new();
        let seeded = known
            .iter()
            .flat_map(|tie| [(tie.high, tie.high_seed), (tie.low, tie.low_seed)])
            .collect::<Vec<_>>();
        if seeded.len() != entrants.len()
            || seeded
                .iter()
                .any(|(team, seed)| !teams.insert(*team) || !numbers.insert(*seed))
            || entrants.iter().any(|team| !teams.contains(team))
        {
            return Err(AnalyticsError::InvalidData(
                "known knockout ties do not match entrants".into(),
            ));
        }
        seeded
    };
    seeds.sort_by_key(|(_, number)| *number);
    let mut round = if stage == 0 {
        plan.known_fixtures[0]
            .iter()
            .map(|fixture| fixture.round)
            .min()
            .unwrap_or(start_round)
    } else {
        start_round
    };
    let first_round = round;
    let mut last_runner = seeds[0].0;
    while seeds.len() > 1 {
        aggregate.round(
            stage,
            round,
            &seeds.iter().map(|(team, _)| *team).collect::<Vec<_>>(),
        );
        let pairings = if round == first_round && !known.is_empty() {
            known
                .iter()
                .map(|tie| ((tie.high, tie.high_seed), (tie.low, tie.low_seed)))
                .collect::<Vec<_>>()
        } else {
            (0..seeds.len() / 2)
                .map(|pair| (seeds[pair], seeds[seeds.len() - 1 - pair]))
                .collect::<Vec<_>>()
        };
        let mut winners = Vec::with_capacity(seeds.len() / 2);
        for ((high, high_seed), (low, low_seed)) in pairings {
            aggregate.encounter(stage, round, high, low);
            let fixtures = if round == first_round && stage == 0 {
                plan.known_fixtures[0]
                    .iter()
                    .filter(|f| {
                        (f.home == high && f.away == low) || (f.home == low && f.away == high)
                    })
                    .copied()
                    .collect::<Vec<_>>()
            } else {
                Vec::new()
            };
            if round == first_round && stage == 0 && !known.is_empty() {
                let expected = if leg_format == KnockoutLegFormat::TwoLegAggregate {
                    2
                } else {
                    1
                };
                if fixtures.len() != expected {
                    return Err(AnalyticsError::InvalidData(
                        "opening knockout fixture count does not match its tie".into(),
                    ));
                }
            }
            let (winner, runner) = tie_winner(plan, high, low, leg_format, &fixtures, sampler);
            if seeds.len() == 2 {
                last_runner = runner;
            }
            winners.push((winner, if winner == high { high_seed } else { low_seed }));
        }
        winners.sort_by_key(|(_, number)| *number);
        seeds = winners;
        round += if leg_format == KnockoutLegFormat::TwoLegAggregate {
            2
        } else {
            1
        };
    }
    let champion = seeds[0].0;
    let mut ordered = vec![champion, last_runner];
    ordered.extend(
        entrants
            .iter()
            .copied()
            .filter(|team| *team != champion && *team != last_runner),
    );
    Ok((ordered, champion, last_runner, round))
}

fn tie_winner<R: Rng>(
    plan: &PreparedPlan,
    high: usize,
    low: usize,
    leg_format: KnockoutLegFormat,
    known: &[super::plan::IndexedFixture],
    sampler: &mut MatchSampler<'_, R>,
) -> (usize, usize) {
    let legs = if leg_format == KnockoutLegFormat::TwoLegAggregate {
        2
    } else {
        1
    };
    let mut high_margin = 0_i64;
    let mut high_gp = 0_u32;
    let mut low_gp = 0_u32;
    for leg in 0..legs {
        let (home, away, neutral) = if let Some(fixture) = known.get(leg) {
            (fixture.home, fixture.away, fixture.neutral)
        } else if legs == 1 || leg == 1 {
            (high, low, false)
        } else {
            (low, high, false)
        };
        let (played, margin) = sampler.sample(home, away, neutral, true);
        if home == high {
            high_margin += margin;
            high_gp += played.home_gp;
            low_gp += played.away_gp;
        } else {
            high_margin -= margin;
            high_gp += played.away_gp;
            low_gp += played.home_gp;
        }
    }
    let winner = if high_margin > 0 {
        high
    } else if high_margin < 0 {
        low
    } else if plan.plan.tie_break_criteria.iter().any(|criterion| {
        matches!(
            criterion,
            TieBreakCriterion::GoalPointsTotal | TieBreakCriterion::GoalDifference
        )
    }) && high_gp != low_gp
    {
        if high_gp > low_gp {
            high
        } else {
            low
        }
    } else {
        high
    };
    (winner, if winner == high { low } else { high })
}

pub(super) fn margin_sign(played: PlayedFixture) -> i64 {
    match played.outcome {
        Outcome::Home => 1,
        Outcome::Draw => 0,
        Outcome::Away => -1,
    }
}
