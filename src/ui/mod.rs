//! User interface

pub mod app_state;
pub mod components;
pub mod keyboard;
pub mod main_window;
pub mod notifications;
pub mod screens;
pub mod search;
pub mod tab;
pub mod tab_manager;

pub use app_state::AppState;
pub use keyboard::{KeyboardHandler, KeyboardAction};
pub use main_window::MainWindow;
pub use notifications::NotificationManager;
pub use search::SearchWidget;
pub use tab::{Tab, TabStatus};
pub use tab_manager::TabManager;
