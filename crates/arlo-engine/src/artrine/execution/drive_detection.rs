use crate::artrine::constants::ARTRO_ROW_SPACING_MIRIM;
use arlo_domain::pitch::Pitch;
use smallvec::SmallVec;

pub fn detect_drive_crossings(
    pitch: &Pitch,
    start_x_mirim: f64,
    end_x_mirim: f64,
    attacking_positive_x: bool,
) -> SmallVec<[usize; 4]> {
    let mut drive_row_indices = SmallVec::new();
    let length_mirim = pitch.length_mirim();
    let row_count = (length_mirim / ARTRO_ROW_SPACING_MIRIM).floor() as usize;

    if attacking_positive_x {
        if end_x_mirim > start_x_mirim {
            for i in 0..row_count {
                let row_x = (i as f64 + 1.0) * ARTRO_ROW_SPACING_MIRIM;
                if row_x > start_x_mirim && row_x <= end_x_mirim {
                    drive_row_indices.push(i);
                }
            }
        }
    } else if end_x_mirim < start_x_mirim {
        for i in (0..row_count).rev() {
            let row_x = (i as f64 + 1.0) * ARTRO_ROW_SPACING_MIRIM;
            if row_x < start_x_mirim && row_x >= end_x_mirim {
                drive_row_indices.push(i);
            }
        }
    }

    drive_row_indices
}

pub fn detect_drive_crossings_arithmetic(
    pitch: &Pitch,
    start_x_mirim: f64,
    end_x_mirim: f64,
    attacking_positive_x: bool,
) -> SmallVec<[usize; 4]> {
    detect_drive_crossings(pitch, start_x_mirim, end_x_mirim, attacking_positive_x)
}