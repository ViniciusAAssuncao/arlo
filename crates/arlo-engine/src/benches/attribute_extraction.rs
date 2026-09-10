use arlo_domain::{
    AttributeCategory, AttributeDefinition, AttributeKey, AttributeTarget, CaptaincyRole,
    Player, PlayerAttributeValue, PlayerPosition, Position,
};
use arlo_engine::attributes::PlayerAttributeTable;
use arlo_engine::physical::systems::degradation::{
    extract_effective_attribute_value, extract_effective_attribute_value_with_impulse,
};
use arlo_engine::physical::PhysicalState;
use arlo_engine::psychology::state::ImpulseState;
use arlo_engine::resolution::duel_noise::player_noise_distribution_from_table_with_impulse;
use arlo_engine::spatial::decision_vector::extract_attribute_value;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::collections::HashMap;
use uuid::Uuid;

fn setup_player_and_keys() -> (Player, HashMap<Uuid, AttributeKey>, Vec<AttributeKey>) {
    let mut attribute_keys = HashMap::new();
    let mut attributes = Vec::new();

    let all_keys = vec![
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
        let def_id = Uuid::new_v4();
        attribute_keys.insert(def_id, key);
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
    let player = Player::builder(
        Uuid::new_v4(),
        "Test Player",
        1.85,
        0,
        Uuid::new_v4(),
    )
    .with_team_id(Some(team_id))
    .with_positions(vec![PlayerPosition::new(Position::Artrine, 10).unwrap()])
    .with_attributes(attributes)
    .with_captaincy_role(Some(CaptaincyRole::Captain))
    .build()
    .unwrap();

    (player, attribute_keys, all_keys)
}

fn bench_attribute_extraction(c: &mut Criterion) {
    let (player, attribute_keys, all_keys) = setup_player_and_keys();
    let physical_state = PhysicalState::new(0.85, 0.90, 1500.0, 3);
    let impulse_state = ImpulseState::from_baseline(55.0);
    let table = PlayerAttributeTable::from_player(&player, &attribute_keys);

    let mut group = c.benchmark_group("attribute_extraction");

    group.bench_function("extract_single_key", |b| {
        b.iter(|| {
            extract_attribute_value(black_box(&table), black_box(AttributeKey::Passing))
        })
    });

    group.bench_function("extract_all_keys_batch", |b| {
        b.iter(|| {
            for &key in &all_keys {
                black_box(extract_attribute_value(black_box(&table), black_box(key)));
            }
        })
    });

    group.bench_function("extract_effective_with_fatigue", |b| {
        b.iter(|| {
            extract_effective_attribute_value(
                black_box(&table),
                black_box(AttributeKey::Passing),
                black_box(&physical_state),
            )
        })
    });

    group.bench_function("extract_effective_with_impulse", |b| {
        b.iter(|| {
            extract_effective_attribute_value_with_impulse(
                black_box(&table),
                black_box(AttributeKey::Passing),
                black_box(&physical_state),
                black_box(&impulse_state),
                black_box(55.0),
            )
        })
    });

    group.bench_function("player_noise_distribution", |b| {
        b.iter(|| {
            player_noise_distribution_from_table_with_impulse(
                black_box(&player),
                black_box(&table),
                black_box(&physical_state),
                black_box(&impulse_state),
                black_box(55.0),
            )
        })
    });

    group.finish();
}

criterion_group!(benches, bench_attribute_extraction);
criterion_main!(benches);