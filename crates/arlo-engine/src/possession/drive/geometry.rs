use arlo_domain::pitch::Artro;
use arlo_math::units::Position;

pub fn point_in_artro(point: Position, artro: &Artro) -> bool {
    let half_size = artro.size().value() / 2.0;
    let x_min = artro.x().value() - half_size;
    let x_max = artro.x().value() + half_size;
    let y_min = artro.y().value() - half_size;
    let y_max = artro.y().value() + half_size;

    let px = point.raw().0;
    let py = point.raw().1;

    px >= x_min && px <= x_max && py >= y_min && py <= y_max
}

pub fn segment_intersects_box(
    x0: f64,
    y0: f64,
    x1: f64,
    y1: f64,
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
) -> bool {
    let dx = x1 - x0;
    let dy = y1 - y0;

    let p = [-dx, dx, -dy, dy];
    let q = [x0 - x_min, x_max - x0, y0 - y_min, y_max - y0];

    let mut u1 = 0.0;
    let mut u2 = 1.0;

    for i in 0..4 {
        let pi = p[i];
        let qi = q[i];

        if pi == 0.0 {
            if qi < 0.0 {
                return false;
            }
        } else {
            let t = qi / pi;
            if pi < 0.0 {
                if t > u1 {
                    u1 = t;
                }
            } else if t < u2 {
                u2 = t;
            }
        }
    }

    u1 <= u2
}

pub fn segment_intersects_artro(start: Position, end: Position, artro: &Artro) -> bool {
    let half_size = artro.size().value() / 2.0;
    let x_min = artro.x().value() - half_size;
    let x_max = artro.x().value() + half_size;
    let y_min = artro.y().value() - half_size;
    let y_max = artro.y().value() + half_size;

    segment_intersects_box(
        start.raw().0,
        start.raw().1,
        end.raw().0,
        end.raw().1,
        x_min,
        x_max,
        y_min,
        y_max,
    )
}