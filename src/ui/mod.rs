//! UI module - egui-based user interface components

pub mod components;
pub mod tab;
mod tab_manager;
pub mod screens;

pub use tab::{Tab, TabStatus};
pub use tab_manager::TabManager;
