use super::constants::{AINA_TO_METERS, MIRIM_TO_METERS, VAINA_TO_METERS};

pub fn mirim_to_meters(mirim: f64) -> f64 {
    mirim * MIRIM_TO_METERS
}

pub fn meters_to_mirim(meters: f64) -> f64 {
    meters / MIRIM_TO_METERS
}

pub fn aina_to_meters(aina: f64) -> f64 {
    aina * AINA_TO_METERS
}

pub fn meters_to_aina(meters: f64) -> f64 {
    meters / AINA_TO_METERS
}

pub fn vaina_to_meters(vaina: f64) -> f64 {
    vaina * VAINA_TO_METERS
}

pub fn meters_to_vaina(meters: f64) -> f64 {
    meters / VAINA_TO_METERS
}
