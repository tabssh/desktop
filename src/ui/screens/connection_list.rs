//! Connection list screen

use egui::{Context, Ui};

pub struct ConnectionListScreen {
    search_query: String,
    selected_connection: Option<String>,
}

impl ConnectionListScreen {
    pub fn new() -> Self {
        Self {
            search_query: String::new(),
            selected_connection: None,
        }
    }
    
    pub fn render(&mut self, ctx: &Context, ui: &mut Ui) -> Option<ConnectionAction> {
        let mut action = None;
        
        ui.heading("Connections");
        
        // Search bar
        ui.horizontal(|ui| {
            ui.label("🔍");
            ui.text_edit_singleline(&mut self.search_query);
            
            if ui.button("➕ New").clicked() {
                action = Some(ConnectionAction::New);
            }
            
            if ui.button("📥 Import SSH Config").clicked() {
                action = Some(ConnectionAction::ImportConfig);
            }
        });
        
        ui.separator();
        
        // Connection groups/categories
        ui.collapsing("Recent", |ui| {
            self.render_connection_list(ui, &mut action, true);
        });
        
        ui.collapsing("All Connections", |ui| {
            self.render_connection_list(ui, &mut action, false);
        });
        
        action
    }
    
    fn render_connection_list(&mut self, ui: &mut Ui, action: &mut Option<ConnectionAction>, recent_only: bool) {
        let connections = vec![
            ("Production Server", "prod.example.com", "22", "admin"),
            ("Dev Server", "dev.example.com", "22", "user"),
            ("Database Server", "db.example.com", "22", "dbadmin"),
        ];
        
        for (name, host, port, user) in connections {
            ui.horizontal(|ui| {
                let is_selected = self.selected_connection.as_deref() == Some(name);
                
                if ui.selectable_label(is_selected, format!("🖥{}",name)).clicked(){
                    self.selected_connection = Some(name.to_string());
                }
                
                ui.label(format!("{}@{}:{}",user,host,port));
                
                if ui.small_button("🔌").clicked() {
                    *action = Some(ConnectionAction::Connect(name.to_string()));
                }
                
                if ui.small_button("✏").clicked() {
                    *action = Some(ConnectionAction::Edit(name.to_string()));
                }
                
                if ui.small_button("🗑").clicked() {
                    *action = Some(ConnectionAction::Delete(name.to_string()));
                }
            });
        }
    }
}

impl Default for ConnectionListScreen {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub enum ConnectionAction {
    New,
    Connect(String),
    Edit(String),
    Delete(String),
    ImportConfig,
}
