use crate::intents::{
    ChallengeIntent, ForcedSubstitutionIntent, InjuryDecisionIntent, KickFoulDecisionIntent, KickFoulRealignmentIntent,
    PlayCallIntent, SubstitutionIntent, TacticalSwitchIntent, TimeCallIntent,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TeamDecisionInbox {
    substitutions: Vec<SubstitutionIntent>,
    forced_substitutions: Vec<ForcedSubstitutionIntent>,
    injury_decisions: Vec<InjuryDecisionIntent>,
    time_call: Option<TimeCallIntent>,
    challenge: Option<ChallengeIntent>,
    tactical_switch: Option<TacticalSwitchIntent>,
    play_call: Option<PlayCallIntent>,
    kick_foul_decision: Option<KickFoulDecisionIntent>,
    kick_foul_realignment: Option<KickFoulRealignmentIntent>,
}

impl TeamDecisionInbox {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn submit_substitution(&mut self, intent: SubstitutionIntent) {
        self.substitutions.push(intent);
    }

    pub fn submit_substitutions(&mut self, intents: impl IntoIterator<Item = SubstitutionIntent>) {
        self.substitutions.extend(intents);
    }

    pub fn submit_forced_substitution(&mut self, intent: ForcedSubstitutionIntent) {
        self.forced_substitutions.retain(|existing| existing.outgoing_player_id() != intent.outgoing_player_id());
        self.forced_substitutions.push(intent);
    }

    pub fn submit_injury_decision(&mut self, intent: InjuryDecisionIntent) {
        self.injury_decisions.retain(|existing| existing.injured_player_id() != intent.injured_player_id());
        self.injury_decisions.push(intent);
    }

    pub fn take_injury_decision(&mut self, player_id: uuid::Uuid) -> Option<InjuryDecisionIntent> {
        let index = self.injury_decisions.iter().position(|intent| intent.injured_player_id() == player_id)?;
        Some(self.injury_decisions.remove(index))
    }

    pub fn injury_decision(&self, player_id: uuid::Uuid) -> Option<InjuryDecisionIntent> {
        self.injury_decisions.iter().find(|intent| intent.injured_player_id() == player_id).copied()
    }

    pub fn submit_forced_substitutions(
        &mut self,
        intents: impl IntoIterator<Item = ForcedSubstitutionIntent>,
    ) {
        for intent in intents {
            self.submit_forced_substitution(intent);
        }
    }

    pub fn submit_time_call(&mut self, intent: TimeCallIntent) {
        self.time_call = Some(intent);
    }

    pub fn submit_challenge(&mut self, intent: ChallengeIntent) {
        self.challenge = Some(intent);
    }

    pub fn submit_tactical_switch(&mut self, intent: TacticalSwitchIntent) {
        self.tactical_switch = Some(intent);
    }

    pub fn submit_play_call(&mut self, intent: PlayCallIntent) {
        self.play_call = Some(intent);
    }

    pub fn submit_kick_foul_decision(&mut self, intent: KickFoulDecisionIntent) {
        self.kick_foul_decision = Some(intent);
    }

    pub fn submit_kick_foul_realignment(&mut self, intent: KickFoulRealignmentIntent) {
        self.kick_foul_realignment = Some(intent);
    }

    pub fn take_substitutions(&mut self) -> Vec<SubstitutionIntent> {
        std::mem::take(&mut self.substitutions)
    }

    pub fn take_forced_substitutions(&mut self) -> Vec<ForcedSubstitutionIntent> {
        std::mem::take(&mut self.forced_substitutions)
    }

    pub fn forced_substitution(&self, player_id: uuid::Uuid) -> Option<ForcedSubstitutionIntent> {
        self.forced_substitutions.iter().find(|intent| intent.outgoing_player_id() == player_id).copied()
    }

    pub fn take_forced_substitution(&mut self, player_id: uuid::Uuid) -> Option<ForcedSubstitutionIntent> {
        let index = self.forced_substitutions.iter().position(|intent| intent.outgoing_player_id() == player_id)?;
        Some(self.forced_substitutions.remove(index))
    }

    pub fn take_time_call(&mut self) -> Option<TimeCallIntent> {
        self.time_call.take()
    }

    pub fn take_challenge(&mut self) -> Option<ChallengeIntent> {
        self.challenge.take()
    }

    pub fn take_tactical_switch(&mut self) -> Option<TacticalSwitchIntent> {
        self.tactical_switch.take()
    }

    pub fn take_play_call(&mut self) -> Option<PlayCallIntent> {
        self.play_call.take()
    }

    pub fn take_kick_foul_decision(&mut self) -> Option<KickFoulDecisionIntent> {
        self.kick_foul_decision.take()
    }

    pub fn take_kick_foul_realignment(&mut self) -> Option<KickFoulRealignmentIntent> {
        self.kick_foul_realignment.take()
    }

    pub fn substitutions(&self) -> &[SubstitutionIntent] {
        &self.substitutions
    }

    pub fn forced_substitutions(&self) -> &[ForcedSubstitutionIntent] {
        &self.forced_substitutions
    }

    pub fn time_call(&self) -> Option<TimeCallIntent> {
        self.time_call
    }

    pub fn challenge(&self) -> Option<ChallengeIntent> {
        self.challenge
    }

    pub fn tactical_switch(&self) -> Option<TacticalSwitchIntent> {
        self.tactical_switch
    }

    pub fn play_call(&self) -> Option<PlayCallIntent> {
        self.play_call
    }

    pub fn kick_foul_decision(&self) -> Option<KickFoulDecisionIntent> {
        self.kick_foul_decision
    }

    pub fn kick_foul_realignment(&self) -> Option<KickFoulRealignmentIntent> {
        self.kick_foul_realignment
    }

    pub fn clear(&mut self) {
        self.substitutions.clear();
        self.forced_substitutions.clear();
        self.injury_decisions.clear();
        self.time_call = None;
        self.challenge = None;
        self.tactical_switch = None;
        self.play_call = None;
        self.kick_foul_decision = None;
        self.kick_foul_realignment = None;
    }
}
