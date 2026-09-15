pub mod manager;
pub mod migrations;
pub mod paths;
pub mod template;

pub use manager::{create_new_save, create_new_save_from_template, resolve_current_save_pool};
pub use paths::{
    list_existing_saves, list_existing_templates, most_recent_save, project_root_dir,
    save_database_url, save_filename, save_path, saves_dir, template_database_url,
    template_database_url_for, template_dir, template_path, template_path_for,
    TEMPLATE_FILENAME,
};
pub use template::{create_save_copy, create_save_copy_from_path, ensure_template_database};