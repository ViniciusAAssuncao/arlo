mod eligibility;
mod resolution;
mod roster;
mod scoring;
mod voting;
mod validation;

pub use resolution::{
    resolve_award, AwardError, AwardResolution, CandidateResult, ElectorateResult,
};
pub use roster::{resolve_roster_award, AwardRosterResolution, RosterSelection};
