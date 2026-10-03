use super::context::ManagerDecisionContext;
use super::random::sample;

pub(super) fn aspiration(context: &ManagerDecisionContext) -> f64 {
    let chase = (context.deficit / 15.0).clamp(0.0, 1.0) * context.progress.powi(2);
    0.045 + context.skills.patience * 0.025 + (1.0 - context.skills.flexibility) * 0.025
        - chase * 0.02
}

pub(super) fn choose(
    context: &ManagerDecisionContext,
    candidates: &[f64],
    threshold: f64,
) -> Option<usize> {
    let best = *candidates.first()?;
    if best < threshold {
        return None;
    }
    let competence =
        (context.skills.judging + context.skills.adjustments + context.skills.knowledge) / 3.0;
    let temperature = 0.004 + (1.0 - competence) * 0.025;
    let proximity = 0.015 + (1.0 - competence) * 0.05;
    let plausible: Vec<_> = candidates
        .iter()
        .enumerate()
        .filter(|(_, candidate)| **candidate >= threshold && **candidate >= best - proximity)
        .take(3)
        .map(|(index, candidate)| (index, ((*candidate - best) / temperature).exp()))
        .collect();
    let total: f64 = plausible.iter().map(|(_, weight)| weight).sum();
    let mut draw = sample(
        context,
        context.manager_id,
        context.sequence ^ 0x63686f696365,
    ) * total;
    for &(index, weight) in &plausible {
        if draw < weight {
            return Some(index);
        }
        draw -= weight;
    }
    plausible.last().map(|(index, _)| *index)
}
