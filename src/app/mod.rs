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
        let mut notifications = Vec::new();
        
        if !save_data.is_frozen {
            // Apply offline progression if fish exists
            let elapsed = save_data.time_since_last_save();
            let elapsed_seconds = elapsed.num_seconds() as f64;
            
            // Cap offline time to prevent excessive decay (24 hours max)
            let capped_seconds = elapsed_seconds.min(24.0 * 3600.0);
            
            // Offline progression is slower (div by 3) to be fair
            // But game speed is 3x. So accumulating "Game Time" means:
            // - Online: 1 real sec = 3 game sec
            // - Offline: 1 real sec = 1 game sec (slower decay)
            let offline_game_seconds = (capped_seconds * 1.0); 

            if !save_data.fish.is_empty() && capped_seconds > 60.0 {
                notifications.push(format!(
                    "⏰ You were away for {}. fish aged naturally.",
                    Self::format_duration(elapsed_seconds)
                ));
                
                let mut deaths = 0;
                for fish in &mut save_data.fish {
                    // For offline updates, assume average water quality or use last known if possible
                    let default_water = crate::persistence::WaterParams::default(); 
                    fish.update(offline_game_seconds, &default_water);
                    if !fish.alive {
                        deaths += 1;
                    }
                }
                
                if deaths > 0 {
                    notifications.push(format!("💀 {} fish passed away during your absence...", deaths));
                }
            }
            
            // Update total time with offline duration (Game Time)
            // Advance world clock by 3x real time to maintain day/night cycle continuity
            save_data.total_time += (elapsed_seconds * 3.0);
        } else {
             notifications.push("❄️ Welcome back! World was FROZEN.".to_string());
        }
        
        // Calculate start time based on total game time
        let now = Utc::now();
        let start_time = now - chrono::Duration::seconds(save_data.total_time as i64);
        
        Ok(Self {
            state: AppState::Running,
            save_data,
            last_update: now,
            animation_frame: 0,
            auto_save_timer: 0.0,
            notifications,
            selected_species: 0,
            start_time,
        })
    }

    pub fn update(&mut self, delta_seconds: f64) {
        // If frozen, just verify auto-save and skip simulation
        if self.save_data.is_frozen {
            self.auto_save_timer += delta_seconds;
            if self.auto_save_timer >= 30.0 {
                let _ = self.save_data.save();
                self.auto_save_timer = 0.0;
            }
            return;
        }

        // Update accumulated time - 3x Speed: 1 Real Sec = 3 Game Sec
        let game_delta = delta_seconds * 3.0;
        self.save_data.total_time += game_delta;
        
        let is_night = self.is_night();

        // Update water quality
        let hours = game_delta / 3600.0; // Use game hours for simulation
        
        // Purity degrades over time (-1.0 per hour, faster with more fish)
        let mut degradation_rate = 1.0 + (self.save_data.fish.len() as f32 * 0.5);
        
        // Equipment effects
        if self.save_data.equipment.has_filter {
            degradation_rate *= 0.5; // Filter reduces dirtying by 50%
        }
        if self.save_data.equipment.has_plants {
             degradation_rate *= 0.9; // Plants help a little (10%)
        }

        self.save_data.water.purity = (self.save_data.water.purity - (degradation_rate * hours as f32)).max(0.0);
        
        // Temperature fluctuations (Warmer day, Cooler night)
        let target_temp = if is_night { 23.0 } else { 26.0 };
        let mut temp_diff = target_temp - self.save_data.water.temperature;
        
        if self.save_data.equipment.has_heater {
            temp_diff *= 0.2; // Heater stabilizes temp (80% reduction in fluctuation)
        }
        
        self.save_data.water.temperature += temp_diff * (0.5 * hours as f32);

        // Update all fish
        let mut new_fry = Vec::new();

        // 1. Basic Update & Movement (Iterate all)
        for fish in &mut self.save_data.fish {
             fish.update(game_delta, &self.save_data.water);
             fish.update_for_time_of_day(is_night);
             fish.update_position(delta_seconds);
        }

        // 2. Breeding Pass (Separate to avoid complex borrow issues in one loop)
        // We need mutable access to pairs.
        if self.save_data.fish.len() < 10 {
            let count = self.save_data.fish.len();
            for i in 0..count {
                for j in (i+1)..count {
                    // Use split_at_mut to get two mutable references
                    let (left, right) = self.save_data.fish.split_at_mut(j);
                    // left[i] is first fish, right[0] is second fish
                    
                    if let Some(fry) = left[i].try_breed(&mut right[0]) {
                        new_fry.push(fry);
                    }
                }
            }
        }
        
        // Add new fry
        for mut fry in new_fry {
             if self.save_data.fish.len() < 10 {
                 fry.name = format!("Baby {}", self.save_data.fish.len() + 1);
                 self.save_data.fish.push(fry);
                 self.add_notification("💕 Love is in the water! A baby is born!".to_string());
             }
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
            KeyCode::Char('e') => {
                self.toggle_equipment();
            }
            KeyCode::Char('z') => {
                self.toggle_freeze();
            }
            KeyCode::Char('t') => {
                self.toggle_theme();
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

    pub fn new_fish(&mut self) {
        const MAX_FISH: usize = 10;
        
        if self.save_data.fish.len() >= MAX_FISH {
            self.add_notification(format!("⚠️  Tank full! Maximum {} fish.", MAX_FISH));
            return;
        }

        // Rotate species (0..7)
        self.selected_species = (self.selected_species + 1) % 8;
        
        // Get species info
        let (species_name, emoji) = match self.selected_species {
            0 => ("Goldfish", "🟡"),
            1 => ("Betta", "🔵"),
            2 => ("Guppy", "🟢"),
            3 => ("Neon Tetra", "🔴"),
            4 => ("Angelfish", "⚪"),
            5 => ("Clownfish", "🟠"),
            6 => ("Koi", "🎏"),
            7 => ("Pufferfish", "🐡"),
            _ => ("Goldfish", "🟡"),
        };
        
        // Generate name based on count (or random)
        let fish_names = [
            "Goldie", "Bubbles", "Splash", "Finny", "Gill", 
            "Dorsal", "Nemo", "Dory", "Marlin", "Coral", 
            "Sushi", "Sashimi", "Scale", "Ripple", "Wave",
            "Azure", "Crimson", "Shadow", "Flash", "Spark"
        ];
        let name_idx = self.save_data.fish.len() % fish_names.len();
        let name = fish_names[name_idx].to_string();
        
        // Create fish based on selected species
        let fish = match self.selected_species {
            0 => Fish::new_goldfish(name.clone()),
            1 => Fish::new_betta(name.clone()),
            2 => Fish::new_guppy(name.clone()),
            3 => Fish::new_neon_tetra(name.clone()),
            4 => Fish::new_angelfish(name.clone()),
            5 => Fish::new_clownfish(name.clone()),
            6 => Fish::new_koi(name.clone()),
            7 => Fish::new_pufferfish(name.clone()),
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

    fn toggle_equipment(&mut self) {
        let eq = &mut self.save_data.equipment;
        
        // Simple cycle: None -> Filter -> Heater -> Plants -> All -> None
        if !eq.has_filter && !eq.has_heater && !eq.has_plants {
            eq.has_filter = true;
            self.add_notification("⚙️ Filter installed!");
        } else if eq.has_filter && !eq.has_heater {
            eq.has_heater = true;
            self.add_notification("🌡️ Heater installed!");
        } else if eq.has_filter && eq.has_heater && !eq.has_plants {
             eq.has_plants = true;
             self.add_notification("🌿 Plants added!");
        } else {
            eq.has_filter = false;
            eq.has_heater = false;
            eq.has_plants = false;
            self.add_notification("❌ All equipment removed.");
        }
    }

    fn toggle_freeze(&mut self) {
        self.save_data.is_frozen = !self.save_data.is_frozen;
        if self.save_data.is_frozen {
            self.add_notification("❄️  World FROZEN! (Weekend Mode)");
        } else {
            self.add_notification("▶️  World UNPAUSED!");
        }
    }

    fn toggle_theme(&mut self) {
        let themes = crate::ui::theme::ThemeManager::get_themes();
        self.save_data.theme_index = (self.save_data.theme_index + 1) % themes.len();
        self.add_notification(format!("🎨 Theme: {}", themes[self.save_data.theme_index].name));
    }

    pub fn get_current_theme(&self) -> crate::ui::theme::Theme {
        let themes = crate::ui::theme::ThemeManager::get_themes();
        themes.get(self.save_data.theme_index).cloned().unwrap_or_default()
    }

    /// Get current game time (accelerated 2x - 12 hour real = 24 hour game)
    /// Get current game time (accelerated 3x - 8 hour real = 24 hour game)
    pub fn get_game_time(&self) -> (u8, u8) {
        // We use the start_time delta, which effectively tracks total_time
        // properties: start_time = now - total_time
        // so: now - start_time = total_time
        // total_time is ALREADY accumulated at 3x speed in update()
        // So we just take the raw elapsed seconds as the Game Time.
        
        let elapsed = Utc::now().signed_duration_since(self.start_time);
        let game_seconds = elapsed.num_seconds();
        
        let game_time = game_seconds % (24 * 3600);  // 24-hour cycle
        
        // Wrap gracefully if time is negative (shouldn't happen but safe)
        let game_time = if game_time < 0 { 0 } else { game_time };
        
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
