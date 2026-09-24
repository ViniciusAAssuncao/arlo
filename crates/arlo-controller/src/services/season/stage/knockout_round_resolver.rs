use crate::domain::season::{Fixture, FixtureStatus, KnockoutTie};
use arlo_domain::TieBreakCriterion;
use uuid::Uuid;

pub fn resolve_tie_winner(
    tie: &KnockoutTie,
    fixtures: &[Fixture],
    criteria: &[TieBreakCriterion],
) -> Option<Uuid> {
    if let Some(winner) = tie.aggregate_winner_team_id() {
        return Some(winner);
    }

    let leg1 = fixtures
        .iter()
        .find(|f| f.id() == tie.leg_one_fixture_id())?;

    if let Some(leg2_id) = tie.leg_two_fixture_id() {
        let leg2 = fixtures.iter().find(|f| f.id() == leg2_id)?;

        if leg1.status() != FixtureStatus::Completed && leg1.result().is_none() {
            return None;
        }
        if leg2.status() != FixtureStatus::Completed && leg2.result().is_none() {
            return None;
        }

        let leg1_res = leg1.result()?;
        let leg2_res = leg2.result()?;

        let team_high = tie.high_seed().team_id();
        let team_low = tie.low_seed().team_id();

        let mut high_score = 0;
        let mut low_score = 0;
        let mut high_gp = 0;
        let mut low_gp = 0;

        for (f, res) in [(leg1, leg1_res), (leg2, leg2_res)] {
            if f.home_team_id() == team_high {
                high_score += res.home_score();
                high_gp += res.home_goal_points();
            } else if f.home_team_id() == team_low {
                low_score += res.home_score();
                low_gp += res.home_goal_points();
            }

            if f.away_team_id() == team_high {
                high_score += res.away_score();
                high_gp += res.away_goal_points();
            } else if f.away_team_id() == team_low {
                low_score += res.away_score();
                low_gp += res.away_goal_points();
            }
        }

        if high_score > low_score {
            return Some(team_high);
        }
        if low_score > high_score {
            return Some(team_low);
        }

        for criterion in criteria {
            match criterion {
                TieBreakCriterion::GoalPointsTotal | TieBreakCriterion::GoalDifference => {
                    if high_gp > low_gp {
                        return Some(team_high);
                    }
                    if low_gp > high_gp {
                        return Some(team_low);
                    }
                }
                _ => {}
            }
        }

        Some(team_high)
    } else {
        if leg1.status() != FixtureStatus::Completed && leg1.result().is_none() {
            return None;
        }
        let leg1_res = leg1.result()?;

        if let Some(winner) = leg1_res.winner_team_id() {
            return Some(winner);
        }

        if leg1_res.home_score() > leg1_res.away_score() {
            return Some(leg1.home_team_id());
        }
        if leg1_res.away_score() > leg1_res.home_score() {
            return Some(leg1.away_team_id());
        }

        let (high_gp, low_gp) = if leg1.home_team_id() == tie.high_seed().team_id() {
            (leg1_res.home_goal_points(), leg1_res.away_goal_points())
        } else {
            (leg1_res.away_goal_points(), leg1_res.home_goal_points())
        };

        for criterion in criteria {
            match criterion {
                TieBreakCriterion::GoalPointsTotal | TieBreakCriterion::GoalDifference => {
                    if high_gp > low_gp {
                        return Some(tie.high_seed().team_id());
                    }
                    if low_gp > high_gp {
                        return Some(tie.low_seed().team_id());
                    }
                }
                _ => {}
            }
        }

        Some(tie.high_seed().team_id())
    }
}
