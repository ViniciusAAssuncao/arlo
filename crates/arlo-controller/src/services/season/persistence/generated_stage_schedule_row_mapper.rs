use crate::domain::season::KnockoutTie;
use crate::services::season::persistence::generated_season_row_mapper::map_fixture_to_row;
use crate::services::season::persistence::season_persistence_codes::{
    stage_status_to_code, stage_type_to_code,
};
use crate::services::season::stage::stage_schedule_generator::GeneratedStageSchedule;
use arlo_persistence::models::season::{FixtureRow, KnockoutTieRow, SeasonStageRow};

pub fn map_knockout_tie_to_row(tie: &KnockoutTie) -> KnockoutTieRow {
    KnockoutTieRow::new(
        tie.id(),
        tie.season_stage_id(),
        tie.round_index(),
        tie.tie_index(),
        tie.high_seed().team_id(),
        tie.high_seed().seed_number(),
        tie.low_seed().team_id(),
        tie.low_seed().seed_number(),
        tie.leg_one_fixture_id(),
        tie.leg_two_fixture_id(),
        tie.aggregate_winner_team_id(),
    )
}

pub fn map_generated_stage_schedule_to_rows(
    generated: &GeneratedStageSchedule,
) -> (SeasonStageRow, Vec<FixtureRow>, Vec<KnockoutTieRow>) {
    let stage_row = SeasonStageRow::new(
        generated.stage_instance.id(),
        generated.stage_instance.season_instance_id(),
        generated.stage_instance.stage_order_index(),
        stage_type_to_code(generated.stage_instance.stage_type()),
        stage_status_to_code(generated.stage_instance.status()),
    );

    let fixture_rows = generated.fixtures.iter().map(map_fixture_to_row).collect();
    let tie_rows = generated
        .knockout_ties
        .iter()
        .map(map_knockout_tie_to_row)
        .collect();

    (stage_row, fixture_rows, tie_rows)
}