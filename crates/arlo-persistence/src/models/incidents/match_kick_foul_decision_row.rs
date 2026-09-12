use arlo_domain::KickFoulDecisionKind;
use arlo_events::{KickFoulDecisionMade, MatchClockInstant};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct MatchKickFoulDecisionRow {
    pub id: String,
    pub match_id: String,
    pub sequence_number: i64,
    pub period: i32,
    pub seconds_in_period: f64,
    pub taker_id: String,
    pub decision: String,
}

impl MatchKickFoulDecisionRow {
    pub fn new(
        id: Uuid,
        match_id: Uuid,
        sequence_number: u64,
        period: u32,
        seconds_in_period: f64,
        taker_id: Uuid,
        decision: impl Into<String>,
    ) -> Self {
        Self {
            id: id.to_string(),
            match_id: match_id.to_string(),
            sequence_number: sequence_number as i64,
            period: period as i32,
            seconds_in_period,
            taker_id: taker_id.to_string(),
            decision: decision.into(),
        }
    }

    pub fn from_event(
        id: Uuid,
        match_id: Uuid,
        seq: u64,
        clock: MatchClockInstant,
        event: &KickFoulDecisionMade,
    ) -> Self {
        let dec_str = match event.decision() {
            KickFoulDecisionKind::Shoot => "Shoot",
            KickFoulDecisionKind::Cross => "Cross",
            KickFoulDecisionKind::ShortPass => "ShortPass",
            KickFoulDecisionKind::LongLaunch => "LongLaunch",
        };

        Self::new(
            id,
            match_id,
            seq,
            clock.period(),
            clock.seconds_in_period(),
            event.taker_id(),
            dec_str,
        )
    }
}
