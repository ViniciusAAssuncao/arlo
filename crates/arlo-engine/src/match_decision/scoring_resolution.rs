use crate::attributes::profiles::get_duel_attribute_profiles as get_duel_profiles;
use crate::match_decision::scoring_request::ScoringAttemptRequest;
use crate::match_decision::scoring_types::{ScoringDecision, ScoringOpportunity};
use crate::possession::{locate_zone_default, LiveSequenceTracker};
use crate::resolution::calculate_player_duel_rating_with_state;
use crate::resolution::group_rating::calculate_player_duel_rating_from_table;
use crate::resolution::{AttributedDuelOutcome, DuelKind, DuelOutcome};
use crate::scoring_model::{
    calculate_scoring_probability, select_post_for_field_goal, ScoringKind, ScoringSituation,
};
use arlo_domain::sport_constants::{
    FIELD_GOAL_FIELDPOST_VALUE, FIELD_GOAL_GOALPOST_VALUE, FIELD_POINT_VALUE, GOAL_POINT_VALUE,
};
use arlo_domain::{AttributeKey, Position};
use arlo_events::ScoringPost;
use rand::Rng;
use smallvec::smallvec;
use uuid::Uuid;

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

pub fn duel_kind_for_opportunity(opportunity: ScoringOpportunity) -> DuelKind {
    match opportunity {
        ScoringOpportunity::FieldPoint | ScoringOpportunity::FieldGoal => {
            DuelKind::FieldGoalAttempt
        }
        ScoringOpportunity::GoalPoint | ScoringOpportunity::None => DuelKind::FinishingAttempt,
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
    let duel_kind = duel_kind_for_opportunity(request.opportunity);
    let (attacker_profile, defender_profile) = get_duel_profiles(duel_kind);

    let attacker_rating = match request.finisher_table {
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

    let zone = locate_zone_default(request.normalized_proximity, 145.0);

    let mut situation = ScoringSituation::new(
        zone,
        request.normalized_proximity,
        request.drives_completed,
        request.territory_advance_mirim,
        attacker_rating,
        defender_rating,
        request.defense_closed,
        request.origin,
    )
    .with_duel_context(request.context);

    if let Some(margin) = request.margin_context {
        situation = situation.with_margin(margin);
    }

    let difficulty_profile = request.difficulty_profile.unwrap_or_default();

    let scoring_kind = match request.opportunity {
        ScoringOpportunity::GoalPoint => ScoringKind::GoalPoint,
        ScoringOpportunity::FieldPoint => ScoringKind::FieldPoint,
        ScoringOpportunity::FieldGoal => {
            let decisions_val = request
                .finisher_table
                .map(|t| t.get(AttributeKey::Decisions))
                .unwrap_or(10.0);
            ScoringKind::FieldGoal(select_post_for_field_goal(
                &situation,
                &difficulty_profile,
                decisions_val,
                rng,
            ))
        }
        ScoringOpportunity::None => ScoringKind::FieldPoint,
    };

    let win_prob = calculate_scoring_probability(scoring_kind, &situation, &difficulty_profile);
    let attacker_won = win_prob.sample(rng);

    let raw_outcome = DuelOutcome::new(
        duel_kind,
        attacker_won,
        attacker_rating,
        defender_rating,
        win_prob,
        attacker_rating - defender_rating,
    );

    let decision = if attacker_won {
        match scoring_kind {
            ScoringKind::GoalPoint => ScoringDecision::GoalPoint {
                team_id: request.team_id,
                scorer_id: request.finisher.id(),
                artrine_id: request.artrine_id,
                assister_id: request.assister_id,
                drives_completed: request.drives_completed,
                points: goal_point_points(),
                post: ScoringPost::Goalpost,
            },
            ScoringKind::FieldPoint => ScoringDecision::FieldPoint {
                team_id: request.team_id,
                scorer_id: request.finisher.id(),
                territory_advance_mirim: request.territory_advance_mirim,
                drives_completed: request.drives_completed,
                points: field_point_points(),
                post: ScoringPost::Fieldpost,
            },
            ScoringKind::FieldGoal(post) => ScoringDecision::FieldGoal {
                team_id: request.team_id,
                scorer_id: request.finisher.id(),
                points: field_goal_points(post),
                post,
            },
        }
    } else {
        let attempted_post = match scoring_kind {
            ScoringKind::GoalPoint => ScoringPost::Goalpost,
            ScoringKind::FieldPoint => ScoringPost::Fieldpost,
            ScoringKind::FieldGoal(post) => post,
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
