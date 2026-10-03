use crate::Result;
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::path::Path;

pub fn fingerprint(value: &Value) -> String {
    format!(
        "{:x}",
        Sha256::digest(serde_json::to_vec(value).expect("JSON value"))
    )
}

pub fn compare(expected: &Value, actual: &Value) -> bool {
    match (expected, actual) {
        (Value::Number(left), Value::Number(right)) if left.is_f64() || right.is_f64() => {
            (left.as_f64().unwrap() - right.as_f64().unwrap()).abs() <= 1e-10
        }
        (Value::Array(left), Value::Array(right)) => {
            left.len() == right.len()
                && left
                    .iter()
                    .zip(right)
                    .all(|(left, right)| compare(left, right))
        }
        (Value::Object(left), Value::Object(right)) => {
            left.len() == right.len()
                && left
                    .iter()
                    .all(|(key, left)| right.get(key).is_some_and(|right| compare(left, right)))
        }
        _ => expected == actual,
    }
}

pub fn verify(path: &Path, actual: &Value) -> Result<()> {
    if path.exists() {
        let expected = serde_json::from_slice(&std::fs::read(path)?)?;
        if !compare(&expected, actual) {
            std::fs::write(
                path.with_extension("actual.json"),
                serde_json::to_vec(actual)?,
            )?;
            return Err(format!("Functional divergence {}", path.display()).into());
        }
    } else {
        std::fs::write(path, serde_json::to_vec(actual)?)?;
    }
    Ok(())
}

#[cfg(not(feature = "pre-epic"))]
pub fn assessment(value: Option<&arlo_match_runner::manager_ai::ManagerAssessment>) -> Value {
    let Some(value) = value else {
        return Value::Null;
    };
    let diagnosis = value.diagnosis;
    serde_json::json!({
        "diagnosis":[diagnosis.attack,diagnosis.defense,diagnosis.control,diagnosis.security,diagnosis.energy,diagnosis.discipline,diagnosis.collective_attack],
        "aspiration":value.aspiration,
        "selected":format!("{:?}",value.selected()),
        "candidates":value.candidates.iter().map(|candidate| serde_json::json!([format!("{:?}",candidate.action),candidate.utility])).collect::<Vec<_>>(),
        "substitutions":value.substitutions.iter().map(|candidate| serde_json::json!([format!("{:?}",candidate.intent),format!("{:?}",candidate.reason),candidate.utility,candidate.contribution_gain,candidate.individual_need,candidate.fatigue_need,candidate.change_cost])).collect::<Vec<_>>(),
        "realignments":value.realignments.iter().map(|candidate| serde_json::json!([format!("{:?}",candidate.intent),candidate.utility,candidate.contribution_gain,candidate.tactical_gain,candidate.change_cost])).collect::<Vec<_>>(),
        "plans":value.prepared_plans.iter().map(|candidate| serde_json::json!([format!("{:?}",candidate.intent),candidate.utility,candidate.collective_gain,candidate.fit_gain,candidate.change_cost,candidate.changed_roles,candidate.formation_changed])).collect::<Vec<_>>(),
    })
}
