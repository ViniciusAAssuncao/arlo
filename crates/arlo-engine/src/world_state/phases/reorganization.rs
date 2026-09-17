use crate::attributes::DEFAULT_PLAYER_ATTRIBUTE_TABLE;
use crate::team_identity::tempo::huddle_duration_scale;
use crate::world_state::constants::{
    DEFAULT_ATTRIBUTE_VALUE, HUDDLE_BASE_MAX_SECONDS, HUDDLE_LEADERSHIP_WEIGHT,
    HUDDLE_MAX_SECONDS, HUDDLE_MIN_SECONDS, HUDDLE_TACTICAL_WEIGHT,
};
use crate::world_state::play_transition::publisher::EventPublisher;
use arlo_domain::{AttributeKey, Position as DomainPosition};
use arlo_events::EventSink;
use arlo_math::units::Duration;
use uuid::Uuid;

pub fn derive_and_apply_reorganization(
    publisher: &mut EventPublisher<'_, impl EventSink>,
    _scrimmage_x_mirim: f64,
    _is_post_turnover: bool,
    _recovering_player_id: Option<Uuid>,
) -> (Duration, Duration) {
    let home_lineup = publisher.state().home_lineup_arc();
    let away_lineup = publisher.state().away_lineup_arc();
    let player_attribute_tables = publisher.state().teams.player_attribute_tables().clone();

    let is_home_offense = publisher
        .state()
        .possession()
        .role()
        .is_offense(publisher.state().home_team_id());

    let (offense_lineup, offense_instructions) = if is_home_offense {
        (home_lineup, *publisher.state().home_instructions())
    } else {
        (away_lineup, *publisher.state().away_instructions())
    };

    let offense_artrine = offense_lineup
        .assignments()
        .iter()
        .map(|a| a.player())
        .find(|p| {
            p.positions()
                .iter()
                .any(|pos| pos.position() == DomainPosition::Artrine && pos.proficiency() > 0)
        });

    let (tac, lead) = if let Some(artrine) = offense_artrine {
        let table = player_attribute_tables
            .get(&artrine.id())
            .unwrap_or(&DEFAULT_PLAYER_ATTRIBUTE_TABLE);
        let tac = table.get(AttributeKey::TacticalKnowledge);
        let lead = table.get(AttributeKey::Leadership);
        (tac, lead)
    } else {
        (DEFAULT_ATTRIBUTE_VALUE, DEFAULT_ATTRIBUTE_VALUE)
    };

    let base_huddle_seconds = (HUDDLE_BASE_MAX_SECONDS
        - (tac * HUDDLE_TACTICAL_WEIGHT + lead * HUDDLE_LEADERSHIP_WEIGHT))
        .clamp(HUDDLE_MIN_SECONDS, HUDDLE_MAX_SECONDS);
    let huddle_scale = huddle_duration_scale(offense_instructions.in_possession().tempo());
    let huddle_seconds =
        (base_huddle_seconds * huddle_scale).clamp(HUDDLE_MIN_SECONDS, HUDDLE_MAX_SECONDS);

    (Duration::new(0.0), Duration::new(huddle_seconds))
}
