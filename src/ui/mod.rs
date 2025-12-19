//! User interface

pub mod app_state;
pub mod components;
// pub mod dialogs;  // TODO: Create dialogs module
pub mod keyboard;
pub mod notifications;
pub mod screens;
pub mod search;

pub use app_state::AppState;
pub use keyboard::{KeyboardHandler, KeyboardAction};
pub use notifications::NotificationManager;
pub use search::SearchWidget;
