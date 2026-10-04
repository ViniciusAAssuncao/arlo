mod model;

pub use model::{
    calculate, estimate, AttendanceOutcome, EventDemand, ExpectedAttendance, TeamDemand,
    VenueDemand, ATTENDANCE_MODEL_VERSION, EXPECTED_ATTENDANCE_MODEL_VERSION,
};
