use arlo_events::DuelKind;
use arlo_stats::DuelKindStats;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct MatchPlayerDuelByKindRow {
    pub id: String,
    pub match_id: String,
    pub player_id: String,
    pub duel_kind: String,
    pub total: i32,
    pub wins: i32,
    pub losses: i32,
    pub as_attacker_wins: i32,
    pub as_attacker_losses: i32,
    pub as_defender_wins: i32,
    pub as_defender_losses: i32,
    pub win_rate: f64,
    pub attacker_win_rate: f64,
    pub defender_win_rate: f64,
}

impl MatchPlayerDuelByKindRow {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: Uuid,
        match_id: Uuid,
        player_id: Uuid,
        duel_kind: impl Into<String>,
        total: u32,
        wins: u32,
        losses: u32,
        as_attacker_wins: u32,
        as_attacker_losses: u32,
        as_defender_wins: u32,
        as_defender_losses: u32,
        win_rate: f64,
        attacker_win_rate: f64,
        defender_win_rate: f64,
    ) -> Self {
        Self {
            id: id.to_string(),
            match_id: match_id.to_string(),
            player_id: player_id.to_string(),
            duel_kind: duel_kind.into(),
            total: total as i32,
            wins: wins as i32,
            losses: losses as i32,
            as_attacker_wins: as_attacker_wins as i32,
            as_attacker_losses: as_attacker_losses as i32,
            as_defender_wins: as_defender_wins as i32,
            as_defender_losses: as_defender_losses as i32,
            win_rate,
            attacker_win_rate,
            defender_win_rate,
        }
    }

    pub fn from_stats(
        id: Uuid,
        match_id: Uuid,
        player_id: Uuid,
        duel_kind: DuelKind,
        stats: &DuelKindStats,
    ) -> Self {
        Self::new(
            id,
            match_id,
            player_id,
            duel_kind.as_str(),
            stats.total,
            stats.wins,
            stats.losses,
            stats.as_attacker_wins,
            stats.as_attacker_losses,
            stats.as_defender_wins,
            stats.as_defender_losses,
            stats.win_rate(),
            stats.attacker_win_rate(),
            stats.defender_win_rate(),
        )
    }
}
