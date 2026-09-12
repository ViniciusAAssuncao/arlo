use arlo_domain::KickFoulDecisionKind;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MatchPlayerKickFoulByDecisionRow {
    pub id: String,
    pub match_id: String,
    pub player_id: String,
    pub decision_kind: String,
    pub takes_count: i32,
}

impl MatchPlayerKickFoulByDecisionRow {
    pub fn new(
        id: Uuid,
        match_id: Uuid,
        player_id: Uuid,
        decision_kind: impl Into<String>,
        takes_count: u32,
    ) -> Self {
        Self {
            id: id.to_string(),
            match_id: match_id.to_string(),
            player_id: player_id.to_string(),
            decision_kind: decision_kind.into(),
            takes_count: takes_count as i32,
        }
    }

    pub fn from_stats(
        id: Uuid,
        match_id: Uuid,
        player_id: Uuid,
        decision_kind: KickFoulDecisionKind,
        count: u32,
    ) -> Self {
        let dec_str = match decision_kind {
            KickFoulDecisionKind::Shoot => "Shoot",
            KickFoulDecisionKind::Cross => "Cross",
            KickFoulDecisionKind::ShortPass => "ShortPass",
            KickFoulDecisionKind::LongLaunch => "LongLaunch",
        };
        Self::new(id, match_id, player_id, dec_str, count)
    }
}
