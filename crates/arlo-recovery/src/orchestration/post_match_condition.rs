use crate::domain::PlayerCondition;
use crate::error::{RecoveryError, RecoveryResult};
use crate::fatigue_recovery::calculate_player_age_years;
use crate::injury_recovery::{load_recovery_profiles, register_injury};
use crate::tuning::RecoveryTuningProfile;
use arlo_domain::{AttributeKey, BodyRegion, InjurySeverityGrade, Player};
use arlo_engine::MatchInput;
use arlo_events::{MatchEvent, MatchEventEnvelope};
use arlo_persistence::models::condition::{PlayerConditionRow, PlayerInjuryHistoryRow};
use arlo_persistence::repositories::condition::{player_condition, player_injury_history};
use sqlx::SqlitePool;
use std::collections::{HashMap, HashSet};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

fn severity_grade_to_str(grade: InjurySeverityGrade) -> &'static str {
    match grade {
        InjurySeverityGrade::Grade1 => "Grade1",
        InjurySeverityGrade::Grade2 => "Grade2",
        InjurySeverityGrade::Grade3 => "Grade3",
    }
}

fn body_region_to_str(region: BodyRegion) -> &'static str {
    arlo_db::models::body_region_code::body_region_to_code(region)
}

fn player_for(input: &MatchInput, player_id: Uuid) -> RecoveryResult<&Player> {
    input
        .home()
        .roster()
        .iter()
        .chain(input.away().roster().iter())
        .find(|player| player.id() == player_id)
        .ok_or_else(|| RecoveryError::InvalidData(format!("Unknown player {player_id}")))
}

fn natural_fitness(input: &MatchInput, player: &Player) -> RecoveryResult<f64> {
    let definition_id = input
        .player_attribute_definitions()
        .iter()
        .find(|definition| definition.key() == AttributeKey::NaturalFitness)
        .map(|definition| definition.id())
        .ok_or_else(|| RecoveryError::InvalidData("Missing NaturalFitness definition".into()))?;
    player
        .attributes()
        .iter()
        .find(|value| value.attribute_definition_id() == definition_id)
        .map(|value| f64::from(value.value()))
        .ok_or_else(|| RecoveryError::InvalidData("Player lacks NaturalFitness".into()))
}

pub async fn capture_post_match_condition(
    pool: &SqlitePool,
    input: &MatchInput,
    initial_conditions: &HashMap<Uuid, PlayerCondition>,
    raw_events: &[MatchEventEnvelope],
    match_year: i64,
    match_day_of_year: u32,
) -> RecoveryResult<()> {
    let mut participating_ids: HashSet<Uuid> = input
        .home()
        .lineup()
        .assignments()
        .iter()
        .chain(input.away().lineup().assignments().iter())
        .map(|assignment| assignment.player_id())
        .collect();
    let mut energy_by_player = HashMap::new();
    let mut reserve_by_player = HashMap::new();
    let mut impulse_by_player = HashMap::new();
    for envelope in raw_events {
        match envelope.event() {
            MatchEvent::SubstitutionMade(event) => {
                participating_ids.insert(event.player_in());
            }
            MatchEvent::PhysicalStrainRecorded(event) => {
                energy_by_player.insert(event.player_id(), event.energy_remaining());
                reserve_by_player.insert(event.player_id(), event.w_prime_balance());
            }
            MatchEvent::RecoveryIntervalProcessed(event) => {
                reserve_by_player.insert(event.player_id(), event.new_w_prime_balance());
            }
            MatchEvent::ImpulseShiftRecorded(event) => {
                impulse_by_player.insert(event.player_id(), event.new_value());
            }
            _ => {}
        }
    }
    let mut condition_rows = Vec::with_capacity(participating_ids.len());
    for player_id in participating_ids {
        let condition = initial_conditions.get(&player_id).ok_or_else(|| {
            RecoveryError::InvalidData(format!("Missing initial condition for {player_id}"))
        })?;
        let condition_row = PlayerConditionRow::new(
            player_id,
            energy_by_player
                .get(&player_id)
                .copied()
                .unwrap_or(condition.fatigue().energy())
                .clamp(0.0, 1.0),
            reserve_by_player
                .get(&player_id)
                .copied()
                .unwrap_or(condition.fatigue().w_prime())
                .clamp(0.0, 1.0),
            impulse_by_player.get(&player_id).copied().unwrap_or_else(|| {
                condition.impulse().current().round().clamp(0.0, 100.0) as u8
            }),
            condition.impulse().baseline(),
            condition.conditioning().readiness(),
            match_year,
            match_day_of_year,
            Some(match_year),
            Some(match_day_of_year),
        );
        condition_rows.push(condition_row);
    }
    let mut condition_tx = pool.begin().await?;
    player_condition::upsert_many_with_tx(&mut condition_tx, &condition_rows).await?;
    condition_tx.commit().await?;
    let now_seconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs() as i64)
        .unwrap_or(0);
    let match_date_unix_seconds =
        (match_year - 1970) * 31_557_600 + i64::from(match_day_of_year) * 86_400;
    let tuning = RecoveryTuningProfile::default();
    let recovery_profiles = load_recovery_profiles(pool).await?;
    for envelope in raw_events {
        if let MatchEvent::InjuryIncidentRecorded(event) = envelope.event() {
            let player = player_for(input, event.player_id())?;
            let age_years = calculate_player_age_years(
                player.birthdate_unix_seconds(),
                match_date_unix_seconds,
            );
            let (record, treatment) = register_injury(
                Uuid::new_v4(),
                event.injury_definition_id(),
                event.body_region(),
                event.severity_grade(),
                natural_fitness(input, player)?,
                age_years,
                &tuning,
                &recovery_profiles,
            )?;
            let injury_extent = recovery_profiles
                .get(&(record.injury_definition_id(), severity_grade_to_str(record.severity_grade()).into(), treatment.as_str().into()))
                .and_then(|profile| profile.injury_extent.clone());
            let injury_row = PlayerInjuryHistoryRow::new(
                record.id(),
                event.player_id(),
                record.injury_definition_id(),
                body_region_to_str(record.body_region()),
                severity_grade_to_str(record.severity_grade()),
                injury_extent,
                treatment.as_str(),
                match_year,
                match_day_of_year,
                record.days_remaining(),
                record.days_remaining(),
                0,
                "Injured",
                false,
                None,
                None,
                now_seconds,
            );
            let active = player_injury_history::get_active_by_player_id(pool, event.player_id()).await?;
            let mut tx = pool.begin().await?;
            if let Some(active) = active {
                if active.status == "Observation" && active.days_remaining == 0 {
                    let active_id = Uuid::parse_str(&active.id)
                        .map_err(|error| RecoveryError::InvalidData(error.to_string()))?;
                    player_injury_history::mark_resolved_with_tx(&mut tx, active_id, now_seconds).await?;
                }
            }
            player_injury_history::insert_with_tx(&mut tx, &injury_row).await?;
            tx.commit().await?;
        }
    }
    Ok(())
}
