use super::head_to_head::resolve_ties;
use super::plan::PreparedPlan;
use arlo_domain::TieBreakCriterion;
use rand::Rng;
use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Outcome {
    Home,
    Draw,
    Away,
}

#[derive(Debug, Clone, Copy)]
pub(super) struct PlayedFixture {
    pub home: usize,
    pub away: usize,
    pub outcome: Outcome,
    pub home_gp: u32,
    pub away_gp: u32,
}

#[derive(Debug, Clone, Default)]
pub(super) struct Record {
    played: u32,
    won: u32,
    drawn: u32,
    lost: u32,
    home_won: u32,
    away_won: u32,
    home_drawn: u32,
    away_drawn: u32,
    home_lost: u32,
    away_lost: u32,
    gp_for: u32,
    gp_against: u32,
}

pub(super) fn rank_stage<R: Rng>(
    plan: &PreparedPlan,
    participants: &[usize],
    fixtures: &[PlayedFixture],
    rng: &mut R,
) -> Vec<usize> {
    let records = records_for(plan.ids.len(), fixtures);
    let mut ordered = participants.to_vec();
    ordered.sort_by(|&a, &b| compare(plan, &records[a], &records[b]));
    resolve_ties(plan, ordered, &records, fixtures, rng)
}

pub(super) fn records_for(team_count: usize, fixtures: &[PlayedFixture]) -> Vec<Record> {
    let mut records = vec![Record::default(); team_count];
    for fixture in fixtures {
        let home = &mut records[fixture.home];
        home.played += 1;
        home.gp_for += fixture.home_gp;
        home.gp_against += fixture.away_gp;
        match fixture.outcome {
            Outcome::Home => {
                home.won += 1;
                home.home_won += 1;
            }
            Outcome::Draw => {
                home.drawn += 1;
                home.home_drawn += 1;
            }
            Outcome::Away => {
                home.lost += 1;
                home.home_lost += 1;
            }
        }
        let away = &mut records[fixture.away];
        away.played += 1;
        away.gp_for += fixture.away_gp;
        away.gp_against += fixture.home_gp;
        match fixture.outcome {
            Outcome::Home => {
                away.lost += 1;
                away.away_lost += 1;
            }
            Outcome::Draw => {
                away.drawn += 1;
                away.away_drawn += 1;
            }
            Outcome::Away => {
                away.won += 1;
                away.away_won += 1;
            }
        }
    }
    records
}

pub(super) fn compare(plan: &PreparedPlan, a: &Record, b: &Record) -> Ordering {
    let criteria = if plan.plan.tie_break_criteria.is_empty() {
        &[
            TieBreakCriterion::IspaTotal,
            TieBreakCriterion::QtaScore,
            TieBreakCriterion::GoalDifference,
            TieBreakCriterion::GoalPointsTotal,
        ][..]
    } else {
        &plan.plan.tie_break_criteria
    };
    for criterion in criteria {
        let order = match criterion {
            TieBreakCriterion::IspaTotal => spa(plan, b).total_cmp(&spa(plan, a)),
            TieBreakCriterion::QtaScore => qta(plan, b).total_cmp(&qta(plan, a)),
            TieBreakCriterion::GoalDifference => {
                let ad = a.gp_for as i64 - a.gp_against as i64;
                let bd = b.gp_for as i64 - b.gp_against as i64;
                bd.cmp(&ad)
            }
            TieBreakCriterion::GoalPointsTotal => b.gp_for.cmp(&a.gp_for),
            TieBreakCriterion::HeadToHead | TieBreakCriterion::Random => Ordering::Equal,
        };
        if order != Ordering::Equal {
            return order;
        }
    }
    Ordering::Equal
}

fn spa(plan: &PreparedPlan, record: &Record) -> f64 {
    if record.played == 0 {
        return 0.0;
    }
    let policy = &plan.plan.spa_policy;
    let base = (record.won as f64 * policy.win_weight()
        + record.drawn as f64 * policy.draw_weight()
        + record.lost as f64 * policy.loss_weight())
        / record.played as f64;
    let feo = if plan.plan.goal_point_model.is_some() {
        1.0 + ((record.gp_for as f64 - record.gp_against as f64)
            / (record.played as f64 * policy.feo_k_factor()))
        .tanh()
    } else {
        1.0
    };
    base * feo
}

fn qta(plan: &PreparedPlan, record: &Record) -> f64 {
    if record.played == 0 {
        return 0.0;
    }
    let policy = &plan.plan.qta_policy;
    (record.home_won as f64 * policy.home_win_weight()
        + record.away_won as f64 * policy.away_win_weight()
        + record.home_drawn as f64 * policy.home_draw_weight()
        + record.away_drawn as f64 * policy.away_draw_weight()
        + record.home_lost as f64 * policy.home_loss_weight()
        + record.away_lost as f64 * policy.away_loss_weight())
        / record.played as f64
}
