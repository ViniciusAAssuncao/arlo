use crate::artrine::execution_outcome::ArtrineExecutionOutcome;
use crate::match_decision::finisher_selection::select_finisher;
use crate::match_decision::scoring::{
    evaluate_scoring_opportunity, resolve_scoring_attempt, ScoringOpportunity,
};
use crate::resolution::DuelContext;
use crate::spatial::DynamicSpatialMap;
use arlo_domain::pitch::Pitch;
use arlo_domain::sport_constants::{
    FIELD_GOAL_MIN_TERRITORY_ADVANCE_MIRIM_FIELDPOST,
    FIELD_GOAL_MIN_TERRITORY_ADVANCE_MIRIM_GOALPOST,
};
use arlo_domain::{AttributeKey, Player};
use arlo_events::ScoringPost;
use arlo_math::units::Position as VectorPosition;
use rand::Rng;
use std::collections::HashMap;
use uuid::Uuid;

pub fn execute_self_finish<R: Rng + ?Sized>(
    artrine: &Player,
    goalguard: &Player,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    offense_team_id: Uuid,
    defense_team_id: Uuid,
    drives_in_series: u32,
    accumulated_advance_mirim: f64,
    is_last_down: bool,
    start_pos: VectorPosition,
    context: &DuelContext,
    rng: &mut R,
) -> ArtrineExecutionOutcome {
    execute_finishing_with_player(
        artrine,
        artrine,
        goalguard,
        attribute_keys,
        offense_team_id,
        defense_team_id,
        drives_in_series,
        accumulated_advance_mirim,
        is_last_down,
        start_pos,
        context,
        rng,
    )
}

pub fn execute_cross_finish<R: Rng + ?Sized>(
    artrine: &Player,
    teammates: &[&Player],
    goalguard: &Player,
    spatial_map: &DynamicSpatialMap,
    pitch: &Pitch,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    offense_team_id: Uuid,
    defense_team_id: Uuid,
    drives_in_series: u32,
    accumulated_advance_mirim: f64,
    is_last_down: bool,
    attacking_positive_x: bool,
    start_pos: VectorPosition,
    context: &DuelContext,
    rng: &mut R,
) -> ArtrineExecutionOutcome {
    let eligible_teammates: Vec<&Player> = teammates
        .iter()
        .copied()
        .filter(|p| p.id() != artrine.id())
        .collect();

    let chosen_finisher_id = select_finisher(
        &eligible_teammates,
        spatial_map,
        pitch,
        attacking_positive_x,
        rng,
    );

    let finisher = chosen_finisher_id
        .and_then(|fid| eligible_teammates.iter().copied().find(|p| p.id() == fid))
        .unwrap_or(artrine);

    execute_finishing_with_player(
        finisher,
        artrine,
        goalguard,
        attribute_keys,
        offense_team_id,
        defense_team_id,
        drives_in_series,
        accumulated_advance_mirim,
        is_last_down,
        start_pos,
        context,
        rng,
    )
}

pub fn execute_finishing_with_player<R: Rng + ?Sized>(
    finisher: &Player,
    artrine: &Player,
    goalguard: &Player,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    offense_team_id: Uuid,
    defense_team_id: Uuid,
    drives_in_series: u32,
    accumulated_advance_mirim: f64,
    is_last_down: bool,
    start_pos: VectorPosition,
    context: &DuelContext,
    rng: &mut R,
) -> ArtrineExecutionOutcome {
    let mut opportunity =
        evaluate_scoring_opportunity(drives_in_series, accumulated_advance_mirim);

    if opportunity == ScoringOpportunity::None && is_last_down {
        if drives_in_series >= 2 {
            opportunity = ScoringOpportunity::FieldPoint;
        } else if accumulated_advance_mirim >= FIELD_GOAL_MIN_TERRITORY_ADVANCE_MIRIM_GOALPOST {
            opportunity = ScoringOpportunity::FieldGoal(ScoringPost::Goalpost);
        } else if accumulated_advance_mirim >= FIELD_GOAL_MIN_TERRITORY_ADVANCE_MIRIM_FIELDPOST
            || drives_in_series >= 1
        {
            opportunity = ScoringOpportunity::FieldGoal(ScoringPost::Fieldpost);
        }
    }

    let (scoring_decision, finish_duel) = resolve_scoring_attempt(
        finisher,
        goalguard,
        attribute_keys,
        offense_team_id,
        artrine.id(),
        opportunity,
        drives_in_series,
        accumulated_advance_mirim,
        context,
        rng,
    );

    let (turnover, recovering_player_id) = match &scoring_decision {
        crate::match_decision::scoring::ScoringDecision::Missed { .. } => {
            (Some(defense_team_id), Some(goalguard.id()))
        }
        _ => (None, None),
    };

    ArtrineExecutionOutcome {
        mirins_advanced: 0.0,
        drives_recorded: 0,
        drive_row_indices: Vec::new(),
        turnover,
        recovering_player_id,
        scoring_decision,
        elapsed_seconds: 1.5,
        end_position: start_pos,
        duels: vec![finish_duel],
    }
}