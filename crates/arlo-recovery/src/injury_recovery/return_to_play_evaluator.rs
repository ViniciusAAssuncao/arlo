use crate::domain::InjuryStatusKind;

pub fn is_available_for_selection(status: InjuryStatusKind) -> bool {
    match status {
        InjuryStatusKind::Injured => false,
        InjuryStatusKind::Observation | InjuryStatusKind::Healthy => true,
    }
}