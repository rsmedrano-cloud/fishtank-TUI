use anyhow::Result;
use chrono::Utc;
use crossterm::event::{KeyCode, KeyEvent};

use crate::models::Fish;
use crate::persistence::SaveData;

pub enum AppState {
    Running,
    Quit,
}

pub struct App {
    pub state: AppState,
    pub save_data: SaveData,
    pub last_update: chrono::DateTime<Utc>,
    pub animation_frame: u8,
    pub auto_save_timer: f64,
    pub notifications: Vec<String>,
    pub selected_species: usize,  // For cycling through species
    pub start_time: chrono::DateTime<Utc>,  // For day/night cycle calculation
}

impl App {
    pub fn new() -> Result<Self> {
        let mut save_data = SaveData::load()?;
        
        // Apply offline progression if fish exists
        let elapsed = save_data.time_since_last_save();
        let elapsed_seconds = elapsed.num_seconds() as f64;
        
        // Cap offline time to prevent excessive decay (24 hours max)
        let capped_seconds = elapsed_seconds.min(24.0 * 3600.0);
        
        let mut notifications = Vec::new();
        
        if !save_data.fish.is_empty() && capped_seconds > 60.0 {
            notifications.push(format!(
                "⏰ You were away for {}. Your fish missed you!",
                Self::format_duration(elapsed_seconds)
            ));
            
            let mut deaths = 0;
            for fish in &mut save_data.fish {
                // For offline updates, assume average water quality or use last known if possible
                // Using default acceptable water to prevent mass death from offline updates
                let default_water = crate::persistence::WaterParams::default(); 
                fish.update(capped_seconds, &default_water);
                if !fish.alive {
                    deaths += 1;
                }
            }
            
            if deaths > 0 {
                notifications.push(format!("💀 {} fish passed away during your absence...", deaths));
            }
        }
        
        let now = Utc::now();
        
        Ok(Self {
            state: AppState::Running,
            save_data,
            last_update: now,
            animation_frame: 0,
            auto_save_timer: 0.0,
            notifications,
            selected_species: 0,
            start_time: now,
        })
    }

    pub fn update(&mut self, delta_seconds: f64) {
        let is_night = self.is_night();

        // Update water quality
        let hours = delta_seconds / 3600.0;
        
        // Purity degrades over time (-1.0 per hour, faster with more fish)
        let degradation_rate = 1.0 + (self.save_data.fish.len() as f32 * 0.5);
        self.save_data.water.purity = (self.save_data.water.purity - (degradation_rate * hours as f32)).max(0.0);
        
        // Temperature fluctuations (Warmer day, Cooler night)
        let target_temp = if is_night { 23.0 } else { 26.0 };
        let temp_diff = target_temp - self.save_data.water.temperature;
        self.save_data.water.temperature += temp_diff * (0.5 * hours as f32); // Slow drift

        // Update all fish
        for fish in &mut self.save_data.fish {
            // Pass water params to fish update
            fish.update(delta_seconds, &self.save_data.water);
            fish.update_position(delta_seconds);
            
            // Apply day/night behavior
            fish.update_for_time_of_day(is_night);
        }

        // Animation frame
        self.animation_frame = (self.animation_frame + 1) % 60;

        // Auto-save every 30 seconds
        self.auto_save_timer += delta_seconds;
        if self.auto_save_timer >= 30.0 {
            let _ = self.save_data.save();
            self.auto_save_timer = 0.0;
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => {
                self.state = AppState::Quit;
            }
            KeyCode::Char('f') => {
                self.feed_fish();
            }
            KeyCode::Char('n') => {
                self.new_fish();
            }
            KeyCode::Char('c') => {
                self.clear_notifications();
            }
            KeyCode::Char('r') => {
                self.restart_tank();
            }
            KeyCode::Char('w') => {
                self.clean_tank();
            }
            _ => {}
        }
    }

    fn feed_fish(&mut self) {
        if self.save_data.fish.is_empty() {
            self.add_notification("❌ No fish in tank! Press 'N' to add one.");
            return;
        }

        let mut fed_count = 0;
        for fish in &mut self.save_data.fish {
            if fish.alive {
                fish.feed();
                fed_count += 1;
            }
        }

        if fed_count > 0 {
            self.add_notification(format!("🍽️  Fed {} fish!", fed_count));
        } else {
            self.add_notification("💀 All fish have passed away.");
        }
    }

    fn new_fish(&mut self) {
        const MAX_FISH: usize = 3;
        
        if self.save_data.fish.len() >= MAX_FISH {
            self.add_notification(format!("⚠️  Tank full! Maximum {} fish.", MAX_FISH));
            return;
        }

        // Rotate species
        self.selected_species = (self.selected_species + 1) % 5;
        
        // Get species info
        let (species_name, emoji) = match self.selected_species {
            0 => ("Goldfish", "🟡"),
            1 => ("Betta", "🔵"),
            2 => ("Guppy", "🟢"),
            3 => ("Neon Tetra", "🔴"),
            4 => ("Angelfish", "⚪"),
            _ => ("Goldfish", "🟡"),
        };
        
        // Generate name based on count
        let fish_names = ["Goldie", "Bubbles", "Splash"];
        let name = fish_names[self.save_data.fish.len()].to_string();
        
        // Create fish based on selected species
        let fish = match self.selected_species {
            0 => Fish::new_goldfish(name.clone()),
            1 => Fish::new_betta(name.clone()),
            2 => Fish::new_guppy(name.clone()),
            3 => Fish::new_neon_tetra(name.clone()),
            4 => Fish::new_angelfish(name.clone()),
            _ => Fish::new_goldfish(name.clone()),
        };
        
        self.save_data.fish.push(fish);
        self.add_notification(format!("✨ {} {} added! ({}/{})", emoji, species_name, self.save_data.fish.len(), MAX_FISH));
    }

    fn clear_notifications(&mut self) {
        self.notifications.clear();
    }

    fn restart_tank(&mut self) {
        self.save_data.fish.clear();
        self.save_data.water = SaveData::default().water; // Reset water too
        self.notifications.clear();
        self.add_notification("🔄 Tank restarted! Press 'N' to add fish.");
    }

    fn clean_tank(&mut self) {
        if self.save_data.water.purity >= 100.0 {
            self.add_notification("✨ Water is already crystal clear!");
            return;
        }
        
        self.save_data.water.purity = (self.save_data.water.purity + 30.0).min(100.0);
        self.save_data.water.ph = 7.0; // Stabilize pH
        self.add_notification("🧼 Water changed! Tank is cleaner.");
    }

    /// Get current game time (accelerated 2x - 12 hour real = 24 hour game)
    pub fn get_game_time(&self) -> (u8, u8) {
        let elapsed = Utc::now().signed_duration_since(self.start_time);
        let real_seconds = elapsed.num_seconds() as f64;
        
        // 2x speed: 1 real hour = 2 game hours
        let game_seconds = (real_seconds * 2.0) as i64;
        let game_time = game_seconds % (24 * 3600);  // 24-hour cycle
        
        let hour = (game_time / 3600) as u8;
        let minute = ((game_time % 3600) / 60) as u8;
        (hour, minute)
    }

    /// Check if it's currently night time
    pub fn is_night(&self) -> bool {
        let (hour, _) = self.get_game_time();
        hour < 6 || hour >= 18
    }

    pub fn add_notification(&mut self, msg: impl Into<String>) {
        self.notifications.push(msg.into());
        // Keep only last 5 notifications
        if self.notifications.len() > 5 {
            self.notifications.remove(0);
        }
    }

    fn format_duration(seconds: f64) -> String {
        let hours = (seconds / 3600.0) as i64;
        let minutes = ((seconds % 3600.0) / 60.0) as i64;
        
        if hours > 24 {
            let days = hours / 24;
            format!("{}d {}h", days, hours % 24)
        } else if hours > 0 {
            format!("{}h {}m", hours, minutes)
        } else {
            format!("{}m", minutes)
        }
    }

    pub fn save_and_quit(&mut self) -> Result<()> {
        self.save_data.save()?;
        Ok(())
    }
}
