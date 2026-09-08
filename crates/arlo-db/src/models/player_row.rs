use crate::error::{DbError, DbResult};
use arlo_domain::{CaptaincyRole, Player, PlayerAttributeValue, PlayerPosition};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct PlayerRow {
    pub id: String,
    pub name: String,
    pub height_m: f64,
    pub birthdate_unix_seconds: i64,
    pub nationality_id: String,
    pub team_id: Option<String>,
    pub squad_number: Option<i32>,
    pub captaincy_role: Option<String>,
}

impl PlayerRow {
    pub fn to_domain(
        &self,
        positions: Vec<PlayerPosition>,
        attributes: Vec<PlayerAttributeValue>,
    ) -> DbResult<Player> {
        let id = Uuid::parse_str(&self.id)?;
        let nationality_id = Uuid::parse_str(&self.nationality_id)?;
        let team_id = match &self.team_id {
            Some(tid) => Some(Uuid::parse_str(tid)?),
            None => None,
        };
        let captaincy_role = match &self.captaincy_role {
            Some(role) => match role.as_str() {
                "Captain" => Some(CaptaincyRole::Captain),
                "ViceCaptain" => Some(CaptaincyRole::ViceCaptain),
                _ => {
                    return Err(DbError::InvalidEnum(format!(
                        "Invalid captaincy role: {role}"
                    )))
                }
            },
            None => None,
        };
        let builder = Player::builder(
            id,
            &self.name,
            self.height_m,
            self.birthdate_unix_seconds,
            nationality_id,
        )
        .with_team_id(team_id)
        .with_squad_number(self.squad_number)
        .with_captaincy_role(captaincy_role)
        .with_positions(positions)
        .with_attributes(attributes);

        builder.build().map_err(Into::into)
    }
}
