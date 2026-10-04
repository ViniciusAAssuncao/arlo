use super::season_age_cutoff::season_start_unix_seconds;
use crate::domain::calendar::CalendarDate;
use crate::error::ControllerResult;
use arlo_domain::{award_position_family, AwardCandidateEvidence, AwardMetric, AwardRecipientKind};
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
    defensive_position: String,
    birthdate_unix_seconds: i64,
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
    goalpoint_assists: i64,
    passes_attempted: i64,
    passes_received: i64,
    recoveries: i64,
    turnovers_conceded: i64,
    defender_wins: i64,
    defender_duels: i64,
    kicker_points: i64,
    goalguard_recoveries: i64,
    goalguard_hand_recoveries: i64,
}

struct PlayerTotals {
    age_years: Option<u32>,
    country_id: Uuid,
    continent_id: Uuid,
    matches_played: u32,
    position_seconds: BTreeMap<String, f64>,
    defensive_position_seconds: BTreeMap<String, f64>,
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
    goalpoint_assists: i64,
    passes_attempted: i64,
    passes_received: i64,
    recoveries: i64,
    turnovers_conceded: i64,
    defender_wins: i64,
    defender_duels: i64,
    kicker_points: i64,
    goalguard_recoveries: i64,
    goalguard_hand_recoveries: i64,
}

impl PlayerTotals {
    fn new(country_id: Uuid, continent_id: Uuid, age_years: Option<u32>) -> Self {
        Self {
            age_years,
            country_id,
            continent_id,
            matches_played: 0,
            position_seconds: BTreeMap::new(),
            defensive_position_seconds: BTreeMap::new(),
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
            goalpoint_assists: 0,
            passes_attempted: 0,
            passes_received: 0,
            recoveries: 0,
            turnovers_conceded: 0,
            defender_wins: 0,
            defender_duels: 0,
            kicker_points: 0,
            goalguard_recoveries: 0,
            goalguard_hand_recoveries: 0,
        }
    }

    fn add(&mut self, row: &SeasonPlayerRow) -> ControllerResult<()> {
        self.matches_played += 1;
        *self
            .position_seconds
            .entry(row.offensive_position.clone())
            .or_default() += row.seconds_played;
        *self
            .defensive_position_seconds
            .entry(row.defensive_position.clone())
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
        self.goalpoint_assists += row.goalpoint_assists;
        self.passes_attempted += row.passes_attempted;
        self.passes_received += row.passes_received;
        self.recoveries += row.recoveries;
        self.turnovers_conceded += row.turnovers_conceded;
        self.defender_wins += row.defender_wins;
        self.defender_duels += row.defender_duels;
        self.kicker_points += row.kicker_points;
        self.goalguard_recoveries += row.goalguard_recoveries;
        self.goalguard_hand_recoveries += row.goalguard_hand_recoveries;
        Ok(())
    }

    fn evidence(self, player_id: Uuid, competition_id: Uuid) -> AwardCandidateEvidence {
        let matches = f64::from(self.matches_played);
        let position = self
            .position_seconds
            .into_iter()
            .max_by(|a, b| a.1.total_cmp(&b.1).then_with(|| b.0.cmp(&a.0)))
            .map(|(position, _)| position);
        let defensive_position = self
            .defensive_position_seconds
            .into_iter()
            .max_by(|a, b| a.1.total_cmp(&b.1).then_with(|| b.0.cmp(&a.0)))
            .map(|(position, _)| position);
        let positions = position
            .iter()
            .chain(defensive_position.iter())
            .cloned()
            .collect();
        let position_family = position
            .as_deref()
            .and_then(award_position_family)
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
            positions,
            position_family,
            country_id: Some(self.country_id),
            continent_id: Some(self.continent_id),
            competition_id: Some(competition_id),
            competition_ids: vec![competition_id],
            team_id,
            matches_played: self.matches_played,
            age_years: self.age_years,
            metrics: vec![
                metric("matches_played", matches),
                metric("total_points_scored", self.total_points as f64),
                metric("goal_points_scored", self.goal_points as f64),
                metric("field_points_scored", self.field_points as f64),
                metric("field_goals_scored", self.field_goals as f64),
                metric("goalpoint_assists", self.goalpoint_assists as f64),
                metric("passes_attempted", self.passes_attempted as f64),
                metric("passes_received", self.passes_received as f64),
                metric("recoveries", self.recoveries as f64),
                metric("turnovers_conceded", self.turnovers_conceded as f64),
                metric("defender_wins", self.defender_wins as f64),
                metric("defender_duels", self.defender_duels as f64),
                metric("kicker_points", self.kicker_points as f64),
                metric("goalguard_recoveries", self.goalguard_recoveries as f64),
                metric(
                    "goalguard_hand_recoveries",
                    self.goalguard_hand_recoveries as f64,
                ),
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
    load_season_player_evidence_between(
        pool,
        season_instance_id,
        competition_id,
        CalendarDate::new(i64::MIN, 0),
        CalendarDate::new(i64::MAX, u32::MAX),
        season_instance_id,
    )
    .await
}

pub(crate) async fn load_season_player_evidence_between(
    pool: &SqlitePool,
    season_instance_id: Uuid,
    competition_id: Uuid,
    start: CalendarDate,
    end: CalendarDate,
    age_cutoff_season_id: Uuid,
) -> ControllerResult<Vec<AwardCandidateEvidence>> {
    let rows = sqlx::query_as::<_, SeasonPlayerRow>(
        "SELECT perf.player_id, perf.team_id, p.nationality_id, c.continent_id, p.birthdate_unix_seconds, perf.offensive_position, perf.defensive_position, perf.seconds_played, perf.performance_rating, perf.final_rating, perf.high_impact_score, perf.production_score, perf.defense_score, perf.discipline_score, COALESCE(s.total_points_scored, 0) AS total_points_scored, COALESCE(s.goal_points_scored, 0) AS goal_points_scored, COALESCE(s.field_points_scored, 0) AS field_points_scored, COALESCE(s.field_goals_scored, 0) AS field_goals_scored, COALESCE(a.goalpoint_assists, 0) AS goalpoint_assists, COALESCE(t.passes_attempted, 0) AS passes_attempted, COALESCE(t.passes_received, 0) AS passes_received, COALESCE(t.recoveries, 0) AS recoveries, COALESCE(t.turnovers_conceded, 0) AS turnovers_conceded, COALESCE(d.defender_wins, 0) AS defender_wins, COALESCE(d.defender_duels, 0) AS defender_duels, COALESCE(k.points, 0) AS kicker_points, COALESCE(g.recoveries, 0) AS goalguard_recoveries, COALESCE(g.hand_recoveries, 0) AS goalguard_hand_recoveries FROM match_player_performance perf JOIN matches m ON m.id = perf.match_id JOIN fixtures f ON f.id = m.fixture_id JOIN season_stages ss ON ss.id = f.season_stage_id JOIN players p ON p.id = perf.player_id JOIN countries c ON c.id = p.nationality_id LEFT JOIN match_player_scoring_attempts s ON s.match_id = perf.match_id AND s.player_id = perf.player_id LEFT JOIN match_player_assists a ON a.match_id = perf.match_id AND a.player_id = perf.player_id LEFT JOIN match_player_touches t ON t.match_id = perf.match_id AND t.player_id = perf.player_id LEFT JOIN match_player_duels d ON d.match_id = perf.match_id AND d.player_id = perf.player_id LEFT JOIN (SELECT match_id, scorer_id, SUM(points) AS points FROM match_scoring_plays sp WHERE sp.play_type IN ('FieldPoint', 'FieldGoal') AND NOT EXISTS (SELECT 1 FROM match_play_invalidations i WHERE i.match_id = sp.match_id AND sp.sequence_number BETWEEN i.first_invalidated_sequence AND i.last_invalidated_sequence) GROUP BY sp.match_id, sp.scorer_id) k ON k.match_id = perf.match_id AND k.scorer_id = perf.player_id LEFT JOIN (SELECT gr.match_id, gr.goalguard_id, COUNT(*) AS recoveries, SUM(gr.used_hands) AS hand_recoveries FROM match_goalguard_recoveries gr WHERE NOT EXISTS (SELECT 1 FROM match_play_invalidations i WHERE i.match_id = gr.match_id AND gr.sequence_number BETWEEN i.first_invalidated_sequence AND i.last_invalidated_sequence) GROUP BY gr.match_id, gr.goalguard_id) g ON g.match_id = perf.match_id AND g.goalguard_id = perf.player_id WHERE ss.season_instance_id = ? AND perf.effective_opportunities > 0 AND perf.confidence_evidence > 0.0 AND (f.scheduled_year > ? OR (f.scheduled_year = ? AND f.scheduled_day_of_year >= ?)) AND (f.scheduled_year < ? OR (f.scheduled_year = ? AND f.scheduled_day_of_year <= ?)) ORDER BY perf.player_id, perf.match_id"
    )
    .bind(season_instance_id.to_string())
    .bind(start.year())
    .bind(start.year())
    .bind(i64::from(start.day_of_year()))
    .bind(end.year())
    .bind(end.year())
    .bind(i64::from(end.day_of_year()))
    .fetch_all(pool).await?;
    if rows.is_empty() {
        return Ok(Vec::new());
    }
    let season_start = season_start_unix_seconds(pool, age_cutoff_season_id).await?;
    let mut totals = BTreeMap::<Uuid, PlayerTotals>::new();
    for row in rows {
        let player_id = Uuid::parse_str(&row.player_id)?;
        let country_id = Uuid::parse_str(&row.nationality_id)?;
        let continent_id = Uuid::parse_str(&row.continent_id)?;
        totals
            .entry(player_id)
            .or_insert_with(|| {
                let age_years = season_start
                    .checked_sub(row.birthdate_unix_seconds)
                    .and_then(|seconds| u32::try_from(seconds / 31_557_600).ok());
                PlayerTotals::new(country_id, continent_id, age_years)
            })
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
