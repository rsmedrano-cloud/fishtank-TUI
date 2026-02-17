    fn trigger_random_event(&mut self) {
        let events = [
            "mystery_delivery",
            "lucky_day",
            "surprise_fry",
            "algae_eater",
            "power_surge",
            "disease",
            "cloudy_water",
        ];
        
        let event = events[rand::random::<usize>() % events.len()];
        
        match event {
            "mystery_delivery" => {
                // Free random decoration
                let types = [crate::models::DecorationType::Rock, crate::models::DecorationType::Plant, 
                             crate::models::DecorationType::Castle, crate::models::DecorationType::Skull];
                let rand_type = types[rand::random::<usize>() % types.len()];
                let x = rand::random::<f32>().clamp(0.1, 0.9);
                if self.save_data.decorations.iter().all(|d| (d.position.0 - x).abs() >= 0.15) {
                    self.save_data.decorations.push(crate::models::Decoration::new(rand_type, (x, 0.0)));
                    self.add_notification("🎁 Mystery Delivery! Free decoration added!".to_string());
                }
            }
            "lucky_day" => {
                self.lucky_day_timer = 120.0; // 2 minutes of 2x income
                self.add_notification("⭐ Lucky Day! 2x income for 2 minutes!".to_string());
            }
            "surprise_fry" => {
                // Add a random baby fish if not at max
                if self.save_data.fish.iter().filter(|f| f.alive).count() < 10 {
                    let species_id = rand::random::<usize>() % 8;
                    let mut new_fish = crate::models::Fish::new_adult(species_id, 0.5, 0.5);
                    new_fish.growth_stage = crate::models::GrowthStage::Fry;
                    new_fish.age = 0.0;
                    self.save_data.fish.push(new_fish);
                    self.add_notification("🐣 Surprise Fry appeared in your tank!".to_string());
                } else {
                    self.add_notification("🌟 Lucky! But tank is full.".to_string());
                }
            }
            "algae_eater" => {
                self.save_data.algae_level = (self.save_data.algae_level * 0.5).max(0.0);
                self.add_notification("🐌 Algae Eater visited! Algae reduced by 50%!".to_string());
            }
            "power_surge" => {
                // Randomly break one piece of equipment
                let mut broken_equipment = Vec::new();
                if self.save_data.equipment.has_filter { broken_equipment.push("filter"); }
                if self.save_data.equipment.has_heater { broken_equipment.push("heater"); }
                if self.save_data.equipment.has_plants { broken_equipment.push("plants"); }
                
                if !broken_equipment.is_empty() {
                    let to_break = broken_equipment[rand::random::<usize>() % broken_equipment.len()];
                    match to_break {
                        "filter" => {
                            self.save_data.equipment.has_filter = false;
                            self.add_notification("⚡ Power Surge! Filter malfunctioned!".to_string());
                        }
                        "heater" => {
                            self.save_data.equipment.has_heater = false;
                            self.add_notification("⚡ Power Surge! Heater malfunctioned!".to_string());
                        }
                        "plants" => {
                            self.save_data.equipment.has_plants = false;
                            self.add_notification("⚡ Power Surge! Plants died!".to_string());
                        }
                        _ => {}
                    }
                }
            }
            "disease" => {
                for fish in &mut self.save_data.fish {
                    if fish.alive {
                        fish.health = (fish.health - 20.0).max(1.0);
                    }
                }
                self.add_notification("🦠 Disease outbreak! All fish lost 20 health!".to_string());
            }
            "cloudy_water" => {
                self.save_data.water.purity = (self.save_data.water.purity - 30.0).max(0.0);
                self.add_notification("☁️ Cloudy Water! Purity dropped by 30!".to_string());
            }
            _ => {}
        }
    }
