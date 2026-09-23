use crate::domain::season::Fixture;
use crate::error::{ControllerError, ControllerResult};
use crate::repositories::venue::eligible_venue_cache::{
    get_or_load_all_venues, get_or_load_venues_for_country,
};
use crate::services::season::standings::random_tiebreak_resolver::seed_from_uuid;
use crate::services::season::venue::neutral_venue_selector::select_neutral_venue;
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn assign_neutral_venues_to_fixtures(
    pool: &SqlitePool,
    competition_id: Uuid,
    fixtures: &mut [Fixture],
) -> ControllerResult<()> {
    let has_unassigned_neutral = fixtures
        .iter()
        .any(|f| f.is_neutral_venue() && f.venue_id().is_none());

    if !has_unassigned_neutral {
        return Ok(());
    }

    let competition = arlo_db::repositories::competition::get_by_id(pool, competition_id)
        .await
        .map_err(|e| ControllerError::InvalidData(e.to_string()))?
        .ok_or_else(|| {
            ControllerError::NotFound(format!("Competition {} not found", competition_id))
        })?;

    let venues = if let Some(country_id) = competition.country_id() {
        get_or_load_venues_for_country(pool, country_id).await?
    } else {
        get_or_load_all_venues(pool).await?
    };

    for fixture in fixtures.iter_mut() {
        if fixture.is_neutral_venue() && fixture.venue_id().is_none() {
            let seed = seed_from_uuid(fixture.id());
            if let Some(venue_id) = select_neutral_venue(
                &venues,
                fixture.home_team_id(),
                fixture.away_team_id(),
                seed,
            ) {
                *fixture = fixture.with_venue_id(Some(venue_id));
            }
        }
    }

    Ok(())
}
