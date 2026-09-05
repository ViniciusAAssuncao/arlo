use crate::domain::pitch::pitch::Pitch;
use crate::domain::sport_constants::{
    ARTRO_ROW_SPACING_MIRIM, ARTRO_SIZE_VAINA, ARTROS_PER_ROW, DEFAULT_ARTRO_LATERAL_OFFSET_MIRIM,
};
use arlo_math::units::{Length, MIRIM_TO_METERS, VAINA_TO_METERS};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ArtroPlacement {
    LeftLateral,
    Central,
    RightLateral,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Artro {
    placement: ArtroPlacement,
    x: Length,
    y: Length,
    size: Length,
}

impl Artro {
    pub fn new(placement: ArtroPlacement, x: Length, y: Length, size: Length) -> Self {
        Self {
            placement,
            x,
            y,
            size,
        }
    }

    pub fn placement(&self) -> ArtroPlacement {
        self.placement
    }

    pub fn x(&self) -> Length {
        self.x
    }

    pub fn y(&self) -> Length {
        self.y
    }

    pub fn size(&self) -> Length {
        self.size
    }

    pub fn x_mirim(&self) -> f64 {
        self.x.value() / MIRIM_TO_METERS
    }

    pub fn y_mirim(&self) -> f64 {
        self.y.value() / MIRIM_TO_METERS
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArtroRow {
    row_index: usize,
    x: Length,
    artros: [Artro; ARTROS_PER_ROW],
}

impl ArtroRow {
    pub fn new(row_index: usize, x: Length, artros: [Artro; ARTROS_PER_ROW]) -> Self {
        Self {
            row_index,
            x,
            artros,
        }
    }

    pub fn row_index(&self) -> usize {
        self.row_index
    }

    pub fn x(&self) -> Length {
        self.x
    }

    pub fn x_mirim(&self) -> f64 {
        self.x.value() / MIRIM_TO_METERS
    }

    pub fn artros(&self) -> &[Artro; ARTROS_PER_ROW] {
        &self.artros
    }
}

pub fn artro_rows_for_pitch(pitch: &Pitch) -> Vec<ArtroRow> {
    let length_mirim = pitch.length_mirim();
    let width_mirim = pitch.width_mirim();
    let artro_size = Length::new(ARTRO_SIZE_VAINA * VAINA_TO_METERS);

    let center_y_mirim = width_mirim / 2.0;
    let left_y_mirim = center_y_mirim - DEFAULT_ARTRO_LATERAL_OFFSET_MIRIM;
    let right_y_mirim = center_y_mirim + DEFAULT_ARTRO_LATERAL_OFFSET_MIRIM;

    let center_y = Length::new(center_y_mirim * MIRIM_TO_METERS);
    let left_y = Length::new(left_y_mirim * MIRIM_TO_METERS);
    let right_y = Length::new(right_y_mirim * MIRIM_TO_METERS);

    let row_count = (length_mirim / ARTRO_ROW_SPACING_MIRIM).floor() as usize;
    let mut rows = Vec::with_capacity(row_count);

    for i in 0..row_count {
        let x_mirim = (i as f64 + 1.0) * ARTRO_ROW_SPACING_MIRIM;
        let x = Length::new(x_mirim * MIRIM_TO_METERS);

        let left_artro = Artro::new(ArtroPlacement::LeftLateral, x, left_y, artro_size);
        let center_artro = Artro::new(ArtroPlacement::Central, x, center_y, artro_size);
        let right_artro = Artro::new(ArtroPlacement::RightLateral, x, right_y, artro_size);

        rows.push(ArtroRow::new(
            i,
            x,
            [left_artro, center_artro, right_artro],
        ));
    }

    rows
}
