use arlo_domain::KickFoulScoringTier;
use arlo_events::{KickFoulAwarded, MatchClockInstant};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct MatchKickFoulAwardRow {
    pub id: String,
    pub match_id: String,
    pub sequence_number: i64,
    pub period: i32,
    pub seconds_in_period: f64,
    pub awarded_team_id: String,
    pub offending_team_id: String,
    pub scoring_tier: String,
    pub spot_x_mirim: f64,
    pub spot_y_mirim: f64,
}

impl MatchKickFoulAwardRow {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: Uuid,
        match_id: Uuid,
        sequence_number: u64,
        period: u32,
        seconds_in_period: f64,
        awarded_team_id: Uuid,
        offending_team_id: Uuid,
        scoring_tier: impl Into<String>,
        spot_x_mirim: f64,
        spot_y_mirim: f64,
    ) -> Self {
        Self {
            id: id.to_string(),
            match_id: match_id.to_string(),
            sequence_number: sequence_number as i64,
            period: period as i32,
            seconds_in_period,
            awarded_team_id: awarded_team_id.to_string(),
            offending_team_id: offending_team_id.to_string(),
            scoring_tier: scoring_tier.into(),
            spot_x_mirim,
            spot_y_mirim,
        }
    }

    pub fn from_event(
        id: Uuid,
        match_id: Uuid,
        seq: u64,
        clock: MatchClockInstant,
        event: &KickFoulAwarded,
    ) -> Self {
        let tier_str = match event.scoring_tier() {
            KickFoulScoringTier::FirstZone => "FirstZone",
            KickFoulScoringTier::Standard => "Standard",
        };

        Self::new(
            id,
            match_id,
            seq,
            clock.period(),
            clock.seconds_in_period(),
            event.awarded_team_id(),
            event.offending_team_id(),
            tier_str,
            event.spot_x_mirim(),
            event.spot_y_mirim(),
        )
    }
}
