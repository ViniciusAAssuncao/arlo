use crate::error::{DomainError, DomainResult};
use std::collections::HashSet;
use std::hash::Hash;

pub fn validate_not_empty(value: &str, field: &str) -> DomainResult<()> {
    if value.trim().is_empty() {
        Err(DomainError::InvalidInvariant {
            field: field.to_string(),
            reason: "must not be empty".to_string(),
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
            reason: "must be positive and finite".to_string(),
        })
    }
}

pub fn validate_integer_range(value: i32, min: i32, max: i32, field: &str) -> DomainResult<()> {
    if value >= min && value <= max {
        Ok(())
    } else {
        Err(DomainError::InvalidInvariant {
            field: field.to_string(),
            reason: format!("must be between {min} and {max}"),
        })
    }
}

pub fn validate_hex_color(value: &str, field: &str) -> DomainResult<()> {
    if value.len() == 7
        && value.starts_with('#')
        && value[1..].chars().all(|c| c.is_ascii_hexdigit())
    {
        Ok(())
    } else {
        Err(DomainError::InvalidInvariant {
            field: field.to_string(),
            reason: "must be a valid 6-digit hex color starting with #".to_string(),
        })
    }
}

pub fn validate_no_duplicate_keys<T, K, F>(
    items: &[T],
    get_key: F,
    field: &str,
    key_name: &str,
) -> DomainResult<()>
where
    K: Eq + Hash,
    F: Fn(&T) -> K,
{
    let mut seen = HashSet::new();
    for item in items {
        let key = get_key(item);
        if !seen.insert(key) {
            return Err(DomainError::InvalidInvariant {
                field: field.to_string(),
                reason: format!("duplicate {key_name} found"),
            });
        }
    }
    Ok(())
}
