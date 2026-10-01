use crate::playcall::situational::context::SituationalContext;
use crate::playcall::situational::profile::SituationalProfile;

pub trait HasSituationalProfile {
    fn situational_profile(&self) -> Option<&SituationalProfile>;
}

pub fn rank_by_situational_fit<'a, T: HasSituationalProfile>(
    candidates: &'a [T],
    context: &SituationalContext,
) -> Vec<(&'a T, f64)> {
    let mut scored: Vec<(&'a T, f64)> = candidates
        .iter()
        .map(|item| {
            let score = match item.situational_profile() {
                Some(profile) => profile.fit_score(context),
                None => 0.0,
            };
            (item, score)
        })
        .collect();

    scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    scored
}
