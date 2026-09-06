use arlo_domain::ArtrineDecisionKind;
use arlo_events::ArtrineDecisionMade;
use arlo_math::Probability;
use uuid::Uuid;

pub fn translate_artrine_decision_made(
    artrine_id: Uuid,
    decision_kind: ArtrineDecisionKind,
    down_number: u32,
    decision_probability: Probability,
) -> ArtrineDecisionMade {
    ArtrineDecisionMade::new(
        artrine_id,
        decision_kind,
        down_number,
        decision_probability,
    )
}