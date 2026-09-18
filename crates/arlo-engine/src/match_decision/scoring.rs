pub use crate::match_decision::rules::{
    can_attempt_field_goal, can_attempt_field_point, can_attempt_goal_point,
    evaluate_scoring_opportunity, validate_bonus_phase_field_goal, validate_field_point,
    validate_goal_point, validate_scoring_opportunity, ScoringValidationError,
};

use crate::attributes::profiles::get_duel_attribute_profiles as get_duel_profiles;
use crate::attributes::PlayerAttributeTable;
use crate::physical::PhysicalState;
use crate::possession::LiveSequenceTracker;
use crate::resolution::calculate_player_duel_rating_with_state;
use crate::resolution::group_rating::calculate_player_duel_rating_from_table;
use crate::resolution::outcome_distribution::scoring_distance_adjustment;
use crate::resolution::resolver::{resolve_duel, DuelResolutionRequest};
use crate::resolution::{AttributedDuelOutcome, DuelContext, DuelKind};
use arlo_domain::sport_constants::{
    FIELD_GOAL_FIELDPOST_VALUE, FIELD_GOAL_GOALPOST_VALUE, FIELD_POINT_VALUE, GOAL_POINT_VALUE,
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

    pub fn scorer_id(&self) -> Option<Uuid> {
        match self {
            Self::GoalPoint { scorer_id, .. } => Some(*scorer_id),
            Self::FieldPoint { scorer_id, .. } => Some(*scorer_id),
            Self::FieldGoal { scorer_id, .. } => Some(*scorer_id),
            Self::Missed { scorer_id, .. } => Some(*scorer_id),
            Self::NoOpportunity => None,
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
    pub finisher_table: Option<&'a PlayerAttributeTable>,
    pub goalguard_table: Option<&'a PlayerAttributeTable>,
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
            finisher_table: None,
            goalguard_table: None,
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

    pub fn with_tables(
        mut self,
        finisher_table: Option<&'a PlayerAttributeTable>,
        goalguard_table: Option<&'a PlayerAttributeTable>,
    ) -> Self {
        self.finisher_table = finisher_table;
        self.goalguard_table = goalguard_table;
        self
    }
}

pub fn determine_field_goal_post(
    finisher_rating: f64,
    territory_advance_mirim: f64,
) -> ScoringPost {
    crate::set_piece::select_kick_post(finisher_rating, territory_advance_mirim)
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

pub fn duel_kind_for_opportunity(opportunity: ScoringOpportunity) -> DuelKind {
    match opportunity {
        ScoringOpportunity::FieldPoint | ScoringOpportunity::FieldGoal(_) => {
            DuelKind::FieldGoalAttempt
        }
        ScoringOpportunity::GoalPoint | ScoringOpportunity::None => DuelKind::FinishingAttempt,
    }
}

pub fn resolve_scoring_attempt<R: Rng + ?Sized>(
    request: ScoringAttemptRequest<'_>,
    rng: &mut R,
) -> (ScoringDecision, AttributedDuelOutcome) {
    let duel_kind = duel_kind_for_opportunity(request.opportunity);
    let (attacker_profile, defender_profile) = get_duel_profiles(duel_kind);
    let mut attacker_rating = match request.finisher_table {
        Some(table) => calculate_player_duel_rating_from_table(
            request.finisher,
            Position::CenterOffense,
            table,
            &attacker_profile,
            &request.finisher_state,
        ),
        None => calculate_player_duel_rating_with_state(
            request.finisher,
            Position::CenterOffense,
            request.attribute_keys,
            &attacker_profile,
            &request.finisher_state,
        ),
    };
    let defender_rating = match request.goalguard_table {
        Some(table) => calculate_player_duel_rating_from_table(
            request.goalguard,
            Position::Goalguard,
            table,
            &defender_profile,
            &request.goalguard_state,
        ),
        None => calculate_player_duel_rating_with_state(
            request.goalguard,
            Position::Goalguard,
            request.attribute_keys,
            &defender_profile,
            &request.goalguard_state,
        ),
    };

    let is_valid = request.opportunity != ScoringOpportunity::None;
    let distance_adjustment =
        scoring_distance_adjustment(request.territory_advance_mirim, is_valid);

    attacker_rating += distance_adjustment;

    match request.opportunity {
        ScoringOpportunity::GoalPoint => {
            attacker_rating -= 1.6;
        }
        ScoringOpportunity::FieldPoint | ScoringOpportunity::FieldGoal(_) => {
            attacker_rating += 1.2;
        }
        ScoringOpportunity::None => {}
    }

    let req = DuelResolutionRequest::with_states(
        duel_kind,
        attacker_rating,
        defender_rating,
        request.finisher,
        request.goalguard,
        request.finisher_state,
        request.goalguard_state,
        request.attribute_keys,
        request.context,
    )
    .with_tables(request.finisher_table, request.goalguard_table);

    let raw_outcome = resolve_duel(req, rng);

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

    (decision, outcome)
}