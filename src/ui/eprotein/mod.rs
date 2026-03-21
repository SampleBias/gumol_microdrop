//! eProtein Discovery workflow panels (Nuclera-aligned).

pub mod design;
pub mod screen_palette;
pub mod placeholders;

pub use design::render_construct_design;
pub use screen_palette::render_screen_matrix;
pub use placeholders::{render_export_protocol, render_results, render_scale_up};
