use crate::TeamInstructions;
use arlo_domain::{AttributeKey, Player, Position, PositionLine, SlotRole};

pub fn position_proficiency(player: &Player, position: Position) -> f64 {
    player
        .positions()
        .iter()
        .filter(|entry| entry.position() == position)
        .map(|entry| entry.proficiency())
        .max()
        .unwrap_or(0) as f64
        / 10.0
}

pub fn position_skill(position: Position, attribute: impl Fn(AttributeKey) -> f64) -> f64 {
    let keys: &[AttributeKey] = match position {
        Position::Artrine => &[
            AttributeKey::ArloControl,
            AttributeKey::Decisions,
            AttributeKey::Passing,
            AttributeKey::DriveTechnique,
        ],
        Position::Passer => &[
            AttributeKey::Passing,
            AttributeKey::Vision,
            AttributeKey::Decisions,
            AttributeKey::HandsReception,
        ],
        Position::Goalguard => &[
            AttributeKey::Reflexes,
            AttributeKey::Handling,
            AttributeKey::OneOnOne,
            AttributeKey::AreaCommand,
        ],
        Position::CenterOffense => &[
            AttributeKey::Finishing,
            AttributeKey::Positioning,
            AttributeKey::Anticipation,
            AttributeKey::Strength,
        ],
        _ => match position.line() {
            PositionLine::OffensiveLine => &[
                AttributeKey::Finishing,
                AttributeKey::HandsReception,
                AttributeKey::Pace,
                AttributeKey::Positioning,
            ],
            PositionLine::BackLine => &[
                AttributeKey::Passing,
                AttributeKey::ArloControl,
                AttributeKey::Teamwork,
                AttributeKey::WorkRate,
            ],
            PositionLine::DefenseLine => &[
                AttributeKey::DefensiveContainment,
                AttributeKey::PasserPressure,
                AttributeKey::Anticipation,
                AttributeKey::Strength,
            ],
            PositionLine::Goalguard => &[AttributeKey::Reflexes, AttributeKey::Handling],
        },
    };
    keys.iter().map(|key| attribute(*key)).sum::<f64>() / keys.len() as f64 / 20.0
}

pub fn static_role_fit(
    player: &Player,
    position: Position,
    role: SlotRole,
    instructions: &TeamInstructions,
    attribute: impl Fn(AttributeKey) -> f64,
) -> f64 {
    let proficiency = position_proficiency(player, position);
    let skill = position_skill(position, &attribute);
    let offense = instructions.in_possession();
    let defense = instructions.out_of_possession();
    let style = match position.line() {
        PositionLine::OffensiveLine | PositionLine::BackLine => {
            let direct = (offense.directness().value() + 1.0) / 2.0;
            (1.0 - direct) * (attribute(AttributeKey::Teamwork) + attribute(AttributeKey::Vision))
                + direct * (attribute(AttributeKey::Pace) + attribute(AttributeKey::Strength))
        }
        PositionLine::DefenseLine => {
            let press = defense.pressing_intensity().value();
            (1.0 - press)
                * (attribute(AttributeKey::Positioning) + attribute(AttributeKey::Concentration))
                + press
                    * (attribute(AttributeKey::PasserPressure) + attribute(AttributeKey::WorkRate))
        }
        PositionLine::Goalguard => {
            attribute(AttributeKey::Handling) + attribute(AttributeKey::AreaCommand)
        }
    } / 40.0;
    let specialist = match role {
        SlotRole::Kicker => attribute(AttributeKey::GoalKicking) / 20.0,
        SlotRole::Launcher => {
            (attribute(AttributeKey::Passing) + attribute(AttributeKey::Vision)) / 40.0
        }
        SlotRole::Blocker => {
            (attribute(AttributeKey::OffensiveBlocking) + attribute(AttributeKey::Strength)) / 40.0
        }
        SlotRole::Safeguard => {
            (attribute(AttributeKey::DefensiveContainment) + attribute(AttributeKey::Positioning))
                / 40.0
        }
        SlotRole::FalseArtrine => {
            (attribute(AttributeKey::FalseArtrineBluff) + attribute(AttributeKey::ArloControl))
                / 40.0
        }
        SlotRole::Standard => skill,
    };
    (0.50 * proficiency + 0.38 * skill + 0.07 * style + 0.05 * specialist).clamp(0.0, 1.0)
}
