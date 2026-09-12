pub mod fault_catalog_cache;
pub mod injury_catalog_cache;

pub use fault_catalog_cache::get_or_load_fault_catalog;
pub use injury_catalog_cache::get_or_load_injury_catalog;