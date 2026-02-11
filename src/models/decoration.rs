use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Decoration {
    pub id: Uuid,
    pub deco_type: DecorationType,
    pub position: (f32, f32), // 0.0-1.0 coords
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum DecorationType {
    Rock,
    Plant,
    Castle,
    Skull,
}

impl Decoration {
    pub fn new(deco_type: DecorationType, position: (f32, f32)) -> Self {
        Self {
            id: Uuid::new_v4(),
            deco_type,
            position,
        }
    }

    pub fn get_sprite(&self) -> Vec<&'static str> {
        match self.deco_type {
            DecorationType::Rock => vec![
                "      _.--r-._ ",
                "    /  _..--  \\ ",
                "   / .'.---.   \\ ",
                "  / / /     \\   \\ ",
                " /_/_/       \\_\\_\\",
                "|__  __________  __|",
                "   `'          `'   "
            ],
            DecorationType::Plant => vec![
                "      .   ",
                "   _.|._  ",
                "  /  |  \\ ",
                " |.  |  .|",
                "  \\_ | _/ ",
                "  / .|. \\ ",
                " / / | \\ \\",
                "| |  |  | |",
                " `   |   ` "
            ],
            DecorationType::Castle => vec![
                "      /\\      ",
                "     |  |     ",
                "    _|  |_    ",
                "   [______]   ",
                "   |      |   ",
                "  _|______|_  ",
                " |  _    _  | ",
                " | | |  | | | ",
                " |_|_|__|_|_| "
            ],
            DecorationType::Skull => vec![
                "    .---.    ",
                "   /     \\   ",
                "  | () () |  ",
                "   \\  ^  /   ",
                "    |||||    ",
                "    '---'    "
            ],
        }
    }

    pub fn get_width(&self) -> usize {
        self.get_sprite().iter().map(|s| s.len()).max().unwrap_or(0)
    }
}
