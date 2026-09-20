use crate::manager_ai::challenges::{apply_challenge, apply_foul_challenge};
use crate::manager_ai::cognition::ManagerDecisionKind;
use crate::manager_ai::play_calling::execute_play_call_selection;
use crate::manager_ai::tactical_adjustment::execute_tactical_adjustment_by_id;
use crate::manager_ai::time_calls::execute_time_call;
use crate::rng::RngStream;
use crate::time::DurationLedger;
use crate::world_state::play_transition::publisher::EventPublisher;
use arlo_events::{EventSink, TimeCallReason};
use arlo_manager_control::ManagerDecisionInbox;
use uuid::Uuid;

pub fn try_apply_human_challenge(
    publisher: &mut EventPublisher<'_, impl EventSink>,
    team_id: Uuid,
    inbox: &ManagerDecisionInbox,
) -> bool {
    if inbox.take_challenge(team_id).is_none() {
        return false;
    }
    let Some((call_team_id, call)) = publisher.state().last_reviewable_call().cloned() else {
        return false;
    };
    if call_team_id != team_id {
        return false;
    }
    let seq = publisher.state().event_sequence();
    let mut rng = publisher
        .state()
        .rng_provider()
        .indexed_rng_for(RngStream::ChallengeResolution, seq);
    apply_challenge(publisher, team_id, &call, &mut rng)
}

pub fn try_apply_human_foul_challenge(
    publisher: &mut EventPublisher<'_, impl EventSink>,
    team_id: Uuid,
    inbox: &ManagerDecisionInbox,
) -> bool {
    if inbox.take_challenge(team_id).is_none() {
        return false;
    }
    let Some((foul_team_id, record)) = publisher.state().last_reviewable_foul().cloned() else {
        return false;
    };
    if foul_team_id != team_id {
        return false;
    }
    apply_foul_challenge(publisher, team_id, &record)
}

pub fn try_apply_human_kick_foul_realignment(
    publisher: &mut EventPublisher<'_, impl EventSink>,
    team_id: Uuid,
    is_home: bool,
    inbox: &ManagerDecisionInbox,
) -> bool {
    if inbox.take_kick_foul_realignment(team_id).is_none() {
        return false;
    }
    let Some(pending) = publisher.state().kick_foul_pending().copied() else {
        return false;
    };
    if pending.awarded_team_id() != team_id {
        return false;
    }
    let mut ledger = DurationLedger::new();
    if execute_time_call(
        publisher,
        team_id,
        is_home,
        &mut ledger,
        TimeCallReason::KickFoulRealignment,
    ) {
        publisher.state_mut().clear_kick_foul_pending();
        publisher
            .state_mut()
            .mark_decision_triggered(team_id, ManagerDecisionKind::TimeCall);
        true
    } else {
        false
    }
}

pub fn try_apply_human_play_call(
    publisher: &mut EventPublisher<'_, impl EventSink>,
    team_id: Uuid,
    inbox: &ManagerDecisionInbox,
) -> bool {
    let Some(intent) = inbox.take_play_call(team_id) else {
        return false;
    };
    if let Some(play_call) = publisher
        .state()
        .playbook_for_team(team_id)
        .iter()
        .find(|p| p.id() == intent.play_call_id())
        .cloned()
    {
        execute_play_call_selection(publisher, team_id, play_call);
        true
    } else {
        false
    }
}

pub fn try_apply_human_tactical_switch(
    publisher: &mut EventPublisher<'_, impl EventSink>,
    team_id: Uuid,
    inbox: &ManagerDecisionInbox,
) -> bool {
    let Some(intent) = inbox.take_tactical_switch(team_id) else {
        return false;
    };
    let available_profiles = publisher
        .state()
        .available_profiles_for_team(team_id)
        .to_vec();
    if execute_tactical_adjustment_by_id(
        publisher,
        team_id,
        intent.profile_id(),
        &available_profiles,
    ) {
        publisher
            .state_mut()
            .mark_decision_triggered(team_id, ManagerDecisionKind::TacticalAdjustment);
        true
    } else {
        false
    }
}

pub fn try_apply_human_time_call(
    publisher: &mut EventPublisher<'_, impl EventSink>,
    team_id: Uuid,
    is_home: bool,
    inbox: &ManagerDecisionInbox,
) -> bool {
    if inbox.take_time_call(team_id).is_none() {
        return false;
    }
    let mut ledger = DurationLedger::new();
    if execute_time_call(
        publisher,
        team_id,
        is_home,
        &mut ledger,
        TimeCallReason::Standard,
    ) {
        publisher
            .state_mut()
            .mark_decision_triggered(team_id, ManagerDecisionKind::TimeCall);
        true
    } else {
        false
    }
}