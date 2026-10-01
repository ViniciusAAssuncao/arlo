use crate::dto::player_match::{
    PlayerMatchImpulseDto, PlayerMatchImpulseRunDto, PlayerMatchImpulseShiftByKindDto,
};
use crate::error::ControllerResult;
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn load_impulse_stats(
    pool: &SqlitePool,
    match_id: Uuid,
    player_id: Uuid,
) -> ControllerResult<Option<PlayerMatchImpulseDto>> {
    let impulse_row =
        arlo_persistence::repositories::match_player_impulse::get_by_match_id_and_player_id(
            pool, match_id, player_id,
        )
        .await?;

    let impulse_shift_rows =
        arlo_persistence::repositories::match_player_impulse::list_shifts_by_match_id_and_player_id(
            pool,
            match_id,
            player_id
        ).await?;

    let impulse_run_rows =
        arlo_persistence::repositories::match_player_impulse::list_runs_by_match_id_and_player_id(
            pool, match_id, player_id,
        )
        .await?;

    Ok(match impulse_row {
        Some(imp) => {
            let shifts_by_kind = impulse_shift_rows
                .into_iter()
                .map(|s| PlayerMatchImpulseShiftByKindDto {
                    event_kind: s.event_kind,
                    shifts_count: s.shifts_count as u32,
                })
                .collect();

            let runs = impulse_run_rows
                .into_iter()
                .map(|r| PlayerMatchImpulseRunDto {
                    run_index: r.run_index as u32,
                    start_time_seconds: r.start_time_seconds,
                    end_time_seconds: r.end_time_seconds,
                    duration_seconds: r.duration_seconds,
                    peak_value: r.peak_value as u8,
                    integrated_intensity: r.integrated_intensity,
                    shifts_count: r.shifts_count as u32,
                    average_intensity: r.average_intensity,
                })
                .collect();

            Some(PlayerMatchImpulseDto {
                baseline: imp.baseline,
                current_value: imp.current_value as u8,
                initial_value: imp.initial_value as u8,
                min_value: imp.min_value as u8,
                max_value: imp.max_value as u8,
                average_value: imp.average_value,
                shifts_count: imp.shifts_count as u32,
                positive_shifts: imp.positive_shifts as u32,
                negative_shifts: imp.negative_shifts as u32,
                time_below_baseline_seconds: imp.time_below_baseline_seconds,
                critical_reached_count: imp.critical_reached_count as u32,
                runs_count: imp.runs_count as u32,
                longest_run_duration_seconds: imp.longest_run_duration_seconds,
                peak_run_value: imp.peak_run_value as u8,
                total_integrated_run_intensity: imp.total_integrated_run_intensity,
                average_run_duration_seconds: imp.average_run_duration_seconds,
                average_run_intensity: imp.average_run_intensity,
                shifts_by_kind,
                runs,
            })
        }
        None => None,
    })
}
