pub mod app;
pub mod controls;
pub mod enhanced_piano_roll;
pub mod horizontal_keyboard;
pub mod lego_mode;
pub mod notation;
pub mod piano_roll;
pub mod symbols;
pub mod timeline;

pub use app::{App, AppMode};
pub use controls::handle_input;
pub use enhanced_piano_roll::EnhancedPianoRoll;
pub use horizontal_keyboard::HorizontalKeyboard;
pub use lego_mode::LegoModeState;
pub use notation::render_notation;
pub use timeline::TimelineState;
