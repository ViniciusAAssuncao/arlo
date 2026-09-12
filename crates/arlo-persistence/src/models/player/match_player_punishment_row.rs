use arlo_domain::PunishmentKind;
use arlo_stats::PlayerPunishmentStats;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct MatchPlayerPunishmentRow {
    pub id: String,
    pub match_id: String,
    pub player_id: String,
    pub yardage_loss_count: i32,
    pub loss_of_down_count: i32,
    pub loss_of_drive_count: i32,
    pub time_penalty_count: i32,
    pub expulsion_count: i32,
    pub invalidate_play_count: i32,
    pub total_yardage_loss_mirim: f64,
    pub total_loss_of_down_count: i32,
    pub total_time_penalty_seconds: f64,
    pub total_loss_of_drive_count: i32,
}

impl MatchPlayerPunishmentRow {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: Uuid,
        match_id: Uuid,
        player_id: Uuid,
        yardage_loss_count: u32,
        loss_of_down_count: u32,
        loss_of_drive_count: u32,
        time_penalty_count: u32,
        expulsion_count: u32,
        invalidate_play_count: u32,
        total_yardage_loss_mirim: f64,
        total_loss_of_down_count: u32,
        total_time_penalty_seconds: f64,
        total_loss_of_drive_count: u32,
    ) -> Self {
        Self {
            id: id.to_string(),
            match_id: match_id.to_string(),
            player_id: player_id.to_string(),
            yardage_loss_count: yardage_loss_count as i32,
            loss_of_down_count: loss_of_down_count as i32,
            loss_of_drive_count: loss_of_drive_count as i32,
            time_penalty_count: time_penalty_count as i32,
            expulsion_count: expulsion_count as i32,
            invalidate_play_count: invalidate_play_count as i32,
            total_yardage_loss_mirim,
            total_loss_of_down_count: total_loss_of_down_count as i32,
            total_time_penalty_seconds,
            total_loss_of_drive_count: total_loss_of_drive_count as i32,
        }
    }

    pub fn from_stats(id: Uuid, match_id: Uuid, stats: &PlayerPunishmentStats) -> Self {
        Self::new(
            id,
            match_id,
            stats.player_id,
            stats.count_for_kind(PunishmentKind::YardageLoss),
            stats.count_for_kind(PunishmentKind::LossOfDown),
            stats.count_for_kind(PunishmentKind::LossOfDrive),
            stats.count_for_kind(PunishmentKind::TimePenalty),
            stats.total_expulsions,
            stats.total_plays_invalidated,
            stats.total_yardage_loss_mirim,
            stats.total_loss_of_down_count,
            stats.total_time_penalty_seconds,
            stats.total_loss_of_drive_count,
        )
    }
}
