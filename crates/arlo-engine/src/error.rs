#[derive(thiserror::Error, Debug)]
pub enum EngineError {
    #[error(transparent)]
    Domain(#[from] arlo_domain::DomainError),
}