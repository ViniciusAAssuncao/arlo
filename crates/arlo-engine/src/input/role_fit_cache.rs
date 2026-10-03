use crate::MatchInput;
use arlo_domain::{AttributeKey, Position, SlotRole};
use arlo_tactics::{position_proficiency, static_role_fit, TeamInstructions};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

pub type RoleFitTable = HashMap<Uuid, Vec<(f64, f64)>>;
type Target = (Position, Position, SlotRole);

#[derive(Debug)]
struct TeamFits {
    instructions: TeamInstructions,
    targets: Vec<Target>,
    fits: Arc<RoleFitTable>,
}

#[derive(Debug, Default)]
pub(crate) struct RoleFitCache {
    teams: Mutex<HashMap<Uuid, TeamFits>>,
}

impl RoleFitCache {
    fn get_or_build(
        &self,
        team_id: Uuid,
        instructions: TeamInstructions,
        targets: &[Target],
        build: impl FnOnce() -> RoleFitTable,
    ) -> Arc<RoleFitTable> {
        let mut teams = self.teams.lock().unwrap_or_else(|error| error.into_inner());
        if let Some(cached) = teams.get(&team_id) {
            if cached.instructions == instructions && cached.targets == targets {
                return Arc::clone(&cached.fits);
            }
        }
        let fits = Arc::new(build());
        teams.insert(
            team_id,
            TeamFits {
                instructions,
                targets: targets.to_vec(),
                fits: Arc::clone(&fits),
            },
        );
        fits
    }
}

impl MatchInput {
    pub fn player_role_fits(
        &self,
        team_id: Uuid,
        instructions: TeamInstructions,
        targets: &[Target],
    ) -> Option<Arc<RoleFitTable>> {
        let team = if team_id == self.home().team_id() {
            self.home()
        } else if team_id == self.away().team_id() {
            self.away()
        } else {
            return None;
        };
        Some(
            self.role_fit_cache
                .get_or_build(team_id, instructions, targets, || {
                    team.roster()
                        .iter()
                        .map(|player| {
                            let values = self.player_attributes(player.id());
                            let attribute = |key: AttributeKey| {
                                values
                                    .and_then(|values| values[key.index()])
                                    .unwrap_or(10.0)
                            };
                            let fits = targets
                                .iter()
                                .map(|&(offense, defense, role)| {
                                    (
                                        (static_role_fit(
                                            player,
                                            offense,
                                            role,
                                            &instructions,
                                            attribute,
                                        ) + static_role_fit(
                                            player,
                                            defense,
                                            role,
                                            &instructions,
                                            attribute,
                                        )) / 2.0,
                                        position_proficiency(player, offense),
                                    )
                                })
                                .collect();
                            (player.id(), fits)
                        })
                        .collect()
                }),
        )
    }
}
