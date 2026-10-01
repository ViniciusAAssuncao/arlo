use crate::domain::season::PromotionRelegationOutcome;
use crate::error::{ControllerError, ControllerResult};
use crate::repositories::league_calendar::league_calendar_config_cache::get_or_load_league_calendar_config;
use crate::services::season::progression::promotion_relegation_resolver::resolve_promotion_relegation;
use arlo_persistence::models::season::PromotionRelegationResultRow;
use arlo_persistence::persister::PromotionRelegationResultPersister;
use sqlx::SqlitePool;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

pub async fn resolve_and_apply_promotion_relegation(
    pool: &SqlitePool,
    competition_id: Uuid,
    season_instance_id: Uuid,
) -> ControllerResult<PromotionRelegationOutcome> {
    let outcome = resolve_promotion_relegation(pool, competition_id, season_instance_id).await?;

    if outcome.promoted_team_ids().is_empty() && outcome.relegated_team_ids().is_empty() {
        return Ok(outcome);
    }

    let config = get_or_load_league_calendar_config(pool, competition_id)
        .await?
        .ok_or_else(|| {
            ControllerError::NotFound(format!(
                "League calendar config for competition {} not found",
                competition_id
            ))
        })?;

    let policy = config.promotion_relegation_policy();
    let promo_target = policy.promotion_target_league_id();
    let releg_target = policy.relegation_target_league_id();

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0);

    let mut result_rows =
        Vec::with_capacity(outcome.promoted_team_ids().len() + outcome.relegated_team_ids().len());

    for (idx, &team_id) in outcome.promoted_team_ids().iter().enumerate() {
        result_rows.push(PromotionRelegationResultRow::new(
            Uuid::new_v4(),
            season_instance_id,
            team_id,
            "Promoted",
            competition_id,
            promo_target,
            Some((idx + 1) as u32),
            now,
        ));
    }

    for (idx, &team_id) in outcome.relegated_team_ids().iter().enumerate() {
        result_rows.push(PromotionRelegationResultRow::new(
            Uuid::new_v4(),
            season_instance_id,
            team_id,
            "Relegated",
            competition_id,
            releg_target,
            Some((idx + 1) as u32),
            now,
        ));
    }

    PromotionRelegationResultPersister::persist_results(pool, &result_rows).await?;

    for &team_id in outcome.promoted_team_ids() {
        arlo_db::repositories::team::update_league_id(pool, team_id, promo_target)
            .await
            .map_err(|e| ControllerError::InvalidData(e.to_string()))?;
    }

    for &team_id in outcome.relegated_team_ids() {
        arlo_db::repositories::team::update_league_id(pool, team_id, releg_target)
            .await
            .map_err(|e| ControllerError::InvalidData(e.to_string()))?;
    }

    Ok(outcome)
}
