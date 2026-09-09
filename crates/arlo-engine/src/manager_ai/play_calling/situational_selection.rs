use arlo_tactics::{PlayCall, PlayCallCategory, SituationalContext};

pub fn rank_playbook(
    playbook: &[PlayCall],
    situational_context: &SituationalContext,
    category: PlayCallCategory,
) -> Vec<(usize, f64)> {
    playbook
        .iter()
        .enumerate()
        .filter(|(_, pc)| pc.category() == category)
        .map(|(idx, pc)| {
            let score = pc
                .situational_profile()
                .map(|p| p.fit_score(situational_context))
                .unwrap_or(0.5);
            (idx, score)
        })
        .collect()
}