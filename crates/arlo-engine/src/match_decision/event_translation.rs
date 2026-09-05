use crate::match_decision::scoring::ScoringDecision;
use crate::resolution::{DuelKind as EngineDuelKind, DuelOutcome};
use arlo_events::{
    CallToActionStarted, CountdownReason, CountdownToSizeStarted, DownAdvanced, DriveRecorded,
    DuelKind as PublicDuelKind, DuelResolved, EventArtroPlacement, FieldGoalScored,
    FieldPointScored, GoalPointScored, MatchClockInstant, MatchEvent, MatchEventEnvelope,
    OutOfBounds, PassCompleted, ReceptionResolved, Turnover,
};
use arlo_math::units::Position;
use uuid::Uuid;

pub fn translate_duel_kind(kind: EngineDuelKind) -> PublicDuelKind {
    match kind {
        EngineDuelKind::PassProtection => PublicDuelKind::PassProtection,
        EngineDuelKind::RouteContest => PublicDuelKind::RouteContest,
        EngineDuelKind::RunBreakthrough => PublicDuelKind::RunBreakthrough,
        EngineDuelKind::CentralBlock => PublicDuelKind::CentralBlock,
        EngineDuelKind::LateralBlock => PublicDuelKind::LateralBlock,
        EngineDuelKind::ArtroBreakthrough => PublicDuelKind::ArtroBreakthrough,
        EngineDuelKind::AerialDuel => PublicDuelKind::AerialDuel,
        EngineDuelKind::FinishingAttempt => PublicDuelKind::FinishingAttempt,
        EngineDuelKind::ShortDistribution => PublicDuelKind::ShortDistribution,
        EngineDuelKind::LongDistribution => PublicDuelKind::LongDistribution,
        EngineDuelKind::CrossDistribution => PublicDuelKind::CrossDistribution,
        EngineDuelKind::BallSecurityCarry => PublicDuelKind::BallSecurityCarry,
        EngineDuelKind::BallSecurityDistribution => PublicDuelKind::BallSecurityDistribution,
    }
}

pub fn translate_duel_resolved(
    outcome: &DuelOutcome,
    attacker_ids: Vec<Uuid>,
    defender_ids: Vec<Uuid>,
) -> DuelResolved {
    DuelResolved::new(
        translate_duel_kind(outcome.kind()),
        attacker_ids,
        defender_ids,
        outcome.attacker_won(),
        outcome.win_probability(),
        outcome.net_advantage(),
    )
}

pub fn translate_reception_resolved(
    receiver_id: Uuid,
    passer_id: Uuid,
    caught: bool,
    is_aerial: bool,
) -> ReceptionResolved {
    ReceptionResolved::new(receiver_id, passer_id, caught, is_aerial)
}

pub fn translate_call_to_action_started(
    offense_team_id: Uuid,
    defense_team_id: Uuid,
    passer_id: Uuid,
    artrine_id: Uuid,
    down_number: u32,
    scrimmage_x_mirim: f64,
    target_advance_mirim: f64,
) -> CallToActionStarted {
    CallToActionStarted::new(
        offense_team_id,
        defense_team_id,
        passer_id,
        artrine_id,
        down_number,
        scrimmage_x_mirim,
        target_advance_mirim,
    )
}

pub fn translate_pass_completed(
    passer_id: Uuid,
    receiver_id: Uuid,
    is_aerial: bool,
    reception_point: Position,
    distance_mirim: f64,
) -> PassCompleted {
    PassCompleted::new(
        passer_id,
        receiver_id,
        is_aerial,
        reception_point.raw().0,
        reception_point.raw().1,
        distance_mirim,
    )
}

pub fn translate_drive_recorded(
    artrine_id: Uuid,
    artro_row_index: usize,
    placement: EventArtroPlacement,
    drives_in_series: u32,
    x_mirim: f64,
) -> DriveRecorded {
    DriveRecorded::new(artrine_id, artro_row_index, placement, drives_in_series, x_mirim)
}

pub fn translate_turnover(
    previous_offense: Uuid,
    new_offense: Uuid,
    recovering_player: Option<Uuid>,
    in_live_play: bool,
    point: Position,
) -> Turnover {
    Turnover::new(
        previous_offense,
        new_offense,
        recovering_player,
        in_live_play,
        point.raw().0,
        point.raw().1,
    )
}

pub fn translate_out_of_bounds(
    last_possession_team: Uuid,
    last_player: Option<Uuid>,
    out_point: Position,
    was_immediate_loss: bool,
) -> OutOfBounds {
    OutOfBounds::new(
        last_possession_team,
        last_player,
        out_point.raw().0,
        out_point.raw().1,
        was_immediate_loss,
    )
}

pub fn translate_countdown_started(
    offense_team_id: Uuid,
    size_x_mirim: f64,
    reason: CountdownReason,
) -> CountdownToSizeStarted {
    CountdownToSizeStarted::new(offense_team_id, size_x_mirim, reason)
}

pub fn translate_down_advanced(
    previous_down: u32,
    new_down: u32,
    mirins_advanced_this_down: f64,
    total_mirins_advanced_in_series: f64,
    first_down_achieved: bool,
    scrimmage_x_mirim: f64,
) -> DownAdvanced {
    DownAdvanced::new(
        previous_down,
        new_down,
        mirins_advanced_this_down,
        total_mirins_advanced_in_series,
        first_down_achieved,
        scrimmage_x_mirim,
    )
}

pub fn translate_scoring_decision(decision: &ScoringDecision) -> Option<MatchEvent> {
    match decision {
        ScoringDecision::GoalPoint {
            team_id,
            scorer_id,
            artrine_id,
            drives_completed,
            ..
        } => Some(MatchEvent::GoalPoint(GoalPointScored::new(
            *team_id,
            *scorer_id,
            *artrine_id,
            *drives_completed,
        ))),
        ScoringDecision::FieldPoint {
            team_id,
            scorer_id,
            territory_advance_mirim,
            drives_completed,
            ..
        } => Some(MatchEvent::FieldPoint(FieldPointScored::new(
            *team_id,
            *scorer_id,
            *territory_advance_mirim,
            *drives_completed,
        ))),
        ScoringDecision::FieldGoal {
            team_id,
            scorer_id,
            post,
            ..
        } => Some(MatchEvent::FieldGoal(FieldGoalScored::new(
            *team_id, *scorer_id, *post,
        ))),
        ScoringDecision::Missed { .. } | ScoringDecision::NoOpportunity => None,
    }
}

pub fn create_envelope(
    sequence_number: u64,
    clock: MatchClockInstant,
    event: impl Into<MatchEvent>,
) -> MatchEventEnvelope {
    MatchEventEnvelope::new(sequence_number, clock, event.into())
}