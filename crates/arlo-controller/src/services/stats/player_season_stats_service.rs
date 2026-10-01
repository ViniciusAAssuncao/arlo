use crate::dto::player::PlayerSeasonStatsDto;
use crate::dto::stats::*;
use crate::error::{ControllerError, ControllerResult};
use crate::services::season::active_season_resolver::resolve_active_season;
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn build_player_season_stats(
    pool: &SqlitePool,
    player_id: Uuid,
) -> ControllerResult<PlayerSeasonStatsDto> {
    let player = arlo_db::repositories::player::get_by_id(pool, player_id)
        .await
        .map_err(|e| ControllerError::InvalidData(e.to_string()))?
        .ok_or_else(|| ControllerError::NotFound(format!("Player {} not found", player_id)))?;

    let team_id_opt = player.team_id();
    let league_id_opt = match team_id_opt {
        Some(tid) => {
            let team = arlo_db::repositories::team::get_by_id(pool, tid)
                .await
                .map_err(|e| ControllerError::InvalidData(e.to_string()))?;
            team.and_then(|t| t.league_id())
        }
        None => None,
    };

    let active_season_opt = match league_id_opt {
        Some(lid) => resolve_active_season(pool, lid).await?,
        None => None,
    };

    let (season_instance_id_opt, competition_id_opt, competition_name_opt, season_label_opt) =
        match active_season_opt {
            Some(s) => {
                let s_id = Uuid::parse_str(&s.id)?;
                let c_id = Uuid::parse_str(&s.competition_id)?;
                let comp = arlo_db::repositories::competition::get_by_id(pool, c_id)
                    .await
                    .map_err(|e| ControllerError::InvalidData(e.to_string()))?;
                let c_name = comp.map(|c| c.name().to_string());
                let label = s.reference_year.to_string();
                (Some(s_id), Some(c_id.to_string()), c_name, Some(label))
            }
            None => (None, None, None, None),
        };

    let season_id = match season_instance_id_opt {
        Some(id) => id,
        None => {
            return Ok(PlayerSeasonStatsDto {
                player_id: player_id.to_string(),
                season_instance_id: None,
                competition_id: None,
                competition_name: None,
                season_label: None,
                primary_role: None,
                appearances: PlayerAppearanceStatsDto::default(),
                scoring: PlayerScoringStatsDto::default(),
                assists: PlayerAssistStatsDto::default(),
                touches: PlayerTouchStatsDto::default(),
                duels: PlayerDuelStatsDto::default(),
                receiving: PlayerReceivingStatsDto::default(),
                drives: PlayerDriveStatsDto::default(),
                fouls: PlayerFoulStatsDto::default(),
                kick_fouls: PlayerKickFoulStatsDto::default(),
                artrine_decisions: PlayerArtrineDecisionStatsDto::default(),
            });
        }
    };

    let appearances_row =
        arlo_persistence::repositories::season_stats::player_appearances::get_player_appearances(
            pool, player_id, season_id,
        )
        .await?;

    let scoring_row =
        arlo_persistence::repositories::season_stats::player_scoring::get_player_scoring_stats(
            pool, player_id, season_id,
        )
        .await?;

    let assist_row =
        arlo_persistence::repositories::season_stats::player_assists::get_player_assist_stats(
            pool, player_id, season_id,
        )
        .await?;

    let touch_row =
        arlo_persistence::repositories::season_stats::player_touches::get_player_touch_stats(
            pool, player_id, season_id,
        )
        .await?;

    let duel_totals_row =
        arlo_persistence::repositories::season_stats::player_duels::get_player_duel_totals(
            pool, player_id, season_id,
        )
        .await?;

    let duel_kinds_rows =
        arlo_persistence::repositories::season_stats::player_duels::list_player_duels_by_kind(
            pool, player_id, season_id,
        )
        .await?;

    let receiving_row =
        arlo_persistence::repositories::season_stats::player_receiving::get_player_receiving_stats(
            pool, player_id, season_id,
        )
        .await?;

    let drive_row =
        arlo_persistence::repositories::season_stats::player_drives::get_player_drive_stats(
            pool, player_id, season_id,
        )
        .await?;

    let foul_row =
        arlo_persistence::repositories::season_stats::player_fouls::get_player_foul_stats(
            pool, player_id, season_id,
        )
        .await?;

    let kick_foul_totals_row = arlo_persistence::repositories::season_stats::player_kick_fouls::get_player_kick_foul_totals(
        pool,
        player_id,
        season_id,
    )
    .await?;

    let kick_foul_decisions_rows = arlo_persistence::repositories::season_stats::player_kick_fouls::list_player_kick_fouls_by_decision(
        pool,
        player_id,
        season_id,
    )
    .await?;

    let artrine_row = arlo_persistence::repositories::season_stats::player_artrine_decisions::get_player_artrine_decision_stats(
        pool,
        player_id,
        season_id,
    )
    .await?;

    let primary_role =
        arlo_persistence::repositories::season_stats::player_roles::get_player_primary_role(
            pool, player_id, season_id,
        )
        .await?;

    let appearances = PlayerAppearanceStatsDto {
        squad_selections: appearances_row.squad_selections as u32,
        starts: appearances_row.starts as u32,
        appearances: appearances_row.appearances as u32,
        substitute_appearances: appearances_row.substitute_appearances as u32,
    };

    let conversion_rate = if scoring_row.attempts > 0 {
        (scoring_row.converted as f64) / (scoring_row.attempts as f64)
    } else {
        0.0
    };

    let scoring = PlayerScoringStatsDto {
        attempts: scoring_row.attempts as u32,
        converted: scoring_row.converted as u32,
        missed: scoring_row.missed as u32,
        conversion_rate,
        goal_points_scored: scoring_row.goal_points_scored as u32,
        field_points_scored: scoring_row.field_points_scored as u32,
        field_goals_scored: scoring_row.field_goals_scored as u32,
        total_points_scored: scoring_row.total_points_scored as u32,
        by_post: Vec::new(),
    };

    let assists = PlayerAssistStatsDto {
        goalpoint_assists: assist_row.goalpoint_assists as u32,
    };

    let touches = PlayerTouchStatsDto {
        total_touches: touch_row.total_touches as u32,
        passes_attempted: touch_row.passes_attempted as u32,
        passes_received: touch_row.passes_received as u32,
        drives_recorded: touch_row.drives_recorded as u32,
        recoveries: touch_row.recoveries as u32,
        scoring_attempts: touch_row.scoring_attempts as u32,
        turnovers_conceded: touch_row.turnovers_conceded as u32,
    };

    let duel_win_rate = if duel_totals_row.total_duels > 0 {
        (duel_totals_row.total_wins as f64) / (duel_totals_row.total_duels as f64)
    } else {
        0.0
    };
    let attacker_win_rate = if duel_totals_row.attacker_duels > 0 {
        (duel_totals_row.attacker_wins as f64) / (duel_totals_row.attacker_duels as f64)
    } else {
        0.0
    };
    let defender_win_rate = if duel_totals_row.defender_duels > 0 {
        (duel_totals_row.defender_wins as f64) / (duel_totals_row.defender_duels as f64)
    } else {
        0.0
    };

    let mut saves_attempted = 0u32;
    let mut saves_made = 0u32;

    let by_kind: Vec<PlayerDuelKindStatsDto> = duel_kinds_rows
        .into_iter()
        .map(|k| {
            let k_win_rate = if k.total > 0 {
                (k.wins as f64) / (k.total as f64)
            } else {
                0.0
            };

            let is_save_kind = k.duel_kind == "FinishingAttempt"
                || k.duel_kind == "FieldGoalAttempt"
                || k.duel_kind == "KickBlockAttempt";

            if is_save_kind {
                let def_attempts = (k.as_defender_wins + k.as_defender_losses) as u32;
                saves_attempted += def_attempts;
                saves_made += k.as_defender_wins as u32;
            }

            PlayerDuelKindStatsDto {
                duel_kind: k.duel_kind,
                total: k.total as u32,
                wins: k.wins as u32,
                losses: k.losses as u32,
                as_attacker_wins: k.as_attacker_wins as u32,
                as_attacker_losses: k.as_attacker_losses as u32,
                as_defender_wins: k.as_defender_wins as u32,
                as_defender_losses: k.as_defender_losses as u32,
                win_rate: k_win_rate,
            }
        })
        .collect();

    let save_percentage = if saves_attempted > 0 {
        ((saves_made as f64) / (saves_attempted as f64)) * 100.0
    } else {
        0.0
    };

    let duels = PlayerDuelStatsDto {
        total_duels: duel_totals_row.total_duels as u32,
        total_wins: duel_totals_row.total_wins as u32,
        total_losses: duel_totals_row.total_losses as u32,
        win_rate: duel_win_rate,
        attacker_duels: duel_totals_row.attacker_duels as u32,
        attacker_wins: duel_totals_row.attacker_wins as u32,
        attacker_losses: duel_totals_row.attacker_losses as u32,
        attacker_win_rate,
        defender_duels: duel_totals_row.defender_duels as u32,
        defender_wins: duel_totals_row.defender_wins as u32,
        defender_losses: duel_totals_row.defender_losses as u32,
        defender_win_rate,
        by_kind,
        saves_attempted,
        saves_made,
        save_percentage,
    };

    let catch_rate = if receiving_row.targets > 0 {
        (receiving_row.receptions as f64) / (receiving_row.targets as f64)
    } else {
        0.0
    };
    let drop_rate = if receiving_row.targets > 0 {
        (receiving_row.drops as f64) / (receiving_row.targets as f64)
    } else {
        0.0
    };
    let average_mirins_per_reception = if receiving_row.receptions > 0 {
        receiving_row.receiving_mirins / (receiving_row.receptions as f64)
    } else {
        0.0
    };

    let receiving = PlayerReceivingStatsDto {
        targets: receiving_row.targets as u32,
        receptions: receiving_row.receptions as u32,
        drops: receiving_row.drops as u32,
        catch_rate,
        drop_rate,
        receiving_mirins: receiving_row.receiving_mirins,
        run_after_catch_mirins: receiving_row.run_after_catch_mirins,
        longest_reception_mirim: receiving_row.longest_reception_mirim,
        average_mirins_per_reception,
    };

    let drives = PlayerDriveStatsDto {
        total_drives: drive_row.total_drives as u32,
        central_drives: drive_row.central_drives as u32,
        left_lateral_drives: drive_row.left_lateral_drives as u32,
        right_lateral_drives: drive_row.right_lateral_drives as u32,
        lateral_drives: drive_row.lateral_drives as u32,
        max_drives_in_series: drive_row.max_drives_in_series as u32,
    };

    let fouls = PlayerFoulStatsDto {
        fouls_committed: foul_row.fouls_committed as u32,
        fouls_drawn: foul_row.fouls_drawn as u32,
        correct_calls_committed: foul_row.correct_calls_committed as u32,
        incorrect_calls_committed: foul_row.incorrect_calls_committed as u32,
        expulsions: foul_row.expulsions as u32,
        time_penalties: foul_row.time_penalties as u32,
        by_origin: Vec::new(),
    };

    let by_decision: Vec<PlayerKickFoulDecisionStatsDto> = kick_foul_decisions_rows
        .into_iter()
        .map(|d| PlayerKickFoulDecisionStatsDto {
            decision_kind: d.decision_kind,
            takes_count: d.takes_count as u32,
        })
        .collect();

    let kick_fouls = PlayerKickFoulStatsDto {
        kick_foul_takes: kick_foul_totals_row.kick_foul_takes as u32,
        by_decision,
    };

    let artrine_success_rate = if artrine_row.total_decisions > 0 {
        (artrine_row.total_successful_decisions as f64) / (artrine_row.total_decisions as f64)
    } else {
        0.0
    };
    let average_mirins_per_decision = if artrine_row.total_decisions > 0 {
        artrine_row.total_mirins_advanced / (artrine_row.total_decisions as f64)
    } else {
        0.0
    };

    let artrine_decisions = PlayerArtrineDecisionStatsDto {
        total_decisions: artrine_row.total_decisions as u32,
        total_successful_decisions: artrine_row.total_successful_decisions as u32,
        total_failed_decisions: artrine_row.total_failed_decisions as u32,
        success_rate: artrine_success_rate,
        total_mirins_advanced: artrine_row.total_mirins_advanced,
        average_mirins_per_decision,
        total_points_generated: artrine_row.total_points_generated as u32,
        goal_points_generated: artrine_row.goal_points_generated as u32,
        field_points_generated: artrine_row.field_points_generated as u32,
        field_goals_generated: artrine_row.field_goals_generated as u32,
        by_kind: Vec::new(),
    };

    Ok(PlayerSeasonStatsDto {
        player_id: player_id.to_string(),
        season_instance_id: Some(season_id.to_string()),
        competition_id: competition_id_opt,
        competition_name: competition_name_opt,
        season_label: season_label_opt,
        primary_role,
        appearances,
        scoring,
        assists,
        touches,
        duels,
        receiving,
        drives,
        fouls,
        kick_fouls,
        artrine_decisions,
    })
}
