use arlo_domain::sport_constants::MINIMUM_ADVANCE_MIRINS_PER_SERIES;
use arlo_domain::AttributeKey;
use arlo_engine::{MatchInput, MatchState};
use arlo_tactics::{
    derive_distance_urgency, derive_down_pressure, derive_drive_scarcity, derive_scoring_proximity,
    PlayCall, PlayCallCategory, SituationalContext,
};

pub(crate) fn select<'a>(
    input: &MatchInput,
    state: &MatchState,
    calls: &'a [PlayCall],
) -> Option<&'a PlayCall> {
    let team_id = state.next_call_team_id();
    let team = if team_id == input.home().team_id() {
        input.home()
    } else {
        input.away()
    };
    if team.manager().is_human_controlled() {
        return None;
    }
    let home = team_id == input.home().team_id();
    let drives = if home {
        state.home().drive_progress().completed_drives()
    } else {
        state.away().drive_progress().completed_drives()
    };
    let position = state.last_valid_possession_mirim();
    let proximity = if home {
        position / input.pitch().length_mirim()
    } else {
        1.0 - position / input.pitch().length_mirim()
    };
    let series = state.series();
    let down = if series.team_id() == team_id {
        series.down()
    } else {
        1
    };
    let distance = if series.team_id() == team_id {
        series.distance_to_gain_mirim()
    } else {
        MINIMUM_ADVANCE_MIRINS_PER_SERIES
    };
    let context = SituationalContext::new(
        derive_down_pressure(down),
        derive_distance_urgency(distance),
        derive_scoring_proximity(proximity),
        derive_drive_scarcity(drives),
    );
    let knowledge = manager_value(input, team.manager(), AttributeKey::TacticalKnowledge);
    let adjustments = manager_value(input, team.manager(), AttributeKey::InGameAdjustments);
    let reading = ((knowledge + adjustments) / 40.0).clamp(0.15, 1.0);
    let elapsed = (state.clock().total_elapsed_seconds() / 7200.0).clamp(0.0, 1.0);
    let score_delta = if home {
        state.home().score().total_points() as f64 - state.away().score().total_points() as f64
    } else {
        state.away().score().total_points() as f64 - state.home().score().total_points() as f64
    };
    let chase = (-score_delta / 24.0).clamp(0.0, 1.0) * elapsed * elapsed;
    let protect = (score_delta / 24.0).clamp(0.0, 1.0) * elapsed * elapsed;
    let identity = state.team_instructions(team).default_decision_emphasis();
    let score = |call: &PlayCall| {
        let fit = call
            .situational_profile()
            .map_or(0.5, |profile| profile.fit_score(&context));
        let emphasis = call.decision_emphasis();
        let style_fit = 1.0
            - ((emphasis.short_pass().value() - identity.short_pass().value()).abs()
                + (emphasis.long_launch().value() - identity.long_launch().value()).abs()
                + (emphasis.self_finish().value() - identity.self_finish().value()).abs())
                / 3.0;
        let breadth = call
            .situational_profile()
            .map_or(1.0, |profile| profile.targeting_flexibility().value());
        reading * (fit - breadth * 0.05)
            + (1.0 - reading) * style_fit
            + chase * (emphasis.self_finish().value() + emphasis.long_launch().value()) * 0.08
            + protect * emphasis.short_pass().value() * 0.08
    };
    calls
        .iter()
        .filter(|call| {
            call.team_id() == team_id
                && call.tactical_lineup_id() == state.team_lineup(team).id()
                && call.category() == PlayCallCategory::OpenPlay
        })
        .max_by(|left, right| {
            score(left)
                .total_cmp(&score(right))
                .then_with(|| left.id().cmp(&right.id()))
        })
}

fn manager_value(input: &MatchInput, manager: &arlo_domain::Manager, key: AttributeKey) -> f64 {
    manager
        .attributes()
        .iter()
        .find(|value| {
            input
                .manager_attribute_keys()
                .get(&value.attribute_definition_id())
                == Some(&key)
        })
        .map_or(10.0, |value| f64::from(value.value()))
}
