//! Port forwarding management screen

use crate::ssh::{ForwardType, PortForward};
use egui::{Context, Ui};

pub struct ForwardingScreen {
    forwards: Vec<PortForward>,
    edit_local_port: String,
    edit_remote_host: String,
    edit_remote_port: String,
    forward_type: ForwardType,
}

impl ForwardingScreen {
    pub fn new() -> Self {
        Self {
            forwards: Vec::new(),
            edit_local_port: "8080".to_string(),
            edit_remote_host: "localhost".to_string(),
            edit_remote_port: "80".to_string(),
            forward_type: ForwardType::Local,
        }
    }

    pub fn render(&mut self, _ctx: &Context, ui: &mut Ui) -> Option<ForwardingAction> {
        let mut action = None;

        ui.heading("Port Forwarding");
        ui.separator();

        // Add new forward
        ui.group(|ui| {
            ui.label("Add New Forward");

            ui.horizontal(|ui| {
                ui.label("Type:");
                ui.radio_value(&mut self.forward_type, ForwardType::Local, "Local (-L)");
                ui.radio_value(&mut self.forward_type, ForwardType::Remote, "Remote (-R)");
                ui.radio_value(&mut self.forward_type, ForwardType::Dynamic, "Dynamic (-D)");
            });

            if matches!(self.forward_type, ForwardType::Dynamic) {
                ui.horizontal(|ui| {
                    ui.label("Listen port:");
                    ui.text_edit_singleline(&mut self.edit_local_port);
                });
            } else {
                ui.horizontal(|ui| {
                    ui.label("Local port:");
                    ui.text_edit_singleline(&mut self.edit_local_port);
                    ui.label("→");
                    ui.text_edit_singleline(&mut self.edit_remote_host);
                    ui.label(":");
                    ui.text_edit_singleline(&mut self.edit_remote_port);
                });
            }

            if ui.button("➕ Add Forward").clicked() {
                if let Ok(local_port) = self.edit_local_port.parse::<u16>() {
                    let forward = match self.forward_type {
                        ForwardType::Local => {
                            if let Ok(remote_port) = self.edit_remote_port.parse::<u16>() {
                                Some(PortForward::new_local(
                                    local_port,
                                    self.edit_remote_host.clone(),
                                    remote_port,
                                ))
                            } else {
                                None
                            }
                        }
                        ForwardType::Remote => {
                            if let Ok(remote_port) = self.edit_remote_port.parse::<u16>() {
                                Some(PortForward::new_remote(
                                    local_port,
                                    self.edit_remote_host.clone(),
                                    remote_port,
                                ))
                            } else {
                                None
                            }
                        }
                        ForwardType::Dynamic => Some(PortForward::new_dynamic(local_port)),
                    };

                    if let Some(fwd) = forward {
                        action = Some(ForwardingAction::Add(fwd));
                    }
                }
            }
        });

        ui.separator();

        // List existing forwards
        ui.heading("Active Forwards");

        let mut to_remove = None;

        egui::ScrollArea::vertical().show(ui, |ui| {
            for (idx, forward) in self.forwards.iter().enumerate() {
                ui.horizontal(|ui| {
                    let type_icon = match forward.forward_type {
                        ForwardType::Local => "📥",
                        ForwardType::Remote => "📤",
                        ForwardType::Dynamic => "🔄",
                    };

                    ui.label(type_icon);

                    let status = if forward.active { "🟢" } else { "🔴" };
                    ui.label(status);

                    let desc = match forward.forward_type {
                        ForwardType::Dynamic => {
                            format!("SOCKS proxy on: {}", forward.listen_port)
                        }
                        _ => {
                            format!(
                                ":{} → {}:{}",
                                forward.listen_port, forward.remote_host, forward.remote_port
                            )
                        }
                    };

                    ui.label(desc);

                    if ui.button("🗑 Remove").clicked() {
                        to_remove = Some(idx);
                    }
                });
            }
        });

        if let Some(idx) = to_remove {
            if idx < self.forwards.len() {
                let forward = self.forwards.remove(idx);
                action = Some(ForwardingAction::Remove(forward.id));
            }
        }

        action
    }

    pub fn set_forwards(&mut self, forwards: Vec<PortForward>) {
        self.forwards = forwards;
    }
}

impl Default for ForwardingScreen {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub enum ForwardingAction {
    Add(PortForward),
    Remove(uuid::Uuid),
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Render the screen inside a headless egui context/frame with no
    /// simulated input, returning whatever ForwardingAction it produces.
    fn render_smoke(screen: &mut ForwardingScreen) -> Option<ForwardingAction> {
        let ctx = egui::Context::default();
        let mut action = None;
        let _ = ctx.run_ui(egui::RawInput::default(), |ui| {
            action = screen.render(&ctx, ui);
        });
        action
    }

    #[test]
    fn test_new_has_expected_defaults() {
        let screen = ForwardingScreen::new();
        assert_eq!(screen.edit_local_port, "8080");
        assert_eq!(screen.edit_remote_host, "localhost");
        assert_eq!(screen.edit_remote_port, "80");
        assert_eq!(screen.forward_type, ForwardType::Local);
        assert!(screen.forwards.is_empty());
    }

    #[test]
    fn test_default_matches_new() {
        let screen = ForwardingScreen::default();
        assert_eq!(screen.edit_local_port, "8080");
        assert!(screen.forwards.is_empty());
    }

    #[test]
    fn test_set_forwards_replaces_list() {
        let mut screen = ForwardingScreen::new();
        let forwards = vec![
            PortForward::new_local(8080, "example.com".to_string(), 80),
            PortForward::new_dynamic(1080),
        ];
        screen.set_forwards(forwards);
        assert_eq!(screen.forwards.len(), 2);
    }

    #[test]
    fn test_set_forwards_with_empty_list() {
        let mut screen = ForwardingScreen::new();
        screen.set_forwards(vec![PortForward::new_local(1, "h".to_string(), 1)]);
        screen.set_forwards(Vec::new());
        assert!(screen.forwards.is_empty());
    }

    #[test]
    fn test_render_smoke_with_no_forwards_produces_no_action() {
        let mut screen = ForwardingScreen::new();
        // With no simulated clicks, render() must not spontaneously
        // produce an Add/Remove action.
        assert!(render_smoke(&mut screen).is_none());
    }

    #[test]
    fn test_render_smoke_with_local_remote_and_dynamic_forwards() {
        let mut screen = ForwardingScreen::new();
        let mut local = PortForward::new_local(8080, "example.com".to_string(), 80);
        local.active = true;
        let remote = PortForward::new_remote(9090, "localhost".to_string(), 3000);
        let dynamic = PortForward::new_dynamic(1080);
        screen.set_forwards(vec![local, remote, dynamic]);
        assert!(render_smoke(&mut screen).is_none());
    }

    #[test]
    fn test_render_smoke_with_dynamic_forward_type_selected() {
        let mut screen = ForwardingScreen::new();
        screen.forward_type = ForwardType::Dynamic;
        assert!(render_smoke(&mut screen).is_none());
    }

    #[test]
    fn test_render_smoke_with_remote_forward_type_selected() {
        let mut screen = ForwardingScreen::new();
        screen.forward_type = ForwardType::Remote;
        assert!(render_smoke(&mut screen).is_none());
    }

    #[test]
    fn test_render_smoke_with_malformed_edit_fields() {
        // Non-numeric port text must render without panicking even though
        // the "Add" parse logic would reject it if clicked.
        let mut screen = ForwardingScreen::new();
        screen.edit_local_port = "not-a-port".to_string();
        screen.edit_remote_port = "".to_string();
        screen.edit_remote_host = "x".repeat(300);
        assert!(render_smoke(&mut screen).is_none());
    }

    #[test]
    fn test_render_smoke_with_many_forwards() {
        let mut screen = ForwardingScreen::new();
        let forwards: Vec<_> = (0..50)
            .map(|i| PortForward::new_local(i, format!("host{i}.example.com"), i))
            .collect();
        screen.set_forwards(forwards);
        assert!(render_smoke(&mut screen).is_none());
    }

    #[test]
    fn test_forwarding_action_debug_variants() {
        let add = ForwardingAction::Add(PortForward::new_local(1, "h".to_string(), 1));
        let remove = ForwardingAction::Remove(uuid::Uuid::new_v4());
        assert!(!format!("{:?}", add).is_empty());
        assert!(!format!("{:?}", remove).is_empty());
    }
}
