use crate::error::TacticsResult;
use crate::persistence::models::decision_kind_code::parse_artrine_decision_kind;
use crate::playcall::decision_emphasis::DecisionEmphasis;
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct PlayCallDecisionEmphasisRow {
    pub id: String,
    pub play_call_id: String,
    pub decision_kind: String,
    pub weight: f64,
}

pub fn build_decision_emphasis(
    rows: &[PlayCallDecisionEmphasisRow],
) -> TacticsResult<DecisionEmphasis> {
    let mut emphasis = DecisionEmphasis::default();
    for row in rows {
        let kind = parse_artrine_decision_kind(&row.decision_kind)?;
        emphasis = emphasis.with_emphasis(kind, row.weight);
    }
    Ok(emphasis)
}
