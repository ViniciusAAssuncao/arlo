use crate::domain::season::{KnockoutTie, PromotionRelegationOutcome, StandingsEntry};
use crate::error::{ControllerError, ControllerResult};
use crate::repositories::league_calendar::league_calendar_config_cache::get_or_load_league_calendar_config;
use crate::repositories::season::standings_cache::get_or_compute_standings;
use crate::services::season::persistence::load_stage_knockout_ties;
use arlo_domain::LeagueMovementRule;
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn resolve_promotion_relegation(
    pool: &SqlitePool,
    competition_id: Uuid,
    season_instance_id: Uuid,
) -> ControllerResult<PromotionRelegationOutcome> {
    let config = get_or_load_league_calendar_config(pool, competition_id)
        .await?
        .ok_or_else(|| {
            ControllerError::NotFound(format!(
                "League calendar config for competition {} not found",
                competition_id
            ))
        })?;

    let policy = config.promotion_relegation_policy();
    let promotion_rule = policy.promotion_rule();
    let relegation_rule = policy.relegation_rule();

    if promotion_rule == LeagueMovementRule::None && relegation_rule == LeagueMovementRule::None {
        return Ok(PromotionRelegationOutcome::empty());
    }

    let stages = arlo_persistence::repositories::season::season_stages::list_by_season_instance_id(
        pool,
        season_instance_id,
    )
    .await?;

    let mut cached_standings: Option<Vec<StandingsEntry>> = None;

    let needs_standings = matches!(promotion_rule, LeagueMovementRule::Automatic { .. })
        || matches!(relegation_rule, LeagueMovementRule::Automatic { .. });

    if needs_standings {
        let standings_stage_row = stages
            .iter()
            .find(|s| s.stage_order_index == (policy.standings_stage_order_index() as i32))
            .ok_or_else(|| {
                ControllerError::NotFound(format!(
                    "Standings stage with order index {} not found for season {}",
                    policy.standings_stage_order_index(),
                    season_instance_id
                ))
            })?;

        let standings_stage_id = Uuid::parse_str(&standings_stage_row.id)?;
        let standings = get_or_compute_standings(pool, standings_stage_id).await?;
        cached_standings = Some((*standings).clone());
    }

    let promoted_team_ids = match promotion_rule {
        LeagueMovementRule::None => Vec::new(),
        LeagueMovementRule::Automatic { count } => {
            let standings = cached_standings.as_ref().unwrap();
            let count = count as usize;
            if standings.len() < count {
                return Err(ControllerError::Validation(format!(
                    "Not enough teams in standings ({}) for automatic promotion count ({})",
                    standings.len(),
                    count
                )));
            }
            standings[..count].iter().map(|e| e.team_id()).collect()
        }
        LeagueMovementRule::PlayoffStage {
            stage_order_index,
            count,
        } => {
            let playoff_stage_row = stages
                .iter()
                .find(|s| s.stage_order_index == (stage_order_index as i32))
                .ok_or_else(|| {
                    ControllerError::NotFound(format!(
                        "Promotion playoff stage with order index {} not found",
                        stage_order_index
                    ))
                })?;

            let playoff_stage_id = Uuid::parse_str(&playoff_stage_row.id)?;
            let ties: Vec<KnockoutTie> = load_stage_knockout_ties(pool, playoff_stage_id).await?;

            let mut winners = Vec::with_capacity(ties.len());
            for tie in &ties {
                let winner = tie.aggregate_winner_team_id().ok_or_else(|| {
                    ControllerError::Validation(format!(
                        "Promotion playoff tie {} has no aggregate winner",
                        tie.id()
                    ))
                })?;
                winners.push(winner);
            }

            if winners.len() != (count as usize) {
                return Err(ControllerError::Validation(format!(
                    "Promotion playoff winners count ({}) does not match expected count ({})",
                    winners.len(),
                    count
                )));
            }

            winners
        }
    };

    let relegated_team_ids = match relegation_rule {
        LeagueMovementRule::None => Vec::new(),
        LeagueMovementRule::Automatic { count } => {
            let standings = cached_standings.as_ref().unwrap();
            let count = count as usize;
            if standings.len() < count {
                return Err(ControllerError::Validation(format!(
                    "Not enough teams in standings ({}) for automatic relegation count ({})",
                    standings.len(),
                    count
                )));
            }
            let start = standings.len() - count;
            standings[start..].iter().map(|e| e.team_id()).collect()
        }
        LeagueMovementRule::PlayoffStage {
            stage_order_index,
            count,
        } => {
            let playoff_stage_row = stages
                .iter()
                .find(|s| s.stage_order_index == (stage_order_index as i32))
                .ok_or_else(|| {
                    ControllerError::NotFound(format!(
                        "Relegation playoff stage with order index {} not found",
                        stage_order_index
                    ))
                })?;

            let playoff_stage_id = Uuid::parse_str(&playoff_stage_row.id)?;
            let ties: Vec<KnockoutTie> = load_stage_knockout_ties(pool, playoff_stage_id).await?;

            let mut losers = Vec::with_capacity(ties.len());
            for tie in &ties {
                let loser = tie.loser_team_id().ok_or_else(|| {
                    ControllerError::Validation(format!(
                        "Relegation playoff tie {} has no aggregate loser",
                        tie.id()
                    ))
                })?;
                losers.push(loser);
            }

            if losers.len() != (count as usize) {
                return Err(ControllerError::Validation(format!(
                    "Relegation playoff losers count ({}) does not match expected count ({})",
                    losers.len(),
                    count
                )));
            }

            losers
        }
    };

    Ok(PromotionRelegationOutcome::new(
        promoted_team_ids,
        relegated_team_ids,
    ))
}