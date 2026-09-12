use arlo_domain::sport_constants::{
    INJURY_SEVERITY_GRADE_2_THRESHOLD, INJURY_SEVERITY_GRADE_3_THRESHOLD,
};
use arlo_domain::InjurySeverityGrade;

pub fn estimate_injury_severity(stimulus: f64) -> InjurySeverityGrade {
    let normalized = stimulus.clamp(0.0, 1.0);
    if normalized >= INJURY_SEVERITY_GRADE_3_THRESHOLD {
        InjurySeverityGrade::Grade3
    } else if normalized >= INJURY_SEVERITY_GRADE_2_THRESHOLD {
        InjurySeverityGrade::Grade2
    } else {
        InjurySeverityGrade::Grade1
    }
}
