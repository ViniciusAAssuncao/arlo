pub fn map_z_gap_to_rating(z_gap: f64, gain: f64) -> (f64, f64) {
    let rating_diff = z_gap * gain;
    let att = (10.0 + rating_diff / 2.0).clamp(1.0, 20.0);
    let def = (10.0 - rating_diff / 2.0).clamp(1.0, 20.0);
    (att, def)
}