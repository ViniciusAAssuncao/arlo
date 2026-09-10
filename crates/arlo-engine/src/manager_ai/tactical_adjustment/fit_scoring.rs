use crate::manager_ai::tactical_adjustment::approach_alignment::{
    aeriality_alignment, defensive_approach_alignment, offensive_approach_alignment,
    passing_range_alignment, physicality_alignment, press_block_shape_alignment,
    structure_alignment, transition_pace_alignment,
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
        let pass_align = passing_range_alignment(
            mtp.passing_range_preference(),
            profile.instructions().in_possession(),
        );
        let aer_align = aeriality_alignment(
            mtp.aeriality_preference(),
            profile.instructions().in_possession(),
        );
        let struct_align = structure_alignment(
            mtp.structure_preference(),
            profile.instructions().in_possession(),
        );
        let phys_align = physicality_alignment(
            mtp.physicality_preference(),
            profile.instructions().in_possession(),
        );
        let trans_align = transition_pace_alignment(
            mtp.transition_pace_preference(),
            profile.instructions().transition(),
        );
        let press_align = press_block_shape_alignment(
            mtp.press_block_shape_preference(),
            profile.instructions().transition(),
        );
        let alignment = (off_align
            + def_align
            + pass_align
            + aer_align
            + struct_align
            + phys_align
            + trans_align
            + press_align)
            / 8.0;
        (0.60 * sit_fit + 0.40 * alignment).clamp(0.0, 1.0)
    } else {
        sit_fit.clamp(0.0, 1.0)
    }
}
