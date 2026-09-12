use crate::match_decision::scoring::ScoringDecision;
use crate::officiating::foul::FoulResolution;
use crate::physical::PhysicalState;
use crate::possession::drive::ArtrineCarrier;
use crate::psychology::state::ImpulseState;
use crate::psychology::systems::events::{
    apply_impulse_event_at, ImpulseEvent, ImpulseEventKind, ImpulseShift,
};
use crate::resolution::AttributedDuelOutcome;
use arlo_domain::sport_constants::FOUL_IMPULSE_EPV_DELTA;
use arlo_domain::{AttributeKey, Player, Position};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DispatchedImpulseEvent {
    pub target_id: Uuid,
    pub event: ImpulseEvent,
}

impl DispatchedImpulseEvent {
    pub fn new(target_id: Uuid, event: ImpulseEvent) -> Self {
        Self { target_id, event }
    }

    pub fn target_id(&self) -> Uuid {
        self.target_id
    }

    pub fn event(&self) -> &ImpulseEvent {
        &self.event
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ImpulseEventBus {
    events: Vec<DispatchedImpulseEvent>,
}

impl ImpulseEventBus {
    pub fn new() -> Self {
        Self { events: Vec::new() }
    }

    pub fn publish(&mut self, target_id: Uuid, event: ImpulseEvent) {
        self.events
            .push(DispatchedImpulseEvent::new(target_id, event));
    }

    pub fn publish_events(&mut self, events: impl IntoIterator<Item = DispatchedImpulseEvent>) {
        self.events.extend(events);
    }

    pub fn publish_carrier_event(&mut self, carrier: ArtrineCarrier, event: ImpulseEvent) {
        self.publish(carrier.id(), event);
    }

    pub fn publish_player_event(&mut self, player: &Player, event: ImpulseEvent) {
        self.publish(player.id(), event);
    }

    pub fn publish_position_event(
        &mut self,
        position: Position,
        lineup_players: &[&Player],
        position_index: &HashMap<Uuid, Position>,
        event: ImpulseEvent,
    ) {
        if let Some(&player) = lineup_players.iter().find(|p| {
            position_index.get(&p.id()).copied() == Some(position)
                || p.positions()
                    .iter()
                    .any(|pos| pos.position() == position && pos.proficiency() > 0)
        }) {
            self.publish(player.id(), event);
        }
    }

    pub fn publish_team_collective_event(
        &mut self,
        team_players: &[&Player],
        involved_player_ids: &[Uuid],
        kind: ImpulseEventKind,
        surprisal: f64,
        epv_delta: f64,
    ) {
        for player in team_players {
            let pid = player.id();
            let is_involved = involved_player_ids.contains(&pid);
            let event = ImpulseEvent::new(kind, surprisal, epv_delta, is_involved);
            self.publish(pid, event);
        }
    }

    pub fn publish_attributed_duel(
        &mut self,
        attributed_outcome: &AttributedDuelOutcome,
        offense_players: &[&Player],
        defense_players: &[&Player],
    ) {
        let outcome = attributed_outcome.outcome();
        let win_p = outcome.win_probability().value().clamp(0.0001, 0.9999);
        let net_adv = outcome.net_advantage();
        let epv_delta = (net_adv * 0.1).abs();

        let (att_kind, att_p) = if outcome.attacker_won() {
            (ImpulseEventKind::DuelWon, win_p)
        } else {
            (ImpulseEventKind::DuelLost, 1.0 - win_p)
        };

        let (def_kind, def_p) = if outcome.attacker_won() {
            (ImpulseEventKind::DuelLost, win_p)
        } else {
            (ImpulseEventKind::DuelWon, 1.0 - win_p)
        };

        let att_surprisal = -att_p.ln();
        let def_surprisal = -def_p.ln();

        for player in offense_players {
            let pid = player.id();
            let involved = attributed_outcome.is_active_attacker(&pid);
            let event = ImpulseEvent::new(att_kind, att_surprisal, epv_delta, involved);
            self.publish(pid, event);
        }

        for player in defense_players {
            let pid = player.id();
            let involved = attributed_outcome.is_active_defender(&pid);
            let event = ImpulseEvent::new(def_kind, def_surprisal, epv_delta, involved);
            self.publish(pid, event);
        }
    }

    pub fn publish_foul(&mut self, resolution: &FoulResolution) {
        let p = resolution.trigger_probability().clamp(0.0001, 0.9999);
        let surprisal = -p.ln();
        let committed_event = ImpulseEvent::new(
            ImpulseEventKind::FoulCommitted,
            surprisal,
            FOUL_IMPULSE_EPV_DELTA,
            true,
        );
        let drawn_event = ImpulseEvent::new(
            ImpulseEventKind::FoulDrawn,
            surprisal,
            FOUL_IMPULSE_EPV_DELTA,
            true,
        );
        self.publish(resolution.offending_player_id(), committed_event);
        self.publish(resolution.opposing_player_id(), drawn_event);
    }

    pub fn publish_scoring_decision(
        &mut self,
        decision: &ScoringDecision,
        finisher_id: Uuid,
        goalguard_id: Uuid,
        win_probability: f64,
        offense_players: &[&Player],
        defense_players: &[&Player],
    ) {
        let p = win_probability.clamp(0.0001, 0.9999);
        let points = decision.points() as f64;

        if decision.is_scored() {
            let att_surprisal = -p.ln();
            let def_surprisal = -(1.0 - p).ln();

            for player in offense_players {
                let pid = player.id();
                let involved = pid == finisher_id;
                let event =
                    ImpulseEvent::new(ImpulseEventKind::ScoreFor, att_surprisal, points, involved);
                self.publish(pid, event);
            }

            for player in defense_players {
                let pid = player.id();
                let involved = pid == goalguard_id;
                let event = ImpulseEvent::new(
                    ImpulseEventKind::ScoreAgainst,
                    def_surprisal,
                    points,
                    involved,
                );
                self.publish(pid, event);
            }
        } else if matches!(decision, ScoringDecision::Missed { .. }) {
            let att_surprisal = -(1.0 - p).ln();
            let def_surprisal = -p.ln();

            for player in offense_players {
                let pid = player.id();
                let involved = pid == finisher_id;
                let event = ImpulseEvent::new(
                    ImpulseEventKind::DuelLost,
                    att_surprisal,
                    points.max(1.0),
                    involved,
                );
                self.publish(pid, event);
            }

            for player in defense_players {
                let pid = player.id();
                let involved = pid == goalguard_id;
                let event = ImpulseEvent::new(
                    ImpulseEventKind::DuelWon,
                    def_surprisal,
                    points.max(1.0),
                    involved,
                );
                self.publish(pid, event);
            }
        }
    }

    pub fn publish_turnover(
        &mut self,
        lost_by_player_id: Option<Uuid>,
        recovering_player_id: Option<Uuid>,
        offense_players: &[&Player],
        defense_players: &[&Player],
    ) {
        for player in offense_players {
            let pid = player.id();
            let involved = Some(pid) == lost_by_player_id;
            let event = ImpulseEvent::new(ImpulseEventKind::TurnoverCommitted, 1.20, 2.5, involved);
            self.publish(pid, event);
        }

        for player in defense_players {
            let pid = player.id();
            let involved = Some(pid) == recovering_player_id;
            let event = ImpulseEvent::new(ImpulseEventKind::TurnoverWon, 1.20, 2.5, involved);
            self.publish(pid, event);
        }
    }

    pub fn publish_series_result(
        &mut self,
        success: bool,
        offense_players: &[&Player],
        defense_players: &[&Player],
        involved_offense_ids: &[Uuid],
    ) {
        let (off_kind, def_kind, surprisal, epv) = if success {
            (
                ImpulseEventKind::SeriesSuccess,
                ImpulseEventKind::SeriesFailure,
                0.70,
                1.5,
            )
        } else {
            (
                ImpulseEventKind::SeriesFailure,
                ImpulseEventKind::SeriesSuccess,
                0.90,
                2.0,
            )
        };

        for player in offense_players {
            let pid = player.id();
            let involved = involved_offense_ids.contains(&pid);
            let event = ImpulseEvent::new(off_kind, surprisal, epv, involved);
            self.publish(pid, event);
        }

        for player in defense_players {
            let pid = player.id();
            let event = ImpulseEvent::new(def_kind, surprisal, epv, false);
            self.publish(pid, event);
        }
    }

    pub fn apply_dispatched_event(
        state: &mut ImpulseState,
        player: &Player,
        attribute_keys: &HashMap<Uuid, AttributeKey>,
        physical_state: &PhysicalState,
        event: &ImpulseEvent,
        timestamp_seconds: f64,
    ) -> ImpulseShift {
        apply_impulse_event_at(
            state,
            player,
            attribute_keys,
            physical_state,
            event,
            timestamp_seconds,
        )
    }

    pub fn events(&self) -> &[DispatchedImpulseEvent] {
        &self.events
    }

    pub fn drain_events(&mut self) -> Vec<DispatchedImpulseEvent> {
        std::mem::take(&mut self.events)
    }

    pub fn clear(&mut self) {
        self.events.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }

    pub fn len(&self) -> usize {
        self.events.len()
    }
}
