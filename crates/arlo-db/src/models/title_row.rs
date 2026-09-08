use crate::error::{DbError, DbResult};
use arlo_domain::{Title, TitleWinner};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct TitleRow {
    pub id: String,
    pub competition_id: String,
    pub season_label: String,
    pub winner_team_id: Option<String>,
    pub winner_federation_id: Option<String>,
}

pub fn parse_title_winner(
    winner_team_id: Option<String>,
    winner_federation_id: Option<String>,
) -> DbResult<TitleWinner> {
    match (winner_team_id, winner_federation_id) {
        (Some(team_id), None) => {
            let id = Uuid::parse_str(&team_id)?;
            Ok(TitleWinner::Team(id))
        }
        (None, Some(fed_id)) => {
            let id = Uuid::parse_str(&fed_id)?;
            Ok(TitleWinner::Federation(id))
        }
        _ => Err(DbError::InvalidData(
            "Exactly one of winner_team_id or winner_federation_id must be set".to_string(),
        )),
    }
}

impl TitleRow {
    pub fn to_domain(&self) -> DbResult<Title> {
        let id = Uuid::parse_str(&self.id)?;
        let competition_id = Uuid::parse_str(&self.competition_id)?;
        let winner = parse_title_winner(
            self.winner_team_id.clone(),
            self.winner_federation_id.clone(),
        )?;
        Title::new(id, competition_id, &self.season_label, winner).map_err(Into::into)
    }
}
