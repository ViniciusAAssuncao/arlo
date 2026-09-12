use crate::error::{DbError, DbResult};
use arlo_domain::PunishmentKind;

pub fn parse_punishment_kind(code: &str) -> DbResult<PunishmentKind> {
    match code {
        "LossOfDown" | "loss_of_down" => Ok(PunishmentKind::LossOfDown),
        "InvalidatePreviousPlay" | "invalidate_previous_play" => {
            Ok(PunishmentKind::InvalidatePreviousPlay)
        }
        "YardageLoss" | "yardage_loss" => Ok(PunishmentKind::YardageLoss),
        "LossOfDrive" | "loss_of_drive" => Ok(PunishmentKind::LossOfDrive),
        "KickFoulAwarded" | "kick_foul_awarded" => Ok(PunishmentKind::KickFoulAwarded),
        "TimePenalty" | "time_penalty" => Ok(PunishmentKind::TimePenalty),
        "Expulsion" | "expulsion" => Ok(PunishmentKind::Expulsion),
        _ => Err(DbError::InvalidEnum(format!(
            "Invalid punishment kind: {code}"
        ))),
    }
}

pub fn punishment_kind_to_code(kind: PunishmentKind) -> &'static str {
    match kind {
        PunishmentKind::LossOfDown => "LossOfDown",
        PunishmentKind::InvalidatePreviousPlay => "InvalidatePreviousPlay",
        PunishmentKind::YardageLoss => "YardageLoss",
        PunishmentKind::LossOfDrive => "LossOfDrive",
        PunishmentKind::KickFoulAwarded => "KickFoulAwarded",
        PunishmentKind::TimePenalty => "TimePenalty",
        PunishmentKind::Expulsion => "Expulsion",
    }
}