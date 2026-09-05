pub mod manager;
pub mod migrations;
pub mod paths;
pub mod template;

pub use manager::{create_new_save, resolve_current_save_pool};
pub use paths::{
    save_database_url, save_filename, save_path, template_database_url, template_path,
    SAVES_DIRECTORY, TEMPLATE_DIRECTORY, TEMPLATE_FILENAME,
};
pub use template::{create_save_copy, ensure_template_database};