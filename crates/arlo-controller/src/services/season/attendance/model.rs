use arlo_persistence::persister::match_persistence_context::AttendanceSnapshot;
use uuid::Uuid;

pub const ATTENDANCE_MODEL_VERSION: &str = "attendance-v1";

#[derive(Clone, Copy)]
pub struct TeamDemand {
    pub id: Uuid,
    pub prestige: i32,
    pub minimum: i32,
    pub maximum: i32,
    pub titles: i64,
    pub form: f64,
    pub rating: Option<f64>,
    pub squad_quality: f64,
    pub country_id: Uuid,
    pub home_venue_id: Option<Uuid>,
}

#[derive(Clone, Copy)]
pub struct VenueDemand {
    pub id: Uuid,
    pub capacity: i32,
    pub owner_team_id: Option<Uuid>,
    pub country_id: Uuid,
}

#[derive(Clone, Copy)]
pub struct EventDemand {
    pub competition_prestige: i32,
    pub international: bool,
    pub knockout: bool,
    pub final_round: bool,
    pub stage_progress: f64,
    pub table_stakes: f64,
    pub calendar_effect: f64,
    pub declared_neutral: bool,
    pub seed: u64,
}

fn sample(seed: u64) -> f64 {
    let mut value = seed.wrapping_add(0x9e3779b97f4a7c15);
    value = (value ^ (value >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    value = (value ^ (value >> 27)).wrapping_mul(0x94d049bb133111eb);
    let value = value ^ (value >> 31);
    (value >> 11) as f64 / (1_u64 << 53) as f64
}

fn popularity(team: TeamDemand) -> f64 {
    let prestige = (team.prestige as f64 / 1000.0).clamp(0.0, 1.0);
    let titles = ((team.titles.max(0) as f64).ln_1p() / 21_f64.ln()).clamp(0.0, 1.0);
    let support = ((team.maximum.max(0) as f64).ln_1p() / 100_001_f64.ln()).clamp(0.0, 1.0);
    (0.68 * prestige + 0.14 * titles + 0.18 * support).clamp(0.0, 1.0)
}

fn appeal(
    event: EventDemand,
    home: TeamDemand,
    away: TeamDemand,
    home_popularity: f64,
    away_popularity: f64,
) -> (f64, f64) {
    let stakes = if event.final_round {
        1.0
    } else if event.knockout {
        0.72
    } else {
        0.25 + 0.25 * event.stage_progress.clamp(0.0, 1.0)
            + 0.30 * event.table_stakes.clamp(0.0, 1.0)
    };
    let competition = (event.competition_prestige as f64 / 1000.0).clamp(0.0, 1.0);
    let home_rating = home.rating.map_or(home_popularity, |rating| {
        ((rating - 1300.0) / 400.0).clamp(0.0, 1.0)
    });
    let away_rating = away.rating.map_or(away_popularity, |rating| {
        ((rating - 1300.0) / 400.0).clamp(0.0, 1.0)
    });
    let home_strength =
        0.75 * home_rating + 0.25 * ((home.squad_quality - 7.0) / 10.0).clamp(0.0, 1.0);
    let away_strength =
        0.75 * away_rating + 0.25 * ((away.squad_quality - 7.0) / 10.0).clamp(0.0, 1.0);
    let quality = (home_strength + away_strength) / 2.0;
    let balance = 1.0 - (home_strength - away_strength).abs();
    let appeal = (0.06
        + 0.32 * stakes
        + 0.22 * competition
        + 0.20 * quality
        + 0.08 * away_popularity
        + 0.07 * balance
        + if event.international { 0.04 } else { 0.0 })
    .clamp(0.0, 1.0);
    (appeal, stakes)
}

fn variation(seed: u64) -> f64 {
    let value = sample(seed);
    if value < 0.75 {
        -0.34 * (1.0 - value / 0.75).powi(3)
    } else {
        0.05 * ((value - 0.75) / 0.25).powi(2)
    }
}

pub fn calculate(
    home: TeamDemand,
    away: TeamDemand,
    venue: VenueDemand,
    event: EventDemand,
) -> AttendanceSnapshot {
    let capacity = venue.capacity.max(0) as f64;
    let home_popularity = popularity(home);
    let away_popularity = popularity(away);
    let (match_appeal, stakes) = appeal(event, home, away, home_popularity, away_popularity);
    let home_host = !event.declared_neutral
        && (home.home_venue_id == Some(venue.id) || venue.owner_team_id == Some(home.id));
    let away_host = !event.declared_neutral
        && (away.home_venue_id == Some(venue.id) || venue.owner_team_id == Some(away.id));
    let host = if home_host {
        Some(true)
    } else if away_host {
        Some(false)
    } else {
        None
    };
    let total = if let Some(home_is_host) = host {
        let team = if home_is_host { home } else { away };
        let span = (team.maximum - team.minimum).max(0) as f64;
        let base = team.minimum.max(0) as f64 + span * (0.12 + 0.72 * match_appeal);
        let effect = 1.0
            + 0.10 * team.form.clamp(-1.0, 1.0)
            + 0.10 * stakes
            + event.calendar_effect
            + variation(event.seed);
        let demand = base * effect;
        let softened = if demand > team.maximum as f64 {
            team.maximum as f64 + 0.30 * (demand - team.maximum as f64)
        } else {
            demand
        };
        softened.round().clamp(0.0, capacity) as i32
    } else {
        let high = home_popularity.max(away_popularity);
        let low = home_popularity.min(away_popularity);
        let fill = 0.16
            + 0.35 * match_appeal
            + 0.26 * high
            + 0.22 * low
            + 0.10 * stakes
            + event.calendar_effect
            + variation(event.seed);
        (capacity * fill.clamp(0.0, 1.0)).round() as i32
    };

    let (home_supporters, away_supporters) = if let Some(home_is_host) = host {
        let visitor_popularity = if home_is_host {
            away_popularity
        } else {
            home_popularity
        };
        let same_country = home.country_id == away.country_id;
        let visitor_share = (0.035
            + 0.075 * visitor_popularity
            + 0.035 * match_appeal
            + if same_country { 0.025 } else { 0.0 })
        .clamp(0.025, 0.19);
        let neutral_share = (0.035 * (1.0 - match_appeal)).clamp(0.0, 0.035);
        let visitor = (total as f64 * visitor_share).round() as i32;
        let neutral = (total as f64 * neutral_share).round() as i32;
        if home_is_host {
            (total - visitor - neutral, visitor)
        } else {
            (visitor, total - visitor - neutral)
        }
    } else {
        let neutral_share =
            0.07 * (1.0 - (home_popularity + away_popularity) / 2.0) * (1.0 - 0.7 * match_appeal);
        let identified = total - (total as f64 * neutral_share).round() as i32;
        let home_local = if venue.country_id == home.country_id {
            0.32
        } else {
            0.0
        };
        let away_local = if venue.country_id == away.country_id {
            0.32
        } else {
            0.0
        };
        let difference = 2.2 * (home_popularity - away_popularity) + home_local - away_local
            + 0.14 * (home.form - away.form);
        let home_share = 1.0 / (1.0 + (-difference).exp());
        let home_count = (identified as f64 * home_share).round() as i32;
        (home_count, identified - home_count)
    };
    AttendanceSnapshot {
        total,
        home_supporters,
        away_supporters,
        unaffiliated_spectators: total - home_supporters - away_supporters,
        match_appeal,
        home_popularity,
        away_popularity,
        model_version: ATTENDANCE_MODEL_VERSION,
    }
}
