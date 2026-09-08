use arlo_domain::sport_constants::{
    FIELD_GOAL_FIELDPOST_VALUE, FIELD_GOAL_GOALPOST_VALUE, FIELD_POINT_VALUE, GOAL_POINT_VALUE,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ScoringPost {
    Goalpost,
    Fieldpost,
}

impl ScoringPost {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Goalpost => "Goalpost",
            Self::Fieldpost => "Fieldpost",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GoalPointScored {
    team_id: Uuid,
    scorer_id: Uuid,
    artrine_id: Uuid,
    assister_id: Option<Uuid>,
    drives_completed: u32,
    points: u32,
    post: ScoringPost,
}

impl GoalPointScored {
    pub fn new(
        team_id: Uuid,
        scorer_id: Uuid,
        artrine_id: Uuid,
        assister_id: Option<Uuid>,
        drives_completed: u32,
    ) -> Self {
        Self {
            team_id,
            scorer_id,
            artrine_id,
            assister_id,
            drives_completed,
            points: GOAL_POINT_VALUE as u32,
            post: ScoringPost::Goalpost,
        }
    }

    pub fn team_id(&self) -> Uuid {
        self.team_id
    }

    pub fn scorer_id(&self) -> Uuid {
        self.scorer_id
    }

    pub fn artrine_id(&self) -> Uuid {
        self.artrine_id
    }

    pub fn assister_id(&self) -> Option<Uuid> {
        self.assister_id
    }

    pub fn drives_completed(&self) -> u32 {
        self.drives_completed
    }

    pub fn points(&self) -> u32 {
        self.points
    }

    pub fn post(&self) -> ScoringPost {
        self.post
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FieldPointScored {
    team_id: Uuid,
    scorer_id: Uuid,
    territory_advance_mirim: f64,
    drives_completed: u32,
    points: u32,
    post: ScoringPost,
}

impl FieldPointScored {
    pub fn new(
        team_id: Uuid,
        scorer_id: Uuid,
        territory_advance_mirim: f64,
        drives_completed: u32,
    ) -> Self {
        Self {
            team_id,
            scorer_id,
            territory_advance_mirim,
            drives_completed,
            points: FIELD_POINT_VALUE as u32,
            post: ScoringPost::Fieldpost,
        }
    }

    pub fn team_id(&self) -> Uuid {
        self.team_id
    }

    pub fn scorer_id(&self) -> Uuid {
        self.scorer_id
    }

    pub fn territory_advance_mirim(&self) -> f64 {
        self.territory_advance_mirim
    }

    pub fn drives_completed(&self) -> u32 {
        self.drives_completed
    }

    pub fn points(&self) -> u32 {
        self.points
    }

    pub fn post(&self) -> ScoringPost {
        self.post
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FieldGoalScored {
    team_id: Uuid,
    scorer_id: Uuid,
    post: ScoringPost,
    points: u32,
}

impl FieldGoalScored {
    pub fn new(team_id: Uuid, scorer_id: Uuid, post: ScoringPost) -> Self {
        let points = match post {
            ScoringPost::Goalpost => FIELD_GOAL_GOALPOST_VALUE as u32,
            ScoringPost::Fieldpost => FIELD_GOAL_FIELDPOST_VALUE as u32,
        };
        Self {
            team_id,
            scorer_id,
            post,
            points,
        }
    }

    pub fn team_id(&self) -> Uuid {
        self.team_id
    }

    pub fn scorer_id(&self) -> Uuid {
        self.scorer_id
    }

    pub fn post(&self) -> ScoringPost {
        self.post
    }

    pub fn points(&self) -> u32 {
        self.points
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScoringAttemptMissed {
    team_id: Uuid,
    scorer_id: Uuid,
    attempted_post: ScoringPost,
}

impl ScoringAttemptMissed {
    pub fn new(team_id: Uuid, scorer_id: Uuid, attempted_post: ScoringPost) -> Self {
        Self {
            team_id,
            scorer_id,
            attempted_post,
        }
    }

    pub fn team_id(&self) -> Uuid {
        self.team_id
    }

    pub fn scorer_id(&self) -> Uuid {
        self.scorer_id
    }

    pub fn attempted_post(&self) -> ScoringPost {
        self.attempted_post
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ScoringEvent {
    GoalPoint(GoalPointScored),
    FieldPoint(FieldPointScored),
    FieldGoal(FieldGoalScored),
    AttemptMissed(ScoringAttemptMissed),
}

impl ScoringEvent {
    pub fn points(&self) -> u32 {
        match self {
            Self::GoalPoint(ev) => ev.points(),
            Self::FieldPoint(ev) => ev.points(),
            Self::FieldGoal(ev) => ev.points(),
            Self::AttemptMissed(_) => 0,
        }
    }

    pub fn team_id(&self) -> Uuid {
        match self {
            Self::GoalPoint(ev) => ev.team_id(),
            Self::FieldPoint(ev) => ev.team_id(),
            Self::FieldGoal(ev) => ev.team_id(),
            Self::AttemptMissed(ev) => ev.team_id(),
        }
    }

    pub fn scorer_id(&self) -> Uuid {
        match self {
            Self::GoalPoint(ev) => ev.scorer_id(),
            Self::FieldPoint(ev) => ev.scorer_id(),
            Self::FieldGoal(ev) => ev.scorer_id(),
            Self::AttemptMissed(ev) => ev.scorer_id(),
        }
    }

    pub fn post(&self) -> ScoringPost {
        match self {
            Self::GoalPoint(ev) => ev.post(),
            Self::FieldPoint(ev) => ev.post(),
            Self::FieldGoal(ev) => ev.post(),
            Self::AttemptMissed(ev) => ev.attempted_post(),
        }
    }
}

impl From<GoalPointScored> for ScoringEvent {
    fn from(ev: GoalPointScored) -> Self {
        Self::GoalPoint(ev)
    }
}

impl From<FieldPointScored> for ScoringEvent {
    fn from(ev: FieldPointScored) -> Self {
        Self::FieldPoint(ev)
    }
}

impl From<FieldGoalScored> for ScoringEvent {
    fn from(ev: FieldGoalScored) -> Self {
        Self::FieldGoal(ev)
    }
}

impl From<ScoringAttemptMissed> for ScoringEvent {
    fn from(ev: ScoringAttemptMissed) -> Self {
        Self::AttemptMissed(ev)
    }
}