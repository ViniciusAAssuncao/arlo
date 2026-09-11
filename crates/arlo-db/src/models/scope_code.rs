use crate::error::{DbError, DbResult};
use arlo_domain::Scope;

pub fn parse_scope(code: &str) -> DbResult<Scope> {
    match code {
        "Regional" | "regional" => Ok(Scope::Regional),
        "National" | "national" => Ok(Scope::National),
        "Continental" | "continental" => Ok(Scope::Continental),
        "International" | "international" => Ok(Scope::International),
        _ => Err(DbError::InvalidEnum(format!("Invalid scope: {code}"))),
    }
}

pub fn scope_to_code(scope: Scope) -> &'static str {
    match scope {
        Scope::Regional => "Regional",
        Scope::National => "National",
        Scope::Continental => "Continental",
        Scope::International => "International",
    }
}