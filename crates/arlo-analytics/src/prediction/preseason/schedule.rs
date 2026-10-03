use super::plan::{IndexedFixture, PreparedPlan};
use crate::error::{AnalyticsError, AnalyticsResult};
use arlo_domain::{ScheduleAlgorithmKind, ScheduleBlock, StageType, TeamPoolRef};
use rand::seq::SliceRandom;
use rand::Rng;
use std::collections::HashSet;
use uuid::Uuid;

pub(super) fn stage_fixtures<R: Rng>(
    plan: &PreparedPlan,
    stage_index: usize,
    participants: &[usize],
    start_round: u32,
    rng: &mut R,
) -> AnalyticsResult<Vec<IndexedFixture>> {
    if stage_index == 0 && !plan.known_fixtures[0].is_empty() {
        return Ok(plan.known_fixtures[0].clone());
    }
    let stage = &plan.plan.stages[stage_index];
    match stage.definition.stage_type() {
        StageType::RoundRobinTable => {
            Ok(round_robin(participants, plan.plan.algorithm, start_round))
        }
        StageType::GroupedCompetitionTable => {
            grouped_fixtures(plan, stage_index, participants, start_round, rng)
        }
        StageType::KnockoutBracket => Ok(Vec::new()),
    }
}

fn round_robin(
    teams: &[usize],
    algorithm: ScheduleAlgorithmKind,
    start: u32,
) -> Vec<IndexedFixture> {
    let mut circle: Vec<Option<usize>> = teams.iter().copied().map(Some).collect();
    if circle.len() % 2 == 1 {
        circle.push(None);
    }
    if circle.len() < 2 {
        return Vec::new();
    }
    let rounds = circle.len() - 1;
    let mut fixtures = Vec::new();
    for round in 0..rounds {
        for slot in 0..circle.len() / 2 {
            let (Some(a), Some(b)) = (circle[slot], circle[circle.len() - 1 - slot]) else {
                continue;
            };
            let (home, away) = if slot == 0 {
                if round % 2 == 0 {
                    (a, b)
                } else {
                    (b, a)
                }
            } else if slot % 2 == 1 {
                (a, b)
            } else {
                (b, a)
            };
            fixtures.push(IndexedFixture {
                round: start + round as u32,
                home,
                away,
                neutral: false,
            });
        }
        if circle.len() > 2 {
            let last = circle.pop().unwrap();
            circle.insert(1, last);
        }
    }
    if algorithm == ScheduleAlgorithmKind::RoundRobinDoubleLeg {
        let first = fixtures.clone();
        for fixture in first {
            fixtures.push(IndexedFixture {
                round: fixture.round + rounds as u32,
                home: fixture.away,
                away: fixture.home,
                neutral: false,
            });
        }
    }
    fixtures
}

fn group_members(
    plan: &PreparedPlan,
    id: Uuid,
    participants: &HashSet<usize>,
) -> AnalyticsResult<Vec<usize>> {
    let group = plan
        .plan
        .groups
        .iter()
        .find(|group| group.id() == id)
        .ok_or_else(|| AnalyticsError::InvalidData("projection group not found".into()))?;
    Ok(group
        .team_ids()
        .iter()
        .filter_map(|id| plan.index.get(id).copied())
        .filter(|index| participants.contains(index))
        .collect())
}

fn grouped_fixtures<R: Rng>(
    plan: &PreparedPlan,
    stage_index: usize,
    teams: &[usize],
    start_round: u32,
    rng: &mut R,
) -> AnalyticsResult<Vec<IndexedFixture>> {
    let blocks = plan.plan.stages[stage_index]
        .definition
        .schedule_blocks()
        .ok_or_else(|| AnalyticsError::InvalidData("grouped stage lacks schedule blocks".into()))?;
    let participants: HashSet<usize> = teams.iter().copied().collect();
    let mut fixtures = Vec::new();
    let mut round = start_round;
    let mut block_index = 0;
    while block_index < blocks.len() {
        match &blocks[block_index] {
            ScheduleBlock::GroupRoundRobin { .. } => {
                let mut rounds_used = 0;
                while let Some(ScheduleBlock::GroupRoundRobin {
                    group_id,
                    algorithm,
                }) = blocks.get(block_index)
                {
                    let group = group_members(plan, *group_id, &participants)?;
                    let generated = round_robin(&group, *algorithm, round);
                    rounds_used = rounds_used.max(
                        generated
                            .iter()
                            .map(|f| f.round + 1 - round)
                            .max()
                            .unwrap_or(0),
                    );
                    fixtures.extend(generated);
                    block_index += 1;
                }
                round += rounds_used;
            }
            ScheduleBlock::CrossGroupPairing {
                group_a_id,
                group_b_id,
                mirrored,
            } => {
                let a = group_members(plan, *group_a_id, &participants)?;
                let b = group_members(plan, *group_b_id, &participants)?;
                if a.len() != b.len() {
                    return Err(AnalyticsError::InvalidData(
                        "cross group pairing requires equal group sizes".into(),
                    ));
                }
                for (slot, &home) in a.iter().enumerate() {
                    let away = b[slot];
                    fixtures.push(IndexedFixture {
                        round,
                        home,
                        away,
                        neutral: false,
                    });
                    if *mirrored {
                        fixtures.push(IndexedFixture {
                            round: round + 1,
                            home: away,
                            away: home,
                            neutral: false,
                        });
                    }
                }
                round += if *mirrored { 2 } else { 1 };
                block_index += 1;
            }
            ScheduleBlock::RandomPoolRounds { pool, rounds_count } => {
                let mut members = match pool {
                    TeamPoolRef::AllGroups => teams.to_vec(),
                    TeamPoolRef::SpecificGroups(ids) => {
                        let mut found = Vec::new();
                        for id in ids {
                            found.extend(group_members(plan, *id, &participants)?);
                        }
                        found
                    }
                };
                members.sort_unstable();
                members.dedup();
                random_pool_rounds(&mut fixtures, &members, &mut round, *rounds_count, rng);
                block_index += 1;
            }
        }
    }
    Ok(fixtures)
}

fn random_pool_rounds<R: Rng>(
    fixtures: &mut Vec<IndexedFixture>,
    members: &[usize],
    round: &mut u32,
    count: u32,
    rng: &mut R,
) {
    let mut used = HashSet::new();
    let mut bye_counts = vec![0_u32; members.iter().copied().max().unwrap_or(0) + 1];
    for _ in 0..count {
        let mut active = members.to_vec();
        if active.len() % 2 == 1 {
            let least = active
                .iter()
                .map(|&team| bye_counts[team])
                .min()
                .unwrap_or(0);
            let candidates: Vec<_> = active
                .iter()
                .copied()
                .filter(|&team| bye_counts[team] == least)
                .collect();
            let bye = candidates[rng.gen_range(0..candidates.len())];
            bye_counts[bye] += 1;
            active.retain(|&team| team != bye);
        }
        let mut best = active.clone();
        let mut best_repeats = usize::MAX;
        for _ in 0..50 {
            active.shuffle(rng);
            let repeats = active
                .chunks_exact(2)
                .filter(|pair| used.contains(&(pair[0].min(pair[1]), pair[0].max(pair[1]))))
                .count();
            if repeats < best_repeats {
                best = active.clone();
                best_repeats = repeats;
            }
            if repeats == 0 {
                break;
            }
        }
        for pair in best.chunks_exact(2) {
            used.insert((pair[0].min(pair[1]), pair[0].max(pair[1])));
            fixtures.push(IndexedFixture {
                round: *round,
                home: pair[0],
                away: pair[1],
                neutral: false,
            });
        }
        *round += 1;
    }
}
