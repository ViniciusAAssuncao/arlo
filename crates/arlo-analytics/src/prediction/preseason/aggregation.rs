use super::plan::{GoalPointModel, PreparedPlan, PreseasonConfig, PRESEASON_MODEL_VERSION};
use arlo_domain::StageType;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StageTeamProjection {
    pub stage_index: u32,
    pub team_id: Uuid,
    pub reach_probability: f64,
    pub mean_position: Option<f64>,
    pub median_position: Option<u32>,
    pub modal_position: Option<u32>,
    pub position_probabilities: Vec<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TeamProjection {
    pub team_id: Uuid,
    pub projected_table_position: Option<u32>,
    pub top_four_probability: Option<f64>,
    pub champion_probability: f64,
    pub runner_up_probability: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EncounterProjection {
    pub stage_index: u32,
    pub round_index: u32,
    pub first_team_id: Uuid,
    pub second_team_id: Uuid,
    pub probability: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RoundTeamProjection {
    pub stage_index: u32,
    pub round_index: u32,
    pub team_id: Uuid,
    pub reach_probability: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PreseasonProjection {
    pub season_instance_id: Uuid,
    pub model_version: u32,
    pub simulation_count: u32,
    pub random_seed: u64,
    pub config: PreseasonConfig,
    pub goal_point_model: Option<GoalPointModel>,
    pub teams: Vec<TeamProjection>,
    pub stages: Vec<StageTeamProjection>,
    pub encounters: Vec<EncounterProjection>,
    pub rounds: Vec<RoundTeamProjection>,
}

pub(super) struct Aggregator {
    positions: Vec<Vec<Vec<u32>>>,
    reached: Vec<Vec<u32>>,
    champions: Vec<u32>,
    runners: Vec<u32>,
    encounters: HashMap<(usize, u32, usize, usize), u32>,
    encounter_seen: HashSet<(usize, u32, usize, usize)>,
    rounds: HashMap<(usize, u32, usize), u32>,
}

impl Aggregator {
    pub fn new(stages: usize, teams: usize) -> Self {
        Self {
            positions: vec![vec![vec![0; teams]; teams]; stages],
            reached: vec![vec![0; teams]; stages],
            champions: vec![0; teams],
            runners: vec![0; teams],
            encounters: HashMap::new(),
            encounter_seen: HashSet::new(),
            rounds: HashMap::new(),
        }
    }

    pub fn stage(&mut self, plan: &PreparedPlan, index: usize, ordered: &[usize], kind: StageType) {
        for &team in ordered {
            self.reached[index][team] += 1;
        }
        match kind {
            StageType::RoundRobinTable => {
                for (position, &team) in ordered.iter().enumerate() {
                    self.positions[index][team][position] += 1;
                }
            }
            StageType::GroupedCompetitionTable => {
                for group in &plan.plan.groups {
                    for (position, &team) in ordered
                        .iter()
                        .filter(|&&team| group.team_ids().contains(&plan.ids[team]))
                        .enumerate()
                    {
                        self.positions[index][team][position] += 1;
                    }
                }
            }
            StageType::KnockoutBracket => {}
        }
    }

    pub fn encounter(&mut self, stage: usize, round: u32, a: usize, b: usize) {
        let key = (stage, round, a.min(b), a.max(b));
        if self.encounter_seen.insert(key) {
            *self.encounters.entry(key).or_default() += 1;
        }
    }

    pub fn begin_simulation(&mut self) {
        self.encounter_seen.clear();
    }

    pub fn round(&mut self, stage: usize, round: u32, teams: &[usize]) {
        for &team in teams {
            *self.rounds.entry((stage, round, team)).or_default() += 1;
        }
    }

    pub fn finish(&mut self, champion: usize, runner: usize) {
        self.champions[champion] += 1;
        self.runners[runner] += 1;
    }

    pub fn projection(self, plan: &PreparedPlan, seed: u64) -> PreseasonProjection {
        let total = plan.plan.config.simulation_count as f64;
        let team_count = plan.ids.len();
        let mut stages = Vec::new();
        for stage in 0..self.positions.len() {
            for team in 0..team_count {
                let reached = self.reached[stage][team];
                let histogram = &self.positions[stage][team];
                let position_total: u32 = histogram.iter().sum();
                let mean = if position_total > 0 {
                    Some(
                        histogram
                            .iter()
                            .enumerate()
                            .map(|(i, &n)| (i + 1) as f64 * n as f64)
                            .sum::<f64>()
                            / position_total as f64,
                    )
                } else {
                    None
                };
                let median = if position_total > 0 {
                    let mut running = 0;
                    Some(
                        histogram
                            .iter()
                            .position(|&n| {
                                running += n;
                                running * 2 >= position_total
                            })
                            .unwrap_or(0) as u32
                            + 1,
                    )
                } else {
                    None
                };
                let mode = if position_total > 0 {
                    Some(
                        histogram
                            .iter()
                            .enumerate()
                            .max_by_key(|(_, count)| *count)
                            .map(|(i, _)| i as u32 + 1)
                            .unwrap_or(1),
                    )
                } else {
                    None
                };
                stages.push(StageTeamProjection {
                    stage_index: stage as u32,
                    team_id: plan.ids[team],
                    reach_probability: reached as f64 / total,
                    mean_position: mean,
                    median_position: median,
                    modal_position: mode,
                    position_probabilities: histogram.iter().map(|&n| n as f64 / total).collect(),
                });
            }
        }
        let initial_table =
            plan.plan.stages[0].definition.stage_type() == StageType::RoundRobinTable;
        let mut table_rank: Vec<(usize, f64)> = stages
            .iter()
            .filter(|stage| stage.stage_index == 0)
            .filter(|_| initial_table)
            .filter_map(|stage| {
                stage
                    .mean_position
                    .map(|mean| (plan.index[&stage.team_id], mean))
            })
            .collect();
        table_rank.sort_by(|a, b| a.1.total_cmp(&b.1).then(a.0.cmp(&b.0)));
        let mut predicted_positions = vec![None; team_count];
        for (rank, &(team, _)) in table_rank.iter().enumerate() {
            predicted_positions[team] = Some(rank as u32 + 1);
        }
        let teams = (0..team_count)
            .map(|team| TeamProjection {
                team_id: plan.ids[team],
                projected_table_position: predicted_positions[team],
                top_four_probability: if initial_table && stages[team].mean_position.is_some() {
                    Some(self.positions[0][team].iter().take(4).sum::<u32>() as f64 / total)
                } else {
                    None
                },
                champion_probability: self.champions[team] as f64 / total,
                runner_up_probability: self.runners[team] as f64 / total,
            })
            .collect();
        let mut encounters: Vec<_> = self
            .encounters
            .into_iter()
            .map(|((stage, round, a, b), count)| EncounterProjection {
                stage_index: stage as u32,
                round_index: round,
                first_team_id: plan.ids[a],
                second_team_id: plan.ids[b],
                probability: count as f64 / total,
            })
            .collect();
        encounters.sort_by_key(|item| {
            (
                item.stage_index,
                item.round_index,
                item.first_team_id,
                item.second_team_id,
            )
        });
        let mut rounds: Vec<_> = self
            .rounds
            .into_iter()
            .map(|((stage, round, team), count)| RoundTeamProjection {
                stage_index: stage as u32,
                round_index: round,
                team_id: plan.ids[team],
                reach_probability: count as f64 / total,
            })
            .collect();
        rounds.sort_by_key(|item| (item.stage_index, item.round_index, item.team_id));
        PreseasonProjection {
            season_instance_id: plan.plan.season_instance_id,
            model_version: PRESEASON_MODEL_VERSION,
            simulation_count: plan.plan.config.simulation_count,
            random_seed: seed,
            config: plan.plan.config,
            goal_point_model: plan.plan.goal_point_model,
            teams,
            stages,
            encounters,
            rounds,
        }
    }
}
