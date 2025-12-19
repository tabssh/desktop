//! Terminal search functionality

use egui::{Context, Window};

pub struct SearchWidget {
    pub open: bool,
    pub query: String,
    pub case_sensitive: bool,
    pub regex: bool,
    pub current_match: usize,
    pub total_matches: usize,
}

impl SearchWidget {
    pub fn new() -> Self {
        Self {
            open: false,
            query: String::new(),
            case_sensitive: false,
            regex: false,
            current_match: 0,
            total_matches: 0,
        }
    }
    
    pub fn show(&mut self, ctx: &Context) -> Option<SearchAction> {
        let mut action = None;
        
        Window::new("Find in Terminal")
            .open(&mut self.open)
            .resizable(false)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Search:");
                    if ui.text_edit_singleline(&mut self.query).changed() {
                        action = Some(SearchAction::Search);
                    }
                });
                
                ui.horizontal(|ui| {
                    if ui.checkbox(&mut self.case_sensitive, "Case sensitive").changed() {
                        action = Some(SearchAction::Search);
                    }
                    
                    if ui.checkbox(&mut self.regex, "Regex").changed() {
                        action = Some(SearchAction::Search);
                    }
                });
                
                ui.horizontal(|ui| {
                    ui.label(format!("{}/{}",self.current_match+1,self.total_matches));
                    
                    if ui.button("⬆ Previous").clicked() {
                        action = Some(SearchAction::Previous);
                    }
                    
                    if ui.button("⬇ Next").clicked() {
                        action = Some(SearchAction::Next);
                    }
                });
            });
        
        action
    }
}

impl Default for SearchWidget {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SearchAction {
    Search,
    Next,
    Previous,
}
