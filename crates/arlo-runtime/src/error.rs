#[derive(thiserror::Error, Debug)]
pub enum RuntimeError {
    #[error(transparent)]
    Db(#[from] arlo_db::DbError),
    #[error(transparent)]
    Engine(#[from] arlo_engine::error::EngineError),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
