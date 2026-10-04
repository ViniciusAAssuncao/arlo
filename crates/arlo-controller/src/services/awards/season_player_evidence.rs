use crate::error::ControllerResult;
use arlo_domain::{AwardCandidateEvidence, AwardMetric, AwardRecipientKind};
use sqlx::{FromRow, SqlitePool};
use std::collections::BTreeMap;
use uuid::Uuid;

#[derive(FromRow)]
struct SeasonPlayerRow {
    player_id: String,
    team_id: String,
    nationality_id: String,
    continent_id: String,
    offensive_position: String,
    seconds_played: f64,
    performance_rating: f64,
    final_rating: f64,
    high_impact_score: f64,
    production_score: f64,
    defense_score: f64,
    discipline_score: f64,
    total_points_scored: i64,
    goal_points_scored: i64,
    field_points_scored: i64,
    field_goals_scored: i64,
}

struct PlayerTotals {
    country_id: Uuid,
    continent_id: Uuid,
    matches_played: u32,
    position_seconds: BTreeMap<String, f64>,
    team_seconds: BTreeMap<Uuid, f64>,
    performance_rating: f64,
    final_rating: f64,
    high_impact: f64,
    production: f64,
    defense: f64,
    discipline: f64,
    total_points: i64,
    goal_points: i64,
    field_points: i64,
    field_goals: i64,
}

impl PlayerTotals {
    fn new(country_id: Uuid, continent_id: Uuid) -> Self {
        Self {
            country_id,
            continent_id,
            matches_played: 0,
            position_seconds: BTreeMap::new(),
            team_seconds: BTreeMap::new(),
            performance_rating: 0.0,
            final_rating: 0.0,
            high_impact: 0.0,
            production: 0.0,
            defense: 0.0,
            discipline: 0.0,
            total_points: 0,
            goal_points: 0,
            field_points: 0,
            field_goals: 0,
        }
    }

    fn add(&mut self, row: &SeasonPlayerRow) -> ControllerResult<()> {
        self.matches_played += 1;
        *self
            .position_seconds
            .entry(row.offensive_position.clone())
            .or_default() += row.seconds_played;
        *self
            .team_seconds
            .entry(Uuid::parse_str(&row.team_id)?)
            .or_default() += row.seconds_played;
        self.performance_rating += row.performance_rating;
        self.final_rating += row.final_rating;
        self.high_impact += row.high_impact_score;
        self.production += row.production_score;
        self.defense += row.defense_score;
        self.discipline += row.discipline_score;
        self.total_points += row.total_points_scored;
        self.goal_points += row.goal_points_scored;
        self.field_points += row.field_points_scored;
        self.field_goals += row.field_goals_scored;
        Ok(())
    }

    fn evidence(self, player_id: Uuid, competition_id: Uuid) -> AwardCandidateEvidence {
        let matches = f64::from(self.matches_played);
        let position = self
            .position_seconds
            .into_iter()
            .max_by(|a, b| a.1.total_cmp(&b.1).then_with(|| b.0.cmp(&a.0)))
            .map(|(position, _)| position);
        let position_family = position
            .as_deref()
            .and_then(position_family)
            .map(str::to_string);
        let team_id = self
            .team_seconds
            .into_iter()
            .max_by(|a, b| a.1.total_cmp(&b.1).then_with(|| b.0.cmp(&a.0)))
            .map(|(id, _)| id);
        AwardCandidateEvidence {
            subject_kind: AwardRecipientKind::Player,
            subject_id: player_id,
            position,
            position_family,
            country_id: Some(self.country_id),
            continent_id: Some(self.continent_id),
            competition_id: Some(competition_id),
            team_id,
            matches_played: self.matches_played,
            metrics: vec![
                metric("matches_played", matches),
                metric("total_points_scored", self.total_points as f64),
                metric("goal_points_scored", self.goal_points as f64),
                metric("field_points_scored", self.field_points as f64),
                metric("field_goals_scored", self.field_goals as f64),
                metric(
                    "average_performance_rating",
                    self.performance_rating / matches,
                ),
                metric("average_final_rating", self.final_rating / matches),
                metric("average_high_impact", self.high_impact / matches),
                metric("average_production", self.production / matches),
                metric("average_defense", self.defense / matches),
                metric("average_discipline", self.discipline / matches),
            ],
        }
    }
}

pub(crate) async fn load_season_player_evidence(
    pool: &SqlitePool,
    season_instance_id: Uuid,
    competition_id: Uuid,
) -> ControllerResult<Vec<AwardCandidateEvidence>> {
    let rows = sqlx::query_as::<_, SeasonPlayerRow>(
        "SELECT perf.player_id, perf.team_id, p.nationality_id, c.continent_id, perf.offensive_position, perf.seconds_played, perf.performance_rating, perf.final_rating, perf.high_impact_score, perf.production_score, perf.defense_score, perf.discipline_score, COALESCE(s.total_points_scored, 0) AS total_points_scored, COALESCE(s.goal_points_scored, 0) AS goal_points_scored, COALESCE(s.field_points_scored, 0) AS field_points_scored, COALESCE(s.field_goals_scored, 0) AS field_goals_scored FROM match_player_performance perf JOIN matches m ON m.id = perf.match_id JOIN fixtures f ON f.id = m.fixture_id JOIN season_stages ss ON ss.id = f.season_stage_id JOIN players p ON p.id = perf.player_id JOIN countries c ON c.id = p.nationality_id LEFT JOIN match_player_scoring_attempts s ON s.match_id = perf.match_id AND s.player_id = perf.player_id WHERE ss.season_instance_id = ? AND perf.effective_opportunities > 0 AND perf.confidence_evidence > 0.0 ORDER BY perf.player_id, perf.match_id"
    )
    .bind(season_instance_id.to_string())
    .fetch_all(pool).await?;
    let mut totals = BTreeMap::<Uuid, PlayerTotals>::new();
    for row in rows {
        let player_id = Uuid::parse_str(&row.player_id)?;
        let country_id = Uuid::parse_str(&row.nationality_id)?;
        let continent_id = Uuid::parse_str(&row.continent_id)?;
        totals
            .entry(player_id)
            .or_insert_with(|| PlayerTotals::new(country_id, continent_id))
            .add(&row)?;
    }
    Ok(totals
        .into_iter()
        .map(|(id, total)| total.evidence(id, competition_id))
        .collect())
}

fn metric(key: &str, value: f64) -> AwardMetric {
    AwardMetric {
        key: key.into(),
        value,
    }
}

fn position_family(position: &str) -> Option<&'static str> {
    match position {
        "CenterOffense" | "WingOffense" | "Midcenter" | "TightWing" | "CenterTight"
        | "Corridor" => Some("OffensiveLine"),
        "Artrine" | "Passer" | "PassRusher" | "WideEnd" | "RunningEnd" | "Lineback"
        | "Fullback" => Some("BackLine"),
        "Centerback" | "DefensiveEnd" | "Rougieback" | "DefensiveBlocker" | "WideBlocker"
        | "OutsideZonerback" | "MiddleZonerback" => Some("DefenseLine"),
        "Goalguard" => Some("Goalguard"),
        _ => None,
    }
}
