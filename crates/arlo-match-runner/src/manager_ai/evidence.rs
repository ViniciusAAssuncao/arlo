use super::context::PerformanceEvidence;
use arlo_analytics::PlayerPerformanceAggregator;
use uuid::Uuid;

pub(super) fn read_evidence(
    analytics: Option<&PlayerPerformanceAggregator>,
    player_id: Uuid,
) -> PerformanceEvidence {
    let Some(analytics) = analytics else {
        return PerformanceEvidence::default();
    };
    let Some(snapshot) = analytics.current_player_snapshot(&player_id) else {
        return PerformanceEvidence::default();
    };
    let opportunities = snapshot
        .diagnostics()
        .effective_opportunity_weight()
        .max(f64::from(snapshot.effective_opportunities()));
    let denominator = opportunities.max(4.0);
    let breakdown = snapshot.breakdown();
    let signal = |value: f64| (value / denominator / 0.30).tanh();
    PerformanceEvidence {
        deviation: ((snapshot.performance_rating().value() - analytics.config().baseline_rating())
            / 3.0)
            .clamp(-1.0, 1.0),
        confidence: snapshot.confidence().value(),
        seconds_played: snapshot.seconds_played(),
        opportunities,
        execution: signal(breakdown.execution()),
        production: signal(breakdown.production()),
        defense: signal(breakdown.defense()),
        security: signal(breakdown.ball_security()),
        discipline: signal(breakdown.discipline()),
    }
}
