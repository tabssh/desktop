//! Notification system

use egui::{Context, Window, Color32};
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
        self.notifications.retain(|n| n.created_at.elapsed() < n.duration);
        
        // Show active notifications
        for (idx, notification) in self.notifications.iter().enumerate() {
            let pos = egui::pos2(
                ctx.screen_rect().width() - 320.0,
                10.0 + (idx as f32 * 70.0),
            );
            
            Window::new(format!("notification_{}",notification.id))
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
