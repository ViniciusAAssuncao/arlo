use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InjurySeverityGrade {
    Grade1,
    Grade2,
    Grade3,
}