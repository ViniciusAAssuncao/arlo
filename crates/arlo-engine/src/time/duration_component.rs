use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DurationComponentKind {
    PassProtectionEngagement,
    InitialHandoffFlight,
    ArtroBreakthroughEngagement,
    CarrierMovement,
    DistributionEngagement,
    DistributionFlight,
    BallSecurityEngagement,
    FinishingEngagement,
    ShotFlight,
    CrossFlight,
    Reorganization,
}
