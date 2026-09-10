use arlo_domain::pitch::Pitch;
use arlo_domain::{
    AttributeCategory, AttributeDefinition, AttributeKey, AttributeTarget, CaptaincyRole,
    Formation, FormationSlot, Manager, ManagerAttributeValue, MatchFormatRules, Person, Player,
    PlayerAttributeValue, PlayerPosition, Position,
};
use arlo_engine::rng::MatchSeed;
use arlo_engine::world_state::match_state::{MatchSetupParams, MatchState, TeamSetupParams};
use arlo_engine::world_state::step::step_call_to_action;
use arlo_events::{EventSink, MatchEventEnvelope};
use arlo_tactics::{TacticalLineup, TeamInstructions, TeamTacticalProfile};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::collections::HashMap;
use uuid::Uuid;

struct NullEventSink;

impl EventSink for NullEventSink {
    fn record(&mut self, _envelope: MatchEventEnvelope) {}
}

fn create_full_player(
    id: Uuid,
    team_id: Uuid,
    name: &str,
    pos: Position,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
) -> Player {
    let mut attributes = Vec::new();
    for (&def_id, &key) in attribute_keys {
        let def = AttributeDefinition::new(
            def_id,
            key,
            "Attr",
            AttributeCategory::Technical,
            AttributeTarget::Player,
        )
        .unwrap();
        attributes.push(PlayerAttributeValue::new(&def, 14).unwrap());
    }
    Player::builder(
        id,
        name,
        1.85,
        0,
        Uuid::new_v4(),
    )
    .with_team_id(Some(team_id))
    .with_positions(vec![PlayerPosition::new(pos, 10).unwrap()])
    .with_attributes(attributes)
    .with_captaincy_role(Some(CaptaincyRole::Captain))
    .build()
    .unwrap()
}

fn build_14_slot_formation() -> Formation {
    let positions = [
        (Position::Goalguard, Position::Goalguard, 0.05, 0.5),
        (Position::Passer, Position::PassRusher, 0.20, 0.5),
        (Position::Artrine, Position::Lineback, 0.30, 0.5),
        (Position::CenterOffense, Position::Centerback, 0.85, 0.5),
        (Position::WingOffense, Position::OutsideZonerback, 0.75, 0.2),
        (Position::WingOffense, Position::OutsideZonerback, 0.75, 0.8),
        (Position::Midcenter, Position::MiddleZonerback, 0.50, 0.5),
        (Position::TightWing, Position::WideBlocker, 0.60, 0.3),
        (Position::TightWing, Position::WideBlocker, 0.60, 0.7),
        (Position::CenterTight, Position::DefensiveBlocker, 0.55, 0.5),
        (Position::Corridor, Position::DefensiveEnd, 0.40, 0.3),
        (Position::Corridor, Position::DefensiveEnd, 0.40, 0.7),
        (Position::WideEnd, Position::Rougieback, 0.65, 0.15),
        (Position::RunningEnd, Position::Rougieback, 0.45, 0.5),
    ];

    let mut slots = Vec::new();
    for (off_p, def_p, rx, ry) in positions {
        slots.push(FormationSlot::with_dual_positions(off_p, def_p, rx, ry).unwrap());
    }

    Formation::new(Uuid::new_v4(), "Standard 14", slots).unwrap()
}

fn setup_benchmark_match() -> (MatchState, NullEventSink) {
    let pitch = Pitch::from_mirim(145.0, 85.0).unwrap();
    let mut attribute_keys = HashMap::new();
    let all_keys = [
        AttributeKey::Passing,
        AttributeKey::Vision,
        AttributeKey::Technique,
        AttributeKey::Decisions,
        AttributeKey::Composure,
        AttributeKey::Anticipation,
        AttributeKey::Teamwork,
        AttributeKey::Balance,
        AttributeKey::Stamina,
        AttributeKey::Flair,
        AttributeKey::WorkRate,
        AttributeKey::Concentration,
        AttributeKey::Leadership,
        AttributeKey::Positioning,
        AttributeKey::Determination,
        AttributeKey::Acceleration,
        AttributeKey::Pace,
        AttributeKey::Strength,
        AttributeKey::Agility,
        AttributeKey::Bravery,
        AttributeKey::Crossing,
        AttributeKey::Dribbling,
        AttributeKey::Reflexes,
        AttributeKey::Handling,
        AttributeKey::RushingOut,
        AttributeKey::Communication,
        AttributeKey::Distribution,
        AttributeKey::OneOnOne,
        AttributeKey::ControlledAggression,
        AttributeKey::DefensiveContainment,
        AttributeKey::PasserPressure,
        AttributeKey::JumpingReach,
        AttributeKey::NaturalFitness,
        AttributeKey::OffensiveBlocking,
        AttributeKey::GoalKicking,
        AttributeKey::DriveTechnique,
        AttributeKey::ArloControl,
        AttributeKey::HandsReception,
        AttributeKey::FalseArtrineBluff,
        AttributeKey::Consistency,
        AttributeKey::TacticalKnowledge,
    ];

    for &key in &all_keys {
        attribute_keys.insert(Uuid::new_v4(), key);
    }

    let home_team_id = Uuid::new_v4();
    let away_team_id = Uuid::new_v4();

    let formation = build_14_slot_formation();

    let mut home_roster = Vec::new();
    let mut away_roster = Vec::new();

    for (idx, slot) in formation.slots().iter().enumerate() {
        home_roster.push(create_full_player(
            Uuid::new_v4(),
            home_team_id,
            &format!("HomePlayer_{idx}"),
            slot.offensive_position(),
            &attribute_keys,
        ));
        away_roster.push(create_full_player(
            Uuid::new_v4(),
            away_team_id,
            &format!("AwayPlayer_{idx}"),
            slot.offensive_position(),
            &attribute_keys,
        ));
    }

    let mut home_builder = TacticalLineup::builder(Uuid::new_v4(), home_team_id, "Home Lineup")
        .with_formation(&formation);
    let mut away_builder = TacticalLineup::builder(Uuid::new_v4(), away_team_id, "Away Lineup")
        .with_formation(&formation);

    for (idx, _slot) in formation.slots().iter().enumerate() {
        home_builder = home_builder.assign(idx, home_roster[idx].id());
        away_builder = away_builder.assign(idx, away_roster[idx].id());
    }

    let home_tactical_lineup = home_builder.build(&home_roster).unwrap();
    let away_tactical_lineup = away_builder.build(&away_roster).unwrap();

    let home_profile = TeamTacticalProfile::new(
        Uuid::new_v4(),
        home_team_id,
        "Home Profile",
        TeamInstructions::default(),
        None,
        true,
    );
    let away_profile = TeamTacticalProfile::new(
        Uuid::new_v4(),
        away_team_id,
        "Away Profile",
        TeamInstructions::default(),
        None,
        true,
    );

    let mut mgr_attrs = Vec::new();
    for (&def_id, &key) in &attribute_keys {
        let def = AttributeDefinition::new(
            def_id,
            key,
            "Attr",
            AttributeCategory::Managerial,
            AttributeTarget::Manager,
        )
        .unwrap();
        mgr_attrs.push(ManagerAttributeValue::new(&def, 14).unwrap());
    }

    let home_person = Person::new(Uuid::new_v4(), "Home Manager", 1.80, 0, Uuid::new_v4()).unwrap();
    let away_person = Person::new(Uuid::new_v4(), "Away Manager", 1.80, 0, Uuid::new_v4()).unwrap();

    let home_manager = Manager::new(
        home_person,
        Some(home_team_id),
        mgr_attrs.clone(),
        None,
    ).unwrap();

    let away_manager = Manager::new(
        away_person,
        Some(away_team_id),
        mgr_attrs,
        None,
    ).unwrap();

    let home_params = TeamSetupParams::new(
        home_team_id,
        home_tactical_lineup,
        formation.clone(),
        home_roster,
        home_profile.clone(),
        home_manager,
        vec![home_profile],
        vec![],
    );

    let away_params = TeamSetupParams::new(
        away_team_id,
        away_tactical_lineup,
        formation,
        away_roster,
        away_profile.clone(),
        away_manager,
        vec![away_profile],
        vec![],
    );

    let setup = MatchSetupParams::new(
        home_params,
        away_params,
        pitch,
        attribute_keys,
        MatchFormatRules::default_ruleset(),
        MatchSeed::new(12345),
    );

    let state = MatchState::new(setup).unwrap();
    (state, NullEventSink)
}

fn bench_full_play_simulation(c: &mut Criterion) {
    let (base_state, mut sink) = setup_benchmark_match();

    let mut group = c.benchmark_group("full_play_simulation");

    group.bench_function("step_call_to_action_single", |b| {
        b.iter(|| {
            let mut state = base_state.clone();
            step_call_to_action(black_box(&mut state), black_box(&mut sink))
        })
    });

    group.finish();
}

criterion_group!(benches, bench_full_play_simulation);
criterion_main!(benches);
