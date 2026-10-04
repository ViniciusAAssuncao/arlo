mod eligibility;
mod resolution;
mod scoring;
mod voting;

pub use resolution::{
    resolve_award, AwardError, AwardResolution, CandidateResult, ElectorateResult,
};
