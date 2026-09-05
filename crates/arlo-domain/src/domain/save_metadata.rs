use crate::domain::validation::validate_not_empty;
use crate::error::DomainResult;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SaveMetadata {
    save_uuid: Uuid,
    created_at_unix_seconds: i64,
    source_template_path: String,
}

impl SaveMetadata {
    pub fn new(
        save_uuid: Uuid,
        created_at_unix_seconds: i64,
        source_template_path: impl Into<String>,
    ) -> DomainResult<Self> {
        let source_template_path = source_template_path.into();
        validate_not_empty(&source_template_path, "source_template_path")?;

        Ok(Self {
            save_uuid,
            created_at_unix_seconds,
            source_template_path,
        })
    }

    pub fn save_uuid(&self) -> Uuid {
        self.save_uuid
    }

    pub fn created_at_unix_seconds(&self) -> i64 {
        self.created_at_unix_seconds
    }

    pub fn source_template_path(&self) -> &str {
        &self.source_template_path
    }
}