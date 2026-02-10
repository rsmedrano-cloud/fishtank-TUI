use crate::models::{Fish, Species, GrowthStage};

/// ASCII fish sprites - simple and compact like asciiquarium
pub struct FishSprite;

impl FishSprite {
    /// Get fish sprite based on species and growth stage
    /// Returns a slice of strings (lines)
    pub fn from_fish(fish: &Fish, _frame: u8) -> &[&'static str] {
        let facing_right = fish.velocity.0 >= 0.0;
        
        match fish.stage {
            GrowthStage::Fry => {
                 if facing_right { &[".>"] } else { &["<."] }
            },
            GrowthStage::Juvenile => {
                match fish.species {
                    Species::Goldfish => if facing_right { &["><>"] } else { &["<><"] },
                    Species::Betta => if facing_right { &[">∫>"] } else { &["<∫<"] },
                    Species::Guppy => if facing_right { &[">°>"] } else { &["<°<"] },
                    Species::NeonTetra => if facing_right { &[">->"] } else { &["<-<"] },
                    Species::Angelfish => if facing_right { &[">^>"] } else { &["<^<"] },
                    Species::Clownfish => if facing_right { &[">|>"] } else { &["<|<"] },
                    Species::Koi => if facing_right { &[">S>"] } else { &["<S<"] },
                    Species::Pufferfish => if facing_right { &[">()"] } else { &["()<"] },
                }
            },
            GrowthStage::Adult => {
                // Multi-line sprites for adults
                match fish.species {
                    Species::Goldfish => if facing_right { 
                        &[
                            "  ,·´", 
                            "><(((º>" 
                        ] 
                    } else { 
                        &[
                             "  `·.",
                             "<º)))><" 
                        ] 
                    },
                    Species::Betta => if facing_right { 
                        &[
                            "  /\\", 
                            "«(ll)>>" 
                        ] 
                    } else { 
                        &[
                             "  /\\", 
                             "<<(ll)»" 
                        ] 
                    },
                    Species::Guppy => if facing_right { 
                        &[
                            " ¸.·´", 
                            "><>°>" 
                        ] 
                    } else { 
                        &[
                             " `·.¸", 
                             "<°<><" 
                        ] 
                    },
                    Species::NeonTetra => if facing_right { 
                        &[
                            "  __", 
                            "><===>" 
                        ] 
                    } else { 
                        &[
                             "  __", 
                             "<===><" 
                        ] 
                    },
                    Species::Angelfish => if facing_right { 
                        &[
                            "   />", 
                            " >( ))>" 
                        ] 
                    } else { 
                        &[
                             " <\\", 
                             "<(( )<" 
                        ] 
                    },
                    Species::Clownfish => if facing_right { 
                        &[
                            "   ,·", 
                            "><|(|)?>" 
                        ] 
                    } else { 
                        &[
                             "  ·,", 
                             "<?(|)|><" 
                        ] 
                    },
                    Species::Koi => if facing_right { 
                        &[
                            "   _,,", 
                            "><((('>" 
                        ] 
                    } else { 
                        &[
                             "   ,,_", 
                             "<')))><" 
                        ] 
                    },
                    Species::Pufferfish => if facing_right { 
                        &[
                            "  ..", 
                            ">(())>" 
                        ] 
                    } else { 
                        &[
                             "  ..", 
                             "<(( ))<" 
                        ] 
                    },
                }
            }
        }
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
