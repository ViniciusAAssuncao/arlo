use crate::conditioning::advance_conditioning;
use crate::domain::{FatigueCondition, ImpulseCondition, InjuryRecord};
use crate::error::{RecoveryError, RecoveryResult};
use crate::fatigue_recovery::{calculate_fatigue_recovery, calculate_player_age_years};
use crate::impulse_recovery::calculate_impulse_recovery;
use crate::injury_recovery::{
    advance_injury_days, evaluate_reinjury_risk, load_recovery_profiles, register_injury,
    InjuryProgressionOutcome,
};
use crate::injury_recovery::outside_match::OutsideMatchCatalog;
use crate::injury_recovery::recovery_profile::profile_for;
use crate::tuning::RecoveryTuningProfile;
use arlo_domain::{BodyRegion, InjurySeverityGrade};
use arlo_persistence::models::condition::{PlayerConditionRow, PlayerInjuryHistoryRow};
use arlo_persistence::repositories::condition::{player_condition, player_injury_history};
use rayon::prelude::*;
use sqlx::SqlitePool;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

enum InjuryAction {
    UpdateProgress {
        id: Uuid,
        days_remaining: u32,
        observation_days_remaining: u32,
        status: &'static str,
    },
    Relapse {
        resolved_id: Uuid,
        relapse_row: PlayerInjuryHistoryRow,
    },
    MarkResolved {
        id: Uuid,
    },
    Register {
        row: PlayerInjuryHistoryRow,
    },
}

struct PlayerDailyPlan {
    condition_row: PlayerConditionRow,
    injury_action: Option<InjuryAction>,
}

fn parse_severity_grade(code: &str) -> Result<InjurySeverityGrade, RecoveryError> {
    match code {
        "Grade1" | "grade1" | "Grade_1" | "grade_1" | "1" => Ok(InjurySeverityGrade::Grade1),
        "Grade2" | "grade2" | "Grade_2" | "grade_2" | "2" => Ok(InjurySeverityGrade::Grade2),
        "Grade3" | "grade3" | "Grade_3" | "grade_3" | "3" => Ok(InjurySeverityGrade::Grade3),
        _ => Err(RecoveryError::InvalidData(format!(
            "Invalid injury severity grade: {code}"
        ))),
    }
}

fn severity_grade_to_str(grade: InjurySeverityGrade) -> &'static str {
    match grade {
        InjurySeverityGrade::Grade1 => "Grade1",
        InjurySeverityGrade::Grade2 => "Grade2",
        InjurySeverityGrade::Grade3 => "Grade3",
    }
}

fn parse_body_region_str(code: &str) -> Result<BodyRegion, RecoveryError> {
    arlo_db::models::body_region_code::parse_body_region(code)
        .map_err(|e| RecoveryError::InvalidData(e.to_string()))
}

fn body_region_to_str(region: BodyRegion) -> &'static str {
    arlo_db::models::body_region_code::body_region_to_code(region)
}

pub async fn advance_all_players_one_day(
    pool: &SqlitePool,
    current_year: i64,
    current_day_of_year: u32,
) -> RecoveryResult<()> {
    let players = arlo_db::repositories::player::list_daily_recovery_inputs(pool).await?;
    if players.is_empty() {
        return Ok(());
    }

    let active_injuries = player_injury_history::list_all_active(pool).await?;
    let mut injuries_by_player: HashMap<Uuid, PlayerInjuryHistoryRow> =
        HashMap::with_capacity(active_injuries.len());
    for inj in active_injuries {
        if let Ok(pid) = Uuid::parse_str(&inj.player_id) {
            injuries_by_player.entry(pid).or_insert(inj);
        }
    }

    let condition_rows = player_condition::list_all(pool).await?;
    let mut conditions_by_player: HashMap<Uuid, PlayerConditionRow> =
        HashMap::with_capacity(condition_rows.len());
    for cond in condition_rows {
        if let Ok(pid) = Uuid::parse_str(&cond.player_id) {
            conditions_by_player.insert(pid, cond);
        }
    }

    let now_seconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0);

    let current_date_unix_seconds =
        (current_year - 1970) * 31_557_600 + (current_day_of_year as i64) * 86_400;

    let tuning = RecoveryTuningProfile::default();
    let recovery_profiles = load_recovery_profiles(pool).await?;
    let outside_match_catalog = OutsideMatchCatalog::load(pool).await?;

    let plans: Vec<PlayerDailyPlan> = players
        .par_iter()
        .map(|p| -> RecoveryResult<PlayerDailyPlan> {
            let player_id = p.id;

            let stamina = p.stamina;
            let natural_fitness = p.natural_fitness;
            let determination = p.determination;
            let composure = p.composure;
            let consistency = p.consistency;

            let age_years =
                calculate_player_age_years(p.birthdate_unix_seconds, current_date_unix_seconds);

            let (
                current_energy,
                current_w_prime,
                current_impulse,
                impulse_baseline,
                conditioning_score,
                last_match_year,
                last_match_day_of_year,
                was_active_today,
            ) = match conditions_by_player.get(&player_id) {
                Some(row) => {
                    let active = match (row.last_match_year, row.last_match_day_of_year) {
                        (Some(my), Some(md)) => {
                            my == row.last_updated_year && md == row.last_updated_day_of_year
                        }
                        _ => false,
                    };
                    (
                        row.energy_level,
                        row.anaerobic_reserve,
                        row.impulse_current_value as f64,
                        row.impulse_baseline,
                        row.conditioning_score,
                        row.last_match_year,
                        row.last_match_day_of_year.map(|d| d as u32),
                        active,
                    )
                }
                None => (1.0, 1.0, 50.0, 50.0, 0.5, None, None, false),
            };

            let mut is_injured_today = false;
            let mut is_post_long_injury = false;
            let mut injury_action = None;

            if let Some(inj_row) = injuries_by_player.get(&player_id) {
                let inj_id = Uuid::parse_str(&inj_row.id)
                    .map_err(|e| RecoveryError::InvalidData(e.to_string()))?;
                let def_id = Uuid::parse_str(&inj_row.injury_definition_id)
                    .map_err(|e| RecoveryError::InvalidData(e.to_string()))?;
                let body_region = parse_body_region_str(&inj_row.body_region)?;
                let severity_grade = parse_severity_grade(&inj_row.severity_grade)?;
                let origin_id = inj_row
                    .origin_record_id
                    .as_deref()
                    .map(Uuid::parse_str)
                    .transpose()
                    .map_err(|e| RecoveryError::InvalidData(e.to_string()))?;

                let is_relapse = inj_row.is_relapse;
                let original_id = if is_relapse && origin_id.is_none() {
                    Some(inj_id)
                } else if !is_relapse && origin_id.is_some() {
                    None
                } else {
                    origin_id
                };

                let record = InjuryRecord::new(
                    inj_id,
                    def_id,
                    body_region,
                    severity_grade,
                    inj_row.days_remaining as u32,
                    inj_row.observation_days_remaining as u32,
                    is_relapse,
                    original_id,
                )?;

                is_injured_today = record.days_remaining() > 0;
                is_post_long_injury = severity_grade == InjurySeverityGrade::Grade3;

                let progression = advance_injury_days(&record, 1, &tuning)?;
                match progression {
                    InjuryProgressionOutcome::StillInjured(updated) => {
                        injury_action = Some(InjuryAction::UpdateProgress {
                            id: updated.id(),
                            days_remaining: updated.days_remaining(),
                            observation_days_remaining: 0,
                            status: "Injured",
                        });
                    }
                    InjuryProgressionOutcome::TransitionedToObservation(updated) => {
                        injury_action = Some(InjuryAction::UpdateProgress {
                            id: updated.id(),
                            days_remaining: 0,
                            observation_days_remaining: updated.observation_days_remaining(),
                            status: "Observation",
                        });
                    }
                    InjuryProgressionOutcome::ObservationProgressed(updated) => {
                        let reinjury = evaluate_reinjury_risk(
                            &updated,
                            conditioning_score,
                            natural_fitness,
                            age_years,
                            &tuning,
                            &recovery_profiles,
                        )?;

                        if let Some((relapse, treatment)) = reinjury {
                            let relapse_row = PlayerInjuryHistoryRow::new(
                                relapse.id(),
                                player_id,
                                relapse.injury_definition_id(),
                                body_region_to_str(relapse.body_region()),
                                severity_grade_to_str(relapse.severity_grade()),
                                inj_row.injury_extent.clone(),
                                treatment.as_str(),
                                current_year,
                                current_day_of_year,
                                relapse.days_remaining(),
                                relapse.days_remaining(),
                                0,
                                "Injured",
                                true,
                                relapse.original_injury_id(),
                                None,
                                now_seconds,
                            );

                            injury_action = Some(InjuryAction::Relapse {
                                resolved_id: updated.id(),
                                relapse_row,
                            });

                            is_injured_today = true;
                        } else {
                            injury_action = Some(InjuryAction::UpdateProgress {
                                id: updated.id(),
                                days_remaining: 0,
                                observation_days_remaining: updated.observation_days_remaining(),
                                status: "Observation",
                            });
                        }
                    }
                    InjuryProgressionOutcome::FullyRecovered => {
                        injury_action = Some(InjuryAction::MarkResolved { id: record.id() });
                    }
                }
            }

            if !injuries_by_player.contains_key(&player_id) {
                if let Some(condition) = outside_match_catalog.sample(tuning.outside_match_daily_incident_probability) {
                    let (record, treatment) = register_injury(
                        Uuid::new_v4(), condition.definition_id, condition.body_region,
                        condition.severity_grade, natural_fitness, age_years, &tuning,
                        &recovery_profiles,
                    )?;
                    let injury_extent = profile_for(
                        &recovery_profiles, condition.definition_id,
                        condition.severity_grade, treatment,
                    ).and_then(|profile| profile.injury_extent.clone());
                    let row = PlayerInjuryHistoryRow::new(
                        record.id(), player_id, condition.definition_id,
                        body_region_to_str(condition.body_region),
                        severity_grade_to_str(condition.severity_grade), injury_extent,
                        treatment.as_str(), current_year, current_day_of_year,
                        record.days_remaining(), record.days_remaining(), 0,
                        "Injured", false, None, None, now_seconds,
                    );
                    injury_action = Some(InjuryAction::Register { row });
                    is_injured_today = true;
                }
            }

            let new_conditioning = advance_conditioning(
                conditioning_score,
                was_active_today,
                is_injured_today,
                &tuning,
            );

            let fatigue_condition = FatigueCondition::new(current_energy, current_w_prime)?;
            let new_fatigue = calculate_fatigue_recovery(
                &fatigue_condition,
                stamina,
                natural_fitness,
                age_years,
                new_conditioning,
                1,
                &tuning,
            )?;

            let impulse_condition = ImpulseCondition::new(current_impulse, impulse_baseline)?;
            let new_impulse = calculate_impulse_recovery(
                &impulse_condition,
                determination,
                composure,
                consistency,
                1,
                is_post_long_injury,
                &tuning,
            )?;

            let updated_condition = PlayerConditionRow::new(
                player_id,
                new_fatigue.energy(),
                new_fatigue.w_prime(),
                new_impulse.current().clamp(0.0, 100.0).round() as u8,
                new_impulse.baseline(),
                new_conditioning,
                current_year,
                current_day_of_year,
                last_match_year,
                last_match_day_of_year,
            );

            Ok(PlayerDailyPlan {
                condition_row: updated_condition,
                injury_action,
            })
        })
        .collect::<RecoveryResult<Vec<_>>>()?;

    let mut tx = pool.begin().await?;
    let mut condition_updates = Vec::with_capacity(plans.len());

    for plan in plans {
        if let Some(action) = plan.injury_action {
            match action {
                InjuryAction::UpdateProgress {
                    id,
                    days_remaining,
                    observation_days_remaining,
                    status,
                } => {
                    player_injury_history::update_progress_with_tx(
                        &mut tx,
                        id,
                        days_remaining,
                        observation_days_remaining,
                        status,
                    )
                    .await?;
                }
                InjuryAction::Relapse {
                    resolved_id,
                    relapse_row,
                } => {
                    player_injury_history::mark_resolved_with_tx(&mut tx, resolved_id, now_seconds)
                        .await?;
                    player_injury_history::insert_with_tx(&mut tx, &relapse_row).await?;
                }
                InjuryAction::MarkResolved { id } => {
                    player_injury_history::mark_resolved_with_tx(&mut tx, id, now_seconds).await?;
                }
                InjuryAction::Register { row } => {
                    player_injury_history::insert_with_tx(&mut tx, &row).await?;
                }
            }
        }

        condition_updates.push(plan.condition_row);
    }

    player_condition::upsert_many_with_tx(&mut tx, &condition_updates).await?;

    tx.commit().await?;

    Ok(())
}
