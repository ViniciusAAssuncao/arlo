use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct VoronoiSite {
    pub x: f64,
    pub y: f64,
    pub speed: f64,
    pub reaction_time: f64,
    pub team_id: u8,
}

impl VoronoiSite {
    pub fn new(x: f64, y: f64, speed: f64, reaction_time: f64, team_id: u8) -> Self {
        Self {
            x,
            y,
            speed: if speed > 0.0 { speed } else { 1.0 },
            reaction_time: reaction_time.max(0.0),
            team_id,
        }
    }

    pub fn time_to_reach(&self, qx: f64, qy: f64) -> f64 {
        let dx = qx - self.x;
        let dy = qy - self.y;
        let dist = (dx * dx + dy * dy).sqrt();
        self.reaction_time + (dist / self.speed)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct VoronoiRegion {
    pub x_min: f64,
    pub x_max: f64,
    pub y_min: f64,
    pub y_max: f64,
}

impl VoronoiRegion {
    pub fn new(x_min: f64, x_max: f64, y_min: f64, y_max: f64) -> Self {
        Self {
            x_min: x_min.min(x_max),
            x_max: x_min.max(x_max),
            y_min: y_min.min(y_max),
            y_max: y_min.max(y_max),
        }
    }

    pub fn contains(&self, x: f64, y: f64) -> bool {
        x >= self.x_min && x <= self.x_max && y >= self.y_min && y <= self.y_max
    }

    pub fn width(&self) -> f64 {
        self.x_max - self.x_min
    }

    pub fn height(&self) -> f64 {
        self.y_max - self.y_min
    }

    pub fn area(&self) -> f64 {
        self.width() * self.height()
    }
}

pub fn compute_site_dominance(sites: &[VoronoiSite], qx: f64, qy: f64) -> Option<usize> {
    if sites.is_empty() {
        return None;
    }
    let mut min_idx = 0;
    let mut min_time = f64::INFINITY;
    for (i, site) in sites.iter().enumerate() {
        let t = site.time_to_reach(qx, qy);
        if t < min_time {
            min_time = t;
            min_idx = i;
        }
    }
    Some(min_idx)
}

pub fn compute_point_team_control(
    attackers: &[VoronoiSite],
    defenders: &[VoronoiSite],
    qx: f64,
    qy: f64,
) -> f64 {
    if attackers.is_empty() && defenders.is_empty() {
        return 0.5;
    }
    if attackers.is_empty() {
        return 0.0;
    }
    if defenders.is_empty() {
        return 1.0;
    }

    let min_att_time = attackers
        .iter()
        .map(|s| s.time_to_reach(qx, qy))
        .fold(f64::INFINITY, f64::min);
    let min_def_time = defenders
        .iter()
        .map(|s| s.time_to_reach(qx, qy))
        .fold(f64::INFINITY, f64::min);

    let time_diff = min_def_time - min_att_time;
    let scaling = 2.5;
    1.0 / (1.0 + (-scaling * time_diff).exp())
}

pub fn compute_team_control_fraction(
    attackers: &[VoronoiSite],
    defenders: &[VoronoiSite],
    region: &VoronoiRegion,
    samples_x: usize,
    samples_y: usize,
) -> f64 {
    if attackers.is_empty() && defenders.is_empty() {
        return 0.5;
    }
    if attackers.is_empty() {
        return 0.0;
    }
    if defenders.is_empty() {
        return 1.0;
    }

    let nx = samples_x.max(2);
    let ny = samples_y.max(2);
    let dx = region.width() / (nx as f64);
    let dy = region.height() / (ny as f64);

    let mut total_control = 0.0;
    let total_samples = (nx * ny) as f64;

    for ix in 0..nx {
        let qx = region.x_min + (ix as f64 + 0.5) * dx;
        for iy in 0..ny {
            let qy = region.y_min + (iy as f64 + 0.5) * dy;
            total_control += compute_point_team_control(attackers, defenders, qx, qy);
        }
    }

    (total_control / total_samples).clamp(0.0, 1.0)
}

pub fn compute_region_control(
    sites: &[VoronoiSite],
    region: &VoronoiRegion,
    samples_x: usize,
    samples_y: usize,
) -> Vec<f64> {
    if sites.is_empty() {
        return Vec::new();
    }
    let mut counts = vec![0.0; sites.len()];
    let nx = samples_x.max(2);
    let ny = samples_y.max(2);
    let dx = region.width() / (nx as f64);
    let dy = region.height() / (ny as f64);
    let total_samples = (nx * ny) as f64;

    for ix in 0..nx {
        let qx = region.x_min + (ix as f64 + 0.5) * dx;
        for iy in 0..ny {
            let qy = region.y_min + (iy as f64 + 0.5) * dy;
            if let Some(best) = compute_site_dominance(sites, qx, qy) {
                counts[best] += 1.0;
            }
        }
    }

    for c in &mut counts {
        *c /= total_samples;
    }
    counts
}
