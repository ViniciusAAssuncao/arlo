use crate::manager_ai::tactical_adjustment::approach_alignment::{
    defensive_approach_alignment, offensive_approach_alignment,
};
use arlo_domain::ManagerTacticalProfile;
use arlo_tactics::{SituationalContext, TeamTacticalProfile};

pub fn score_candidate(
    profile: &TeamTacticalProfile,
    situational_context: &SituationalContext,
    manager_tactical_profile: Option<&ManagerTacticalProfile>,
) -> f64 {
    let sit_fit = profile
        .situational_profile()
        .map(|p| p.fit_score(situational_context))
        .unwrap_or(0.5);

    if let Some(mtp) = manager_tactical_profile {
        let off_align = offensive_approach_alignment(
            mtp.offensive_approach(),
            profile.instructions().in_possession(),
        );
        let def_align = defensive_approach_alignment(
            mtp.defensive_approach(),
            profile.instructions().out_of_possession(),
        );
        let alignment = (off_align + def_align) / 2.0;
        (0.60 * sit_fit + 0.40 * alignment).clamp(0.0, 1.0)
    } else {
        sit_fit.clamp(0.0, 1.0)
    }
}