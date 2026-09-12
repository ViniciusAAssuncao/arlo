use arlo_stats::PlayerDuelStats;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct MatchPlayerDuelRow {
    pub id: String,
    pub match_id: String,
    pub player_id: String,
    pub total_duels: i32,
    pub total_wins: i32,
    pub total_losses: i32,
    pub win_rate: f64,
    pub attacker_duels: i32,
    pub attacker_wins: i32,
    pub attacker_losses: i32,
    pub attacker_win_rate: f64,
    pub defender_duels: i32,
    pub defender_wins: i32,
    pub defender_losses: i32,
    pub defender_win_rate: f64,
}

impl MatchPlayerDuelRow {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: Uuid,
        match_id: Uuid,
        player_id: Uuid,
        total_duels: u32,
        total_wins: u32,
        total_losses: u32,
        win_rate: f64,
        attacker_duels: u32,
        attacker_wins: u32,
        attacker_losses: u32,
        attacker_win_rate: f64,
        defender_duels: u32,
        defender_wins: u32,
        defender_losses: u32,
        defender_win_rate: f64,
    ) -> Self {
        Self {
            id: id.to_string(),
            match_id: match_id.to_string(),
            player_id: player_id.to_string(),
            total_duels: total_duels as i32,
            total_wins: total_wins as i32,
            total_losses: total_losses as i32,
            win_rate,
            attacker_duels: attacker_duels as i32,
            attacker_wins: attacker_wins as i32,
            attacker_losses: attacker_losses as i32,
            attacker_win_rate,
            defender_duels: defender_duels as i32,
            defender_wins: defender_wins as i32,
            defender_losses: defender_losses as i32,
            defender_win_rate,
        }
    }

    pub fn from_stats(id: Uuid, match_id: Uuid, stats: &PlayerDuelStats) -> Self {
        Self::new(
            id,
            match_id,
            stats.player_id,
            stats.total_duels,
            stats.total_wins,
            stats.total_losses,
            stats.win_rate(),
            stats.attacker_duels,
            stats.attacker_wins,
            stats.attacker_losses,
            stats.attacker_win_rate(),
            stats.defender_duels,
            stats.defender_wins,
            stats.defender_losses,
            stats.defender_win_rate(),
        )
    }
}
