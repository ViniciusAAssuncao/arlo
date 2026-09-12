use arlo_domain::PunishmentKind;
use arlo_events::{FoulOrigin, FoulRaised, MatchClockInstant};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct MatchFoulRow {
    pub id: String,
    pub match_id: String,
    pub sequence_number: i64,
    pub period: i32,
    pub seconds_in_period: f64,
    pub offending_player_id: String,
    pub offending_team_id: String,
    pub opposing_player_id: String,
    pub opposing_team_id: String,
    pub origin: String,
    pub original_call_correct: bool,
    pub peace_referee_intervened: bool,
    pub fault_definition_id: Option<String>,
    pub punishment_kind: Option<String>,
    pub punishment_magnitude: Option<i32>,
}

impl MatchFoulRow {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: Uuid,
        match_id: Uuid,
        sequence_number: u64,
        period: u32,
        seconds_in_period: f64,
        offending_player_id: Uuid,
        offending_team_id: Uuid,
        opposing_player_id: Uuid,
        opposing_team_id: Uuid,
        origin: impl Into<String>,
        original_call_correct: bool,
        peace_referee_intervened: bool,
        fault_definition_id: Option<Uuid>,
        punishment_kind: Option<impl Into<String>>,
        punishment_magnitude: Option<i32>,
    ) -> Self {
        Self {
            id: id.to_string(),
            match_id: match_id.to_string(),
            sequence_number: sequence_number as i64,
            period: period as i32,
            seconds_in_period,
            offending_player_id: offending_player_id.to_string(),
            offending_team_id: offending_team_id.to_string(),
            opposing_player_id: opposing_player_id.to_string(),
            opposing_team_id: opposing_team_id.to_string(),
            origin: origin.into(),
            original_call_correct,
            peace_referee_intervened,
            fault_definition_id: fault_definition_id.map(|id| id.to_string()),
            punishment_kind: punishment_kind.map(|k| k.into()),
            punishment_magnitude,
        }
    }

    pub fn from_event(
        id: Uuid,
        match_id: Uuid,
        seq: u64,
        clock: MatchClockInstant,
        event: &FoulRaised,
    ) -> Self {
        let origin_str = match event.origin() {
            FoulOrigin::ContactDuel(kind) => format!("ContactDuel:{}", kind.as_str()),
            FoulOrigin::LineFault => "LineFault".to_string(),
        };

        let punishment_kind_str = event.punishment_kind().map(|k| match k {
            PunishmentKind::YardageLoss => "YardageLoss",
            PunishmentKind::LossOfDown => "LossOfDown",
            PunishmentKind::LossOfDrive => "LossOfDrive",
            PunishmentKind::TimePenalty => "TimePenalty",
            PunishmentKind::Expulsion => "Expulsion",
            PunishmentKind::InvalidatePreviousPlay => "InvalidatePreviousPlay",
            PunishmentKind::KickFoulAwarded => "KickFoulAwarded",
        });

        Self::new(
            id,
            match_id,
            seq,
            clock.period(),
            clock.seconds_in_period(),
            event.offending_player_id(),
            event.offending_team_id(),
            event.opposing_player_id(),
            event.opposing_team_id(),
            origin_str,
            event.original_call_correct(),
            event.peace_referee_intervened(),
            event.fault_definition_id(),
            punishment_kind_str,
            event.punishment_magnitude(),
        )
    }
}
