use arlo_domain::{DomainError, InvariantViolation};

pub fn format_domain_error(error: &DomainError) -> String {
    match error {
        DomainError::InvalidInvariant { field, violation } => {
            let message = match violation {
                InvariantViolation::Empty => "não pode ser vazio".to_string(),
                InvariantViolation::NotPositiveFinite => "deve ser positivo e finito".to_string(),
                InvariantViolation::OutOfIntegerRange { min, max } => {
                    format!("deve estar entre {min} e {max}")
                }
                InvariantViolation::OutOfFloatRange { min, max } => {
                    format!("deve estar entre {min} e {max}")
                }
                InvariantViolation::InvalidHexColor => {
                    "deve ser uma cor hexadecimal válida de 6 dígitos iniciando com #".to_string()
                }
                InvariantViolation::DuplicateKey { key_name } => {
                    format!("chave duplicada '{key_name}' encontrada")
                }
                InvariantViolation::MissingRequiredValue => "valor obrigatório ausente".to_string(),
                InvariantViolation::UnexpectedValue => "valor inesperado".to_string(),
                InvariantViolation::SelfReference => "não pode referenciar a si mesmo".to_string(),
                InvariantViolation::CountMismatch { expected, actual } => {
                    format!("esperava {expected}, mas encontrou {actual}")
                }
            };
            format!("Campo '{field}': {message}")
        }
    }
}
