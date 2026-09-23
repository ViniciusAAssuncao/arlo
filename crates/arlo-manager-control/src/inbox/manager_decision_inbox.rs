use crate::inbox::team_decision_inbox::TeamDecisionInbox;
use crate::intents::{
    ChallengeIntent, ForcedSubstitutionIntent, KickFoulRealignmentIntent, PlayCallIntent,
    SubstitutionIntent, TacticalSwitchIntent, TimeCallIntent,
};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

#[derive(Debug, Clone, Default)]
pub struct ManagerDecisionInbox {
    teams: Arc<Mutex<HashMap<Uuid, TeamDecisionInbox>>>,
}

impl ManagerDecisionInbox {
    pub fn new() -> Self {
        Self {
            teams: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn submit_substitution(&self, team_id: Uuid, intent: SubstitutionIntent) {
        let mut teams = self.teams.lock().unwrap_or_else(|p| p.into_inner());
        teams
            .entry(team_id)
            .or_default()
            .submit_substitution(intent);
    }

    pub fn submit_substitutions(
        &self,
        team_id: Uuid,
        intents: impl IntoIterator<Item = SubstitutionIntent>,
    ) {
        let mut teams = self.teams.lock().unwrap_or_else(|p| p.into_inner());
        teams
            .entry(team_id)
            .or_default()
            .submit_substitutions(intents);
    }

    pub fn submit_forced_substitution(&self, team_id: Uuid, intent: ForcedSubstitutionIntent) {
        let mut teams = self.teams.lock().unwrap_or_else(|p| p.into_inner());
        teams
            .entry(team_id)
            .or_default()
            .submit_forced_substitution(intent);
    }

    pub fn submit_forced_substitutions(
        &self,
        team_id: Uuid,
        intents: impl IntoIterator<Item = ForcedSubstitutionIntent>,
    ) {
        let mut teams = self.teams.lock().unwrap_or_else(|p| p.into_inner());
        teams
            .entry(team_id)
            .or_default()
            .submit_forced_substitutions(intents);
    }

    pub fn submit_time_call(&self, team_id: Uuid, intent: TimeCallIntent) {
        let mut teams = self.teams.lock().unwrap_or_else(|p| p.into_inner());
        teams.entry(team_id).or_default().submit_time_call(intent);
    }

    pub fn submit_challenge(&self, team_id: Uuid, intent: ChallengeIntent) {
        let mut teams = self.teams.lock().unwrap_or_else(|p| p.into_inner());
        teams.entry(team_id).or_default().submit_challenge(intent);
    }

    pub fn submit_tactical_switch(&self, team_id: Uuid, intent: TacticalSwitchIntent) {
        let mut teams = self.teams.lock().unwrap_or_else(|p| p.into_inner());
        teams
            .entry(team_id)
            .or_default()
            .submit_tactical_switch(intent);
    }

    pub fn submit_play_call(&self, team_id: Uuid, intent: PlayCallIntent) {
        let mut teams = self.teams.lock().unwrap_or_else(|p| p.into_inner());
        teams.entry(team_id).or_default().submit_play_call(intent);
    }

    pub fn submit_kick_foul_realignment(&self, team_id: Uuid, intent: KickFoulRealignmentIntent) {
        let mut teams = self.teams.lock().unwrap_or_else(|p| p.into_inner());
        teams
            .entry(team_id)
            .or_default()
            .submit_kick_foul_realignment(intent);
    }

    pub fn take_substitutions(&self, team_id: Uuid) -> Vec<SubstitutionIntent> {
        let mut teams = self.teams.lock().unwrap_or_else(|p| p.into_inner());
        teams.entry(team_id).or_default().take_substitutions()
    }

    pub fn take_forced_substitutions(&self, team_id: Uuid) -> Vec<ForcedSubstitutionIntent> {
        let mut teams = self.teams.lock().unwrap_or_else(|p| p.into_inner());
        teams
            .entry(team_id)
            .or_default()
            .take_forced_substitutions()
    }

    pub fn take_time_call(&self, team_id: Uuid) -> Option<TimeCallIntent> {
        let mut teams = self.teams.lock().unwrap_or_else(|p| p.into_inner());
        teams.entry(team_id).or_default().take_time_call()
    }

    pub fn take_challenge(&self, team_id: Uuid) -> Option<ChallengeIntent> {
        let mut teams = self.teams.lock().unwrap_or_else(|p| p.into_inner());
        teams.entry(team_id).or_default().take_challenge()
    }

    pub fn take_tactical_switch(&self, team_id: Uuid) -> Option<TacticalSwitchIntent> {
        let mut teams = self.teams.lock().unwrap_or_else(|p| p.into_inner());
        teams.entry(team_id).or_default().take_tactical_switch()
    }

    pub fn take_play_call(&self, team_id: Uuid) -> Option<PlayCallIntent> {
        let mut teams = self.teams.lock().unwrap_or_else(|p| p.into_inner());
        teams.entry(team_id).or_default().take_play_call()
    }

    pub fn take_kick_foul_realignment(&self, team_id: Uuid) -> Option<KickFoulRealignmentIntent> {
        let mut teams = self.teams.lock().unwrap_or_else(|p| p.into_inner());
        teams
            .entry(team_id)
            .or_default()
            .take_kick_foul_realignment()
    }
}
