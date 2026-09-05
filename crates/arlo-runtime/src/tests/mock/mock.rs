use arlo_domain::pitch::Pitch;
use arlo_domain::{
    AttributeCategory, AttributeDefinition, AttributeKey, AttributeTarget, Formation,
    FormationSlot, Player, PlayerAttributeValue, PlayerPosition, Position, Team,
};
use arlo_engine::tactics::Lineup;
use arlo_engine::world_state::MatchState;
use arlo_engine::MatchSeed;
use std::collections::HashMap;
use uuid::Uuid;

pub fn create_mock_attribute_definitions() -> (Vec<AttributeDefinition>, HashMap<Uuid, AttributeKey>) {
    let keys = [
        (AttributeKey::ArloControl, "Arlo Control", AttributeCategory::Technical),
        (AttributeKey::Passing, "Passing", AttributeCategory::Technical),
        (AttributeKey::Dribbling, "Dribbling", AttributeCategory::Technical),
        (AttributeKey::Finishing, "Finishing", AttributeCategory::Technical),
        (AttributeKey::Crossing, "Crossing", AttributeCategory::Technical),
        (AttributeKey::OffensiveBlocking, "Offensive Blocking", AttributeCategory::Technical),
        (AttributeKey::DefensiveContainment, "Defensive Containment", AttributeCategory::Technical),
        (AttributeKey::PasserPressure, "Passer Pressure", AttributeCategory::Technical),
        (AttributeKey::HandsReception, "Hands Reception", AttributeCategory::Technical),
        (AttributeKey::DriveTechnique, "Drive Technique", AttributeCategory::Technical),
        (AttributeKey::FalseArtrineBluff, "False Artrine Bluff", AttributeCategory::Technical),
        (AttributeKey::Technique, "Technique", AttributeCategory::Technical),
        (AttributeKey::Anticipation, "Anticipation", AttributeCategory::Mental),
        (AttributeKey::Decisions, "Decisions", AttributeCategory::Mental),
        (AttributeKey::Composure, "Composure", AttributeCategory::Mental),
        (AttributeKey::Concentration, "Concentration", AttributeCategory::Mental),
        (AttributeKey::Vision, "Vision", AttributeCategory::Mental),
        (AttributeKey::Positioning, "Positioning", AttributeCategory::Mental),
        (AttributeKey::Teamwork, "Teamwork", AttributeCategory::Mental),
        (AttributeKey::Determination, "Determination", AttributeCategory::Mental),
        (AttributeKey::Leadership, "Leadership", AttributeCategory::Mental),
        (AttributeKey::ControlledAggression, "Controlled Aggression", AttributeCategory::Mental),
        (AttributeKey::Bravery, "Bravery", AttributeCategory::Mental),
        (AttributeKey::Flair, "Flair", AttributeCategory::Mental),
        (AttributeKey::WorkRate, "Work Rate", AttributeCategory::Mental),
        (AttributeKey::Acceleration, "Acceleration", AttributeCategory::Physical),
        (AttributeKey::Pace, "Pace", AttributeCategory::Physical),
        (AttributeKey::Agility, "Agility", AttributeCategory::Physical),
        (AttributeKey::Balance, "Balance", AttributeCategory::Physical),
        (AttributeKey::Strength, "Strength", AttributeCategory::Physical),
        (AttributeKey::Stamina, "Stamina", AttributeCategory::Physical),
        (AttributeKey::JumpingReach, "Jumping Reach", AttributeCategory::Physical),
        (AttributeKey::NaturalFitness, "Natural Fitness", AttributeCategory::Physical),
        (AttributeKey::Reflexes, "Reflexes", AttributeCategory::Goalkeeping),
        (AttributeKey::Handling, "Handling", AttributeCategory::Goalkeeping),
        (AttributeKey::AreaCommand, "Area Command", AttributeCategory::Goalkeeping),
        (AttributeKey::Communication, "Communication", AttributeCategory::Goalkeeping),
        (AttributeKey::OneOnOne, "One On One", AttributeCategory::Goalkeeping),
        (AttributeKey::Distribution, "Distribution", AttributeCategory::Goalkeeping),
        (AttributeKey::GoalKicking, "Goal Kicking", AttributeCategory::Goalkeeping),
        (AttributeKey::RushingOut, "Rushing Out", AttributeCategory::Goalkeeping),
    ];

    let mut definitions = Vec::with_capacity(keys.len());
    let mut key_map = HashMap::with_capacity(keys.len());

    for (index, &(key, name, category)) in keys.iter().enumerate() {
        let id = Uuid::from_u128(0x1000_0000_0000_0000_0000_0000_0000_0000 | (index as u128));
        let def = AttributeDefinition::new(id, key, name, category, AttributeTarget::Player).unwrap();
        key_map.insert(id, key);
        definitions.push(def);
    }

    (definitions, key_map)
}

pub fn create_mock_formation(id: Uuid, name: &str) -> Formation {
    let slot_configs = [
        (Position::Goalguard, 0.02, 0.50),
        (Position::Centerback, 0.15, 0.50),
        (Position::DefensiveEnd, 0.20, 0.20),
        (Position::Rougieback, 0.20, 0.80),
        (Position::DefensiveBlocker, 0.28, 0.35),
        (Position::WideBlocker, 0.28, 0.65),
        (Position::Lineback, 0.40, 0.50),
        (Position::Fullback, 0.42, 0.30),
        (Position::PassRusher, 0.42, 0.70),
        (Position::Passer, 0.48, 0.50),
        (Position::Artrine, 0.55, 0.50),
        (Position::Midcenter, 0.68, 0.50),
        (Position::WingOffense, 0.75, 0.20),
        (Position::CenterOffense, 0.85, 0.50),
    ];

    let mut slots = Vec::with_capacity(14);
    for (pos, rx, ry) in slot_configs {
        slots.push(FormationSlot::new(pos, rx, ry).unwrap());
    }

    Formation::new(id, name, slots).unwrap()
}

pub fn create_mock_players(
    team_id: Uuid,
    nationality_id: Uuid,
    is_home: bool,
    defs: &[AttributeDefinition],
) -> Vec<Player> {
    let base_offset = if is_home { 100u128 } else { 200u128 };
    let prefix = if is_home { "Valoria" } else { "Aethelgard" };

    let player_roles = [
        (1, "Eldor Vance", Position::Goalguard, 16, 17, 15, 10),
        (2, "Goran Shield", Position::Centerback, 15, 14, 16, 8),
        (3, "Thorne Edge", Position::DefensiveEnd, 14, 15, 15, 8),
        (4, "Kaelen Ward", Position::Rougieback, 15, 16, 14, 9),
        (5, "Boran Stone", Position::DefensiveBlocker, 17, 13, 17, 6),
        (6, "Darin Flank", Position::WideBlocker, 14, 15, 14, 8),
        (7, "Roric Sentinel", Position::Lineback, 16, 15, 15, 11),
        (8, "Alric Haven", Position::Fullback, 14, 14, 16, 11),
        (9, "Joran Strike", Position::PassRusher, 16, 17, 16, 9),
        (10, "Marek Vector", Position::Passer, 18, 16, 15, 18),
        (11, "Valen Swift", Position::Artrine, 19, 18, 17, 19),
        (12, "Corin Pivot", Position::Midcenter, 15, 15, 15, 16),
        (13, "Loran Gale", Position::WingOffense, 16, 17, 14, 15),
        (14, "Draven Apex", Position::CenterOffense, 18, 17, 16, 14),
    ];

    let mut players = Vec::with_capacity(14);

    for (squad_num, name_suffix, primary_pos, core_stat, speed_stat, power_stat, creativity_stat) in player_roles {
        let player_id = Uuid::from_u128((base_offset << 32) | (squad_num as u128));
        let full_name = format!("{prefix} {name_suffix}");

        let primary_position = PlayerPosition::new(primary_pos, 10).unwrap();
        let secondary_pos = match primary_pos {
            Position::CenterOffense => Position::WingOffense,
            Position::WingOffense => Position::CenterOffense,
            Position::Midcenter => Position::Artrine,
            Position::Artrine => Position::Passer,
            Position::Passer => Position::Midcenter,
            Position::PassRusher => Position::DefensiveEnd,
            Position::Lineback => Position::Centerback,
            Position::Fullback => Position::Lineback,
            Position::Centerback => Position::DefensiveBlocker,
            Position::DefensiveEnd => Position::PassRusher,
            Position::Rougieback => Position::WideBlocker,
            Position::DefensiveBlocker => Position::Centerback,
            Position::WideBlocker => Position::Rougieback,
            Position::Goalguard => Position::Goalguard,
            _ => Position::Midcenter,
        };
        let secondary_position = PlayerPosition::new(secondary_pos, 7).unwrap();

        let positions = if primary_pos == Position::Goalguard {
            vec![primary_position]
        } else {
            vec![primary_position, secondary_position]
        };

        let mut attributes = Vec::with_capacity(defs.len());
        for def in defs {
            let val = match def.key() {
                AttributeKey::Finishing | AttributeKey::Reflexes | AttributeKey::Passing | AttributeKey::DriveTechnique => core_stat,
                AttributeKey::Pace | AttributeKey::Acceleration | AttributeKey::Agility => speed_stat,
                AttributeKey::Strength | AttributeKey::Stamina | AttributeKey::Balance | AttributeKey::Bravery => power_stat,
                AttributeKey::Vision | AttributeKey::Flair | AttributeKey::Crossing | AttributeKey::ArloControl | AttributeKey::Leadership | AttributeKey::Technique | AttributeKey::FalseArtrineBluff => creativity_stat,
                AttributeKey::Decisions | AttributeKey::Composure | AttributeKey::Anticipation => ((core_stat + creativity_stat) / 2).clamp(1, 20),
                AttributeKey::Positioning | AttributeKey::Teamwork | AttributeKey::Concentration | AttributeKey::WorkRate => ((power_stat + creativity_stat) / 2).clamp(1, 20),
                AttributeKey::HandsReception | AttributeKey::OffensiveBlocking | AttributeKey::DefensiveContainment | AttributeKey::PasserPressure | AttributeKey::ControlledAggression => power_stat,
                _ => 12,
            };
            attributes.push(PlayerAttributeValue::new(def, val).unwrap());
        }

        let player = Player::builder(player_id, full_name, 1.86, 946684800, nationality_id)
            .with_team_id(Some(team_id))
            .with_squad_number(Some(squad_num))
            .with_positions(positions)
            .with_attributes(attributes)
            .build()
            .unwrap();

        players.push(player);
    }

    players
}

pub fn create_custom_mock_player(
    player_id: Uuid,
    name: impl Into<String>,
    primary_pos: Position,
    defs: &[AttributeDefinition],
    nationality_id: Uuid,
    team_id: Option<Uuid>,
    overrides: &[(AttributeKey, i32)],
) -> Player {
    let pos = PlayerPosition::new(primary_pos, 10).unwrap();
    let mut attributes = Vec::with_capacity(defs.len());
    for def in defs {
        let mut val = 10;
        for (k, v) in overrides {
            if *k == def.key() {
                val = *v;
                break;
            }
        }
        attributes.push(PlayerAttributeValue::new(def, val).unwrap());
    }

    Player::builder(player_id, name, 1.85, 946684800, nationality_id)
        .with_team_id(team_id)
        .with_squad_number(Some(10))
        .with_positions(vec![pos])
        .with_attributes(attributes)
        .build()
        .unwrap()
}

pub fn create_mock_teams() -> (Team, Team, Uuid) {
    let country_id = Uuid::from_u128(0x9999_0000_0000_0000_0000_0000_0000_0001);
    let home_id = Uuid::from_u128(0xAAAA_0000_0000_0000_0000_0000_0000_0001);
    let away_id = Uuid::from_u128(0xBBBB_0000_0000_0000_0000_0000_0000_0002);

    let home_team = Team::builder(home_id, "Valoria Arlo Club", country_id, 100000000, 180)
        .with_primary_color_hex(Some("#1E3A8A".to_string()))
        .with_secondary_color_hex(Some("#F59E0B".to_string()))
        .build()
        .unwrap();

    let away_team = Team::builder(away_id, "Aethelgard Titans", country_id, 105000000, 175)
        .with_primary_color_hex(Some("#991B1B".to_string()))
        .with_secondary_color_hex(Some("#FFFFFF".to_string()))
        .build()
        .unwrap();

    (home_team, away_team, country_id)
}

pub fn build_mock_match_state(seed_val: u64) -> (MatchState, HashMap<Uuid, AttributeKey>, Team, Team) {
    let (home_team, away_team, country_id) = create_mock_teams();
    let (defs, key_map) = create_mock_attribute_definitions();

    let home_players = create_mock_players(home_team.id(), country_id, true, &defs);
    let away_players = create_mock_players(away_team.id(), country_id, false, &defs);

    let home_form_id = Uuid::from_u128(0xF001);
    let away_form_id = Uuid::from_u128(0xF002);
    let home_formation = create_mock_formation(home_form_id, "Valoria 4-3-3-4 Attack");
    let away_formation = create_mock_formation(away_form_id, "Aethelgard 5-2-4-3 Compact");

    let home_lineup = Lineup::new(home_formation, home_players).unwrap();
    let away_lineup = Lineup::new(away_formation, away_players).unwrap();

    let pitch = Pitch::from_mirim(145.0, 85.0).unwrap();
    let seed = MatchSeed::new(seed_val);

    let state = MatchState::new(
        home_team.id(),
        away_team.id(),
        home_lineup,
        away_lineup,
        pitch,
        key_map.clone(),
        seed,
    )
    .unwrap();

    (state, key_map, home_team, away_team)
}