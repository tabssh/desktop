//! Terminal renderer for egui

use eframe::egui::{self, Color32, FontId, Pos2, Rect, Stroke, Vec2};
use super::buffer::TerminalBuffer;
use super::Color;

/// Terminal renderer configuration
pub struct RendererConfig {
    pub font_size: f32,
    pub font_family: String,
    pub cursor_style: CursorStyle,
    pub cursor_blink: bool,
    pub show_scrollbar: bool,
}

impl Default for RendererConfig {
    fn default() -> Self {
        Self {
            font_size: 14.0,
            font_family: "monospace".to_string(),
            cursor_style: CursorStyle::Block,
            cursor_blink: true,
            show_scrollbar: true,
        }
    }
}

/// Cursor display style
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CursorStyle {
    Block,
    Underline,
    Beam,
}

/// Terminal renderer
pub struct TerminalRenderer {
    config: RendererConfig,
    scroll_offset: usize,
    char_width: f32,
    char_height: f32,
}

impl TerminalRenderer {
    pub fn new(config: RendererConfig) -> Self {
        Self {
            config,
            scroll_offset: 0,
            char_width: 0.0,
            char_height: 0.0,
        }
    }

    /// Calculate character dimensions based on font
    fn calculate_char_size(&mut self, ui: &egui::Ui) {
        let font_id = FontId::monospace(self.config.font_size);
        let galley = ui.fonts(|f| {
            f.layout_no_wrap("M".to_string(), font_id.clone(), Color32::WHITE)
        });
        self.char_width = galley.rect.width();
        self.char_height = self.config.font_size * 1.2;
    }

    /// Render the terminal buffer
    pub fn render(&mut self, ui: &mut egui::Ui, buffer: &TerminalBuffer) {
        self.calculate_char_size(ui);

        let available = ui.available_size();
        let visible_rows = (available.y / self.char_height) as usize;
        let visible_cols = (available.x / self.char_width) as usize;

        let total_rows = buffer.scrollback_len() + buffer.size().rows as usize;

        let max_scroll = total_rows.saturating_sub(visible_rows);
        self.scroll_offset = self.scroll_offset.min(max_scroll);

        let (response, painter) = ui.allocate_painter(available, egui::Sense::click_and_drag());
        let rect = response.rect;

        painter.rect_filled(rect, 0.0, Color32::from_rgb(30, 30, 30));

        ui.input(|i| {
            let scroll = i.raw_scroll_delta.y;
            if scroll != 0.0 {
                let scroll_lines = (scroll / 20.0).abs() as usize;
                if scroll > 0.0 {
                    self.scroll_offset = self.scroll_offset.saturating_sub(scroll_lines);
                } else {
                    self.scroll_offset = (self.scroll_offset + scroll_lines).min(max_scroll);
                }
            }
        });

        let font_id = FontId::monospace(self.config.font_size);

        for row_idx in 0..visible_rows {
            let absolute_row = self.scroll_offset + row_idx;
            let y = rect.top() + (row_idx as f32 * self.char_height);

            let cells = if absolute_row < buffer.scrollback_len() {
                buffer.get_scrollback_row(absolute_row)
            } else {
                let screen_row = absolute_row - buffer.scrollback_len();
                buffer.get_row(screen_row)
            };

            if let Some(cells) = cells {
                let mut x = rect.left();

                for (_col_idx, cell) in cells.iter().enumerate().take(visible_cols) {
                    if cell.bg != Color::BLACK {
                        let bg_rect = Rect::from_min_size(
                            Pos2::new(x, y),
                            Vec2::new(self.char_width, self.char_height),
                        );
                        painter.rect_filled(bg_rect, 0.0, color_to_egui(cell.bg));
                    }

                    if !cell.is_empty() {
                        let mut fg_color = color_to_egui(cell.fg);

                        if cell.attrs.inverse {
                            let bg = color_to_egui(cell.bg);
                            painter.rect_filled(
                                Rect::from_min_size(
                                    Pos2::new(x, y),
                                    Vec2::new(self.char_width, self.char_height),
                                ),
                                0.0,
                                fg_color,
                            );
                            fg_color = bg;
                        }

                        if cell.attrs.dim {
                            fg_color = Color32::from_rgba_unmultiplied(
                                fg_color.r(),
                                fg_color.g(),
                                fg_color.b(),
                                128,
                            );
                        }

                        if !cell.attrs.hidden {
                            painter.text(
                                Pos2::new(x, y),
                                egui::Align2::LEFT_TOP,
                                cell.character,
                                font_id.clone(),
                                fg_color,
                            );
                        }

                        if cell.attrs.underline {
                            let underline_y = y + self.char_height - 2.0;
                            painter.line_segment(
                                [
                                    Pos2::new(x, underline_y),
                                    Pos2::new(x + self.char_width, underline_y),
                                ],
                                Stroke::new(1.0, fg_color),
                            );
                        }

                        if cell.attrs.strikethrough {
                            let strike_y = y + self.char_height / 2.0;
                            painter.line_segment(
                                [
                                    Pos2::new(x, strike_y),
                                    Pos2::new(x + self.char_width, strike_y),
                                ],
                                Stroke::new(1.0, fg_color),
                            );
                        }
                    }

                    x += self.char_width;
                }
            }
        }

        let (cursor_x, cursor_y) = buffer.cursor_position();
        let cursor_screen_row = cursor_y + buffer.scrollback_len();

        if cursor_screen_row >= self.scroll_offset
            && cursor_screen_row < self.scroll_offset + visible_rows
        {
            let cursor_display_row = cursor_screen_row - self.scroll_offset;
            let cursor_px_x = rect.left() + (cursor_x as f32 * self.char_width);
            let cursor_px_y = rect.top() + (cursor_display_row as f32 * self.char_height);

            let should_show = if self.config.cursor_blink {
                (ui.ctx().input(|i| i.time) * 2.0) as i32 % 2 == 0
            } else {
                true
            };

            if should_show {
                let cursor_color = Color32::from_rgb(200, 200, 200);

                match self.config.cursor_style {
                    CursorStyle::Block => {
                        painter.rect_filled(
                            Rect::from_min_size(
                                Pos2::new(cursor_px_x, cursor_px_y),
                                Vec2::new(self.char_width, self.char_height),
                            ),
                            0.0,
                            cursor_color,
                        );

                        if let Some(cell) = buffer.get_cell(cursor_x, cursor_y) {
                            if !cell.is_empty() {
                                painter.text(
                                    Pos2::new(cursor_px_x, cursor_px_y),
                                    egui::Align2::LEFT_TOP,
                                    cell.character,
                                    font_id.clone(),
                                    Color32::from_rgb(30, 30, 30),
                                );
                            }
                        }
                    }
                    CursorStyle::Underline => {
                        let underline_y = cursor_px_y + self.char_height - 2.0;
                        painter.line_segment(
                            [
                                Pos2::new(cursor_px_x, underline_y),
                                Pos2::new(cursor_px_x + self.char_width, underline_y),
                            ],
                            Stroke::new(2.0, cursor_color),
                        );
                    }
                    CursorStyle::Beam => {
                        painter.line_segment(
                            [
                                Pos2::new(cursor_px_x, cursor_px_y),
                                Pos2::new(cursor_px_x, cursor_px_y + self.char_height),
                            ],
                            Stroke::new(2.0, cursor_color),
                        );
                    }
                }
            }
        }

        if self.config.show_scrollbar && total_rows > visible_rows {
            let scrollbar_width = 8.0;
            let scrollbar_x = rect.right() - scrollbar_width - 2.0;
            let scrollbar_height = (visible_rows as f32 / total_rows as f32) * rect.height();
            let scrollbar_y = rect.top()
                + (self.scroll_offset as f32 / total_rows as f32) * rect.height();

            painter.rect_filled(
                Rect::from_min_size(
                    Pos2::new(scrollbar_x, scrollbar_y),
                    Vec2::new(scrollbar_width, scrollbar_height.max(20.0)),
                ),
                4.0,
                Color32::from_rgba_unmultiplied(100, 100, 100, 150),
            );
        }

        ui.ctx().request_repaint();
    }

    /// Scroll to bottom of buffer
    pub fn scroll_to_bottom(&mut self, buffer: &TerminalBuffer) {
        let total_rows = buffer.scrollback_len() + buffer.size().rows as usize;
        let visible_rows = 24;
        self.scroll_offset = total_rows.saturating_sub(visible_rows);
    }

    /// Get current scroll offset
    pub fn scroll_offset(&self) -> usize {
        self.scroll_offset
    }

    /// Set scroll offset
    pub fn set_scroll_offset(&mut self, offset: usize) {
        self.scroll_offset = offset;
    }

    /// Calculate terminal size in characters for given pixel dimensions
    pub fn calculate_size(&self, width: f32, height: f32) -> (u16, u16) {
        let cols = (width / self.char_width.max(1.0)) as u16;
        let rows = (height / self.char_height.max(1.0)) as u16;
        (cols.max(1), rows.max(1))
    }
}

fn color_to_egui(color: Color) -> Color32 {
    Color32::from_rgb(color.r, color.g, color.b)
}
