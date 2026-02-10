use crate::models::Fish;

/// ASCII fish sprites with animation frames
pub struct FishSprite;

impl FishSprite {
    /// Get fish sprite based on fish index (for variety)
    pub fn get_sprite_for_fish(fish_id: &str, facing_right: bool) -> &'static str {
        // Use last char of UUID to determine fish style
        let fish_type = fish_id.chars().last().unwrap_or('0');
        
        match fish_type as u8 % 3 {
            0 => if facing_right { "><(((*>" } else { "<*)))><" },  // Classic fish
            1 => if facing_right { "><>>" } else { "<<><" },         // Simple fish
            _ => if facing_right { ">==>" } else { "<==" },          // Sleek fish
        }
    }
    
    /// Get fish sprite based on velocity
    pub fn from_fish(fish: &Fish, _frame: u8) -> &'static str {
        let facing_right = fish.velocity.0 >= 0.0;
        Self::get_sprite_for_fish(&fish.id.to_string(), facing_right)
    }
}

/// ASCII decorative elements
pub struct TankElements;

impl TankElements {
    pub fn water_line(width: usize) -> String {
        "≈".repeat(width)
    }

    pub fn substrate_line(width: usize) -> String {
        "▓".repeat(width)
    }

    #[allow(dead_code)]
    pub fn bubble() -> &'static str {
        "○"
    }

    #[allow(dead_code)]
    pub fn plant() -> &'static str {
        "Y"
    }
}

/// Helper to draw status bars
pub fn draw_stat_bar(value: f32, width: usize) -> String {
    let filled = ((value / 100.0) * width as f32) as usize;
    let empty = width.saturating_sub(filled);
    
    format!("{}{}", "█".repeat(filled), "░".repeat(empty))
}

/// Get color based on stat value
pub fn stat_color_indicator(value: f32) -> &'static str {
    if value >= 70.0 {
        "🟢" // Good
    } else if value >= 40.0 {
        "🟡" // Warning
    } else {
        "🔴" // Critical
    }
}
