use crate::injury::outcome::InjuryIncidentResolution;
use arlo_events::InjuryIncidentRecorded;
use arlo_math::Probability;

pub fn translate_injury_incident(
    resolution: &InjuryIncidentResolution,
) -> InjuryIncidentRecorded {
    InjuryIncidentRecorded::new(
        resolution.injured_player_id,
        resolution.team_id,
        resolution.mechanism,
        resolution.body_region,
        resolution.severity_grade,
        resolution.injury_definition_id.unwrap_or_default(),
        Probability::new_clamped(resolution.trigger_probability),
    )
}