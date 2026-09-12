use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BodyRegion {
    Head,
    Neck,
    Shoulder,
    Arm,
    Hand,
    Trunk,
    Hip,
    Groin,
    Thigh,
    Knee,
    Calf,
    Ankle,
    Foot,
}