use arlo_domain::pitch::Pitch;
use arlo_domain::{
    AttributeCategory, AttributeDefinition, AttributeKey, AttributeTarget, CaptaincyRole,
    Formation, FormationSlot, Player, PlayerAttributeValue, PlayerPosition, Position,
};
use arlo_engine::attributes::PlayerAttributeTable;
use arlo_engine::lineup_runtime::Lineup;
use arlo_engine::physical::PhysicalState;
use arlo_engine::spatial::movement_context::MovementContext;
use arlo_engine::spatial::steering::{
    calculate_dynamic_boid_steering_velocity_with_context, SpatialNeighbor,
};
use arlo_engine::spatial::tick_loop::run_spatial_tick_loop_with_context;
use arlo_engine::spatial::DynamicSpatialMap;
use arlo_math::units::{
    Duration, Position as VectorPosition, Speed, Velocity, MIRIM_TO_METERS,
};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::collections::HashMap;
use uuid::Uuid;

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

fn create_player(id: Uuid, name: &str, pos: Position, attribute_keys: &HashMap<Uuid, AttributeKey>) -> Player {
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
    let team_id = Uuid::new_v4();
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

fn setup_spatial_benchmark() -> (
    Pitch,
    DynamicSpatialMap,
    Vec<Player>,
    Vec<(Uuid, VectorPosition)>,
    HashMap<Uuid, PlayerAttributeTable>,
) {
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

    let positions_list = [
        Position::Goalguard,
        Position::Passer,
        Position::Artrine,
        Position::CenterOffense,
        Position::WingOffense,
        Position::WingOffense,
        Position::Midcenter,
        Position::TightWing,
        Position::TightWing,
        Position::CenterTight,
        Position::Corridor,
        Position::Corridor,
        Position::WideEnd,
        Position::RunningEnd,
    ];

    let mut home_players = Vec::new();
    let mut away_players = Vec::new();
    let mut players = Vec::new();
    let mut targets = Vec::new();
    let mut attribute_tables = HashMap::new();

    for (i, &pos) in positions_list.iter().enumerate() {
        let home_id = Uuid::new_v4();
        let away_id = Uuid::new_v4();

        let home_p = create_player(home_id, &format!("Home_{i}"), pos, &attribute_keys);
        let away_p = create_player(away_id, &format!("Away_{i}"), pos, &attribute_keys);

        attribute_tables.insert(home_id, PlayerAttributeTable::from_player(&home_p, &attribute_keys));
        attribute_tables.insert(away_id, PlayerAttributeTable::from_player(&away_p, &attribute_keys));

        let initial_home = VectorPosition::from_components((20.0 + (i as f64) * 5.0) * MIRIM_TO_METERS, (10.0 + (i as f64) * 4.0) * MIRIM_TO_METERS, 0.0);
        let initial_away = VectorPosition::from_components((100.0 - (i as f64) * 5.0) * MIRIM_TO_METERS, (10.0 + (i as f64) * 4.0) * MIRIM_TO_METERS, 0.0);

        let target_home = VectorPosition::from_components(initial_home.raw().0 + 10.0 * MIRIM_TO_METERS, initial_home.raw().1, 0.0);
        let target_away = VectorPosition::from_components(initial_away.raw().0 - 10.0 * MIRIM_TO_METERS, initial_away.raw().1, 0.0);

        targets.push((home_id, target_home));
        targets.push((away_id, target_away));

        home_players.push(home_p.clone());
        away_players.push(away_p.clone());
        players.push(home_p);
        players.push(away_p);
    }

    let formation = build_14_slot_formation();
    let home_lineup = Lineup::new(formation.clone(), home_players).unwrap();
    let away_lineup = Lineup::new(formation, away_players).unwrap();
    let mut spatial_map = DynamicSpatialMap::from_pitch(&pitch, &home_lineup, &away_lineup).unwrap();

    for (id, target) in &targets {
        spatial_map.set_position(*id, VectorPosition::from_components(target.raw().0 - 10.0 * MIRIM_TO_METERS, target.raw().1, 0.0));
    }

    (pitch, spatial_map, players, targets, attribute_tables)
}

fn bench_spatial_tick_loop(c: &mut Criterion) {
    let (pitch, base_spatial_map, players, targets, attribute_tables) = setup_spatial_benchmark();
    let fatigue_state = PhysicalState::initial();
    let fatigue_lookup = |_id: &Uuid| fatigue_state;
    let effort_lookup = |_id: &Uuid| 1.0;

    let movers: Vec<(&Player, VectorPosition)> = players
        .iter()
        .zip(targets.iter())
        .map(|(p, (_, t))| (p, *t))
        .collect();

    let mut group = c.benchmark_group("spatial_tick_loop");

    group.bench_function("boid_steering_calculation", |b| {
        let neighbor = SpatialNeighbor::new(
            Uuid::new_v4(),
            VectorPosition::from_components(50.0, 50.0, 0.0),
            Velocity::zero(),
            false,
            0.55,
        );
        let neighbors = vec![neighbor; 27];
        b.iter(|| {
            calculate_dynamic_boid_steering_velocity_with_context(
                black_box(Velocity::from_components(3.0, 0.0, 0.0)),
                black_box(VectorPosition::from_components(45.0, 50.0, 0.0)),
                black_box(VectorPosition::from_components(60.0, 50.0, 0.0)),
                black_box(Uuid::new_v4()),
                black_box(true),
                black_box(&neighbors),
                black_box(Speed::new(6.5)),
                black_box(14.0),
                black_box(14.0),
                black_box(14.0),
                black_box(14.0),
                black_box(78.0),
                black_box(1.0),
                black_box(0.55),
                black_box(MovementContext::LivePlay),
                black_box(Duration::new(0.05)),
            )
        })
    });

    group.bench_function("tick_loop_28_players_live", |b| {
        b.iter(|| {
            let mut map = base_spatial_map.clone();
            run_spatial_tick_loop_with_context(
                black_box(&mut map),
                black_box(&movers),
                black_box(&attribute_tables),
                black_box(MovementContext::LivePlay),
                black_box(&pitch),
                black_box(&fatigue_lookup),
                black_box(&effort_lookup),
            )
        })
    });

    group.finish();
}

criterion_group!(benches, bench_spatial_tick_loop);
criterion_main!(benches);
