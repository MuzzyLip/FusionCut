// Responsible for application lifecycle, window management, global state, etc.
mod option;
mod windows;

pub use windows::{WindowsName, create_gpui_window};
