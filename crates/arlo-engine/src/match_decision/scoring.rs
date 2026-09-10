use crate::physical::PhysicalState;
use crate::possession::LiveSequenceTracker;
use crate::resolution::calculate_player_duel_rating_with_state;
use crate::resolution::duel_profiles::get_duel_profiles;
use crate::resolution::resolver::{resolve_duel, DuelResolutionRequest};
use crate::resolution::{AttributedDuelOutcome, DuelContext, DuelKind};
use arlo_domain::sport_constants::{
    FIELD_GOAL_FIELDPOST_VALUE, FIELD_GOAL_GOALPOST_VALUE,
    FIELD_GOAL_MIN_TERRITORY_ADVANCE_MIRIM_FIELDPOST, FIELD_POINT_MIN_TERRITORY_ADVANCE_MIRIM,
    FIELD_POINT_REQUIRED_DRIVES, FIELD_POINT_VALUE, GOAL_POINT_REQUIRED_DRIVES, GOAL_POINT_VALUE,
};
use arlo_domain::{AttributeKey, Player, Position};
use arlo_events::ScoringPost;
use rand::Rng;
use serde::{Deserialize, Serialize};
use smallvec::smallvec;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ScoringOpportunity {
    GoalPoint,
    FieldPoint,
    FieldGoal(ScoringPost),
    None,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ScoringDecision {
    GoalPoint {
        team_id: Uuid,
        scorer_id: Uuid,
        artrine_id: Uuid,
        assister_id: Option<Uuid>,
        drives_completed: u32,
        points: u32,
        post: ScoringPost,
    },
    FieldPoint {
        team_id: Uuid,
        scorer_id: Uuid,
        territory_advance_mirim: f64,
        drives_completed: u32,
        points: u32,
        post: ScoringPost,
    },
    FieldGoal {
        team_id: Uuid,
        scorer_id: Uuid,
        points: u32,
        post: ScoringPost,
    },
    Missed {
        team_id: Uuid,
        scorer_id: Uuid,
        attempted_post: ScoringPost,
    },
    NoOpportunity,
}

impl ScoringDecision {
    pub fn is_scored(&self) -> bool {
        matches!(
            self,
            Self::GoalPoint { .. } | Self::FieldPoint { .. } | Self::FieldGoal { .. }
        )
    }

    pub fn points(&self) -> u32 {
        match self {
            Self::GoalPoint { points, .. } => *points,
            Self::FieldPoint { points, .. } => *points,
            Self::FieldGoal { points, .. } => *points,
            Self::Missed { .. } | Self::NoOpportunity => 0,
        }
    }
}

pub struct ScoringAttemptRequest<'a> {
    pub finisher: &'a Player,
    pub goalguard: &'a Player,
    pub attribute_keys: &'a HashMap<Uuid, AttributeKey>,
    pub team_id: Uuid,
    pub artrine_id: Uuid,
    pub assister_id: Option<Uuid>,
    pub opportunity: ScoringOpportunity,
    pub drives_completed: u32,
    pub territory_advance_mirim: f64,
    pub finisher_state: PhysicalState,
    pub goalguard_state: PhysicalState,
    pub context: &'a DuelContext,
}

impl<'a> ScoringAttemptRequest<'a> {
    pub fn new(
        finisher: &'a Player,
        goalguard: &'a Player,
        attribute_keys: &'a HashMap<Uuid, AttributeKey>,
        team_id: Uuid,
        artrine_id: Uuid,
        assister_id: Option<Uuid>,
        opportunity: ScoringOpportunity,
        drives_completed: u32,
        territory_advance_mirim: f64,
        context: &'a DuelContext,
    ) -> Self {
        Self {
            finisher,
            goalguard,
            attribute_keys,
            team_id,
            artrine_id,
            assister_id,
            opportunity,
            drives_completed,
            territory_advance_mirim,
            finisher_state: PhysicalState::initial(),
            goalguard_state: PhysicalState::initial(),
            context,
        }
    }

    pub fn with_fatigue(
        mut self,
        finisher_state: PhysicalState,
        goalguard_state: PhysicalState,
    ) -> Self {
        self.finisher_state = finisher_state;
        self.goalguard_state = goalguard_state;
        self
    }
}

pub fn can_attempt_goal_point(drives_in_series: u32) -> bool {
    drives_in_series >= GOAL_POINT_REQUIRED_DRIVES
}

pub fn can_attempt_field_point(drives_in_series: u32, territory_advance_mirim: f64) -> bool {
    drives_in_series >= FIELD_POINT_REQUIRED_DRIVES
        && territory_advance_mirim >= FIELD_POINT_MIN_TERRITORY_ADVANCE_MIRIM
}

pub fn can_attempt_field_goal(territory_advance_mirim: f64) -> bool {
    territory_advance_mirim >= FIELD_GOAL_MIN_TERRITORY_ADVANCE_MIRIM_FIELDPOST
}

pub fn determine_field_goal_post(
    finisher_rating: f64,
    territory_advance_mirim: f64,
) -> ScoringPost {
    let normalized_rating = (finisher_rating / 20.0).clamp(0.0, 1.0);
    let normalized_distance = (territory_advance_mirim / 10.0).clamp(0.0, 1.0);
    let score = normalized_rating * 0.6 + normalized_distance * 0.4;
    if score >= 0.5 {
        ScoringPost::Goalpost
    } else {
        ScoringPost::Fieldpost
    }
}

pub fn evaluate_scoring_opportunity(
    is_bonus_phase: bool,
    drives_in_series: u32,
    territory_advance_mirim: f64,
    finisher_rating: f64,
) -> ScoringOpportunity {
    if is_bonus_phase {
        let post = determine_field_goal_post(finisher_rating, territory_advance_mirim);
        ScoringOpportunity::FieldGoal(post)
    } else if can_attempt_goal_point(drives_in_series) {
        ScoringOpportunity::GoalPoint
    } else if can_attempt_field_point(drives_in_series, territory_advance_mirim) {
        ScoringOpportunity::FieldPoint
    } else {
        ScoringOpportunity::None
    }
}

pub fn goal_point_points() -> u32 {
    GOAL_POINT_VALUE as u32
}

pub fn field_point_points() -> u32 {
    FIELD_POINT_VALUE as u32
}

pub fn field_goal_points(post: ScoringPost) -> u32 {
    match post {
        ScoringPost::Goalpost => FIELD_GOAL_GOALPOST_VALUE as u32,
        ScoringPost::Fieldpost => FIELD_GOAL_FIELDPOST_VALUE as u32,
    }
}

pub fn extract_assister_from_sequence(
    live_sequence: &LiveSequenceTracker,
    finisher_id: Uuid,
) -> Option<Uuid> {
    live_sequence.primary_assister(finisher_id)
}

pub fn extract_assist_tree_from_sequence(
    live_sequence: &LiveSequenceTracker,
    finisher_id: Uuid,
) -> (Option<Uuid>, Option<Uuid>) {
    live_sequence.assist_chain(finisher_id)
}

pub fn resolve_scoring_attempt<R: Rng + ?Sized>(
    request: ScoringAttemptRequest<'_>,
    rng: &mut R,
) -> (ScoringDecision, AttributedDuelOutcome) {
    let (attacker_profile, defender_profile) = get_duel_profiles(DuelKind::FinishingAttempt);
    let mut attacker_rating = calculate_player_duel_rating_with_state(
        request.finisher,
        Position::CenterOffense,
        request.attribute_keys,
        attacker_profile,
        &request.finisher_state,
    );
    let defender_rating = calculate_player_duel_rating_with_state(
        request.goalguard,
        Position::Goalguard,
        request.attribute_keys,
        defender_profile,
        &request.goalguard_state,
    );

    let distance_adjustment = match request.opportunity {
        ScoringOpportunity::GoalPoint => 0.5,
        ScoringOpportunity::FieldPoint => {
            ((request.territory_advance_mirim - 8.0) * 0.25).clamp(-1.0, 1.0)
        }
        ScoringOpportunity::FieldGoal(ScoringPost::Goalpost) => {
            ((request.territory_advance_mirim - 5.0) * 0.35 - 0.5).clamp(-3.0, 0.5)
        }
        ScoringOpportunity::FieldGoal(ScoringPost::Fieldpost) => {
            ((request.territory_advance_mirim - 3.0) * 0.35 - 1.0).clamp(-3.5, 0.0)
        }
        ScoringOpportunity::None => -5.0,
    };

    attacker_rating += distance_adjustment;

    let duel_req = DuelResolutionRequest::with_states(
        DuelKind::FinishingAttempt,
        attacker_rating,
        defender_rating,
        request.finisher,
        request.goalguard,
        request.finisher_state,
        request.goalguard_state,
        request.attribute_keys,
        request.context,
    );

    let raw_outcome = resolve_duel(duel_req, rng);

    let decision = if raw_outcome.attacker_won() {
        match request.opportunity {
            ScoringOpportunity::GoalPoint => {
                let pts = goal_point_points();
                ScoringDecision::GoalPoint {
                    team_id: request.team_id,
                    scorer_id: request.finisher.id(),
                    artrine_id: request.artrine_id,
                    assister_id: request.assister_id,
                    drives_completed: request.drives_completed,
                    points: pts,
                    post: ScoringPost::Goalpost,
                }
            }
            ScoringOpportunity::FieldPoint => {
                let pts = field_point_points();
                ScoringDecision::FieldPoint {
                    team_id: request.team_id,
                    scorer_id: request.finisher.id(),
                    territory_advance_mirim: request.territory_advance_mirim,
                    drives_completed: request.drives_completed,
                    points: pts,
                    post: ScoringPost::Fieldpost,
                }
            }
            ScoringOpportunity::FieldGoal(post) => {
                let pts = field_goal_points(post);
                ScoringDecision::FieldGoal {
                    team_id: request.team_id,
                    scorer_id: request.finisher.id(),
                    points: pts,
                    post,
                }
            }
            ScoringOpportunity::None => ScoringDecision::NoOpportunity,
        }
    } else {
        let attempted_post = match request.opportunity {
            ScoringOpportunity::GoalPoint => ScoringPost::Goalpost,
            ScoringOpportunity::FieldPoint => ScoringPost::Fieldpost,
            ScoringOpportunity::FieldGoal(post) => post,
            ScoringOpportunity::None => ScoringPost::Fieldpost,
        };
        ScoringDecision::Missed {
            team_id: request.team_id,
            scorer_id: request.finisher.id(),
            attempted_post,
        }
    };

    let outcome = AttributedDuelOutcome::new(
        raw_outcome,
        smallvec![request.finisher.id()],
        smallvec![request.goalguard.id()],
    );

    crate::psychology::systems::instrumentation::instrument_scoring_attempt(
        &decision,
        request.opportunity,
        request.finisher.id(),
        request.goalguard.id(),
        raw_outcome.win_probability().value(),
    );

    (decision, outcome)
}