use crate::eligibility::eligible;
use crate::scoring::base_utility;
use crate::voting::vote;
use arlo_domain::{
    AwardCandidateEvidence, AwardDefinition, AwardInstanceContext, AwardSelectionPolicy,
};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use std::collections::HashSet;
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum AwardError {
    #[error("invalid award definition: {0}")]
    InvalidDefinition(String),
    #[error("no eligible award candidates")]
    NoEligibleCandidates,
    #[error("invalid award candidates: {0}")]
    InvalidCandidates(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct CandidateResult {
    pub subject_id: Uuid,
    pub utility: f64,
    pub selection_score: f64,
    pub vote_points: u64,
    pub first_place_votes: u32,
    pub rank: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ElectorateResult {
    pub group_code: String,
    pub voter_count: u32,
    pub result_weight: f64,
    pub subject_ids: Vec<Uuid>,
    pub points: Vec<u64>,
    pub first_place: Vec<u32>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AwardResolution {
    pub definition_id: Uuid,
    pub period_key: String,
    pub scope_id: Option<Uuid>,
    pub seed: u64,
    pub model_version: u32,
    pub winner_id: Uuid,
    pub candidates: Vec<CandidateResult>,
    pub electorates: Vec<ElectorateResult>,
}

pub fn resolve_award(
    definition: &AwardDefinition,
    context: &AwardInstanceContext,
    candidates: &[AwardCandidateEvidence],
    seed: u64,
) -> Result<AwardResolution, AwardError> {
    validate(definition)?;
    if context.period_key.is_empty() || context.selection_model_version == 0 {
        return Err(AwardError::InvalidDefinition(
            "instance context requires a period and model version".into(),
        ));
    }
    let mut pool: Vec<_> = candidates
        .iter()
        .filter(|candidate| eligible(definition, candidate))
        .cloned()
        .collect();
    pool.sort_by_key(|candidate| candidate.subject_id);
    let mut subject_ids = HashSet::new();
    if pool.iter().any(|candidate| {
        !subject_ids.insert(candidate.subject_id)
            || candidate
                .metrics
                .iter()
                .any(|metric| !metric.value.is_finite())
    }) {
        return Err(AwardError::InvalidCandidates(
            "duplicate subjects or nonfinite metrics".into(),
        ));
    }
    if pool.is_empty() {
        return Err(AwardError::NoEligibleCandidates);
    }
    let mut scored: Vec<_> = pool
        .iter()
        .map(|candidate| CandidateResult {
            subject_id: candidate.subject_id,
            utility: base_utility(definition, candidate, &pool),
            selection_score: 0.0,
            vote_points: 0,
            first_place_votes: 0,
            rank: 0,
        })
        .collect();
    scored.sort_by(|a, b| {
        b.utility
            .total_cmp(&a.utility)
            .then_with(|| a.subject_id.cmp(&b.subject_id))
    });
    if let Some(limit) = definition.nomination_limit {
        scored.truncate(limit);
    }
    let pool: Vec<_> = scored
        .iter()
        .filter_map(|item| {
            pool.iter()
                .find(|candidate| candidate.subject_id == item.subject_id)
                .cloned()
        })
        .collect();
    let mut rng = ChaCha8Rng::seed_from_u64(seed);
    let electorates = match &definition.selection {
        AwardSelectionPolicy::Utility { temperature } => {
            for result in &mut scored {
                let draw = rng.gen_range(f64::MIN_POSITIVE..1.0_f64);
                result.selection_score = result.utility + temperature * -(-draw.ln()).ln();
            }
            Vec::new()
        }
        AwardSelectionPolicy::RankedVoting {
            ballot_points,
            groups,
        } => vote(
            definition,
            &pool,
            &mut scored,
            groups,
            ballot_points,
            &mut rng,
        ),
    };
    scored.sort_by(|a, b| {
        b.selection_score
            .total_cmp(&a.selection_score)
            .then_with(|| b.first_place_votes.cmp(&a.first_place_votes))
            .then_with(|| b.utility.total_cmp(&a.utility))
            .then_with(|| a.subject_id.cmp(&b.subject_id))
    });
    for (index, result) in scored.iter_mut().enumerate() {
        result.rank = index + 1;
    }
    Ok(AwardResolution {
        definition_id: definition.id,
        period_key: context.period_key.clone(),
        scope_id: context.scope_id,
        seed,
        model_version: context.selection_model_version,
        winner_id: scored[0].subject_id,
        candidates: scored,
        electorates,
    })
}

fn validate(definition: &AwardDefinition) -> Result<(), AwardError> {
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
