use ratatui::style::Color;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    pub name: String,
    pub border_color: Color,
    pub title_color: Color,
    pub water_color: Color,      // e.g. Particles/bubbles
    pub substrate_color: Color,
    pub plant_color: Color,
    pub fish_default_color: Color,
    pub substrate_char: char,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            name: "Classic".to_string(),
            border_color: Color::Cyan,
            title_color: Color::Yellow,
            water_color: Color::Cyan,
            substrate_color: Color::Rgb(100, 100, 100), // Grey
            plant_color: Color::Green,
            fish_default_color: Color::Yellow,
            substrate_char: '▓',
        }
    }
}

pub struct ThemeManager;

impl ThemeManager {
    pub fn get_themes() -> Vec<Theme> {
        vec![
            Theme::default(), // Classic
            Theme {
                name: "Ocean Deep".to_string(),
                border_color: Color::Blue,
                title_color: Color::White,
                water_color: Color::Blue,
                substrate_color: Color::Rgb(200, 200, 255), // White sand
                plant_color: Color::Rgb(0, 100, 100), // Dark teal
                fish_default_color: Color::Magenta,
                substrate_char: '░',
            },
            Theme {
                name: "Matrix".to_string(),
                border_color: Color::Green,
                title_color: Color::Green,
                water_color: Color::DarkGray,
                substrate_color: Color::Black,
                plant_color: Color::Green,
                fish_default_color: Color::Green,
                substrate_char: '0',
            },
            Theme {
                name: "Retro Amber".to_string(),
                border_color: Color::Indexed(214), // Amber/Orange
                title_color: Color::Indexed(214),
                water_color: Color::Indexed(94), // Dim amber
                substrate_color: Color::Indexed(214),
                plant_color: Color::Indexed(172),
                fish_default_color: Color::Indexed(220),
                substrate_char: '#',
            },
            Theme {
                 name: "Zen Garden".to_string(),
                 border_color: Color::White,
                 title_color: Color::Red,
                 water_color: Color::White,
                 substrate_color: Color::Rgb(240, 230, 140), // Khaki sand
                 plant_color: Color::Black, // Ink wash style
                 fish_default_color: Color::Red, // Koi style
                 substrate_char: '≈',
            },
        ]
    }
}
