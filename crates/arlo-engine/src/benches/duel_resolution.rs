use arlo_domain::{
    AttributeCategory, AttributeDefinition, AttributeKey, AttributeTarget, CaptaincyRole,
    Player, PlayerAttributeValue, PlayerPosition, Position,
};
use arlo_engine::physical::PhysicalState;
use arlo_engine::resolution::duel_profiles::get_duel_profiles;
use arlo_engine::resolution::group_rating::{
    calculate_anchored_side_rating_from_index_with_fatigue,
    calculate_player_duel_rating_with_state, calculate_side_rating_from_index_with_fatigue,
};
use arlo_engine::resolution::resolver::resolve_duel_with_fatigue;
use arlo_engine::resolution::{DuelContext, DuelKind};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use std::collections::HashMap;
use uuid::Uuid;

fn create_player(name: &str, pos: Position, attribute_keys: &HashMap<Uuid, AttributeKey>) -> Player {
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
        Uuid::new_v4(),
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

fn setup_duel_environment() -> (
    Player,
    Player,
    Vec<Player>,
    Vec<Player>,
    HashMap<Uuid, Position>,
    HashMap<Uuid, Position>,
    HashMap<Uuid, AttributeKey>,
) {
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

    let attacker = create_player("Passer", Position::Passer, &attribute_keys);
    let defender = create_player("PassRusher", Position::PassRusher, &attribute_keys);

    let mut offense_helpers = Vec::new();
    let mut defense_helpers = Vec::new();
    let mut off_pos_index = HashMap::new();
    let mut def_pos_index = HashMap::new();

    off_pos_index.insert(attacker.id(), Position::Passer);
    def_pos_index.insert(defender.id(), Position::PassRusher);

    let helper_positions = [
        (Position::CenterOffense, Position::Centerback),
        (Position::WingOffense, Position::OutsideZonerback),
        (Position::Midcenter, Position::MiddleZonerback),
        (Position::TightWing, Position::WideBlocker),
        (Position::CenterTight, Position::DefensiveBlocker),
    ];

    for (i, (off_p, def_p)) in helper_positions.iter().enumerate() {
        let off_h = create_player(&format!("OffHelper{i}"), *off_p, &attribute_keys);
        let def_h = create_player(&format!("DefHelper{i}"), *def_p, &attribute_keys);
        off_pos_index.insert(off_h.id(), *off_p);
        def_pos_index.insert(def_h.id(), *def_p);
        offense_helpers.push(off_h);
        defense_helpers.push(def_h);
    }

    (
        attacker,
        defender,
        offense_helpers,
        defense_helpers,
        off_pos_index,
        def_pos_index,
        attribute_keys,
    )
}

fn bench_duel_resolution(c: &mut Criterion) {
    let (
        attacker,
        defender,
        offense_helpers,
        defense_helpers,
        off_pos_index,
        def_pos_index,
        attribute_keys,
    ) = setup_duel_environment();

    let fatigue_state = PhysicalState::new(0.9, 0.9, 500.0, 1);
    let fatigue_lookup = |_id: &Uuid| fatigue_state;

    let off_helper_refs: Vec<&Player> = offense_helpers.iter().collect();
    let def_helper_refs: Vec<&Player> = defense_helpers.iter().collect();

    let (off_profile, def_profile) = get_duel_profiles(DuelKind::PassProtection);
    let duel_context = DuelContext::attacker_home();

    let mut group = c.benchmark_group("duel_resolution");

    group.bench_function("player_duel_rating", |b| {
        b.iter(|| {
            calculate_player_duel_rating_with_state(
                black_box(&attacker),
                black_box(Position::Passer),
                black_box(&attribute_keys),
                black_box(&off_profile),
                black_box(&fatigue_state),
            )
        })
    });

    group.bench_function("side_rating_5_players", |b| {
        b.iter(|| {
            calculate_side_rating_from_index_with_fatigue(
                black_box(&def_helper_refs),
                black_box(&def_pos_index),
                black_box(&attribute_keys),
                black_box(&def_profile),
                black_box(&fatigue_lookup),
            )
        })
    });

    group.bench_function("anchored_side_rating", |b| {
        b.iter(|| {
            calculate_anchored_side_rating_from_index_with_fatigue(
                black_box(&attacker),
                black_box(Position::Passer),
                black_box(&off_helper_refs),
                black_box(&off_pos_index),
                black_box(&attribute_keys),
                black_box(&off_profile),
                black_box(&fatigue_lookup),
            )
        })
    });

    group.bench_function("resolve_duel_with_fatigue_raw", |b| {
        let mut rng = ChaCha8Rng::seed_from_u64(42);
        b.iter(|| {
            resolve_duel_with_fatigue(
                black_box(DuelKind::PassProtection),
                black_box(14.5),
                black_box(13.2),
                black_box(&attacker),
                black_box(&defender),
                black_box(&fatigue_state),
                black_box(&fatigue_state),
                black_box(&attribute_keys),
                black_box(&duel_context),
                black_box(&mut rng),
            )
        })
    });

    group.finish();
}

criterion_group!(benches, bench_duel_resolution);
criterion_main!(benches);