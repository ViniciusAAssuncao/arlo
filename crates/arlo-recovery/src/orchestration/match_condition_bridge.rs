use crate::domain::{
    ConditioningProfile, FatigueCondition, ImpulseCondition, InjuryRecord, PlayerCondition,
};
use crate::error::{RecoveryError, RecoveryResult};
use crate::fatigue_recovery::calculate_player_age_years;
use crate::injury_recovery::register_injury;
use crate::tuning::RecoveryTuningProfile;
use arlo_domain::sport_constants::impulse_floor_for_baseline;
use arlo_domain::{AttributeKey, BodyRegion, InjurySeverityGrade};
use arlo_engine::physical::FatigueState;
use arlo_engine::psychology::state::ImpulseState;
use arlo_engine::world_state::MatchState;
use arlo_events::{MatchEvent, MatchEventEnvelope};
use arlo_persistence::models::condition::{PlayerConditionRow, PlayerInjuryHistoryRow};
use arlo_persistence::repositories::condition::{player_condition, player_injury_history};
use sqlx::SqlitePool;
use std::collections::{HashMap, HashSet};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

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

pub async fn load_conditions_for_players(
    pool: &SqlitePool,
    player_ids: &[Uuid],
) -> RecoveryResult<HashMap<Uuid, PlayerCondition>> {
    if player_ids.is_empty() {
        return Ok(HashMap::new());
    }

    let active_injuries =
        player_injury_history::list_active_by_player_ids(pool, player_ids).await?;
    let mut injuries_by_player: HashMap<Uuid, PlayerInjuryHistoryRow> =
        HashMap::with_capacity(active_injuries.len());
    for inj in active_injuries {
        if let Ok(pid) = Uuid::parse_str(&inj.player_id) {
            injuries_by_player.entry(pid).or_insert(inj);
        }
    }

    let condition_rows = player_condition::list_all(pool).await?;
    let mut condition_rows_by_player: HashMap<Uuid, PlayerConditionRow> =
        HashMap::with_capacity(condition_rows.len());
    for cond in condition_rows {
        if let Ok(pid) = Uuid::parse_str(&cond.player_id) {
            condition_rows_by_player.insert(pid, cond);
        }
    }

    let mut conditions = HashMap::with_capacity(player_ids.len());

    for &player_id in player_ids {
        let active_injury = if let Some(inj_row) = injuries_by_player.get(&player_id) {
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
            Some(record)
        } else {
            None
        };

        let (
            energy_level,
            anaerobic_reserve,
            impulse_val,
            impulse_base,
            cond_score,
            last_update_yr,
            last_update_d,
            last_match_yr,
            last_match_d,
        ) = match condition_rows_by_player.get(&player_id) {
            Some(row) => (
                row.energy_level,
                row.anaerobic_reserve,
                row.impulse_current_value as f64,
                row.impulse_baseline,
                row.conditioning_score,
                row.last_updated_year as u32,
                row.last_updated_day_of_year as u32,
                row.last_match_year.map(|y| y as u32),
                row.last_match_day_of_year.map(|d| d as u32),
            ),
            None => (1.0, 1.0, 50.0, 50.0, 0.5, 0, 0, None, None),
        };

        let fatigue = FatigueCondition::new(energy_level, anaerobic_reserve)?;
        let impulse = ImpulseCondition::new(impulse_val, impulse_base)?;
        let conditioning = ConditioningProfile::new(cond_score)?;

        let player_condition = PlayerCondition::new(
            player_id,
            fatigue,
            impulse,
            conditioning,
            active_injury,
            last_update_yr,
            last_update_d,
            last_match_yr,
            last_match_d,
        )?;

        conditions.insert(player_id, player_condition);
    }

    Ok(conditions)
}

pub fn seed_match_state(
    state: &mut MatchState,
    conditions: &HashMap<Uuid, PlayerCondition>,
) {
    let home_player_ids: Vec<Uuid> = state
        .home_lineup()
        .assignments()
        .iter()
        .map(|a| a.player().id())
        .collect();

    let away_player_ids: Vec<Uuid> = state
        .away_lineup()
        .assignments()
        .iter()
        .map(|a| a.player().id())
        .collect();

    for player_id in home_player_ids.into_iter().chain(away_player_ids) {
        if let Some(cond) = conditions.get(&player_id) {
            let fatigue_state = FatigueState::new(
                cond.fatigue().energy(),
                cond.fatigue().w_prime(),
            );
            state.set_player_fatigue(player_id, fatigue_state);

            let baseline = cond.impulse().baseline();
            let floor = impulse_floor_for_baseline(baseline);
            let impulse_state = ImpulseState::new(
                cond.impulse().current(),
                baseline,
                floor,
            );
            state.set_player_impulse(player_id, impulse_state);
        }
    }

    state.refresh_team_powers();
}

pub async fn capture_post_match_condition(
    pool: &SqlitePool,
    state: &MatchState,
    raw_events: &[MatchEventEnvelope],
    match_year: i64,
    match_day_of_year: u32,
) -> RecoveryResult<()> {
    let mut participating_ids: HashSet<Uuid> = HashSet::new();

    for assignment in state.home_lineup().assignments() {
        participating_ids.insert(assignment.player().id());
    }
    for assignment in state.away_lineup().assignments() {
        participating_ids.insert(assignment.player().id());
    }
    for &pid in state.home_fatigue().keys() {
        participating_ids.insert(pid);
    }
    for &pid in state.away_fatigue().keys() {
        participating_ids.insert(pid);
    }

    let existing_condition_rows = player_condition::list_all(pool).await?;
    let mut existing_conditions: HashMap<Uuid, PlayerConditionRow> =
        HashMap::with_capacity(existing_condition_rows.len());
    for row in existing_condition_rows {
        if let Ok(pid) = Uuid::parse_str(&row.player_id) {
            existing_conditions.insert(pid, row);
        }
    }

    for player_id in participating_ids {
        let fatigue = state.fatigue_for(&player_id);
        let impulse = state.impulse_for(&player_id);

        let conditioning_score = existing_conditions
            .get(&player_id)
            .map(|r| r.conditioning_score)
            .unwrap_or(0.5);

        let condition_row = PlayerConditionRow::new(
            player_id,
            fatigue.energy().clamp(0.0, 1.0),
            fatigue.w_prime_balance().clamp(0.0, 1.0),
            impulse.value(),
            impulse.baseline(),
            conditioning_score,
            match_year,
            match_day_of_year,
            Some(match_year),
            Some(match_day_of_year),
        );

        player_condition::upsert(pool, &condition_row).await?;
    }

    let now_seconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0);

    let current_date_unix_seconds =
        (match_year - 1970) * 31_557_600 + (match_day_of_year as i64) * 86_400;

    let tuning = RecoveryTuningProfile::default();

    for envelope in raw_events {
        if let MatchEvent::InjuryIncidentRecorded(ev) = envelope.event() {
            let player_id = ev.player_id();
            let injury_definition_id = ev.injury_definition_id();
            let body_region = ev.body_region();
            let severity_grade = ev.severity_grade();

            let natural_fitness = state
                .attribute_table_for(&player_id)
                .get(AttributeKey::NaturalFitness);

            let player_opt = state
                .home_lineup()
                .assignments()
                .iter()
                .chain(state.away_lineup().assignments().iter())
                .map(|a| a.player())
                .find(|p| p.id() == player_id)
                .or_else(|| {
                    state
                        .home_squad()
                        .bench()
                        .iter()
                        .chain(state.away_squad().bench().iter())
                        .map(|p| p.as_ref())
                        .find(|p| p.id() == player_id)
                });

            let age_years = player_opt
                .map(|p| {
                    calculate_player_age_years(
                        p.birthdate_unix_seconds(),
                        current_date_unix_seconds,
                    )
                })
                .unwrap_or(25.0);

            let injury_id = Uuid::new_v4();
            let record = register_injury(
                injury_id,
                injury_definition_id,
                body_region,
                severity_grade,
                natural_fitness,
                age_years,
                &tuning,
            )?;

            let injury_row = PlayerInjuryHistoryRow::new(
                record.id(),
                player_id,
                record.injury_definition_id(),
                body_region_to_str(record.body_region()),
                severity_grade_to_str(record.severity_grade()),
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

            player_injury_history::insert(pool, &injury_row).await?;
        }
    }

    Ok(())
}