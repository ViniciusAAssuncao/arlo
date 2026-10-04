mod eligibility;
mod resolution;
mod roster;
mod scoring;
mod validation;
mod voting;

pub use resolution::{
    resolve_award, AwardError, AwardResolution, CandidateResult, ElectorateResult,
};
pub use roster::{resolve_roster_award, AwardRosterResolution, RosterSelection};
mod dynamic_roster;
