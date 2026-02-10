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
    #[serde(default = "default_growth_stage")]
    pub stage: GrowthStage,
    #[serde(default = "default_gender")]
    pub gender: Gender,
    pub position: (f32, f32),  // Tank coordinates (0.0-1.0)
    pub velocity: (f32, f32),  // Movement direction
    pub state: FishState,
    pub alive: bool,
    
    pub created_at: DateTime<Utc>,
    pub last_fed: Option<DateTime<Utc>>,
    #[serde(default)]
    pub mate_cooldown: i64, // Seconds until next breeding attempt
}

fn default_growth_stage() -> GrowthStage {
    GrowthStage::Juvenile // Default for existing saves
}

fn default_gender() -> Gender {
    if rand::random() { Gender::Male } else { Gender::Female }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Gender {
    Male,
    Female,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Species {
    Goldfish,
    Betta,
    Guppy,
    NeonTetra,
    Angelfish,
    Clownfish,
    Koi,
    Pufferfish,
}

impl Species {
    pub fn name(&self) -> &'static str {
        match self {
            Species::Goldfish => "Goldfish",
            Species::Betta => "Betta",
            Species::Guppy => "Guppy",
            Species::NeonTetra => "Neon Tetra",
            Species::Angelfish => "Angelfish",
            Species::Clownfish => "Clownfish",
            Species::Koi => "Koi",
            Species::Pufferfish => "Pufferfish",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum FishState {
    Swimming,
    Eating,
    Resting,
    Dead,
}

impl Fish {
    fn random_gender() -> Gender {
        if rand::random() { Gender::Male } else { Gender::Female }
    }

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
            stage: GrowthStage::Fry,
            gender: Self::random_gender(),
            position: (0.5, 0.5),
            velocity: (0.01, 0.0),
            state: FishState::Swimming,
            alive: true,
            created_at: Utc::now(),
            last_fed: None,
            mate_cooldown: 0,
        }
    }

    pub fn new_betta(name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            species: Species::Betta,
            name,
            hunger: 85.0,
            happiness: 70.0,
            health: 100.0,
            energy: 90.0,
            age: Duration::zero(),
            stage: GrowthStage::Fry,
            gender: Self::random_gender(),
            position: (0.5, 0.5),
            velocity: (0.008, 0.0),
            state: FishState::Swimming,
            alive: true,
            created_at: Utc::now(),
            last_fed: None,
            mate_cooldown: 0,
        }
    }

    pub fn new_guppy(name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            species: Species::Guppy,
            name,
            hunger: 70.0,
            happiness: 85.0,
            health: 100.0,
            energy: 100.0,
            age: Duration::zero(),
            stage: GrowthStage::Fry,
            gender: Self::random_gender(),
            position: (0.5, 0.5),
            velocity: (0.015, 0.0),
            state: FishState::Swimming,
            alive: true,
            created_at: Utc::now(),
            last_fed: None,
            mate_cooldown: 0,
        }
    }

    pub fn new_neon_tetra(name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            species: Species::NeonTetra,
            name,
            hunger: 75.0,
            happiness: 80.0,
            health: 100.0,
            energy: 95.0,
            age: Duration::zero(),
            stage: GrowthStage::Fry,
            gender: Self::random_gender(),
            position: (0.5, 0.5),
            velocity: (0.012, 0.0),
            state: FishState::Swimming,
            alive: true,
            created_at: Utc::now(),
            last_fed: None,
            mate_cooldown: 0,
        }
    }

    pub fn new_angelfish(name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            species: Species::Angelfish,
            name,
            hunger: 80.0,
            happiness: 75.0,
            health: 100.0,
            energy: 85.0,
            age: Duration::zero(),
            stage: GrowthStage::Fry,
            gender: Self::random_gender(),
            position: (0.5, 0.5),
            velocity: (0.007, 0.0),
            state: FishState::Swimming,
            alive: true,
            created_at: Utc::now(),
            last_fed: None,
            mate_cooldown: 0,
        }
    }
    
    pub fn new_clownfish(name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            species: Species::Clownfish,
            name,
            hunger: 75.0,
            happiness: 90.0,
            health: 100.0,
            energy: 95.0,
            age: Duration::zero(),
            stage: GrowthStage::Fry,
            gender: Self::random_gender(),
            position: (0.5, 0.5),
            velocity: (0.01, 0.0),
            state: FishState::Swimming,
            alive: true,
            created_at: Utc::now(),
            last_fed: None,
            mate_cooldown: 0,
        }
    }

    pub fn new_koi(name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            species: Species::Koi,
            name,
            hunger: 90.0, // Big fish
            happiness: 80.0,
            health: 100.0,
            energy: 80.0,
            age: Duration::zero(),
            stage: GrowthStage::Fry,
            gender: Self::random_gender(),
            position: (0.5, 0.5),
            velocity: (0.005, 0.0), // Slow and majestic
            state: FishState::Swimming,
            alive: true,
            created_at: Utc::now(),
            last_fed: None,
            mate_cooldown: 0,
        }
    }

    pub fn new_pufferfish(name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            species: Species::Pufferfish,
            name,
            hunger: 70.0, 
            happiness: 70.0,
            health: 100.0,
            energy: 90.0,
            age: Duration::zero(),
            stage: GrowthStage::Fry,
            gender: Self::random_gender(),
            position: (0.5, 0.5),
            velocity: (0.008, 0.0),
            state: FishState::Swimming,
            alive: true,
            created_at: Utc::now(),
            last_fed: None,
            mate_cooldown: 0,
        }
    }


}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum GrowthStage {
    Fry,      // 0-12 hours
    Juvenile, // 12-36 hours
    Adult,    // > 36 hours
}

impl Fish {
    // Methods

    /// Update fish stats based on elapsed time
    pub fn update(&mut self, delta_seconds: f64, water: &crate::persistence::WaterParams) {
        if !self.alive {
            self.state = FishState::Dead;
            return;
        }

        // Update age (Game Time)
        // delta_seconds passed here MUST be game time (3x real time)
        self.age = self.age + Duration::seconds(delta_seconds as i64);

        // Update Growth Stage
        // Fry: < 12 hours
        // Juvenile: 12 - 36 hours
        // Adult: > 36 hours
        let age_hours = self.age.num_hours();
        self.stage = if age_hours < 12 {
            GrowthStage::Fry
        } else if age_hours < 36 {
            GrowthStage::Juvenile
        } else {
            GrowthStage::Adult
        };

        // Stat degradation rates (per real hour)
        // We receive GAME seconds. 1 Real Hour = 3 Game Hours.
        // User wants ~12 Real Hours survival time.
        // 12 Real Hours = 36 Game Hours.
        // So stats should drop 100% in 36 Game Hours.
        // Rate = 100 / 36 = ~2.78 per Game Hour.
        
        let hours = delta_seconds / 3600.0;
        
        // Base hunger rate (adjusted for 12h real time survival = 36h game time)
        let base_rate = 3.0; 
        
        // Species modulation
        let species_mod = match self.species {
            Species::Guppy => 1.2,       // Faster
            Species::Betta => 0.8,       // Slower
            _ => 1.0,
        };
        
        // Stage modulation (Fry eat faster/more often relative to size, but let's keep simple)
        let stage_mod = match self.stage {
            GrowthStage::Fry => 1.5,
            GrowthStage::Juvenile => 1.2,
            GrowthStage::Adult => 1.0,
        };

        let hunger_rate = base_rate * species_mod * stage_mod;
        self.hunger = (self.hunger - (hunger_rate * hours as f32)).max(0.0);
        
        // Happiness decreases
        self.happiness = (self.happiness - (1.5 * hours as f32)).max(0.0);
        
        // Energy decreases during day, regenerates during rest
        if matches!(self.state, FishState::Resting) {
            self.energy = (self.energy + (10.0 * hours as f32)).min(100.0); // Faster sleep recovery
        } else {
            self.energy = (self.energy - (2.0 * hours as f32)).max(0.0);
        }

        // Health penalties from Environment
        let mut health_change = 0.0;
        
        // Water Purity Impact
        if water.purity < 50.0 {
            health_change -= 2.0; // Dirty water hurts health
            self.happiness -= 2.0 * hours as f32; // And happiness
        }
        if water.purity < 20.0 {
            health_change -= 5.0; // Very dirty water is dangerous
        }

        // Temperature Impact (Idea: 24-26 is good)
        if water.temperature < 20.0 || water.temperature > 30.0 {
             health_change -= 1.0;
        }

        // Health is affected by hunger and happiness
        if self.hunger < 20.0 || self.happiness < 20.0 {
            health_change -= 3.0; // Starvation hurts more now
        } else if self.hunger > 50.0 && self.happiness > 50.0 && water.purity > 80.0 {
            // Slowly regenerate health when well cared for AND clean water
            health_change += 0.5;
        }
        
        self.health = (self.health + (health_change * hours as f32)).clamp(0.0, 100.0);

        // Death check
        if self.health <= 0.0 {
            self.alive = false;
            self.state = FishState::Dead;
        }

        // Auto-transition to resting if energy is low
        if self.energy < 20.0 && !matches!(self.state, FishState::Resting) {
            self.state = FishState::Resting;
            self.state = FishState::Swimming;
        }
        
        // Cooldown decay
        if self.mate_cooldown > 0 {
            self.mate_cooldown -= delta_seconds as i64;
        }
    }
    
    /// Attempt to breed with another fish
    pub fn try_breed(&mut self, partner: &mut Fish) -> Option<Fish> {
        // Validation
        if !self.alive || !partner.alive { return None; }
        if self.species != partner.species { return None; }
        if self.gender == partner.gender { return None; }
        if self.stage != GrowthStage::Adult || partner.stage != GrowthStage::Adult { return None; }
        
        // Check cooldowns
        if self.mate_cooldown > 0 || partner.mate_cooldown > 0 { return None; }
        
        // Check stats (Must be healthy and happy)
        if self.health < 80.0 || partner.health < 80.0 { return None; }
        if self.happiness < 80.0 || partner.happiness < 80.0 { return None; }
        
        // Success!
        // Reset cooldowns (e.g. 5 minutes = 300 game seconds)
        // 5 real minutes = 900 game seconds
        let cooldown = 900;
        self.mate_cooldown = cooldown;
        partner.mate_cooldown = cooldown;
        
        // Spawn Fry
        // Name will be placeholder, parent logic in App will name it
        let mut fry = match self.species {
            Species::Goldfish => Fish::new_goldfish("Baby".to_string()),
            Species::Betta => Fish::new_betta("Baby".to_string()),
            Species::Guppy => Fish::new_guppy("Baby".to_string()),
            Species::NeonTetra => Fish::new_neon_tetra("Baby".to_string()),
            Species::Angelfish => Fish::new_angelfish("Baby".to_string()),
            Species::Clownfish => Fish::new_clownfish("Baby".to_string()),
            Species::Koi => Fish::new_koi("Baby".to_string()),
            Species::Pufferfish => Fish::new_pufferfish("Baby".to_string()),
        };
        
        // Inherit some position
        fry.position = self.position;
        
        Some(fry)
    }

    /// Update fish state based on time of day (call from App::update)
    pub fn update_for_time_of_day(&mut self, is_night: bool) {
        if !self.alive {
            return;
        }

        if is_night && !matches!(self.state, FishState::Resting | FishState::Dead) {
            // Fish rest at night
            self.state = FishState::Resting;
        } else if !is_night && matches!(self.state, FishState::Resting) && self.energy > 40.0 {
            // Wake up during day if energy is sufficient
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
