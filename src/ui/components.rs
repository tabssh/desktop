//! Professional UI Components Library
//! Reusable, styled UI widgets for consistent look and feel

#![allow(dead_code)]

use eframe::egui::{self, Color32, RichText, Rounding, Stroke, Vec2};

/// Color palette for the application
pub mod colors {
    use super::Color32;

    // Primary colors
    pub const PRIMARY: Color32 = Color32::from_rgb(59, 130, 246);      // Blue
    pub const PRIMARY_HOVER: Color32 = Color32::from_rgb(37, 99, 235);
    pub const PRIMARY_DARK: Color32 = Color32::from_rgb(29, 78, 216);

    // Secondary colors
    pub const SECONDARY: Color32 = Color32::from_rgb(100, 116, 139);   // Slate
    pub const SECONDARY_HOVER: Color32 = Color32::from_rgb(71, 85, 105);

    // Status colors
    pub const SUCCESS: Color32 = Color32::from_rgb(34, 197, 94);       // Green
    pub const WARNING: Color32 = Color32::from_rgb(234, 179, 8);       // Yellow
    pub const DANGER: Color32 = Color32::from_rgb(239, 68, 68);        // Red
    pub const INFO: Color32 = Color32::from_rgb(14, 165, 233);         // Sky

    // Background colors
    pub const BG_PRIMARY: Color32 = Color32::from_rgb(15, 23, 42);     // Dark slate
    pub const BG_SECONDARY: Color32 = Color32::from_rgb(30, 41, 59);   // Lighter slate
    pub const BG_TERTIARY: Color32 = Color32::from_rgb(51, 65, 85);    // Even lighter
    pub const BG_SURFACE: Color32 = Color32::from_rgb(71, 85, 105);    // Surface

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
        ButtonStyle::Secondary => (colors::BG_TERTIARY, colors::BG_SURFACE, colors::TEXT_PRIMARY),
        ButtonStyle::Danger => (colors::DANGER, Color32::from_rgb(220, 38, 38), colors::TEXT_PRIMARY),
        ButtonStyle::Ghost => (Color32::TRANSPARENT, colors::BG_TERTIARY, colors::TEXT_SECONDARY),
    };

    let button = egui::Button::new(RichText::new(text).color(text_color))
        .fill(bg)
        .stroke(Stroke::NONE)
        .rounding(Rounding::same(6.0))
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
            (colors::BG_TERTIARY.r() as f32 + (colors::PRIMARY.r() as f32 - colors::BG_TERTIARY.r() as f32) * how_on) as u8,
            (colors::BG_TERTIARY.g() as f32 + (colors::PRIMARY.g() as f32 - colors::BG_TERTIARY.g() as f32) * how_on) as u8,
            (colors::BG_TERTIARY.b() as f32 + (colors::PRIMARY.b() as f32 - colors::BG_TERTIARY.b() as f32) * how_on) as u8,
        );

        let circle_x = rect.left() + 12.0 + how_on * 20.0;
        let circle_center = egui::pos2(circle_x, rect.center().y);

        ui.painter().rect_filled(rect, Rounding::same(12.0), bg_color);
        ui.painter().circle_filled(circle_center, 8.0, colors::TEXT_PRIMARY);
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
        }).inner
    }).inner
}

/// Styled checkbox
pub fn checkbox(ui: &mut egui::Ui, checked: &mut bool, label: &str) -> egui::Response {
    let response = ui.checkbox(checked, RichText::new(label).color(colors::TEXT_PRIMARY));
    response
}

/// Dropdown/ComboBox component
pub fn dropdown<'a, T: ToString + PartialEq>(
    ui: &mut egui::Ui,
    id: &str,
    selected: &mut T,
    options: &'a [T],
) -> egui::Response {
    let selected_text = selected.to_string();

    egui::ComboBox::from_id_source(id)
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
pub fn labeled_dropdown<'a, T: ToString + PartialEq + Clone>(
    ui: &mut egui::Ui,
    label: &str,
    id: &str,
    selected: &mut T,
    options: &'a [T],
) {
    ui.horizontal(|ui| {
        ui.label(RichText::new(label).color(colors::TEXT_PRIMARY));
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            let selected_text = selected.to_string();
            egui::ComboBox::from_id_source(id)
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
            .margin(egui::vec2(8.0, 6.0))
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
    ui.label(RichText::new(title).color(colors::TEXT_PRIMARY).strong().size(16.0));
    ui.add_space(spacing::SM);
    ui.separator();
    ui.add_space(spacing::SM);
}

/// Subsection header
pub fn subsection_header(ui: &mut egui::Ui, title: &str) {
    ui.add_space(spacing::MD);
    ui.label(RichText::new(title).color(colors::TEXT_SECONDARY).size(13.0));
    ui.add_space(spacing::XS);
}

/// Card/Panel container
pub fn card(ui: &mut egui::Ui, add_contents: impl FnOnce(&mut egui::Ui)) {
    egui::Frame::none()
        .fill(colors::BG_SECONDARY)
        .rounding(Rounding::same(8.0))
        .inner_margin(egui::Margin::same(spacing::LG))
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
    let bg = if selected { colors::BG_TERTIARY } else { Color32::TRANSPARENT };
    let text_color = if selected { colors::TEXT_PRIMARY } else { colors::TEXT_SECONDARY };

    let button = egui::Button::new(
        RichText::new(format!("{}  {}", icon, label))
            .color(text_color)
            .size(14.0)
    )
        .fill(bg)
        .stroke(Stroke::NONE)
        .rounding(Rounding::same(6.0))
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
        ui.label(RichText::new(title).size(18.0).color(colors::TEXT_PRIMARY).strong());
        ui.add_space(spacing::XS);
        ui.label(RichText::new(description).size(14.0).color(colors::TEXT_SECONDARY));
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
pub fn with_tooltip<R>(ui: &mut egui::Ui, _tooltip: &str, add_contents: impl FnOnce(&mut egui::Ui) -> R) -> R {
    let response = add_contents(ui);
    response
}
