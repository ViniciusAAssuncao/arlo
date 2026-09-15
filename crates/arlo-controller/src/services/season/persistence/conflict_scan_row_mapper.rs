use crate::domain::season::{Fixture, PostponementRecord};
use crate::services::season::conflict::ConflictScanReport;
use crate::services::season::persistence::generated_season_row_mapper::map_fixture_to_row;
use crate::services::season::persistence::season_persistence_codes::postponement_reason_to_code;
use arlo_persistence::models::season::{FixtureRow, PostponementRecordRow};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn map_postponement_record_to_row(
    record: &PostponementRecord,
    now: i64,
) -> PostponementRecordRow {
    PostponementRecordRow::new(
        record.id(),
        record.fixture_id(),
        record.original_date().year(),
        record.original_date().day_of_year(),
        record.new_date().year(),
        record.new_date().day_of_year(),
        postponement_reason_to_code(record.reason()),
        now,
    )
}

pub fn map_conflict_scan_to_rows(
    report: &ConflictScanReport,
    fixtures: &[Fixture],
) -> (Vec<FixtureRow>, Vec<PostponementRecordRow>) {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0);

    let fixture_rows = fixtures.iter().map(map_fixture_to_row).collect();
    let postponement_rows = report
        .postponement_records()
        .iter()
        .map(|r| map_postponement_record_to_row(r, now))
        .collect();

    (fixture_rows, postponement_rows)
}