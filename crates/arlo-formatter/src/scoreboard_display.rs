#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScoreBreakdown {
    pub tier1: u32,
    pub tier2: u32,
    pub tier3: u32,
    pub total_points: u32,
}

impl ScoreBreakdown {
    pub fn new(tier1: u32, tier2: u32, tier3: u32, total_points: u32) -> Self {
        Self {
            tier1,
            tier2,
            tier3,
            total_points,
        }
    }
}

pub fn format_score(tier1: u32, tier2: u32, tier3: u32, total_points: u32) -> String {
    format!("{tier1}–{tier2}–{tier3} ({total_points})")
}

pub fn format_score_breakdown(score: &ScoreBreakdown) -> String {
    format_score(score.tier1, score.tier2, score.tier3, score.total_points)
}

pub fn format_match_result(
    home_name: &str,
    home_score: &ScoreBreakdown,
    away_name: &str,
    away_score: &ScoreBreakdown,
) -> String {
    format!(
        "{home_name} {} x {} {away_name}",
        format_score_breakdown(home_score),
        format_score_breakdown(away_score)
    )
}