// Responsible for application lifecycle, window management, etc.
mod option;
mod windows;

pub use windows::{WindowsName, create_gpui_window};
