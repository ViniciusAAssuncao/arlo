use super::context::ManagerDecisionContext;
use uuid::Uuid;

pub(super) fn sample(context: &ManagerDecisionContext, subject: Uuid, stream: u64) -> f64 {
    let mut value = context.seed ^ stream;
    for id in [context.team_id, context.manager_id, subject] {
        let bits = id.as_u128();
        value = mix(value ^ bits as u64);
        value = mix(value ^ (bits >> 64) as u64);
    }
    (value >> 11) as f64 / (1_u64 << 53) as f64
}

fn mix(mut value: u64) -> u64 {
    value = value.wrapping_add(0x9e3779b97f4a7c15);
    value = (value ^ (value >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    value = (value ^ (value >> 27)).wrapping_mul(0x94d049bb133111eb);
    value ^ (value >> 31)
}
