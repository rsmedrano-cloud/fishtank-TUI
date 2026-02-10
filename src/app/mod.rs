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
                fish.update(capped_seconds);
                if !fish.alive {
                    deaths += 1;
                }
            }
            
            if deaths > 0 {
                notifications.push(format!("💀 {} fish passed away during your absence...", deaths));
            }
        }
        
        Ok(Self {
            state: AppState::Running,
            save_data,
            last_update: Utc::now(),
            animation_frame: 0,
            auto_save_timer: 0.0,
            notifications,
        })
    }

    pub fn update(&mut self, delta_seconds: f64) {
        // Update all fish
        for fish in &mut self.save_data.fish {
            fish.update(delta_seconds);
            fish.update_position(delta_seconds);
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

        // Generate name based on count
        let fish_names = ["Goldie", "Bubbles", "Splash"];
        let name = fish_names[self.save_data.fish.len()].to_string();
        
        let fish = Fish::new_goldfish(name.clone());
        self.save_data.fish.push(fish);
        self.add_notification(format!("✨ Welcome {}! ({}/{})", name, self.save_data.fish.len(), MAX_FISH));
    }

    fn clear_notifications(&mut self) {
        self.notifications.clear();
    }

    fn restart_tank(&mut self) {
        self.save_data.fish.clear();
        self.notifications.clear();
        self.add_notification("🔄 Tank restarted! Press 'N' to add fish.");
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
