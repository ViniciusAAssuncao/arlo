pub mod expulsion;
pub mod invalidate_previous_play;
pub mod kick_foul_awarded;
pub mod ledger;
pub mod loss_of_down;
pub mod loss_of_drive;
pub mod reversal;
pub mod time_penalty;
pub mod yardage_loss;

pub use expulsion::apply_expulsion;
pub use invalidate_previous_play::apply_invalidate_previous_play;
pub use kick_foul_awarded::apply_kick_foul_awarded;
pub use ledger::{reverse_punishment, PunishmentLedgerEntry};
pub use loss_of_down::apply_loss_of_down;
pub use loss_of_drive::apply_loss_of_drive;
pub use reversal::{
    apply_play_reversal, capture_play_reversal_snapshot, PlayReversalSnapshot,
};
pub use time_penalty::apply_time_penalty;
pub use yardage_loss::apply_yardage_loss;

use crate::world_state::match_state::state::MatchState;
use arlo_domain::PunishmentKind;
use uuid::Uuid;

pub fn apply_punishment(
    state: &mut MatchState,
    offending_player_id: Uuid,
    offending_team_id: Uuid,
    kind: PunishmentKind,
    magnitude: Option<i32>,
    pre_play_snapshot: &PlayReversalSnapshot,
) -> PunishmentLedgerEntry {
    let undo_snapshot = capture_play_reversal_snapshot(state);
    let mut availability_before = None;

    match kind {
        PunishmentKind::LossOfDown => {
            apply_loss_of_down(state, magnitude);
        }
        PunishmentKind::YardageLoss => {
            apply_yardage_loss(state, offending_team_id, magnitude);
        }
        PunishmentKind::LossOfDrive => {
            apply_loss_of_drive(state, magnitude);
        }
        PunishmentKind::TimePenalty => {
            let prev = apply_time_penalty(state, offending_player_id, magnitude);
            availability_before = Some(prev);
        }
        PunishmentKind::Expulsion => {
            let prev = apply_expulsion(state, offending_player_id);
            availability_before = Some(prev);
        }
        PunishmentKind::InvalidatePreviousPlay => {
            apply_invalidate_previous_play(state, pre_play_snapshot);
        }
        PunishmentKind::KickFoulAwarded => {
            apply_kick_foul_awarded(state, offending_team_id);
        }
    }

    PunishmentLedgerEntry {
        kind,
        magnitude,
        offending_player_id,
        offending_team_id,
        undo_snapshot: Some(undo_snapshot),
        availability_before,
    }
}