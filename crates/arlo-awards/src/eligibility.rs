use arlo_domain::{AwardCandidateEvidence, AwardDefinition};

pub(crate) fn eligible(definition: &AwardDefinition, candidate: &AwardCandidateEvidence) -> bool {
    candidate.subject_kind == definition.recipient_kind
        && candidate.matches_played >= definition.minimum_matches
        && definition
            .minimum_age
            .is_none_or(|age| candidate.age_years.is_some_and(|value| value >= age))
        && definition
            .maximum_age
            .is_none_or(|age| candidate.age_years.is_some_and(|value| value <= age))
        && (definition.eligible_positions.is_empty()
            || candidate
                .position
                .as_ref()
                .is_some_and(|position| definition.eligible_positions.contains(position))
            || candidate
                .positions
                .iter()
                .any(|position| definition.eligible_positions.contains(position)))
        && (definition.eligible_countries.is_empty()
            || candidate
                .country_id
                .is_some_and(|id| definition.eligible_countries.contains(&id)))
        && (definition.eligible_continents.is_empty()
            || candidate
                .continent_id
                .is_some_and(|id| definition.eligible_continents.contains(&id)))
        && (definition.eligible_competitions.is_empty()
            || candidate
                .competition_id
                .is_some_and(|id| definition.eligible_competitions.contains(&id)))
}
