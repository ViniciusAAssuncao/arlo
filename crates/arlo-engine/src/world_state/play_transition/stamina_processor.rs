use crate::artrine::DistributionFlightInfo;
use crate::resolution::AttributedDuelOutcome;
use crate::world_state::play_transition::fatigue_applier::{
    apply_duel_strain, apply_movement_strain,
};
use crate::world_state::play_transition::publisher::EventPublisher;
use arlo_events::{EventArtroPlacement, EventSink};
use std::collections::HashSet;
use uuid::Uuid;

pub struct StaminaProcessingRequest<'a> {
    pub artrine_id: Uuid,
    pub passer_id: Uuid,
    pub receiver_id: Option<Uuid>,
    pub drives_recorded: u32,
    pub distribution_flight: Option<&'a DistributionFlightInfo>,
    pub play_duels: &'a [AttributedDuelOutcome],
    pub execution_duels: &'a [AttributedDuelOutcome],
    pub live_seconds: f64,
}

pub fn process_play_stamina<S: EventSink>(
    publisher: &mut EventPublisher<'_, S>,
    req: &StaminaProcessingRequest<'_>,
) {
    publisher.emit_drives(
        req.artrine_id,
        req.drives_recorded,
        EventArtroPlacement::Central,
    );

    if let Some(flight_info) = req.distribution_flight {
        publisher.emit_distribution_flight(flight_info);
    }

    apply_duel_strain(publisher, req.play_duels);
    publisher.emit_duel_events(req.execution_duels, req.artrine_id);

    let mut participated_ids = HashSet::new();
    participated_ids.insert(req.passer_id);
    participated_ids.insert(req.artrine_id);
    if let Some(rid) = req.receiver_id {
        participated_ids.insert(rid);
    }
    for d in req.play_duels {
        for id in d.attacker_ids() {
            participated_ids.insert(*id);
        }
        for id in d.defender_ids() {
            participated_ids.insert(*id);
        }
    }

    let live_seconds = req.live_seconds.max(1.0);
    apply_movement_strain(publisher, &participated_ids, live_seconds);
}
