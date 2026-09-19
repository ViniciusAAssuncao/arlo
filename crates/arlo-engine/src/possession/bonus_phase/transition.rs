use crate::possession::possession_origin::PossessionOrigin;
use crate::possession::role::PossessionRole;
use crate::possession::series_state::SeriesState;
use uuid::Uuid;

pub struct BonusPhaseTransitionResult {
    pub role: PossessionRole,
    pub triggers_countdown: bool,
}

pub fn resolve_bonus_phase_transition(
    updated_series: &mut SeriesState,
    new_origin: &mut PossessionOrigin,
    current_role: PossessionRole,
    score_occurred: bool,
    turnover_team: Option<Uuid>,
    is_out_or_arbitral: bool,
    next_scrimmage_x: f64,
) -> BonusPhaseTransitionResult {
    if score_occurred || turnover_team.is_some() || updated_series.should_turnover_on_downs() {
        updated_series.reset(next_scrimmage_x);
        updated_series.set_bonus_phase(false);
        new_origin.reset(next_scrimmage_x);

        let next_role = if let Some(t_team) = turnover_team {
            PossessionRole::new(t_team, current_role.offense())
        } else {
            current_role.swap()
        };

        BonusPhaseTransitionResult {
            role: next_role,
            triggers_countdown: true,
        }
    } else {
        if !is_out_or_arbitral {
            updated_series.advance_down();
            updated_series.set_scrimmage_x_mirim(next_scrimmage_x);
        }
        BonusPhaseTransitionResult {
            role: current_role,
            triggers_countdown: false,
        }
    }
}