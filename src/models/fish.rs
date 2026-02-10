use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represents a single fish in the aquarium
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fish {
    pub id: Uuid,
    pub species: Species,
    pub name: String,
    
    // Core stats (0.0 - 100.0)
    pub hunger: f32,      // 0 = starving, 100 = full
    pub happiness: f32,   // Overall well-being
    pub health: f32,      // Physical health
    pub energy: f32,      // Tired = lower energy
    
    // State
    pub age: Duration,    // Time since birth
    pub position: (f32, f32),  // Tank coordinates (0.0-1.0)
    pub velocity: (f32, f32),  // Movement direction
    pub state: FishState,
    pub alive: bool,
    
    pub created_at: DateTime<Utc>,
    pub last_fed: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Species {
    Goldfish,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum FishState {
    Swimming,
    Eating,
    Resting,
    Dead,
}

impl Fish {
    pub fn new_goldfish(name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            species: Species::Goldfish,
            name,
            hunger: 80.0,
            happiness: 75.0,
            health: 100.0,
            energy: 100.0,
            age: Duration::zero(),
            position: (0.5, 0.5),
            velocity: (0.01, 0.0),
            state: FishState::Swimming,
            alive: true,
            created_at: Utc::now(),
            last_fed: None,
        }
    }

    /// Update fish stats based on elapsed time
    pub fn update(&mut self, delta_seconds: f64) {
        if !self.alive {
            self.state = FishState::Dead;
            return;
        }

        // Stat degradation rates (per real hour)
        // Tamagotchi-style: slow decay for casual gameplay
        let hours = delta_seconds / 3600.0;
        
        // Hunger decreases ~3-4 points per hour (needs feeding every 6-8 hours)
        self.hunger = (self.hunger - (3.5 * hours as f32)).max(0.0);
        
        // Happiness decreases slower (~1.5 per hour)
        self.happiness = (self.happiness - (1.5 * hours as f32)).max(0.0);
        
        // Energy decreases during day, regenerates during rest
        if matches!(self.state, FishState::Resting) {
            self.energy = (self.energy + (5.0 * hours as f32)).min(100.0);
        } else {
            self.energy = (self.energy - (2.0 * hours as f32)).max(0.0);
        }

        // Health is affected by hunger and happiness
        if self.hunger < 20.0 || self.happiness < 20.0 {
            self.health = (self.health - (2.0 * hours as f32)).max(0.0);
        } else if self.hunger > 50.0 && self.happiness > 50.0 {
            // Slowly regenerate health when well cared for
            self.health = (self.health + (0.5 * hours as f32)).min(100.0);
        }

        // Death check (only if severely neglected)
        if self.health <= 0.0 {
            self.alive = false;
            self.state = FishState::Dead;
        }

        // Update age
        self.age = self.age + Duration::seconds(delta_seconds as i64);

        // Auto-transition to resting if energy is low
        if self.energy < 30.0 && !matches!(self.state, FishState::Resting) {
            self.state = FishState::Resting;
        } else if self.energy > 60.0 && matches!(self.state, FishState::Resting) {
            self.state = FishState::Swimming;
        }
    }

    /// Feed the fish
    pub fn feed(&mut self) {
        if !self.alive {
            return;
        }

        self.hunger = (self.hunger + 30.0).min(100.0);
        self.happiness = (self.happiness + 10.0).min(100.0);
        self.last_fed = Some(Utc::now());
        self.state = FishState::Eating;
    }

    /// Get warning status
    pub fn get_warnings(&self) -> Vec<String> {
        let mut warnings = Vec::new();

        if !self.alive {
            warnings.push("💀 Has passed away".to_string());
            return warnings;
        }

        if self.hunger < 30.0 {
            warnings.push("🍽️  Very hungry!".to_string());
        }
        if self.happiness < 30.0 {
            warnings.push("😢 Unhappy".to_string());
        }
        if self.health < 50.0 {
            warnings.push("⚕️  Poor health".to_string());
        }
        if self.energy < 30.0 {
            warnings.push("😴 Exhausted".to_string());
        }

        warnings
    }

    /// Simple movement AI
    pub fn update_position(&mut self, delta_seconds: f64) {
        if !self.alive || matches!(self.state, FishState::Resting | FishState::Dead) {
            return;
        }

        // Random wandering with simple boundary bouncing
        let speed = if matches!(self.state, FishState::Eating) {
            0.5
        } else {
            1.0
        };

        let dt = delta_seconds as f32 * speed;
        
        self.position.0 += self.velocity.0 * dt;
        self.position.1 += self.velocity.1 * dt;

        // Bounce off boundaries (keep within 0.1 - 0.9 range)
        if self.position.0 <= 0.1 || self.position.0 >= 0.9 {
            self.velocity.0 *= -1.0;
            self.position.0 = self.position.0.clamp(0.1, 0.9);
        }
        if self.position.1 <= 0.1 || self.position.1 >= 0.9 {
            self.velocity.1 *= -1.0;
            self.position.1 = self.position.1.clamp(0.1, 0.9);
        }

        // Occasionally change direction
        if rand::random::<f32>() < 0.01 {
            self.velocity = (
                (rand::random::<f32>() - 0.5) * 0.02,
                (rand::random::<f32>() - 0.5) * 0.02,
            );
        }
    }

    pub fn age_display(&self) -> String {
        let days = self.age.num_days();
        let hours = self.age.num_hours() % 24;
        
        if days > 0 {
            format!("{}d {}h", days, hours)
        } else {
            format!("{}h", hours)
        }
    }
}
