//! Theme definitions and parsing

#![allow(dead_code)]

use crate::terminal::Color;
use serde::{Deserialize, Serialize};

/// Available color themes for the terminal
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ColorTheme {
    Dracula,
    SolarizedDark,
    SolarizedLight,
    Nord,
    Monokai,
    OneDark,
    Gruvbox,
    TomorrowNight,
    HighContrast,
}

impl Default for ColorTheme {
    fn default() -> Self {
        Self::Dracula
    }
}

impl std::fmt::Display for ColorTheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ColorTheme::Dracula => write!(f, "Dracula"),
            ColorTheme::SolarizedDark => write!(f, "Solarized Dark"),
            ColorTheme::SolarizedLight => write!(f, "Solarized Light"),
            ColorTheme::Nord => write!(f, "Nord"),
            ColorTheme::Monokai => write!(f, "Monokai"),
            ColorTheme::OneDark => write!(f, "One Dark"),
            ColorTheme::Gruvbox => write!(f, "Gruvbox"),
            ColorTheme::TomorrowNight => write!(f, "Tomorrow Night"),
            ColorTheme::HighContrast => write!(f, "High Contrast"),
        }
    }
}

impl ColorTheme {
    pub fn all() -> &'static [ColorTheme] {
        &[
            ColorTheme::Dracula,
            ColorTheme::SolarizedDark,
            ColorTheme::SolarizedLight,
            ColorTheme::Nord,
            ColorTheme::Monokai,
            ColorTheme::OneDark,
            ColorTheme::Gruvbox,
            ColorTheme::TomorrowNight,
            ColorTheme::HighContrast,
        ]
    }

    pub fn to_theme(&self) -> Theme {
        match self {
            ColorTheme::Dracula => Theme::dracula(),
            ColorTheme::SolarizedDark => Theme::solarized_dark(),
            ColorTheme::SolarizedLight => Theme::solarized_light(),
            ColorTheme::Nord => Theme::nord(),
            ColorTheme::Monokai => Theme::monokai(),
            ColorTheme::OneDark => Theme::one_dark(),
            ColorTheme::Gruvbox => Theme::gruvbox(),
            ColorTheme::TomorrowNight => Theme::tomorrow_night(),
            ColorTheme::HighContrast => Theme::high_contrast(),
        }
    }
}

/// UI theme colors (for the application chrome, not terminal)
#[derive(Debug, Clone)]
pub struct ThemeColors {
    pub primary: [u8; 3],
    pub primary_hover: [u8; 3],
    pub secondary: [u8; 3],
    pub success: [u8; 3],
    pub warning: [u8; 3],
    pub danger: [u8; 3],
    pub info: [u8; 3],
    pub bg_primary: [u8; 3],
    pub bg_secondary: [u8; 3],
    pub bg_tertiary: [u8; 3],
    pub bg_surface: [u8; 3],
    pub text_primary: [u8; 3],
    pub text_secondary: [u8; 3],
    pub text_muted: [u8; 3],
    pub border: [u8; 3],
    pub border_focus: [u8; 3],
}

impl ThemeColors {
    pub fn dark() -> Self {
        Self {
            primary: [59, 130, 246],
            primary_hover: [37, 99, 235],
            secondary: [100, 116, 139],
            success: [34, 197, 94],
            warning: [234, 179, 8],
            danger: [239, 68, 68],
            info: [14, 165, 233],
            bg_primary: [15, 23, 42],
            bg_secondary: [30, 41, 59],
            bg_tertiary: [51, 65, 85],
            bg_surface: [71, 85, 105],
            text_primary: [248, 250, 252],
            text_secondary: [148, 163, 184],
            text_muted: [100, 116, 139],
            border: [71, 85, 105],
            border_focus: [59, 130, 246],
        }
    }

    pub fn light() -> Self {
        Self {
            primary: [37, 99, 235],
            primary_hover: [29, 78, 216],
            secondary: [100, 116, 139],
            success: [22, 163, 74],
            warning: [202, 138, 4],
            danger: [220, 38, 38],
            info: [2, 132, 199],
            bg_primary: [255, 255, 255],
            bg_secondary: [248, 250, 252],
            bg_tertiary: [241, 245, 249],
            bg_surface: [226, 232, 240],
            text_primary: [15, 23, 42],
            text_secondary: [71, 85, 105],
            text_muted: [148, 163, 184],
            border: [226, 232, 240],
            border_focus: [37, 99, 235],
        }
    }
}

/// Terminal color theme
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    pub name: String,
    pub background: [u8; 3],
    pub foreground: [u8; 3],
    pub cursor: [u8; 3],
    pub selection: [u8; 3],
    /// ANSI colors (16 colors: 8 normal + 8 bright)
    pub ansi: [[u8; 3]; 16],
}

impl Theme {
    pub fn dracula() -> Self {
        Self {
            name: "Dracula".to_string(),
            background: [40, 42, 54],
            foreground: [248, 248, 242],
            cursor: [248, 248, 242],
            selection: [68, 71, 90],
            ansi: [
                [33, 34, 44],
                [255, 85, 85],
                [80, 250, 123],
                [241, 250, 140],
                [189, 147, 249],
                [255, 121, 198],
                [139, 233, 253],
                [248, 248, 242],
                [98, 114, 164],
                [255, 110, 103],
                [90, 247, 142],
                [244, 249, 157],
                [202, 169, 250],
                [255, 146, 208],
                [154, 237, 254],
                [255, 255, 255],
            ],
        }
    }

    pub fn solarized_dark() -> Self {
        Self {
            name: "Solarized Dark".to_string(),
            background: [0, 43, 54],
            foreground: [131, 148, 150],
            cursor: [131, 148, 150],
            selection: [7, 54, 66],
            ansi: [
                [7, 54, 66],
                [220, 50, 47],
                [133, 153, 0],
                [181, 137, 0],
                [38, 139, 210],
                [211, 54, 130],
                [42, 161, 152],
                [238, 232, 213],
                [0, 43, 54],
                [203, 75, 22],
                [88, 110, 117],
                [101, 123, 131],
                [131, 148, 150],
                [108, 113, 196],
                [147, 161, 161],
                [253, 246, 227],
            ],
        }
    }

    pub fn solarized_light() -> Self {
        Self {
            name: "Solarized Light".to_string(),
            background: [253, 246, 227],
            foreground: [101, 123, 131],
            cursor: [101, 123, 131],
            selection: [238, 232, 213],
            ansi: [
                [238, 232, 213],
                [220, 50, 47],
                [133, 153, 0],
                [181, 137, 0],
                [38, 139, 210],
                [211, 54, 130],
                [42, 161, 152],
                [7, 54, 66],
                [253, 246, 227],
                [203, 75, 22],
                [88, 110, 117],
                [101, 123, 131],
                [131, 148, 150],
                [108, 113, 196],
                [147, 161, 161],
                [0, 43, 54],
            ],
        }
    }

    pub fn nord() -> Self {
        Self {
            name: "Nord".to_string(),
            background: [46, 52, 64],
            foreground: [216, 222, 233],
            cursor: [216, 222, 233],
            selection: [67, 76, 94],
            ansi: [
                [59, 66, 82],
                [191, 97, 106],
                [163, 190, 140],
                [235, 203, 139],
                [129, 161, 193],
                [180, 142, 173],
                [136, 192, 208],
                [229, 233, 240],
                [76, 86, 106],
                [191, 97, 106],
                [163, 190, 140],
                [235, 203, 139],
                [129, 161, 193],
                [180, 142, 173],
                [143, 188, 187],
                [236, 239, 244],
            ],
        }
    }

    pub fn monokai() -> Self {
        Self {
            name: "Monokai".to_string(),
            background: [39, 40, 34],
            foreground: [248, 248, 242],
            cursor: [248, 248, 242],
            selection: [73, 72, 62],
            ansi: [
                [39, 40, 34],
                [249, 38, 114],
                [166, 226, 46],
                [244, 191, 117],
                [102, 217, 239],
                [174, 129, 255],
                [161, 239, 228],
                [248, 248, 242],
                [117, 113, 94],
                [249, 38, 114],
                [166, 226, 46],
                [244, 191, 117],
                [102, 217, 239],
                [174, 129, 255],
                [161, 239, 228],
                [249, 248, 245],
            ],
        }
    }

    pub fn one_dark() -> Self {
        Self {
            name: "One Dark".to_string(),
            background: [40, 44, 52],
            foreground: [171, 178, 191],
            cursor: [171, 178, 191],
            selection: [62, 68, 81],
            ansi: [
                [40, 44, 52],
                [224, 108, 117],
                [152, 195, 121],
                [229, 192, 123],
                [97, 175, 239],
                [198, 120, 221],
                [86, 182, 194],
                [171, 178, 191],
                [92, 99, 112],
                [224, 108, 117],
                [152, 195, 121],
                [229, 192, 123],
                [97, 175, 239],
                [198, 120, 221],
                [86, 182, 194],
                [255, 255, 255],
            ],
        }
    }

    pub fn gruvbox() -> Self {
        Self {
            name: "Gruvbox".to_string(),
            background: [40, 40, 40],
            foreground: [235, 219, 178],
            cursor: [235, 219, 178],
            selection: [80, 73, 69],
            ansi: [
                [40, 40, 40],
                [204, 36, 29],
                [152, 151, 26],
                [215, 153, 33],
                [69, 133, 136],
                [177, 98, 134],
                [104, 157, 106],
                [168, 153, 132],
                [146, 131, 116],
                [251, 73, 52],
                [184, 187, 38],
                [250, 189, 47],
                [131, 165, 152],
                [211, 134, 155],
                [142, 192, 124],
                [235, 219, 178],
            ],
        }
    }

    pub fn tomorrow_night() -> Self {
        Self {
            name: "Tomorrow Night".to_string(),
            background: [29, 31, 33],
            foreground: [197, 200, 198],
            cursor: [197, 200, 198],
            selection: [55, 59, 65],
            ansi: [
                [29, 31, 33],
                [204, 102, 102],
                [181, 189, 104],
                [240, 198, 116],
                [129, 162, 190],
                [178, 148, 187],
                [138, 190, 183],
                [197, 200, 198],
                [150, 152, 150],
                [204, 102, 102],
                [181, 189, 104],
                [240, 198, 116],
                [129, 162, 190],
                [178, 148, 187],
                [138, 190, 183],
                [255, 255, 255],
            ],
        }
    }

    pub fn high_contrast() -> Self {
        Self {
            name: "High Contrast".to_string(),
            background: [0, 0, 0],
            foreground: [255, 255, 255],
            cursor: [255, 255, 255],
            selection: [68, 68, 68],
            ansi: [
                [0, 0, 0],
                [255, 0, 0],
                [0, 255, 0],
                [255, 255, 0],
                [0, 0, 255],
                [255, 0, 255],
                [0, 255, 255],
                [255, 255, 255],
                [128, 128, 128],
                [255, 128, 128],
                [128, 255, 128],
                [255, 255, 128],
                [128, 128, 255],
                [255, 128, 255],
                [128, 255, 255],
                [255, 255, 255],
            ],
        }
    }

    pub fn background_color(&self) -> Color {
        Color::rgb(self.background[0], self.background[1], self.background[2])
    }

    pub fn foreground_color(&self) -> Color {
        Color::rgb(self.foreground[0], self.foreground[1], self.foreground[2])
    }

    pub fn builtin_themes() -> Vec<Theme> {
        vec![
            Theme::dracula(),
            Theme::solarized_dark(),
            Theme::solarized_light(),
            Theme::nord(),
            Theme::monokai(),
            Theme::one_dark(),
            Theme::gruvbox(),
            Theme::tomorrow_night(),
            Theme::high_contrast(),
        ]
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::dracula()
    }
}
