pub fn depth_from_bipolar(value: f64, pitch_length_m: f64, attacking_positive_x: bool) -> f64 {
    let t = (value.clamp(-1.0, 1.0) + 1.0) * 0.5;
    if attacking_positive_x {
        t * pitch_length_m
    } else {
        (1.0 - t) * pitch_length_m
    }
}

pub fn individual_line_depth_offset(
    bias_value: f64,
    band_min_ratio: f64,
    band_max_ratio: f64,
    pitch_length_m: f64,
    attacking_positive_x: bool,
) -> f64 {
    let band_width_m = (band_max_ratio - band_min_ratio).abs() * pitch_length_m;
    let magnitude = bias_value.clamp(-1.0, 1.0) * (band_width_m / 2.0);
    if attacking_positive_x {
        magnitude
    } else {
        -magnitude
    }
}

pub fn lateral_spread(width_value: f64, base_y_m: f64, pitch_width_m: f64) -> f64 {
    let clamped_width = width_value.clamp(-1.0, 1.0);
    let center_y = pitch_width_m * 0.5;

    if clamped_width < 0.0 {
        center_y + (1.0 + clamped_width) * (base_y_m - center_y)
    } else {
        let edge_y = if base_y_m >= center_y {
            pitch_width_m
        } else {
            0.0
        };
        base_y_m + clamped_width * (edge_y - base_y_m)
    }
}

pub fn lateral_flank_shift(flank_bias_value: f64, y_m: f64, pitch_width_m: f64) -> f64 {
    let bias = flank_bias_value.clamp(-1.0, 1.0);
    if bias < 0.0 {
        y_m + bias.abs() * (0.0 - y_m)
    } else {
        y_m + bias * (pitch_width_m - y_m)
    }
}

pub fn lateral_ratio_from_center(y_meters: f64, pitch_width_meters: f64) -> f64 {
    let center_y_m = pitch_width_meters * 0.5;
    let dist_from_center_m = (y_meters - center_y_m).abs();
    (dist_from_center_m / center_y_m.max(1.0)).clamp(0.0, 1.0)
}