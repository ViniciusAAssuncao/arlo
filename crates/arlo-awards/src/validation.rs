use crate::AwardError;
use arlo_domain::{AwardDefinition, AwardSelectionPolicy};

pub(crate) fn validate(definition: &AwardDefinition) -> Result<(), AwardError> {
    if definition
        .minimum_age
        .zip(definition.maximum_age)
        .is_some_and(|(minimum, maximum)| minimum > maximum)
    {
        return Err(AwardError::InvalidDefinition(
            "minimum age exceeds maximum age".into(),
        ));
    }
    if definition.criteria.is_empty()
        || definition
            .criteria
            .iter()
            .map(|item| item.weight)
            .sum::<f64>()
            <= 0.0
        || definition
            .criteria
            .iter()
            .any(|item| !item.weight.is_finite() || item.weight < 0.0)
    {
        return Err(AwardError::InvalidDefinition(
            "criteria must have finite nonnegative weights".into(),
        ));
    }
    if definition.nomination_limit == Some(0) {
        return Err(AwardError::InvalidDefinition(
            "nomination limit must be positive".into(),
        ));
    }
    match &definition.selection {
        AwardSelectionPolicy::Utility { temperature }
            if !temperature.is_finite() || *temperature < 0.0 =>
        {
            Err(AwardError::InvalidDefinition(
                "temperature must be finite and nonnegative".into(),
            ))
        }
        AwardSelectionPolicy::RankedVoting {
            ballot_points,
            groups,
        } if ballot_points.is_empty()
            || ballot_points[0] == 0
            || groups.is_empty()
            || groups.iter().any(|group| {
                group.voter_count == 0
                    || !group.result_weight.is_finite()
                    || group.result_weight <= 0.0
                    || group
                        .criterion_preferences
                        .iter()
                        .any(|preference| !preference.multiplier.is_finite())
            }) =>
        {
            Err(AwardError::InvalidDefinition(
                "ranked voting requires points and positive electorates".into(),
            ))
        }
        _ => Ok(()),
    }
}
