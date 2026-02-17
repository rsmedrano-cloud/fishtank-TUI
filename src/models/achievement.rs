use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Achievement {
    pub id: String,
    pub name: String,
    pub description: String,
    pub unlocked: bool,
    pub icon: String,
}

impl Achievement {
    pub fn new(id: &str, name: &str, description: &str, icon: &str) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            description: description.to_string(),
            unlocked: false,
            icon: icon.to_string(),
        }
    }
}

pub fn create_achievements() -> Vec<Achievement> {
    vec![
        Achievement::new("first_fry", "First Fry", "Breed your first baby fish", "🐣"),
        Achievement::new("money_100", "Entrepreneur I", "Earn $100 total", "💰"),
        Achievement::new("money_500", "Entrepreneur II", "Earn $500 total", "💎"),
        Achievement::new("money_1000", "Tycoon", "Earn $1000 total", "👑"),
        Achievement::new("fish_10", "Aquarist I", "Raise 10 total fish", "🐟"),
        Achievement::new("fish_25", "Aquarist II", "Raise 25 total fish", "🐠"),
        Achievement::new("fish_50", "Master Aquarist", "Raise 50 total fish", "🌊"),
        Achievement::new("clean_100", "Janitor", "Clean tank 100 times", "🧽"),
        Achievement::new("deco_10", "Decorator", "Place 10 decorations", "🏰"),
        Achievement::new("time_24h", "Veteran", "Keep tank alive for 24 hours", "⏱️"),
        Achievement::new("time_48h", "Dedicated", "Keep tank alive for 48 hours", "⏰"),
        Achievement::new("all_equipment", "Fully Equipped", "Buy all equipment", "⚙️"),
        Achievement::new("max_plants", "Gardener", "Grow corner plants to max height", "🌿"),
    ]
}
