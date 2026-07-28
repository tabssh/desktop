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
                    if ui
                        .checkbox(&mut self.case_sensitive, "Case sensitive")
                        .changed()
                    {
                        action = Some(SearchAction::Search);
                    }

                    if ui.checkbox(&mut self.regex, "Regex").changed() {
                        action = Some(SearchAction::Search);
                    }
                });

                ui.horizontal(|ui| {
                    ui.label(format!("{}/{}", self.current_match + 1, self.total_matches));

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

// `SearchWidget::show` requires a live `egui::Context` inside `Window::show`'s
// closure-driven layout pass and only mutates UI state through interactive
// widgets (text_edit_singleline/checkbox/button), so it cannot be exercised
// meaningfully without a running GUI frame. Only the pure state constructors
// and the `SearchAction` enum are unit-testable here.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_has_default_state() {
        let widget = SearchWidget::new();
        assert!(!widget.open);
        assert_eq!(widget.query, "");
        assert!(!widget.case_sensitive);
        assert!(!widget.regex);
        assert_eq!(widget.current_match, 0);
        assert_eq!(widget.total_matches, 0);
    }

    #[test]
    fn test_default_matches_new() {
        let widget = SearchWidget::default();
        assert!(!widget.open);
        assert_eq!(widget.query, "");
        assert_eq!(widget.current_match, 0);
        assert_eq!(widget.total_matches, 0);
    }

    #[test]
    fn test_search_action_equality() {
        assert_eq!(SearchAction::Search, SearchAction::Search);
        assert_ne!(SearchAction::Search, SearchAction::Next);
        assert_ne!(SearchAction::Next, SearchAction::Previous);
    }

    #[test]
    fn test_widget_fields_mutable_directly() {
        // Fields are public so callers (and tests) can drive state without
        // going through the GUI layer.
        let mut widget = SearchWidget::new();
        widget.query = "needle".to_string();
        widget.case_sensitive = true;
        widget.total_matches = 5;
        widget.current_match = 2;

        assert_eq!(widget.query, "needle");
        assert!(widget.case_sensitive);
        assert_eq!(widget.total_matches, 5);
        assert_eq!(widget.current_match, 2);
    }
}
