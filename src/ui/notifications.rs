//! Notification system

use egui::{Color32, Context, Window};
use std::time::{Duration, Instant};

pub struct NotificationManager {
    notifications: Vec<Notification>,
}

#[derive(Clone)]
pub struct Notification {
    pub id: uuid::Uuid,
    pub message: String,
    pub level: NotificationLevel,
    pub created_at: Instant,
    pub duration: Duration,
}

#[derive(Clone, Copy, PartialEq)]
pub enum NotificationLevel {
    Info,
    Success,
    Warning,
    Error,
}

impl NotificationManager {
    pub fn new() -> Self {
        Self {
            notifications: Vec::new(),
        }
    }

    pub fn info(&mut self, message: impl Into<String>) {
        self.add(message.into(), NotificationLevel::Info);
    }

    pub fn success(&mut self, message: impl Into<String>) {
        self.add(message.into(), NotificationLevel::Success);
    }

    pub fn warning(&mut self, message: impl Into<String>) {
        self.add(message.into(), NotificationLevel::Warning);
    }

    pub fn error(&mut self, message: impl Into<String>) {
        self.add(message.into(), NotificationLevel::Error);
    }

    fn add(&mut self, message: String, level: NotificationLevel) {
        self.notifications.push(Notification {
            id: uuid::Uuid::new_v4(),
            message,
            level,
            created_at: Instant::now(),
            duration: Duration::from_secs(3),
        });
    }

    pub fn render(&mut self, ctx: &Context) {
        // Remove expired notifications
        self.notifications
            .retain(|n| n.created_at.elapsed() < n.duration);

        // Show active notifications
        for (idx, notification) in self.notifications.iter().enumerate() {
            let pos = egui::pos2(
                ctx.content_rect().width() - 320.0,
                10.0 + (idx as f32 * 70.0),
            );

            Window::new(format!("notification_{}", notification.id))
                .title_bar(false)
                .resizable(false)
                .fixed_pos(pos)
                .show(ctx, |ui| {
                    let (icon, color) = match notification.level {
                        NotificationLevel::Info => ("ℹ", Color32::LIGHT_BLUE),
                        NotificationLevel::Success => ("✓", Color32::GREEN),
                        NotificationLevel::Warning => ("⚠", Color32::YELLOW),
                        NotificationLevel::Error => ("✖", Color32::RED),
                    };

                    ui.horizontal(|ui| {
                        ui.colored_label(color, icon);
                        ui.label(&notification.message);
                    });
                });
        }
    }
}

impl Default for NotificationManager {
    fn default() -> Self {
        Self::new()
    }
}

// `render` requires a live `egui::Context` to draw `Window`s and is not
// exercised here; the queue-management logic (add/info/success/warning/error)
// is pure and fully covered below.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_manager_has_empty_queue() {
        let manager = NotificationManager::new();
        assert!(manager.notifications.is_empty());
    }

    #[test]
    fn test_default_matches_new() {
        let manager = NotificationManager::default();
        assert!(manager.notifications.is_empty());
    }

    #[test]
    fn test_info_adds_notification_with_correct_level() {
        let mut manager = NotificationManager::new();
        manager.info("hello");

        assert_eq!(manager.notifications.len(), 1);
        assert_eq!(manager.notifications[0].message, "hello");
        assert!(manager.notifications[0].level == NotificationLevel::Info);
    }

    #[test]
    fn test_success_adds_notification_with_correct_level() {
        let mut manager = NotificationManager::new();
        manager.success("done");

        assert_eq!(manager.notifications.len(), 1);
        assert!(manager.notifications[0].level == NotificationLevel::Success);
    }

    #[test]
    fn test_warning_adds_notification_with_correct_level() {
        let mut manager = NotificationManager::new();
        manager.warning("careful");

        assert_eq!(manager.notifications.len(), 1);
        assert!(manager.notifications[0].level == NotificationLevel::Warning);
    }

    #[test]
    fn test_error_adds_notification_with_correct_level() {
        let mut manager = NotificationManager::new();
        manager.error("failed");

        assert_eq!(manager.notifications.len(), 1);
        assert!(manager.notifications[0].level == NotificationLevel::Error);
    }

    #[test]
    fn test_multiple_notifications_queue_in_order() {
        let mut manager = NotificationManager::new();
        manager.info("first");
        manager.warning("second");
        manager.error("third");

        assert_eq!(manager.notifications.len(), 3);
        assert_eq!(manager.notifications[0].message, "first");
        assert_eq!(manager.notifications[1].message, "second");
        assert_eq!(manager.notifications[2].message, "third");
    }

    #[test]
    fn test_notifications_get_unique_ids() {
        let mut manager = NotificationManager::new();
        manager.info("a");
        manager.info("b");

        assert_ne!(manager.notifications[0].id, manager.notifications[1].id);
    }

    #[test]
    fn test_notification_has_default_three_second_duration() {
        let mut manager = NotificationManager::new();
        manager.info("timed");

        assert_eq!(manager.notifications[0].duration, Duration::from_secs(3));
    }

    #[test]
    fn test_empty_message_is_still_queued() {
        let mut manager = NotificationManager::new();
        manager.info("");

        assert_eq!(manager.notifications.len(), 1);
        assert_eq!(manager.notifications[0].message, "");
    }

    #[test]
    fn test_notification_level_equality() {
        assert!(NotificationLevel::Info == NotificationLevel::Info);
        assert!(NotificationLevel::Info != NotificationLevel::Error);
        assert!(NotificationLevel::Success != NotificationLevel::Warning);
    }
}
