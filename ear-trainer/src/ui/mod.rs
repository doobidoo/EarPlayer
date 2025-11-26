pub mod app;
pub mod controls;
pub mod notation;
pub mod piano_roll;

pub use app::{App, AppMode};
pub use controls::handle_input;
pub use notation::render_notation;
pub use piano_roll::render_piano_roll;
