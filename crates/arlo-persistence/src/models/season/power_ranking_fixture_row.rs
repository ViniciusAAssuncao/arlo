use sqlx::FromRow;

#[derive(Debug, Clone, PartialEq, Eq, FromRow)]
pub struct PowerRankingFixtureRow {
    pub fixture_id: String,
    pub played_year: i64,
    pub played_day_of_year: i64,
    pub home_team_id: String,
    pub away_team_id: String,
    pub home_score: Option<i64>,
    pub away_score: Option<i64>,
    pub neutral_venue: bool,
}
