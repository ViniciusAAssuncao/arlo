mod layouts;
mod repertoire;

use super::manager_match_context::ManagerMatchContext;
use crate::error::{ControllerError, ControllerResult};
use arlo_domain::{AttributeDefinition, AttributeKey, Formation, Position};
use arlo_engine::TeamInput;
use arlo_tactics::{PreparedTacticalPlan, TacticalLayout};
use std::collections::HashMap;
use uuid::Uuid;

pub fn prepare(
    team: TeamInput,
    formations: &[Formation],
    attributes: &HashMap<Uuid, AttributeKey>,
    definitions: &HashMap<Uuid, AttributeDefinition>,
    context: &ManagerMatchContext,
) -> ControllerResult<TeamInput> {
    let primary = TacticalLayout {
        formation: team.formation().clone(),
        lineup: team.lineup().clone(),
    };
    let mut plans = vec![PreparedTacticalPlan {
        id: team.tactics().id(),
        name: team.tactics().name().to_owned(),
        profile_id: team.tactics().id(),
        layout: primary.clone(),
    }];
    let valid_specialists = [Position::Artrine, Position::Passer, Position::Goalguard]
        .iter()
        .all(|position| {
            team.lineup()
                .assignments()
                .iter()
                .filter(|slot| slot.position() == *position)
                .count()
                == 1
        });
    if team.manager().is_human_controlled() || !valid_specialists {
        return team.with_prepared_plans(plans).map_err(invalid);
    }
    let policy = repertoire::PreparationPolicy::new(&team, definitions, context);
    let mut profiles = repertoire::instruction_variants(&team, &policy);
    for profile in &profiles {
        plans.push(PreparedTacticalPlan {
            id: profile.id(),
            name: profile.name().to_owned(),
            profile_id: profile.id(),
            layout: primary.clone(),
        });
    }
    let mut formations: Vec<_> = formations
        .iter()
        .filter(|formation| {
            formation.slots().len() == 14 && formation.id() != team.formation().id()
        })
        .collect();
    formations.sort_by_key(|formation| formation.id());
    for formation in formations {
        let Some((profile, layout)) =
            layouts::prepare_layout(&team, formation, attributes, &policy)
        else {
            continue;
        };
        plans.push(PreparedTacticalPlan {
            id: profile.id(),
            name: profile.name().to_owned(),
            profile_id: profile.id(),
            layout,
        });
        profiles.push(profile);
    }
    team.with_alternative_tactics(profiles)
        .and_then(|team| team.with_prepared_plans(plans))
        .map_err(invalid)
}

fn invalid(error: arlo_engine::EngineError) -> ControllerError {
    ControllerError::InvalidData(error.to_string())
}
