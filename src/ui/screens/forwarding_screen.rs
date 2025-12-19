//! Port forwarding management screen

use egui::{Context, Ui};
use crate::ssh::{PortForward, ForwardType};

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
    
    pub fn render(&mut self, ctx: &Context, ui: &mut Ui) -> Option<ForwardingAction> {
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
            
            if matches!(self.forward_type,ForwardType::Dynamic){
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
                            format!("SOCKSproxyon:{}",forward.listen_port)
                        }
                        _ => {
                            format!(
                                ":{} → {}:{}",
                                forward.listen_port,
                                forward.remote_host,
                                forward.remote_port
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
