//! Professional UI Components Library
//! Reusable, styled UI widgets for consistent look and feel

use eframe::egui::{self, Color32, CornerRadius, RichText, Stroke, Vec2};

/// Color palette for the application
pub mod colors {
    use super::Color32;

    // Primary colors
    pub const PRIMARY: Color32 = Color32::from_rgb(59, 130, 246); // Blue
    pub const PRIMARY_HOVER: Color32 = Color32::from_rgb(37, 99, 235);
    pub const PRIMARY_DARK: Color32 = Color32::from_rgb(29, 78, 216);

    // Secondary colors
    pub const SECONDARY: Color32 = Color32::from_rgb(100, 116, 139); // Slate
    pub const SECONDARY_HOVER: Color32 = Color32::from_rgb(71, 85, 105);

    // Status colors
    pub const SUCCESS: Color32 = Color32::from_rgb(34, 197, 94); // Green
    pub const WARNING: Color32 = Color32::from_rgb(234, 179, 8); // Yellow
    pub const DANGER: Color32 = Color32::from_rgb(239, 68, 68); // Red
    pub const INFO: Color32 = Color32::from_rgb(14, 165, 233); // Sky

    // Background colors
    pub const BG_PRIMARY: Color32 = Color32::from_rgb(15, 23, 42); // Dark slate
    pub const BG_SECONDARY: Color32 = Color32::from_rgb(30, 41, 59); // Lighter slate
    pub const BG_TERTIARY: Color32 = Color32::from_rgb(51, 65, 85); // Even lighter
    pub const BG_SURFACE: Color32 = Color32::from_rgb(71, 85, 105); // Surface

    // Text colors
    pub const TEXT_PRIMARY: Color32 = Color32::from_rgb(248, 250, 252);
    pub const TEXT_SECONDARY: Color32 = Color32::from_rgb(148, 163, 184);
    pub const TEXT_MUTED: Color32 = Color32::from_rgb(100, 116, 139);

    // Border colors
    pub const BORDER: Color32 = Color32::from_rgb(71, 85, 105);
    pub const BORDER_FOCUS: Color32 = Color32::from_rgb(59, 130, 246);

    pub const ERROR: Color32 = DANGER;
    pub const BG_HIGHLIGHT: Color32 = Color32::from_rgb(47, 64, 91);
}

/// Spacing constants
pub mod spacing {
    pub const XS: f32 = 4.0;
    pub const SM: f32 = 8.0;
    pub const MD: f32 = 12.0;
    pub const LG: f32 = 16.0;
    pub const XL: f32 = 24.0;
    pub const XXL: f32 = 32.0;
}

/// Button style variants
#[derive(Clone, Copy, PartialEq)]
pub enum ButtonStyle {
    Primary,
    Secondary,
    Danger,
    Ghost,
}

/// Styled button component
pub fn button(ui: &mut egui::Ui, text: &str, style: ButtonStyle) -> egui::Response {
    let (bg, _bg_hover, text_color) = match style {
        ButtonStyle::Primary => (colors::PRIMARY, colors::PRIMARY_HOVER, colors::TEXT_PRIMARY),
        ButtonStyle::Secondary => (
            colors::BG_TERTIARY,
            colors::BG_SURFACE,
            colors::TEXT_PRIMARY,
        ),
        ButtonStyle::Danger => (
            colors::DANGER,
            Color32::from_rgb(220, 38, 38),
            colors::TEXT_PRIMARY,
        ),
        ButtonStyle::Ghost => (
            Color32::TRANSPARENT,
            colors::BG_TERTIARY,
            colors::TEXT_SECONDARY,
        ),
    };

    let button = egui::Button::new(RichText::new(text).color(text_color))
        .fill(bg)
        .stroke(Stroke::NONE)
        .corner_radius(CornerRadius::same(6))
        .min_size(Vec2::new(0.0, 32.0));

    let response = ui.add(button);

    if response.hovered() {
        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
    }

    response
}

/// Primary button
pub fn primary_button(ui: &mut egui::Ui, text: &str) -> egui::Response {
    button(ui, text, ButtonStyle::Primary)
}

/// Secondary button
pub fn secondary_button(ui: &mut egui::Ui, text: &str) -> egui::Response {
    button(ui, text, ButtonStyle::Secondary)
}

/// Danger button
pub fn danger_button(ui: &mut egui::Ui, text: &str) -> egui::Response {
    button(ui, text, ButtonStyle::Danger)
}

/// Toggle switch component
pub fn toggle(ui: &mut egui::Ui, enabled: &mut bool) -> egui::Response {
    let desired_size = Vec2::new(44.0, 24.0);
    let (rect, response) = ui.allocate_exact_size(desired_size, egui::Sense::click());

    if response.clicked() {
        *enabled = !*enabled;
    }

    if ui.is_rect_visible(rect) {
        let how_on = ui.ctx().animate_bool(response.id, *enabled);

        let bg_color = Color32::from_rgb(
            (colors::BG_TERTIARY.r() as f32
                + (colors::PRIMARY.r() as f32 - colors::BG_TERTIARY.r() as f32) * how_on)
                as u8,
            (colors::BG_TERTIARY.g() as f32
                + (colors::PRIMARY.g() as f32 - colors::BG_TERTIARY.g() as f32) * how_on)
                as u8,
            (colors::BG_TERTIARY.b() as f32
                + (colors::PRIMARY.b() as f32 - colors::BG_TERTIARY.b() as f32) * how_on)
                as u8,
        );

        let circle_x = rect.left() + 12.0 + how_on * 20.0;
        let circle_center = egui::pos2(circle_x, rect.center().y);

        ui.painter()
            .rect_filled(rect, CornerRadius::same(12), bg_color);
        ui.painter()
            .circle_filled(circle_center, 8.0, colors::TEXT_PRIMARY);
    }

    if response.hovered() {
        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
    }

    response
}

/// Toggle with label
pub fn labeled_toggle(ui: &mut egui::Ui, label: &str, enabled: &mut bool) -> egui::Response {
    ui.horizontal(|ui| {
        ui.label(RichText::new(label).color(colors::TEXT_PRIMARY));
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            toggle(ui, enabled)
        })
        .inner
    })
    .inner
}

/// Styled checkbox
pub fn checkbox(ui: &mut egui::Ui, checked: &mut bool, label: &str) -> egui::Response {
    ui.checkbox(checked, RichText::new(label).color(colors::TEXT_PRIMARY))
}

/// Dropdown/ComboBox component
pub fn dropdown<T: ToString + PartialEq>(
    ui: &mut egui::Ui,
    id: &str,
    selected: &mut T,
    options: &[T],
) -> egui::Response {
    let selected_text = selected.to_string();

    egui::ComboBox::from_id_salt(id)
        .selected_text(RichText::new(&selected_text).color(colors::TEXT_PRIMARY))
        .width(200.0)
        .show_ui(ui, |ui| {
            for option in options {
                let text = option.to_string();
                if ui.selectable_label(*selected == *option, &text).clicked() {
                    *selected = unsafe { std::ptr::read(option) };
                }
            }
        })
        .response
}

/// Labeled dropdown
pub fn labeled_dropdown<T: ToString + PartialEq + Clone>(
    ui: &mut egui::Ui,
    label: &str,
    id: &str,
    selected: &mut T,
    options: &[T],
) {
    ui.horizontal(|ui| {
        ui.label(RichText::new(label).color(colors::TEXT_PRIMARY));
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            let selected_text = selected.to_string();
            egui::ComboBox::from_id_salt(id)
                .selected_text(RichText::new(&selected_text).color(colors::TEXT_PRIMARY))
                .width(200.0)
                .show_ui(ui, |ui| {
                    for option in options {
                        let text = option.to_string();
                        if ui.selectable_label(*selected == *option, &text).clicked() {
                            *selected = option.clone();
                        }
                    }
                });
        });
    });
}

/// Styled text input
pub fn text_input(ui: &mut egui::Ui, value: &mut String, hint: &str) -> egui::Response {
    let input = egui::TextEdit::singleline(value)
        .hint_text(RichText::new(hint).color(colors::TEXT_MUTED))
        .text_color(colors::TEXT_PRIMARY)
        .desired_width(200.0)
        .margin(egui::vec2(8.0, 6.0));
    ui.add(input)
}

/// Labeled text input
pub fn labeled_input(ui: &mut egui::Ui, label: &str, value: &mut String, hint: &str) {
    ui.horizontal(|ui| {
        ui.label(RichText::new(label).color(colors::TEXT_PRIMARY));
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            text_input(ui, value, hint);
        });
    });
}

/// Password input (masked)
pub fn password_input(ui: &mut egui::Ui, value: &mut String, hint: &str) -> egui::Response {
    let input = egui::TextEdit::singleline(value)
        .hint_text(RichText::new(hint).color(colors::TEXT_MUTED))
        .text_color(colors::TEXT_PRIMARY)
        .password(true)
        .desired_width(200.0)
        .margin(egui::vec2(8.0, 6.0));
    ui.add(input)
}

/// Number input with validation
pub fn number_input(ui: &mut egui::Ui, value: &mut u16, min: u16, max: u16) -> egui::Response {
    let mut text = value.to_string();
    let response = ui.add(
        egui::TextEdit::singleline(&mut text)
            .text_color(colors::TEXT_PRIMARY)
            .desired_width(80.0)
            .margin(egui::vec2(8.0, 6.0)),
    );

    if response.changed() {
        if let Ok(num) = text.parse::<u16>() {
            *value = num.clamp(min, max);
        }
    }

    response
}

/// Labeled number input
pub fn labeled_number(ui: &mut egui::Ui, label: &str, value: &mut u16, min: u16, max: u16) {
    ui.horizontal(|ui| {
        ui.label(RichText::new(label).color(colors::TEXT_PRIMARY));
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            number_input(ui, value, min, max);
        });
    });
}

/// Section header
pub fn section_header(ui: &mut egui::Ui, title: &str) {
    ui.add_space(spacing::LG);
    ui.label(
        RichText::new(title)
            .color(colors::TEXT_PRIMARY)
            .strong()
            .size(16.0),
    );
    ui.add_space(spacing::SM);
    ui.separator();
    ui.add_space(spacing::SM);
}

/// Subsection header
pub fn subsection_header(ui: &mut egui::Ui, title: &str) {
    ui.add_space(spacing::MD);
    ui.label(
        RichText::new(title)
            .color(colors::TEXT_SECONDARY)
            .size(13.0),
    );
    ui.add_space(spacing::XS);
}

/// Card/Panel container
pub fn card(ui: &mut egui::Ui, add_contents: impl FnOnce(&mut egui::Ui)) {
    egui::Frame::NONE
        .fill(colors::BG_SECONDARY)
        .corner_radius(CornerRadius::same(8))
        .inner_margin(egui::Margin::same(spacing::LG as i8))
        .stroke(Stroke::new(1.0, colors::BORDER))
        .show(ui, add_contents);
}

/// Status badge
#[derive(Clone, Copy)]
pub enum StatusType {
    Connected,
    Connecting,
    Disconnected,
    Error,
}

pub fn status_badge(ui: &mut egui::Ui, status: StatusType) {
    let (color, text) = match status {
        StatusType::Connected => (colors::SUCCESS, "Connected"),
        StatusType::Connecting => (colors::WARNING, "Connecting"),
        StatusType::Disconnected => (colors::TEXT_MUTED, "Disconnected"),
        StatusType::Error => (colors::DANGER, "Error"),
    };

    ui.horizontal(|ui| {
        let (rect, _) = ui.allocate_exact_size(Vec2::new(8.0, 8.0), egui::Sense::hover());
        ui.painter().circle_filled(rect.center(), 4.0, color);
        ui.label(RichText::new(text).color(color).size(12.0));
    });
}

/// Icon button (small, icon only)
pub fn icon_button(ui: &mut egui::Ui, icon: &str, tooltip: &str) -> egui::Response {
    let button = egui::Button::new(RichText::new(icon).size(16.0))
        .fill(Color32::TRANSPARENT)
        .stroke(Stroke::NONE)
        .min_size(Vec2::new(28.0, 28.0));

    let response = ui.add(button).on_hover_text(tooltip);

    if response.hovered() {
        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
    }

    response
}

/// Sidebar navigation item
pub fn nav_item(ui: &mut egui::Ui, icon: &str, label: &str, selected: bool) -> egui::Response {
    let bg = if selected {
        colors::BG_TERTIARY
    } else {
        Color32::TRANSPARENT
    };
    let text_color = if selected {
        colors::TEXT_PRIMARY
    } else {
        colors::TEXT_SECONDARY
    };

    let button = egui::Button::new(
        RichText::new(format!("{}  {}", icon, label))
            .color(text_color)
            .size(14.0),
    )
    .fill(bg)
    .stroke(Stroke::NONE)
    .corner_radius(CornerRadius::same(6))
    .min_size(Vec2::new(ui.available_width(), 36.0));

    let response = ui.add(button);

    if response.hovered() {
        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
    }

    response
}

/// Horizontal divider with optional label
pub fn divider(ui: &mut egui::Ui) {
    ui.add_space(spacing::SM);
    ui.separator();
    ui.add_space(spacing::SM);
}

/// Empty state placeholder
pub fn empty_state(ui: &mut egui::Ui, icon: &str, title: &str, description: &str) {
    ui.vertical_centered(|ui| {
        ui.add_space(spacing::XXL);
        ui.label(RichText::new(icon).size(48.0).color(colors::TEXT_MUTED));
        ui.add_space(spacing::MD);
        ui.label(
            RichText::new(title)
                .size(18.0)
                .color(colors::TEXT_PRIMARY)
                .strong(),
        );
        ui.add_space(spacing::XS);
        ui.label(
            RichText::new(description)
                .size(14.0)
                .color(colors::TEXT_SECONDARY),
        );
        ui.add_space(spacing::XXL);
    });
}

/// Form row with consistent spacing
pub fn form_row(ui: &mut egui::Ui, add_contents: impl FnOnce(&mut egui::Ui)) {
    ui.horizontal(|ui| {
        ui.set_min_height(32.0);
        add_contents(ui);
    });
    ui.add_space(spacing::SM);
}

/// Tooltip wrapper
pub fn with_tooltip<R>(
    ui: &mut egui::Ui,
    _tooltip: &str,
    add_contents: impl FnOnce(&mut egui::Ui) -> R,
) -> R {
    add_contents(ui)
}

/// Tab bar widget — displays open session tabs
pub struct TabBar {
    tabs: Vec<String>,
    active: usize,
}

impl TabBar {
    pub fn new() -> Self {
        Self {
            tabs: Vec::new(),
            active: 0,
        }
    }

    /// Render tabs and return an action if the user interacted
    pub fn render(&mut self, ui: &mut egui::Ui) -> Option<TabBarAction> {
        let mut action = None;
        ui.horizontal(|ui| {
            for (idx, tab) in self.tabs.iter().enumerate() {
                let selected = idx == self.active;
                if ui.selectable_label(selected, tab).clicked() {
                    self.active = idx;
                    action = Some(TabBarAction::SelectTab(idx));
                }
            }
            if ui.button("+").clicked() {
                action = Some(TabBarAction::NewTab);
            }
        });
        action
    }
}

impl Default for TabBar {
    fn default() -> Self {
        Self::new()
    }
}

/// Action emitted by the tab bar
#[derive(Debug, Clone)]
pub enum TabBarAction {
    SelectTab(usize),
    NewTab,
    CloseTab(usize),
}

/// Application toolbar — top action bar
pub struct Toolbar;

impl Toolbar {
    /// Render the toolbar and return an action if the user interacted
    pub fn render(ui: &mut egui::Ui) -> Option<ToolbarAction> {
        let mut action = None;
        ui.horizontal(|ui| {
            if ui.button("➕ New").clicked() {
                action = Some(ToolbarAction::NewConnection);
            }
            if ui.button("⚙ Settings").clicked() {
                action = Some(ToolbarAction::OpenSettings);
            }
        });
        action
    }
}

/// Action emitted by the toolbar
#[derive(Debug, Clone)]
pub enum ToolbarAction {
    NewConnection,
    OpenSettings,
}

/// Status bar — bottom informational strip
pub struct StatusBar {
    message: Option<String>,
}

impl StatusBar {
    pub fn new() -> Self {
        Self { message: None }
    }

    /// Update the status message
    pub fn set_message(&mut self, msg: impl Into<String>) {
        self.message = Some(msg.into());
    }

    /// Render the status bar
    pub fn render(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if let Some(msg) = &self.message {
                ui.label(RichText::new(msg).color(colors::TEXT_SECONDARY).size(12.0));
            } else {
                ui.label(RichText::new("Ready").color(colors::TEXT_MUTED).size(12.0));
            }
        });
    }
}

impl Default for StatusBar {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Run `f` against a freshly built `egui::Ui` inside a headless context,
    /// mirroring the pattern used in `keyboard.rs` for widget code that
    /// needs a live `Ui` rather than just a `Context`.
    fn with_ui(f: impl FnOnce(&mut egui::Ui)) {
        let ctx = egui::Context::default();
        let mut f = Some(f);
        let _ = ctx.run_ui(egui::RawInput::default(), |ui| {
            if let Some(f) = f.take() {
                f(ui);
            }
        });
    }

    #[test]
    fn test_button_styles_render_without_panic() {
        with_ui(|ui| {
            button(ui, "Click", ButtonStyle::Primary);
            button(ui, "Click", ButtonStyle::Secondary);
            button(ui, "Click", ButtonStyle::Danger);
            button(ui, "Click", ButtonStyle::Ghost);
        });
    }

    #[test]
    fn test_button_empty_and_unicode_text() {
        with_ui(|ui| {
            button(ui, "", ButtonStyle::Primary);
            button(ui, "日本語 🚀 テスト", ButtonStyle::Primary);
        });
    }

    #[test]
    fn test_primary_secondary_danger_button_helpers() {
        with_ui(|ui| {
            primary_button(ui, "Save");
            secondary_button(ui, "Cancel");
            danger_button(ui, "Delete");
        });
    }

    #[test]
    fn test_toggle_off_and_on() {
        with_ui(|ui| {
            let mut enabled = false;
            toggle(ui, &mut enabled);
            assert!(!enabled);
        });
        with_ui(|ui| {
            let mut enabled = true;
            toggle(ui, &mut enabled);
            assert!(enabled);
        });
    }

    #[test]
    fn test_labeled_toggle_renders() {
        with_ui(|ui| {
            let mut enabled = false;
            labeled_toggle(ui, "Enable feature", &mut enabled);
        });
    }

    #[test]
    fn test_checkbox_renders() {
        with_ui(|ui| {
            let mut checked = false;
            checkbox(ui, &mut checked, "Remember me");
        });
    }

    #[test]
    fn test_dropdown_renders_with_options() {
        with_ui(|ui| {
            let mut selected = "b".to_string();
            let options = vec!["a".to_string(), "b".to_string(), "c".to_string()];
            dropdown(ui, "dropdown-id", &mut selected, &options);
        });
    }

    #[test]
    fn test_dropdown_renders_with_empty_options() {
        with_ui(|ui| {
            let mut selected = "x".to_string();
            let options: Vec<String> = Vec::new();
            dropdown(ui, "dropdown-empty", &mut selected, &options);
        });
    }

    #[test]
    fn test_labeled_dropdown_renders() {
        with_ui(|ui| {
            let mut selected = "a".to_string();
            let options = vec!["a".to_string(), "b".to_string()];
            labeled_dropdown(ui, "Theme", "labeled-dropdown-id", &mut selected, &options);
        });
    }

    #[test]
    fn test_text_input_renders_with_value_and_hint() {
        with_ui(|ui| {
            let mut value = "hello".to_string();
            text_input(ui, &mut value, "Enter text");
        });
    }

    #[test]
    fn test_text_input_renders_empty_and_unicode() {
        with_ui(|ui| {
            let mut value = String::new();
            text_input(ui, &mut value, "");
        });
        with_ui(|ui| {
            let mut value = "héllo wörld 日本語".to_string();
            text_input(ui, &mut value, "hint");
        });
    }

    #[test]
    fn test_labeled_input_renders() {
        with_ui(|ui| {
            let mut value = "user".to_string();
            labeled_input(ui, "Username", &mut value, "Enter username");
        });
    }

    #[test]
    fn test_password_input_renders() {
        with_ui(|ui| {
            let mut value = "secret".to_string();
            password_input(ui, &mut value, "Password");
        });
    }

    #[test]
    fn test_number_input_renders() {
        with_ui(|ui| {
            let mut value: u16 = 22;
            number_input(ui, &mut value, 1, 65535);
        });
    }

    #[test]
    fn test_number_input_boundary_values() {
        with_ui(|ui| {
            let mut value: u16 = 0;
            number_input(ui, &mut value, 0, 65535);
            assert_eq!(value, 0);
        });
        with_ui(|ui| {
            let mut value: u16 = 65535;
            number_input(ui, &mut value, 0, 65535);
            assert_eq!(value, 65535);
        });
    }

    #[test]
    fn test_labeled_number_renders() {
        with_ui(|ui| {
            let mut value: u16 = 22;
            labeled_number(ui, "Port", &mut value, 1, 65535);
        });
    }

    #[test]
    fn test_section_and_subsection_headers_render() {
        with_ui(|ui| {
            section_header(ui, "General");
            subsection_header(ui, "Advanced");
        });
    }

    #[test]
    fn test_section_header_empty_title() {
        with_ui(|ui| {
            section_header(ui, "");
        });
    }

    #[test]
    fn test_card_renders_contents() {
        with_ui(|ui| {
            card(ui, |ui| {
                ui.label("inside card");
            });
        });
    }

    #[test]
    fn test_status_badge_all_variants_render() {
        with_ui(|ui| {
            status_badge(ui, StatusType::Connected);
            status_badge(ui, StatusType::Connecting);
            status_badge(ui, StatusType::Disconnected);
            status_badge(ui, StatusType::Error);
        });
    }

    #[test]
    fn test_icon_button_renders() {
        with_ui(|ui| {
            icon_button(ui, "🗑", "Delete");
        });
    }

    #[test]
    fn test_nav_item_selected_and_unselected() {
        with_ui(|ui| {
            nav_item(ui, "🏠", "Home", true);
            nav_item(ui, "⚙", "Settings", false);
        });
    }

    #[test]
    fn test_divider_renders() {
        with_ui(|ui| {
            divider(ui);
        });
    }

    #[test]
    fn test_empty_state_renders_with_unicode() {
        with_ui(|ui| {
            empty_state(
                ui,
                "📭",
                "No connections",
                "Add a connection to get started 日本語",
            );
        });
    }

    #[test]
    fn test_empty_state_renders_with_empty_strings() {
        with_ui(|ui| {
            empty_state(ui, "", "", "");
        });
    }

    #[test]
    fn test_form_row_renders_contents() {
        with_ui(|ui| {
            form_row(ui, |ui| {
                ui.label("row content");
            });
        });
    }

    #[test]
    fn test_with_tooltip_returns_inner_value() {
        with_ui(|ui| {
            let value = with_tooltip(ui, "a tooltip", |ui| {
                ui.label("tooltipped");
                42
            });
            assert_eq!(value, 42);
        });
    }

    #[test]
    fn test_tab_bar_new_and_default_start_empty() {
        let bar = TabBar::new();
        assert_eq!(bar.tabs.len(), 0);
        assert_eq!(bar.active, 0);

        let default_bar = TabBar::default();
        assert_eq!(default_bar.tabs.len(), 0);
        assert_eq!(default_bar.active, 0);
    }

    #[test]
    fn test_tab_bar_render_with_no_tabs_returns_none() {
        let mut bar = TabBar::new();
        with_ui(|ui| {
            let action = bar.render(ui);
            assert!(action.is_none());
        });
    }

    #[test]
    fn test_tab_bar_render_with_tabs() {
        let mut bar = TabBar::new();
        bar.tabs.push("Tab 1".to_string());
        bar.tabs.push("Tab 2".to_string());
        with_ui(|ui| {
            let action = bar.render(ui);
            // No click was simulated, so no action should be emitted.
            assert!(action.is_none());
        });
    }

    #[test]
    fn test_tab_bar_action_debug_and_clone() {
        let action = TabBarAction::SelectTab(2);
        let cloned = action.clone();
        assert!(!format!("{:?}", cloned).is_empty());

        assert!(!format!("{:?}", TabBarAction::NewTab).is_empty());
        assert!(!format!("{:?}", TabBarAction::CloseTab(0)).is_empty());
    }

    #[test]
    fn test_toolbar_render_without_click_returns_none() {
        with_ui(|ui| {
            let action = Toolbar::render(ui);
            assert!(action.is_none());
        });
    }

    #[test]
    fn test_toolbar_action_debug_and_clone() {
        let action = ToolbarAction::NewConnection;
        let cloned = action.clone();
        assert!(!format!("{:?}", cloned).is_empty());
        assert!(!format!("{:?}", ToolbarAction::OpenSettings).is_empty());
    }

    #[test]
    fn test_status_bar_new_has_no_message() {
        let bar = StatusBar::new();
        assert!(bar.message.is_none());
    }

    #[test]
    fn test_status_bar_default_matches_new() {
        let bar = StatusBar::default();
        assert!(bar.message.is_none());
    }

    #[test]
    fn test_status_bar_set_message_updates_message() {
        let mut bar = StatusBar::new();
        bar.set_message("Connected to host");
        assert_eq!(bar.message.as_deref(), Some("Connected to host"));
    }

    #[test]
    fn test_status_bar_render_without_message() {
        let mut bar = StatusBar::new();
        with_ui(|ui| {
            bar.render(ui);
        });
    }

    #[test]
    fn test_status_bar_render_with_message() {
        let mut bar = StatusBar::new();
        bar.set_message("Ready 日本語");
        with_ui(|ui| {
            bar.render(ui);
        });
    }
}
