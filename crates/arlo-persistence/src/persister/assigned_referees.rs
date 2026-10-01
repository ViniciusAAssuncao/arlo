use crate::error::{PersistenceError, PersistenceResult};
use arlo_domain::Referee;
use arlo_engine::MatchInput;

pub(super) fn assigned_referees(input: &MatchInput) -> PersistenceResult<(&Referee, &Referee)> {
    let head = input
        .referees()
        .first()
        .ok_or_else(|| PersistenceError::InvalidData("match has no head referee".into()))?;
    let peace = input
        .referees()
        .get(1)
        .ok_or_else(|| PersistenceError::InvalidData("match has no peace referee".into()))?;
    Ok((head, peace))
}
