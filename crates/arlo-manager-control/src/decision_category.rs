use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ManagerDecisionCategory {
    Substitution,
    TimeCall,
    Challenge,
    TacticalSwitch,
    PlayCall,
    KickFoulRealignment,
}