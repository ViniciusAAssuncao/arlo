use crate::domain::calendar::CalendarDate;
use crate::domain::season::fixture_result::FixtureResult;
use crate::domain::season::fixture_status::FixtureStatus;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Fixture {
    id: Uuid,
    season_stage_id: Uuid,
    round_index: u32,
    home_team_id: Uuid,
    away_team_id: Uuid,
    is_neutral_venue: bool,
    venue_id: Option<Uuid>,
    scheduled_date: CalendarDate,
    status: FixtureStatus,
    result: Option<FixtureResult>,
}

impl Fixture {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: Uuid,
        season_stage_id: Uuid,
        round_index: u32,
        home_team_id: Uuid,
        away_team_id: Uuid,
        is_neutral_venue: bool,
        venue_id: Option<Uuid>,
        scheduled_date: CalendarDate,
        status: FixtureStatus,
        result: Option<FixtureResult>,
    ) -> Self {
        Self {
            id,
            season_stage_id,
            round_index,
            home_team_id,
            away_team_id,
            is_neutral_venue,
            venue_id,
            scheduled_date,
            status,
            result,
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn season_stage_id(&self) -> Uuid {
        self.season_stage_id
    }

    pub fn round_index(&self) -> u32 {
        self.round_index
    }

    pub fn home_team_id(&self) -> Uuid {
        self.home_team_id
    }

    pub fn away_team_id(&self) -> Uuid {
        self.away_team_id
    }

    pub fn is_neutral_venue(&self) -> bool {
        self.is_neutral_venue
    }

    pub fn venue_id(&self) -> Option<Uuid> {
        self.venue_id
    }

    pub fn scheduled_date(&self) -> CalendarDate {
        self.scheduled_date
    }

    pub fn status(&self) -> FixtureStatus {
        self.status
    }

    pub fn result(&self) -> Option<FixtureResult> {
        self.result
    }
}
