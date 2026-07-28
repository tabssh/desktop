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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_has_expected_defaults() {
        let window = MainWindow::new();
        assert!(window.sidebar_visible());
        assert_eq!(window.sidebar_width(), 250.0);
    }

    #[test]
    fn test_default_matches_new() {
        let default_window = MainWindow::default();
        let new_window = MainWindow::new();
        assert_eq!(
            default_window.sidebar_visible(),
            new_window.sidebar_visible()
        );
        assert_eq!(default_window.sidebar_width(), new_window.sidebar_width());
    }

    #[test]
    fn test_toggle_sidebar_once() {
        let mut window = MainWindow::new();
        assert!(window.sidebar_visible());
        window.toggle_sidebar();
        assert!(!window.sidebar_visible());
    }

    #[test]
    fn test_toggle_sidebar_twice_returns_to_original() {
        let mut window = MainWindow::new();
        let original = window.sidebar_visible();
        window.toggle_sidebar();
        window.toggle_sidebar();
        assert_eq!(window.sidebar_visible(), original);
    }

    #[test]
    fn test_set_sidebar_width_within_range() {
        let mut window = MainWindow::new();
        window.set_sidebar_width(300.0);
        assert_eq!(window.sidebar_width(), 300.0);
    }

    #[test]
    fn test_set_sidebar_width_clamps_below_min() {
        let mut window = MainWindow::new();
        window.set_sidebar_width(10.0);
        assert_eq!(window.sidebar_width(), 150.0);
    }

    #[test]
    fn test_set_sidebar_width_clamps_above_max() {
        let mut window = MainWindow::new();
        window.set_sidebar_width(9999.0);
        assert_eq!(window.sidebar_width(), 500.0);
    }

    #[test]
    fn test_set_sidebar_width_at_exact_min_boundary() {
        let mut window = MainWindow::new();
        window.set_sidebar_width(150.0);
        assert_eq!(window.sidebar_width(), 150.0);
    }

    #[test]
    fn test_set_sidebar_width_at_exact_max_boundary() {
        let mut window = MainWindow::new();
        window.set_sidebar_width(500.0);
        assert_eq!(window.sidebar_width(), 500.0);
    }

    #[test]
    fn test_set_sidebar_width_negative_clamps_to_min() {
        let mut window = MainWindow::new();
        window.set_sidebar_width(-50.0);
        assert_eq!(window.sidebar_width(), 150.0);
    }
}
