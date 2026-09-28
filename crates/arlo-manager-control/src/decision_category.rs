use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ManagerDecisionCategory {
    VoluntarySubstitution,
    ForcedSubstitution,
    InjuryResponse,
    TimeCall,
    Challenge,
    TacticalSwitch,
    PlayCall,
    KickFoulRealignment,
    KickFoulDecision,
}
