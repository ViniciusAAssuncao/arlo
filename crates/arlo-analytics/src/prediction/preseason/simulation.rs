use super::aggregation::{Aggregator, PreseasonProjection};
use super::knockout::{margin_sign, run_knockout};
use super::plan::{CompetitionProjectionPlan, PreparedPlan, PRESEASON_MODEL_VERSION};
use super::schedule::stage_fixtures;
use super::standings::{rank_stage, Outcome, PlayedFixture};
use super::transition::next_participants;
use crate::error::AnalyticsResult;
use crate::power_ranking::PowerRating;
use crate::prediction::{predict_match, MatchPrediction};
use arlo_domain::StageType;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use rand_distr::StandardNormal;

pub fn project_preseason(plan: CompetitionProjectionPlan) -> AnalyticsResult<PreseasonProjection> {
    let plan = PreparedPlan::new(plan)?;
    let seed = deterministic_seed(
        plan.plan.season_instance_id.as_bytes(),
        PRESEASON_MODEL_VERSION,
    );
    let mut rng = ChaCha8Rng::seed_from_u64(seed);
    let mut aggregate = Aggregator::new(plan.plan.stages.len(), plan.ids.len());
    for _ in 0..plan.plan.config.simulation_count {
        aggregate.begin_simulation();
        let mut ratings = Vec::with_capacity(plan.ids.len());
        for team in &plan.plan.teams {
            let sigma = if team.has_rating_history {
                plan.plan.config.historical_rating_sigma
            } else {
                plan.plan.config.new_team_rating_sigma
            };
            let deviation: f64 = rng.sample(StandardNormal);
            ratings.push(PowerRating::new(
                team.seed.initial_rating().value() + sigma * deviation,
            )?);
        }
        let probabilities = probability_matrices(&plan, &ratings)?;
        let mut participants = plan.initial_participants.clone();
        let mut champion = None;
        let mut runner = None;
        let mut next_round = 0;
        for stage in 0..plan.plan.stages.len() {
            let kind = plan.plan.stages[stage].definition.stage_type();
            let mut sampler = MatchSampler {
                plan: &plan,
                probabilities: &probabilities,
                ratings: &ratings,
                rng: &mut rng,
            };
            let ordered = if kind == StageType::KnockoutBracket {
                let (ordered, winner, second, after_round) = run_knockout(
                    &plan,
                    stage,
                    &participants,
                    next_round,
                    &mut sampler,
                    &mut aggregate,
                )?;
                next_round = after_round;
                champion = Some(winner);
                runner = Some(second);
                ordered
            } else {
                let fixtures = stage_fixtures(&plan, stage, &participants, next_round, &mut rng)?;
                next_round = fixtures
                    .iter()
                    .map(|fixture| fixture.round + 1)
                    .max()
                    .unwrap_or(next_round);
                let mut results = Vec::with_capacity(fixtures.len());
                for fixture in fixtures {
                    aggregate.encounter(stage, fixture.round, fixture.home, fixture.away);
                    let mut sampler = MatchSampler {
                        plan: &plan,
                        probabilities: &probabilities,
                        ratings: &ratings,
                        rng: &mut rng,
                    };
                    results.push(
                        sampler
                            .sample(fixture.home, fixture.away, fixture.neutral, false)
                            .0,
                    );
                }
                rank_stage(&plan, &participants, &results, &mut rng)
            };
            aggregate.stage(&plan, stage, &ordered, kind);
            if stage + 1 < plan.plan.stages.len() {
                participants = next_participants(&plan, stage + 1, &ordered)?;
            } else if kind != StageType::KnockoutBracket {
                champion = ordered.first().copied();
                runner = ordered.get(1).copied();
            }
        }
        if let (Some(winner), Some(second)) = (champion, runner) {
            aggregate.finish(winner, second);
        }
    }
    Ok(aggregate.projection(&plan, seed))
}

pub(super) struct ProbabilityMatrices {
    home: Vec<MatchPrediction>,
    neutral: Vec<MatchPrediction>,
    size: usize,
}

fn probability_matrices(
    plan: &PreparedPlan,
    ratings: &[PowerRating],
) -> AnalyticsResult<ProbabilityMatrices> {
    let size = ratings.len();
    let mut home = Vec::with_capacity(size * size);
    let mut neutral = Vec::with_capacity(size * size);
    for &home_rating in ratings {
        for &away_rating in ratings {
            home.push(predict_match(
                home_rating,
                away_rating,
                false,
                plan.plan.forecast_parameters,
            )?);
            neutral.push(predict_match(
                home_rating,
                away_rating,
                true,
                plan.plan.forecast_parameters,
            )?);
        }
    }
    Ok(ProbabilityMatrices {
        home,
        neutral,
        size,
    })
}

pub(super) struct MatchSampler<'a, R: Rng> {
    pub plan: &'a PreparedPlan,
    pub probabilities: &'a ProbabilityMatrices,
    pub ratings: &'a [PowerRating],
    pub rng: &'a mut R,
}

impl<R: Rng> MatchSampler<'_, R> {
    pub fn sample(
        &mut self,
        home: usize,
        away: usize,
        neutral: bool,
        with_margin: bool,
    ) -> (PlayedFixture, i64) {
        let cell = home * self.probabilities.size + away;
        let probability = if neutral {
            self.probabilities.neutral[cell]
        } else {
            self.probabilities.home[cell]
        };
        let draw = self.rng.gen::<f64>();
        let outcome = if draw < probability.home_win {
            Outcome::Home
        } else if draw < probability.home_win + probability.draw {
            Outcome::Draw
        } else {
            Outcome::Away
        };
        let (home_gp, away_gp) = match self.plan.plan.goal_point_model {
            Some(model) => {
                let difference = (self.ratings[home].value() - self.ratings[away].value()) / 400.0;
                (
                    poisson(
                        self.rng,
                        (model.mean_per_team * (model.rating_slope * difference).exp())
                            .clamp(0.05, 20.0),
                    ),
                    poisson(
                        self.rng,
                        (model.mean_per_team * (-model.rating_slope * difference).exp())
                            .clamp(0.05, 20.0),
                    ),
                )
            }
            None => (0, 0),
        };
        let fixture = PlayedFixture {
            home,
            away,
            outcome,
            home_gp,
            away_gp,
        };
        if !with_margin {
            return (fixture, 0);
        }
        let mean = self.plan.plan.config.knockout_margin_mean.max(1.0);
        let magnitude = if mean <= 1.0 {
            1
        } else {
            (self.rng.gen::<f64>().max(f64::MIN_POSITIVE).ln() / (1.0 - 1.0 / mean).ln()).floor()
                as i64
                + 1
        };
        (fixture, margin_sign(fixture) * magnitude)
    }
}

fn poisson<R: Rng>(rng: &mut R, lambda: f64) -> u32 {
    let limit = (-lambda).exp();
    let mut product = 1.0;
    let mut count = 0;
    loop {
        product *= rng.gen::<f64>();
        if product <= limit {
            return count;
        }
        count += 1;
    }
}

fn deterministic_seed(bytes: &[u8; 16], version: u32) -> u64 {
    let mut hash = 0xcbf29ce484222325_u64;
    for byte in bytes.iter().copied().chain(version.to_le_bytes()) {
        hash = (hash ^ byte as u64).wrapping_mul(0x100000001b3);
    }
    hash
}
