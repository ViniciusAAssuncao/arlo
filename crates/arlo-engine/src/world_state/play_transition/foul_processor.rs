use crate::officiating::foul::FoulResolution;
use crate::officiating::punishment::{apply_punishment, PlayReversalSnapshot};
use crate::world_state::match_state::foul_review::FoulReviewRecord;
use crate::world_state::play_transition::publisher::EventPublisher;
use arlo_domain::PunishmentKind;
use arlo_events::EventSink;

pub fn process_fouls_and_punishments<S: EventSink>(
    publisher: &mut EventPublisher<'_, S>,
    fouls: &[FoulResolution],
    pre_play_snapshot: &PlayReversalSnapshot,
) {
    for foul in fouls {
        publisher.emit_foul_raised(foul);

        if let Some(kind) = foul.punishment_kind {
            let entry = apply_punishment(
                publisher.state_mut(),
                foul.offending_player_id,
                foul.offending_team_id,
                kind,
                foul.punishment_magnitude,
                pre_play_snapshot,
            );
            if kind == PunishmentKind::KickFoulAwarded {
                if let Some(pending) = publisher.state().kick_foul_pending().copied() {
                    publisher.emit_kick_foul_awarded(&pending, foul.offending_team_id);
                }
            }
            if !foul.peace_referee_intervened {
                let record = FoulReviewRecord::new(
                    foul.offending_player_id,
                    entry,
                    foul.original_call_correct,
                );
                publisher
                    .state_mut()
                    .set_last_reviewable_foul(foul.offending_team_id, record);
            }
        }
    }
}
