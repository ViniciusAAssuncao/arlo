use arlo_domain::{LeagueCalendarConfig, QualificationPoolRule};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StageSpotsSummary {
    pub relegation_spots: u32,
    pub promotion_spots: u32,
    pub qualification_spots: u32,
    pub is_final_stage: bool,
}

pub fn compute_stage_spots(
    league_config: Option<&LeagueCalendarConfig>,
    current_stage_idx: u32,
) -> StageSpotsSummary {
    match league_config {
        Some(cfg) => {
            let pr_policy = cfg.promotion_relegation_policy();
            let total_stages = cfg.stages().len() as u32;
            let is_final = total_stages == 0 || current_stage_idx + 1 >= total_stages;
            let is_standings_stage = current_stage_idx == pr_policy.standings_stage_order_index();
            let relegation_spots = if is_standings_stage {
                pr_policy.relegation_rule().count().unwrap_or(0)
            } else {
                0
            };
            let promotion_spots = if is_standings_stage {
                pr_policy.promotion_rule().count().unwrap_or(0)
            } else {
                0
            };
            let qualification_spots = if !is_final {
                if let Some(next_stage) = cfg.stages().iter().find(|s| s.stage_order_index() == current_stage_idx + 1) {
                    let mut count = 0;
                    for pool_rule in next_stage.entry_rule().pools() {
                        match pool_rule {
                            QualificationPoolRule::TopN { count: c } => count += *c,
                            QualificationPoolRule::PositionRange { start_position, end_position } => {
                                if *end_position >= *start_position {
                                    count += *end_position - *start_position + 1;
                                }
                            }
                            _ => {}
                        }
                    }
                    if count == 0 {
                        promotion_spots
                    } else {
                        count
                    }
                } else {
                    promotion_spots
                }
            } else {
                0
            };
            StageSpotsSummary {
                relegation_spots,
                promotion_spots,
                qualification_spots,
                is_final_stage: is_final,
            }
        }
        None => StageSpotsSummary {
            relegation_spots: 0,
            promotion_spots: 0,
            qualification_spots: 0,
            is_final_stage: false,
        },
    }
}