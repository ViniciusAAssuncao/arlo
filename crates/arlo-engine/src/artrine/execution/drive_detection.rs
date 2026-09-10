use crate::artrine::constants::CARRY_ARTRO_SEARCH_MARGIN_MIRIM;
use crate::possession::drive::artrine_identity::TrueArtrine;
use crate::possession::drive::validator::validate_continuous_trajectory;
use crate::spatial::TickSimulationResult;
use arlo_domain::pitch::{artro_rows_for_pitch, Pitch};
use arlo_math::units::{Position as VectorPosition, MIRIM_TO_METERS};
use smallvec::SmallVec;
use uuid::Uuid;

pub fn detect_drive_crossings(
    artrine_id: Uuid,
    pitch: &Pitch,
    tick_result: &TickSimulationResult,
    start_pos: VectorPosition,
    end_position: VectorPosition,
    attacking_positive_x: bool,
) -> SmallVec<[usize; 4]> {
    let all_rows = artro_rows_for_pitch(pitch);
    let true_artrine = TrueArtrine::new(artrine_id);
    let mut drive_row_indices = SmallVec::new();

    let segments = tick_result
        .get_trajectory(&artrine_id)
        .map(|t| t.segments())
        .unwrap_or_default();

    let segments_to_test = if segments.is_empty() {
        vec![(start_pos, end_position)]
    } else {
        segments
    };

    let (min_x, max_x) = if start_pos.raw().0 < end_position.raw().0 {
        (start_pos.raw().0, end_position.raw().0)
    } else {
        (end_position.raw().0, start_pos.raw().0)
    };

    let artro_search_margin = CARRY_ARTRO_SEARCH_MARGIN_MIRIM * MIRIM_TO_METERS;
    for row in &all_rows {
        let rx = row.x().value();
        if rx < min_x - artro_search_margin || rx > max_x + artro_search_margin {
            continue;
        }
        for artro in row.artros() {
            let drive_result =
                validate_continuous_trajectory(true_artrine, &segments_to_test, artro, false);
            if drive_result.is_valid() {
                if !drive_row_indices.contains(&row.row_index()) {
                    drive_row_indices.push(row.row_index());
                }
                break;
            }
        }
    }

    if attacking_positive_x {
        drive_row_indices.sort();
    } else {
        drive_row_indices.sort_by(|a, b| b.cmp(a));
    }

    drive_row_indices
}