//! Main window UI state and rendering

/// Main window state
pub struct MainWindow {
    /// Whether the sidebar is visible
    sidebar_visible: bool,

    /// Current sidebar width
    sidebar_width: f32,
}

impl MainWindow {
    /// Create a new main window state
    pub fn new() -> Self {
        Self {
            sidebar_visible: true,
            sidebar_width: 250.0,
        }
    }

    /// Check if sidebar is visible
    pub fn sidebar_visible(&self) -> bool {
        self.sidebar_visible
    }

    /// Toggle sidebar visibility
    pub fn toggle_sidebar(&mut self) {
        self.sidebar_visible = !self.sidebar_visible;
    }

    /// Get sidebar width
    pub fn sidebar_width(&self) -> f32 {
        self.sidebar_width
    }

    /// Set sidebar width
    pub fn set_sidebar_width(&mut self, width: f32) {
        self.sidebar_width = width.clamp(150.0, 500.0);
    }
}

impl Default for MainWindow {
    fn default() -> Self {
        Self::new()
    }
}
