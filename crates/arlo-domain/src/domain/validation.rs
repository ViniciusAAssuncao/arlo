use crate::domain::InvariantViolation;
use crate::error::{ DomainError, DomainResult };
use std::collections::HashSet;
use std::hash::Hash;

pub fn validate_not_empty(value: &str, field: &str) -> DomainResult<()> {
    if value.trim().is_empty() {
        Err(DomainError::InvalidInvariant {
            field: field.to_string(),
            violation: InvariantViolation::Empty,
        })
    } else {
        Ok(())
    }
}

pub fn validate_positive_finite(value: f64, field: &str) -> DomainResult<()> {
    if value.is_finite() && value > 0.0 {
        Ok(())
    } else {
        Err(DomainError::InvalidInvariant {
            field: field.to_string(),
            violation: InvariantViolation::NotPositiveFinite,
        })
    }
}

pub fn validate_integer_range(value: i32, min: i32, max: i32, field: &str) -> DomainResult<()> {
    if value >= min && value <= max {
        Ok(())
    } else {
        Err(DomainError::InvalidInvariant {
            field: field.to_string(),
            violation: InvariantViolation::OutOfIntegerRange { min, max },
        })
    }
}

pub fn validate_float_range(value: f64, min: f64, max: f64, field: &str) -> DomainResult<()> {
    if value >= min && value <= max {
        Ok(())
    } else {
        Err(DomainError::InvalidInvariant {
            field: field.to_string(),
            violation: InvariantViolation::OutOfFloatRange { min, max },
        })
    }
}

pub fn validate_hex_color(value: &str, field: &str) -> DomainResult<()> {
    if
        value.len() == 7 &&
        value.starts_with('#') &&
        value[1..].chars().all(|c| c.is_ascii_hexdigit())
    {
        Ok(())
    } else {
        Err(DomainError::InvalidInvariant {
            field: field.to_string(),
            violation: InvariantViolation::InvalidHexColor,
        })
    }
}

pub fn validate_no_duplicate_keys<T, K, F>(
    items: &[T],
    get_key: F,
    field: &str,
    key_name: &str
)
    -> DomainResult<()>
    where K: Eq + Hash, F: Fn(&T) -> K
{
    let mut seen = HashSet::new();
    for item in items {
        let key = get_key(item);
        if !seen.insert(key) {
            return Err(DomainError::InvalidInvariant {
                field: field.to_string(),
                violation: InvariantViolation::DuplicateKey {
                    key_name: key_name.to_string(),
                },
            });
        }
    }
    Ok(())
}

pub fn validate_exact_count<T, F>(
    items: &[T],
    predicate: F,
    expected: usize,
    field: &str
) -> DomainResult<()>
    where F: Fn(&T) -> bool
{
    let actual = items
        .iter()
        .filter(|item| predicate(item))
        .count();
    if actual != expected {
        Err(DomainError::InvalidInvariant {
            field: field.to_string(),
            violation: InvariantViolation::CountMismatch { expected, actual },
        })
    } else {
        Ok(())
    }
}
