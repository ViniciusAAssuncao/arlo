use crate::error::{DbError, DbResult};
use arlo_domain::InjuryMechanism;

pub fn parse_injury_mechanism(code: &str) -> DbResult<InjuryMechanism> {
    match code {
        "Contact" | "contact" => Ok(InjuryMechanism::Contact),
        "NonContact" | "non_contact" | "noncontact" => Ok(InjuryMechanism::NonContact),
        _ => Err(DbError::InvalidEnum(format!(
            "Invalid injury mechanism: {code}"
        ))),
    }
}

pub fn injury_mechanism_to_code(mechanism: InjuryMechanism) -> &'static str {
    match mechanism {
        InjuryMechanism::Contact => "Contact",
        InjuryMechanism::NonContact => "NonContact",
    }
}