use crate::injury::exertion::{evaluate_and_resolve_exertion_injury, ExertionInjuryContext};
use crate::physical::models::age::calculate_player_age;
use crate::rng::RngStream;
use crate::world_state::play_transition::publisher::EventPublisher;
use arlo_domain::Position;
use arlo_events::EventSink;
use std::collections::HashSet;
use uuid::Uuid;

pub fn evaluate_and_apply_exertion_injuries(
    publisher: &mut EventPublisher<'_, impl EventSink>,
    participated_ids: &HashSet<Uuid>,
    live_seconds: f64,
) {
    let match_date = publisher.state().match_date_unix_seconds();
    let injury_tuning = *publisher.state().tuning().injury_tuning();
    let catalog = publisher.state().injury_catalog_arc();

    for &pid in participated_ids {
        let is_home = publisher.state().teams.is_home_player(&pid);
        let team_id = if is_home {
            publisher.state().home_team_id()
        } else {
            publisher.state().away_team_id()
        };

        let player = match publisher.state().teams.find_player(&pid) {
            Some(p) => p.clone(),
            None => continue,
        };

        let age_years = calculate_player_age(&player, match_date);
        let injury_profile = publisher.state().player_injury_profile(&pid);
        let player_fatigue = publisher.state().fatigue_for(&pid);
        let intensity_strain = 1.0 - player_fatigue.w_prime_balance();
        let table = publisher.state().attribute_table_for(&pid);

        let pos = publisher
            .state()
            .offensive_position_index_for_team(team_id)
            .get(&pid)
            .copied()
            .unwrap_or(Position::CenterOffense);

        let exertion_ctx = ExertionInjuryContext::new(
            pid,
            team_id,
            table,
            player_fatigue,
            injury_profile,
            intensity_strain,
            live_seconds,
            age_years,
        );

        let seq = publisher.state().event_sequence();
        let mut injury_rng = publisher
            .state()
            .rng_provider()
            .indexed_rng_for(RngStream::InjuryResolution, seq);

        if let Some(injury_resolution) =
            evaluate_and_resolve_exertion_injury(&exertion_ctx, pos, &catalog, &injury_tuning, &mut injury_rng)
        {
            publisher.emit_injury_incident(&injury_resolution);
        }
    }
}
