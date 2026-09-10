use arlo_domain::pitch::Pitch;
use arlo_domain::{
    AttributeCategory, AttributeDefinition, AttributeKey, AttributeTarget, CaptaincyRole,
    Formation, FormationSlot, Player, PlayerAttributeValue, PlayerPosition, Position,
};
use arlo_engine::lineup_runtime::dynamic_anchor::{
    compute_dynamic_anchors, AnchorComputationContext,
};
use arlo_engine::lineup_runtime::fit_calculator::calculate_lineup_fit;
use arlo_engine::lineup_runtime::from_tactical_lineup::hydrate;
use arlo_tactics::{TacticalLineup, TeamInstructions};
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

fn setup_hydration_scenario() -> (
    TacticalLineup,
    Formation,
    Vec<Player>,
    Pitch,
    HashMap<Uuid, AttributeKey>,
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

    let team_id = Uuid::new_v4();
    let formation = build_14_slot_formation();

    let mut roster = Vec::new();
    for (idx, slot) in formation.slots().iter().enumerate() {
        roster.push(create_full_player(
            Uuid::new_v4(),
            team_id,
            &format!("Player_{idx}"),
            slot.offensive_position(),
            &attribute_keys,
        ));
    }

    let mut builder = TacticalLineup::builder(Uuid::new_v4(), team_id, "Lineup")
        .with_formation(&formation);

    for (idx, _slot) in formation.slots().iter().enumerate() {
        builder = builder.assign(idx, roster[idx].id());
    }

    let tactical_lineup = builder.build(&roster).unwrap();

    (tactical_lineup, formation, roster, pitch, attribute_keys)
}

fn bench_lineup_hydration(c: &mut Criterion) {
    let (tactical_lineup, formation, roster, pitch, attribute_keys) = setup_hydration_scenario();
    let instructions = TeamInstructions::default();
    let empty_player_instructions = HashMap::new();
    let anchor_ctx = AnchorComputationContext {
        player_instructions_index: &empty_player_instructions,
        opposing_lineup: None,
        spatial_map: None,
        block_marking_roles: None,
        press_reference_pos: None,
        attribute_tables: None,
    };

    let lineup = hydrate(&tactical_lineup, &formation, &roster).unwrap();

    let mut group = c.benchmark_group("lineup_hydration");

    group.bench_function("hydrate_tactical_lineup", |b| {
        b.iter(|| {
            hydrate(
                black_box(&tactical_lineup),
                black_box(&formation),
                black_box(&roster),
            )
        })
    });

    group.bench_function("calculate_lineup_fit", |b| {
        b.iter(|| calculate_lineup_fit(black_box(&lineup)))
    });

    group.bench_function("compute_dynamic_anchors_offense", |b| {
        b.iter(|| {
            compute_dynamic_anchors(
                black_box(&pitch),
                black_box(&lineup),
                black_box(72.5),
                black_box(true),
                black_box(true),
                black_box(&attribute_keys),
                black_box(&instructions),
                black_box(&anchor_ctx),
            )
        })
    });

    group.finish();
}

criterion_group!(benches, bench_lineup_hydration);
criterion_main!(benches);