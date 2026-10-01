use arlo_domain::ManagerControlMode;

pub fn parse_manager_control_mode(code: Option<&str>) -> ManagerControlMode {
    match code {
        Some("Human" | "human") => ManagerControlMode::Human,
        _ => ManagerControlMode::Ai,
    }
}

pub fn manager_control_mode_to_code(mode: ManagerControlMode) -> Option<&'static str> {
    match mode {
        ManagerControlMode::Human => Some("Human"),
        ManagerControlMode::Ai => Some("Ai"),
    }
}
