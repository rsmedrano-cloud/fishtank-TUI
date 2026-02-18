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
    pub particles: Vec<Particle>,
    pub show_shop: bool,
    pub show_achievements: bool,
    pub event_timer: f64, // For random events
    pub lucky_day_timer: f32, // For 2x income event
    pub show_minigame: bool,
    pub feeding_game: FeedingGame,
}

pub struct FeedingGame {
    pub food_x: f32,      // 0.0-1.0 horizontal position of falling food
    pub food_y: f32,      // 0.0-1.0 vertical position (0=top, 1=bottom)
    pub cursor_x: f32,    // Player cursor position at bottom
    pub score: u32,       // Catches this round
    pub misses: u32,      // Misses this round
    pub timer: f32,       // Seconds remaining
    pub food_speed: f32,  // How fast food falls
    pub round: u32,       // Foods dropped so far
    pub max_rounds: u32,  // Total foods to drop
}

pub struct Particle {
    pub x: f32,
    pub y: f32,
    pub speed: f32,
    pub symbol: char,
    pub lifetime: f32,
}

impl Particle {
    pub fn new(x: f32, y: f32, symbol: char) -> Self {
        Self {
            x,
            y,
            speed: 0.05 + (rand::random::<f32>() * 0.05),
            symbol,
            lifetime: 1.0, // 0.0 - 1.0 (fade out?) or just time based
        }
    }
}

impl FeedingGame {
    pub fn new() -> Self {
        Self {
            food_x: 0.5,
            food_y: 0.0,
            cursor_x: 0.5,
            score: 0,
            misses: 0,
            timer: 30.0,
            food_speed: 0.4,
            round: 0,
            max_rounds: 10,
        }
    }

    pub fn reset(&mut self) {
        self.food_x = 0.1 + rand::random::<f32>() * 0.8;
        self.food_y = 0.0;
        self.cursor_x = 0.5;
        self.score = 0;
        self.misses = 0;
        self.timer = 30.0;
        self.food_speed = 0.4;
        self.round = 0;
    }
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
            particles: Vec::new(),
            show_shop: false,
            show_achievements: false,
            event_timer: 0.0,
            lucky_day_timer: 0.0,
            show_minigame: false,
            feeding_game: FeedingGame::new(),
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

        // Algae Growth & Passive Income
        let dt = delta_seconds as f32; // Use real-time delta for these systems
        
        // Algae Growth: Base 0.02 per second (slower, more manageable)
        let purity_factor = 1.0 + (1.0 - self.save_data.water.purity.clamp(0.0, 1.0)) * 2.0;
        let growth_amount = 0.02 * purity_factor * dt;
        self.save_data.algae_level = (self.save_data.algae_level + growth_amount).min(100.0);

        // Passive Income: $0.05 per happy fish per second (~$3/min for 10 fish)
        let happy_fish = self.save_data.fish.iter().filter(|f| f.alive && f.happiness > 80.0).count();
        if happy_fish > 0 {
            self.save_data.money += 0.05 * happy_fish as f32 * dt;
        }
        
        // Corner Plant Growth: Slow, aesthetic feature
        // Base: 1 level every ~120 seconds, faster with Plants equipment
        let plant_growth_rate = if self.save_data.equipment.has_plants { 
            0.01 // ~100 sec per level with plants
        } else { 
            0.008 // ~125 sec per level without
        };
        
        // Grow left plant (max 8 high)
        if self.save_data.left_plant_height < 8 && rand::random::<f32>() < plant_growth_rate * dt {
            self.save_data.left_plant_height += 1;
        }
        
        // Grow right plant (max 8 high)
        if self.save_data.right_plant_height < 8 && rand::random::<f32>() < plant_growth_rate * dt {
            self.save_data.right_plant_height += 1;
        }
        
        // Random Events System (~1% chance per minute = ~1 event per 100 min)
        self.event_timer += delta_seconds;
        if self.event_timer >= 60.0 { // Check every minute
            self.event_timer = 0.0;
            if rand::random::<f32>() < 0.01 { // 1% chance
                self.trigger_random_event();
            }
        }
        
        // Lucky Day event: 2x income multiplier
        if self.lucky_day_timer > 0.0 {
            self.lucky_day_timer -= dt;
        }
        
        // Passive Income (with Lucky Day multiplier)
        let income_mult = if self.lucky_day_timer > 0.0 { 2.0 } else { 1.0 };
        let happy_fish = self.save_data.fish.iter().filter(|f| f.alive && f.happiness > 80.0).count();
        if happy_fish > 0 {
            let income = 0.05 * happy_fish as f32 * dt * income_mult;
            self.save_data.money += income;
            self.save_data.total_money_earned += income;
        }

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
                 self.save_data.total_fish_bred += 1;
                 self.add_notification("💕 Love is in the water! A baby is born!".to_string());
                 self.check_achievements(); // Check for First Fry achievement
             }
        }

        // Animation frame
        self.animation_frame = (self.animation_frame + 1) % 60;

        // --- PARTICLE SYSTEMS ---
        // Spawn bubbles if filter is on
        if self.save_data.equipment.has_filter && !self.save_data.is_frozen {
            if rand::random::<f32>() < 0.2 { // 20% chance per frame
                 self.particles.push(Particle::new(0.1 + (rand::random::<f32>() * 0.05), 0.9, 'o'));
            }
            if rand::random::<f32>() < 0.2 { 
                 self.particles.push(Particle::new(0.85 + (rand::random::<f32>() * 0.05), 0.9, '.'));
            }
        }
        
        // Update Particles
        // delta_seconds is available in scope
        let dt = delta_seconds as f32;
        self.particles.retain_mut(|p| {
            p.y -= p.speed * dt;
            
            // Wobble
            p.x += (rand::random::<f32>() - 0.5) * 0.01;
            
            p.y > 0.0 // Keep if below surface
        });

        // Auto-save every 30 seconds
        self.auto_save_timer += delta_seconds;
        if self.auto_save_timer >= 30.0 {
            let _ = self.save_data.save();
            self.auto_save_timer = 0.0;
        }

        // Update mini-game if active
        if self.show_minigame {
            self.feeding_game.food_y += self.feeding_game.food_speed * delta_seconds as f32;
            
            // Food reached the bottom — check if caught
            if self.feeding_game.food_y >= 1.0 {
                let distance = (self.feeding_game.food_x - self.feeding_game.cursor_x).abs();
                if distance < 0.08 {
                    // Caught!
                    self.feeding_game.score += 1;
                    self.add_notification(format!("🎯 Catch! Score: {}", self.feeding_game.score));
                } else {
                    self.feeding_game.misses += 1;
                }
                
                self.feeding_game.round += 1;
                
                // Check if game over
                if self.feeding_game.round >= self.feeding_game.max_rounds {
                    // Game over — award prizes
                    let money = self.feeding_game.score as f32 * 0.50;
                    self.save_data.money += money;
                    self.save_data.total_money_earned += money;
                    
                    // Feed fish proportionally to score
                    if self.feeding_game.score > 0 {
                        let feed_amount = (self.feeding_game.score as f32 / self.feeding_game.max_rounds as f32) * 30.0;
                        for fish in &mut self.save_data.fish {
                            if fish.alive {
                                fish.hunger = (fish.hunger + feed_amount).min(100.0);
                                fish.happiness = (fish.happiness + 5.0).min(100.0);
                            }
                        }
                    }
                    
                    self.add_notification(format!(
                        "🎮 Game Over! {}/{} caught! Earned ${:.2}",
                        self.feeding_game.score, self.feeding_game.max_rounds, money
                    ));
                    self.show_minigame = false;
                    self.check_achievements();
                } else {
                    // Spawn next food pellet
                    self.feeding_game.food_x = 0.1 + rand::random::<f32>() * 0.8;
                    self.feeding_game.food_y = 0.0;
                    // Slightly increase speed each round
                    self.feeding_game.food_speed += 0.02;
                }
            }
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        // Mini-game mode: intercept keys
        if self.show_minigame {
            match key.code {
                KeyCode::Left => {
                    self.feeding_game.cursor_x = (self.feeding_game.cursor_x - 0.05).max(0.05);
                }
                KeyCode::Right => {
                    self.feeding_game.cursor_x = (self.feeding_game.cursor_x + 0.05).min(0.95);
                }
                KeyCode::Esc | KeyCode::Char('g') | KeyCode::Char('G') => {
                    self.show_minigame = false;
                    self.add_notification("🎮 Mini-game cancelled.".to_string());
                }
                _ => {}
            }
            return;
        }

        if self.show_shop {
            match key.code {
                KeyCode::Char('p') | KeyCode::Esc => self.show_shop = false,
                KeyCode::Char('1') => { // Filter ($50)
                    if !self.save_data.equipment.has_filter {
                        if self.save_data.money >= 50.0 {
                            self.save_data.money -= 50.0;
                            self.save_data.equipment.has_filter = true;
                            self.add_notification("⚡ Bought Filter! Water  stays cleaner.".to_string());
                            self.check_achievements();
                        } else {
                            self.add_notification("💸 Not enough money! Need $50.".to_string());
                        }
                    } else {
                        self.add_notification("✅ Already own Filter!".to_string());
                    }
                }
                KeyCode::Char('2') => { // Heater ($40)
                    if !self.save_data.equipment.has_heater {
                        if self.save_data.money >= 40.0 {
                            self.save_data.money -= 40.0;
                            self.save_data.equipment.has_heater = true;
                            self.add_notification("🌡️ Bought Heater! Temperature stable.".to_string());
                            self.check_achievements();
                        } else {
                            self.add_notification("💸 Not enough money! Need $40.".to_string());
                        }
                    } else {
                        self.add_notification("✅ Already own Heater!".to_string());
                    }
                }
                KeyCode::Char('3') => { // Plants ($30)
                    if !self.save_data.equipment.has_plants {
                        if self.save_data.money >= 30.0 {
                            self.save_data.money -= 30.0;
                            self.save_data.equipment.has_plants = true;
                            self.add_notification("🌿 Bought Live Plants! Better water.".to_string());
                            self.check_achievements();
                        } else {
                            self.add_notification("💸 Not enough money! Need $30.".to_string());
                        }
                    } else {
                        self.add_notification("✅ Already own Plants!".to_string());
                    }
                }
                KeyCode::Char('4') => { // Decoration ($20)
                    if self.save_data.money >= 20.0 {
                         self.save_data.money -= 20.0;
                         // Add random decoration
                        let types = [crate::models::DecorationType::Rock, crate::models::DecorationType::Plant, crate::models::DecorationType::Castle, crate::models::DecorationType::Skull];
                        let mut placed = false;
                        for _ in 0..10 {
                            let rand_type = types[rand::random::<usize>() % types.len()];
                            let x = rand::random::<f32>().clamp(0.1, 0.9);
                            let overlap = self.save_data.decorations.iter().any(|d| (d.position.0 - x).abs() < 0.15);
                            if !overlap {
                                self.save_data.decorations.push(crate::models::Decoration::new(rand_type, (x, 0.0)));
                                placed = true;
                                break;
                            }
                        }
                        if placed {
                             self.add_notification("🏰 Bought a decoration!".to_string());
                        } else {
                             self.save_data.money += 20.0; // Refund
                             self.add_notification("❌ No space! Refunded.".to_string());
                        }
                    } else {
                        self.add_notification("💸 Not enough money! Need $20.".to_string());
                    }
                }
                _ => {}
            }
            return;
        }

        match key.code {
            KeyCode::Char('q') => {
                self.state = AppState::Quit;
            }
            KeyCode::Char('p') | KeyCode::Char('P') => self.show_shop = true,
            KeyCode::Char('f') | KeyCode::Char('F') => {
                self.feed_fish();
            }
            KeyCode::Char('n') | KeyCode::Char('N') => {
                self.new_fish();
            }
            KeyCode::Char('c') | KeyCode::Char('C') => {
                self.clear_notifications();
            }
            KeyCode::Char('r') | KeyCode::Char('R') => {
                self.restart_tank();
            }
            KeyCode::Char('w') | KeyCode::Char('W') => {
                self.clean_tank();
            }
            KeyCode::Char('e') | KeyCode::Char('E') => {
                self.toggle_equipment();
            }
            KeyCode::Char('z') | KeyCode::Char('Z') => {
                self.toggle_freeze();
            }
            KeyCode::Char('t') | KeyCode::Char('T') => {
                self.toggle_theme();
            }
            KeyCode::Char('d') | KeyCode::Char('D') => {
                self.add_notification("Use [P] to buy decorations now!".to_string());
            }
            KeyCode::Char('s') | KeyCode::Char('S') => {
                 if self.save_data.algae_level > 1.0 {
                     let difficulty_multiplier = self.save_data.algae_level / 100.0;
                     let payout = 0.50 + (2.0 * difficulty_multiplier);
                     
                     self.save_data.algae_level = (self.save_data.algae_level - 20.0).max(0.0);
                     self.save_data.money += payout;
                     self.save_data.total_money_earned += payout;
                     self.save_data.clean_count += 1;
                     
                     self.add_notification(format!("🧽 Scrubbed! Earned ${:.2}", payout));
                     self.check_achievements(); // Check clean_100
                 } else {
                     self.add_notification("✨ Glass is clean! Wait for algae to grow to earn $.".to_string());
                 }
            }
            KeyCode::Char('a') | KeyCode::Char('A') => {
                self.show_achievements = !self.show_achievements;
            }
            KeyCode::Char('g') | KeyCode::Char('G') => {
                if self.save_data.fish.iter().any(|f| f.alive) {
                    self.feeding_game.reset();
                    self.show_minigame = true;
                    self.add_notification("🎮 Feeding Game! ←→ to move, catch the food!".to_string());
                } else {
                    self.add_notification("❌ Need fish to play! Press N first.".to_string());
                }
            }
            KeyCode::Char('x') => {
                self.save_data.decorations.pop(); // Remove last one
                self.add_notification("🗑️ Removed last decoration.".to_string());
            }
            KeyCode::Char('X') => {
                self.save_data.decorations.clear(); // Remove all
                self.add_notification("💥 Cleared all decorations!".to_string());
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
        
        // Count only alive fish
        let alive_count = self.save_data.fish.iter().filter(|f| f.alive).count();
        
        if alive_count >= MAX_FISH {
            self.add_notification(format!("⚠️  Tank full! Maximum {} fish.", MAX_FISH));
            return;
        }
        
        // If there are dead fish, remove them first to make room
        self.save_data.fish.retain(|f| f.alive);

        // Rotate species (0..14)
        self.selected_species = (self.selected_species + 1) % 15;
        
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
            8 => ("Seahorse", "🌊"),
            9 => ("Swordfish", "⚔️"),
            10 => ("Discus", "🔵"),
            11 => ("Piranha", "🦷"),
            12 => ("Jellyfish", "🪼"),
            13 => ("Tang", "💙"),
            14 => ("Catfish", "🐱"),
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
            8 => Fish::new_seahorse(name.clone()),
            9 => Fish::new_swordfish(name.clone()),
            10 => Fish::new_discus(name.clone()),
            11 => Fish::new_piranha(name.clone()),
            12 => Fish::new_jellyfish(name.clone()),
            13 => Fish::new_tang(name.clone()),
            14 => Fish::new_catfish(name.clone()),
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
    
    fn trigger_random_event(&mut self) {
        let events = ["mystery_delivery", "lucky_day", "surprise_fry", "algae_eater", "power_surge", "disease", "cloudy_water"];
        let event = events[rand::random::<usize>() % events.len()];
        
        match event {
            "mystery_delivery" => {
                let types = [crate::models::DecorationType::Rock, crate::models::DecorationType::Plant, 
                             crate::models::DecorationType::Castle, crate::models::DecorationType::Skull];
                let rand_type = types[rand::random::<usize>() % types.len()];
                let x = rand::random::<f32>().clamp(0.1, 0.9);
                if self.save_data.decorations.iter().all(|d| (d.position.0 - x).abs() >= 0.15) {
                    self.save_data.decorations.push(crate::models::Decoration::new(rand_type, (x, 0.0)));
                    self.add_notification("🎁 Mystery Delivery! Free decoration!".to_string());
                }
            }
            "lucky_day" => {
                self.lucky_day_timer = 120.0;
                self.add_notification("⭐ Lucky Day! 2x income for 2 min!".to_string());
            }
            "surprise_fry" => {
                if self.save_data.fish.iter().filter(|f| f.alive).count() < 10 {
                    let species_id = rand::random::<usize>() % 15;
                    let name = format!("Surprise {}", self.save_data.fish.len() + 1);
                    let mut new_fish = match species_id {
                        0 => crate::models::Fish::new_goldfish(name),
                        1 => crate::models::Fish::new_betta(name),
                        2 => crate::models::Fish::new_guppy(name),
                        3 => crate::models::Fish::new_neon_tetra(name),
                        4 => crate::models::Fish::new_angelfish(name),
                        5 => crate::models::Fish::new_clownfish(name),
                        6 => crate::models::Fish::new_koi(name),
                        7 => crate::models::Fish::new_pufferfish(name),
                        8 => crate::models::Fish::new_seahorse(name),
                        9 => crate::models::Fish::new_swordfish(name),
                        10 => crate::models::Fish::new_discus(name),
                        11 => crate::models::Fish::new_piranha(name),
                        12 => crate::models::Fish::new_jellyfish(name),
                        13 => crate::models::Fish::new_tang(name),
                        14 => crate::models::Fish::new_catfish(name),
                        _ => crate::models::Fish::new_goldfish(name),
                    };
                    // Set as baby fish
                    new_fish.stage = crate::models::GrowthStage::Fry;
                    new_fish.position = (0.5, 0.5);
                    self.save_data.fish.push(new_fish);
                    self.add_notification("🐣 Surprise Fry appeared!".to_string());
                }
            }
            "algae_eater" => {
                self.save_data.algae_level *= 0.5;
                self.add_notification("🐌 Algae Eater visited! -50% algae!".to_string());
            }
            "power_surge" => {
                let mut broken = vec![];
                if self.save_data.equipment.has_filter { broken.push("filter"); }
                if self.save_data.equipment.has_heater { broken.push("heater"); }
                if self.save_data.equipment.has_plants { broken.push("plants"); }
                
                if !broken.is_empty() {
                    match broken[rand::random::<usize>() % broken.len()] {
                        "filter" => { self.save_data.equipment.has_filter = false; self.add_notification("⚡ Power Surge! Filter broken!".to_string()); }
                        "heater" => { self.save_data.equipment.has_heater = false; self.add_notification("⚡ Power Surge! Heater broken!".to_string()); }
                        "plants" => { self.save_data.equipment.has_plants = false; self.add_notification("⚡ Power Surge! Plants died!".to_string()); }
                        _ => {}
                    }
                }
            }
            "disease" => {
                for fish in &mut self.save_data.fish {
                    if fish.alive { fish.health = (fish.health - 20.0).max(1.0); }
                }
                self.add_notification("🦠 Disease outbreak! -20 health!".to_string());
            }
            "cloudy_water" => {
                self.save_data.water.purity = (self.save_data.water.purity - 30.0).max(0.0);
                self.add_notification("☁️ Cloudy water! -30 purity!".to_string());
            }
            _ => {}
        }
    }
    
    fn check_achievements(&mut self) {
        let mut unlocked = Vec::new();
        
        for achievement in &mut self.save_data.achievements {
            if achievement.unlocked { continue; }
            
            let should_unlock = match achievement.id.as_str() {
                "first_fry" => self.save_data.total_fish_bred > 0,
                "money_100" => self.save_data.total_money_earned >= 100.0,
                "money_500" => self.save_data.total_money_earned >= 500.0,
                "money_1000" => self.save_data.total_money_earned >= 1000.0,
                "fish_10" => self.save_data.total_fish_bred >= 10,
                "fish_25" => self.save_data.total_fish_bred >= 25,
                "fish_50" => self.save_data.total_fish_bred >= 50,
                "clean_100" => self.save_data.clean_count >= 100,
                "deco_10" => self.save_data.decorations.len() >= 10,
                "time_24h" => self.save_data.total_time >= 86400.0, // 24h in seconds
                "time_48h" => self.save_data.total_time >= 172800.0, // 48h
                "all_equipment" => self.save_data.equipment.has_filter && 
                                   self.save_data.equipment.has_heater && 
                                   self.save_data.equipment.has_plants,
                "max_plants" => self.save_data.left_plant_height == 8 && self.save_data.right_plant_height == 8,
                _ => false,
            };
            
            if should_unlock {
                achievement.unlocked = true;
                unlocked.push(format!("🏆 Achievement: {}", achievement.name));
            }
        }
        
        for msg in unlocked {
            self.add_notification(msg);
        }
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
