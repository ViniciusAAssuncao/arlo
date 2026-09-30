use arlo_domain::{AttributeDefinition, AttributeKey, DefensiveApproach, Manager, OffensiveApproach};
use crate::services::season::matchday::manager_match_context::ManagerMatchContext;
use arlo_tactics::{
    Aeriality, Aggression, Compactness, CounterAttackIntensity, CounterPressIntensity,
    DefensiveLineHeight, Directness, Mentality, PassingRange, Physicality,
    PressBlockShape, PressingIntensity, ScoringPatience, Structure, TeamInstructions,
    TeamTacticalProfile, Tempo, Width,
};
use std::collections::HashMap;
use sqlx::SqlitePool;
use uuid::Uuid;

struct StyleAxes {
    offense: OffensiveApproach,
    defense: DefensiveApproach,
    flexibility: f64,
    passing_range: f64,
    aeriality: f64,
    structure: f64,
    physicality: f64,
    transition_pace: f64,
    press_block_shape: f64,
}

fn style_axes(manager: &Manager, definitions: &HashMap<Uuid, AttributeDefinition>) -> StyleAxes {
    if let Some(style) = manager.tactical_profile() {
        return StyleAxes {
            offense: style.offensive_approach(), defense: style.defensive_approach(),
            flexibility: style.flexibility_tendency(), passing_range: style.passing_range_preference(),
            aeriality: style.aeriality_preference(), structure: style.structure_preference(),
            physicality: style.physicality_preference(), transition_pace: style.transition_pace_preference(),
            press_block_shape: style.press_block_shape_preference(),
        };
    }
    let attribute = |key| manager.attributes().iter()
        .find(|value| definitions.get(&value.attribute_definition_id()).is_some_and(|definition| definition.key() == key))
        .map_or(10.0, |value| f64::from(value.value()));
    let id = manager.id().as_u128();
    let offense = attribute(AttributeKey::OffensePlanning);
    let defense = attribute(AttributeKey::DefenseOrganization);
    let strategy = attribute(AttributeKey::ArtroStrategy);
    let temperament = ((id >> 24) & 7) as f64 / 7.0 - 0.5;
    let direction = (offense - defense) * 0.045 + temperament;
    StyleAxes {
        offense: if direction > 0.35 { OffensiveApproach::Positional }
            else if direction < -0.35 { OffensiveApproach::Direct }
            else if strategy > 12.0 { OffensiveApproach::Functional }
            else { OffensiveApproach::Balanced },
        defense: if defense > 13.0 && direction > 0.0 { DefensiveApproach::HighPress }
            else if defense < 8.0 || direction < -0.4 { DefensiveApproach::DeepLowBlock }
            else { DefensiveApproach::MidBlock },
        flexibility: (attribute(AttributeKey::Adaptability) / 20.0).clamp(0.0, 1.0),
        passing_range: ((strategy - 10.0) / 15.0 + temperament * 0.5).clamp(-1.0, 1.0),
        aeriality: ((attribute(AttributeKey::OffensePlanning) - strategy) / 15.0 + temperament * 0.4).clamp(-1.0, 1.0),
        structure: ((attribute(AttributeKey::TacticalKnowledge) - 10.0) / 12.0).clamp(-1.0, 1.0),
        physicality: (0.25 + defense / 40.0).clamp(0.0, 1.0),
        transition_pace: (0.35 + offense / 40.0 + temperament * 0.2).clamp(0.0, 1.0),
        press_block_shape: (0.3 + defense / 40.0).clamp(0.0, 1.0),
    }
}

pub fn plan_tactics(
    manager: &Manager,
    current: TeamTacticalProfile,
    opponent: &TeamTacticalProfile,
    definitions: &HashMap<Uuid, AttributeDefinition>,
    context: &ManagerMatchContext,
) -> TeamTacticalProfile {
    if manager.is_human_controlled() {
        return current;
    }
    let style = style_axes(manager, definitions);

    let (mentality, directness, patience, width) = match style.offense {
        OffensiveApproach::Positional => (0.40, -0.55, 0.75, 0.35),
        OffensiveApproach::Functional => (0.45, -0.15, 0.55, 0.20),
        OffensiveApproach::Direct => (0.50, 0.65, 0.25, 0.10),
        OffensiveApproach::Balanced => (0.25, 0.0, 0.50, 0.0),
    };
    let (line, press, compactness) = match style.defense {
        DefensiveApproach::HighPress => (0.75, 0.85, 0.60),
        DefensiveApproach::MidBlock => (0.45, 0.50, 0.60),
        DefensiveApproach::DeepLowBlock => (0.20, 0.25, 0.75),
    };
    let flexibility = style.flexibility;
    let opponent_press = opponent.instructions().out_of_possession().pressing_intensity().value();
    let opponent_tempo = opponent.instructions().in_possession().tempo().value();
    let pressure_response = flexibility * (opponent_press - 0.5) * 0.24;
    let tempo_response = flexibility * (opponent_tempo - 0.5) * 0.18;
    let threat = (-context.relative_strength).max(0.0) * flexibility;
    let opportunity = context.relative_strength.max(0.0) * flexibility;
    let instructions = TeamInstructions::builder(Mentality::new_clamped(
        mentality - threat * 0.18 + opportunity * 0.07))
        .with_tempo(Tempo::new_clamped(style.transition_pace - pressure_response - threat * 0.18))
        .with_width(Width::new_clamped(width))
        .with_flank_bias(current.instructions().in_possession().flank_bias())
        .with_directness(Directness::new_clamped(directness + style.passing_range * 0.3 - threat * 0.22))
        .with_structure(Structure::new_clamped(style.structure))
        .with_passing_range(PassingRange::new_clamped(style.passing_range - threat * 0.20))
        .with_aeriality(Aeriality::new_clamped(style.aeriality))
        .with_physicality(Physicality::new_clamped(style.physicality))
        .with_scoring_patience(ScoringPatience::new_clamped(patience + threat * 0.12))
        .with_defensive_line_height(DefensiveLineHeight::new_clamped(line + tempo_response - threat * 0.20))
        .with_compactness(Compactness::new_clamped(compactness))
        .with_pressing_intensity(PressingIntensity::new_clamped(press + tempo_response - threat * 0.15))
        .with_aggression(Aggression::new_clamped(style.physicality * 0.8))
        .with_counter_attack_intensity(CounterAttackIntensity::new_clamped(style.transition_pace))
        .with_counter_press_intensity(CounterPressIntensity::new_clamped(press * 0.8))
        .with_press_block_shape(PressBlockShape::new_clamped(style.press_block_shape))
        .build();
    TeamTacticalProfile::new(
        current.id(), current.team_id(), current.name(), instructions,
        current.situational_profile().cloned(), current.is_active(),
    )
}

pub fn match_variants(profile: &TeamTacticalProfile) -> Vec<TeamTacticalProfile> {
    let base = profile.instructions();
    let offense = base.in_possession();
    let defense = base.out_of_possession();
    let aggressive = TeamInstructions::builder(offense.mentality())
        .with_in_possession(*offense).with_out_of_possession(*defense)
        .with_transition(*base.transition())
        .with_mentality(Mentality::new_clamped(offense.mentality().value() + 0.20))
        .with_tempo(Tempo::new_clamped(offense.tempo().value() + 0.15))
        .with_directness(Directness::new_clamped(offense.directness().value() + 0.10))
        .with_pressing_intensity(PressingIntensity::new_clamped(defense.pressing_intensity().value() + 0.10))
        .build();
    let conservative = TeamInstructions::builder(offense.mentality())
        .with_in_possession(*offense).with_out_of_possession(*defense)
        .with_transition(*base.transition())
        .with_mentality(Mentality::new_clamped(offense.mentality().value() - 0.18))
        .with_tempo(Tempo::new_clamped(offense.tempo().value() - 0.10))
        .with_compactness(Compactness::new_clamped(defense.compactness().value() + 0.10))
        .with_defensive_line_height(DefensiveLineHeight::new_clamped(defense.defensive_line_height().value() - 0.08))
        .build();
    [("Pressão ofensiva", aggressive), ("Controle defensivo", conservative)]
        .into_iter().filter(|(_, instructions)| *instructions != *base)
        .map(|(name, instructions)| TeamTacticalProfile::new(
            Uuid::new_v4(), profile.team_id(), name, instructions, None, false,
        )).collect()
}
use crate::error::{ControllerError, ControllerResult};

pub async fn persist_plan(
    pool: &SqlitePool,
    manager: &Manager,
    base: &TeamTacticalProfile,
    planned: TeamTacticalProfile,
) -> ControllerResult<TeamTacticalProfile> {
    if manager.is_human_controlled() || base.instructions() == planned.instructions() {
        return Ok(planned);
    }
    let id = arlo_tactics::team_instructions::insert_profile(
        pool, planned.team_id(), "Plano do treinador", planned.instructions(), planned.situational_profile(),
    ).await.map_err(|error| ControllerError::InvalidData(error.to_string()))?;
    Ok(TeamTacticalProfile::new(
        id, planned.team_id(), planned.name(), *planned.instructions(),
        planned.situational_profile().cloned(), false,
    ))
}
