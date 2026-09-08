use arlo_domain::Position;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum MarkingAssignment {
    #[default]
    Zonal,
    Man(Position),
}