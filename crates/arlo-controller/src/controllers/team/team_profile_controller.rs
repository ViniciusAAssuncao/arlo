use crate::dto::team::{TeamProfileDto, TeamVenueDto};
use crate::error::{ControllerError, ControllerResult};
use crate::repositories::attribute::attribute_definition_cache::get_or_load_manager_attribute_definitions;
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn get_team_profile(
    pool: &SqlitePool,
    team_id: Uuid,
) -> ControllerResult<TeamProfileDto> {
    let team = arlo_db::repositories::team::get_by_id(pool, team_id)
        .await
        .map_err(|e| ControllerError::InvalidData(e.to_string()))?
        .ok_or_else(|| ControllerError::NotFound(format!("Team {} not found", team_id)))?;

    let country_name = match arlo_db::repositories::country::get_by_id(pool, team.country_id())
        .await
        .map_err(|e| ControllerError::InvalidData(e.to_string()))?
    {
        Some(country) => country.name().to_string(),
        None => String::new(),
    };

    let venue = match team.home_venue_id() {
        Some(venue_id) => {
            let venue_opt = arlo_db::repositories::venue::get_by_id(pool, venue_id)
                .await
                .map_err(|e| ControllerError::InvalidData(e.to_string()))?;
            venue_opt.map(|v| TeamVenueDto {
                id: v.id().to_string(),
                name: v.name().to_string(),
                capacity: v.capacity(),
                pitch_length_mirim: v.pitch_length_mirim(),
                pitch_width_mirim: v.pitch_width_mirim(),
            })
        }
        None => None,
    };

    let manager_defs = get_or_load_manager_attribute_definitions(pool).await?;
    let managers = arlo_db::repositories::manager::list_by_team_id(pool, team_id, &manager_defs)
        .await
        .map_err(|e| ControllerError::InvalidData(e.to_string()))?;
    let (manager_id, manager_name) = match managers.into_iter().next() {
        Some(m) => (Some(m.id().to_string()), Some(m.person().name().to_string())),
        None => (None, None),
    };

    let (league_id, league_name, division_index) = match team.league_id() {
        Some(lid) => {
            let league_opt = arlo_db::repositories::league::get_by_competition_id(pool, lid)
                .await
                .map_err(|e| ControllerError::InvalidData(e.to_string()))?;
            match league_opt {
                Some(league) => (
                    Some(lid.to_string()),
                    Some(league.competition().name().to_string()),
                    Some(league.division_index()),
                ),
                None => {
                    let comp_opt = arlo_db::repositories::competition::get_by_id(pool, lid)
                        .await
                        .map_err(|e| ControllerError::InvalidData(e.to_string()))?;
                    (
                        Some(lid.to_string()),
                        comp_opt.map(|c| c.name().to_string()),
                        None,
                    )
                }
            }
        }
        None => (None, None, None),
    };

    Ok(TeamProfileDto {
        id: team.id().to_string(),
        name: team.name().to_string(),
        country_id: team.country_id().to_string(),
        country_name,
        prestige: team.prestige(),
        founded_at_unix_seconds: team.founded_at_unix_seconds(),
        primary_color_hex: team.primary_color_hex().map(str::to_string),
        secondary_color_hex: team.secondary_color_hex().map(str::to_string),
        venue,
        manager_id,
        manager_name,
        league_id,
        league_name,
        division_index,
    })
}