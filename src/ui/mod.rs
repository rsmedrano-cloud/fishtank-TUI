use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect, Alignment},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Gauge, Clear},
    Frame,
};

use crate::app::App;
use crate::models::FishState;
use crate::utils::{draw_stat_bar, stat_color_indicator, FishSprite, TankElements};

pub mod theme;

pub fn render(frame: &mut Frame, app: &App) {
    let size = frame.size();

    // Main layout: tank on left, stats on right
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(3),      // Main area
            Constraint::Length(3),   // Controls
        ])
        .split(size);

    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(80),  // Tank view
            Constraint::Percentage(20),  // Stats panel
        ])
        .split(chunks[0]);

    render_tank(frame, app, main_chunks[0]);
    render_stats(frame, app, main_chunks[1]);
    render_controls(frame, app, chunks[1]);
    
    // Render Shop Overlay (Top Level - Modal) - Centered on Tank
    if app.show_shop {
        let area = centered_rect(60, 50, main_chunks[0]);
        let block = Block::default()
            .title(Span::styled(" 🛒 Pet Shop ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)))
            .borders(Borders::ALL)
            .style(Style::default().bg(Color::Black));
        
        frame.render_widget(Clear, area); // Clear background
        frame.render_widget(block.clone(), area);

        let inner = block.inner(area);
        
        // Build shop items dynamically based on ownership
        let mut shop_lines = vec![
            Line::from(vec![Span::raw("Balance: "), Span::styled(format!("${:.2}", app.save_data.money), Style::default().fg(Color::Green))]),
            Line::from(""),
            Line::from(Span::styled("━━━ EQUIPMENT ━━━", Style::default().fg(Color::Cyan))),
        ];
        
        // Filter
        if app.save_data.equipment.has_filter {
            shop_lines.push(Line::from(vec![
                Span::styled("[1] Filter", Style::default().fg(Color::DarkGray)),
                Span::raw(" - "),
                Span::styled("✅ OWNED", Style::default().fg(Color::Green)),
            ]));
        } else {
            shop_lines.push(Line::from(vec![
                Span::styled("[1] Filter", Style::default().fg(Color::Cyan)),
                Span::raw(" - $50 (Reduces water degradation)"),
            ]));
        }
        
        // Heater
        if app.save_data.equipment.has_heater {
            shop_lines.push(Line::from(vec![
                Span::styled("[2] Heater", Style::default().fg(Color::DarkGray)),
                Span::raw(" - "),
                Span::styled("✅ OWNED", Style::default().fg(Color::Green)),
            ]));
        } else {
            shop_lines.push(Line::from(vec![
                Span::styled("[2] Heater", Style::default().fg(Color::Cyan)),
                Span::raw(" - $40 (Stabilizes temperature)"),
            ]));
        }
        
        // Plants
        if app.save_data.equipment.has_plants {
            shop_lines.push(Line::from(vec![
                Span::styled("[3] Plants", Style::default().fg(Color::DarkGray)),
                Span::raw(" - "),
                Span::styled("✅ OWNED", Style::default().fg(Color::Green)),
            ]));
        } else {
            shop_lines.push(Line::from(vec![
                Span::styled("[3] Plants", Style::default().fg(Color::Cyan)),
                Span::raw(" - $30 (Improves water quality)"),
            ]));
        }
        
        shop_lines.push(Line::from(""));
        shop_lines.push(Line::from(Span::styled("━━━ DECORATIONS ━━━", Style::default().fg(Color::Yellow))));
        shop_lines.push(Line::from(vec![
            Span::styled("[4] Random Decoration", Style::default().fg(Color::Yellow)),
            Span::raw(" - $20"),
        ]));
        
        shop_lines.push(Line::from(""));
        shop_lines.push(Line::from(Span::styled("Press [1-4] to Buy | [P]/[Esc] to Close", Style::default().fg(Color::Gray))));
        
        let p = Paragraph::new(shop_lines)
            .alignment(Alignment::Center)
            .wrap(ratatui::widgets::Wrap { trim: true });
            
        frame.render_widget(p, inner);
    }
    
    // Render Achievements Overlay
    if app.show_achievements {
        let area = centered_rect(70, 70, main_chunks[0]);
        let block = Block::default()
            .title(Span::styled(" 🏆 Achievements ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)))
            .borders(Borders::ALL)
            .style(Style::default().bg(Color::Black));
        
        frame.render_widget(Clear, area);
        frame.render_widget(block.clone(), area);
        
        let inner = block.inner(area);
        let mut lines = vec![
            Line::from(Span::styled("━━━ YOUR STATS ━━━", Style::default().fg(Color::Cyan))),
            Line::from(format!("Fish Bred: {} | Money Earned: ${:.2} | Cleaned: {} | Playtime: {}h {}m", 
                app.save_data.total_fish_bred,
                app.save_data.total_money_earned,
                app.save_data.clean_count,
                (app.save_data.total_time / 3600.0) as u32,
                ((app.save_data.total_time % 3600.0) / 60.0) as u32
            )),
            Line::from(""),
            Line::from(Span::styled("━━━ ACHIEVEMENTS ━━━", Style::default().fg(Color::Yellow))),
        ];
        
        for achievement in &app.save_data.achievements {
            let status = if achievement.unlocked {
                Span::styled("✅", Style::default().fg(Color::Green))
            } else {
                Span::styled("🔒", Style::default().fg(Color::DarkGray))
            };
            
            let name_style = if achievement.unlocked {
                Style::default().fg(Color::Green)
            } else {
                Style::default().fg(Color::DarkGray)
            };
            
            lines.push(Line::from(vec![
                status,
                Span::raw(" "),
                Span::raw(&achievement.icon),
                Span::raw(" "),
                Span::styled(&achievement.name, name_style),
                Span::raw(" - "),
                Span::styled(&achievement.description, Style::default().fg(Color::Gray)),
            ]));
        }
        
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled("Press [A] to Close", Style::default().fg(Color::Gray))));
        
        let p = Paragraph::new(lines)
            .alignment(Alignment::Left)
            .wrap(ratatui::widgets::Wrap { trim: true });
        
        frame.render_widget(p, inner);
    }
}

fn render_tank(frame: &mut Frame, app: &App, area: Rect) {
    let (hour, minute) = app.get_game_time();
    let is_night = app.is_night();
    
    // Time indicator with emoji
    let time_emoji = if is_night { "🌙" } else { "🌞" };
    let time_str = format!("{} {:02}:{:02}", time_emoji, hour, minute);
    
    let theme = app.get_current_theme();
    
    let title = if app.save_data.is_frozen {
        format!("Fish Tank - {} ❄️ FROZEN ❄️", theme.name)
    } else {
        format!("Fish Tank - {}", theme.name)
    };

    let block = Block::default()
        .title(vec![
            Span::raw("🐠 "),
            Span::styled(title, Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" "),
            Span::styled(time_str, Style::default().fg(if is_night { Color::Blue } else { theme.title_color })),
        ])
        .borders(Borders::ALL)
        .style(Style::default().fg(theme.border_color));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    // Tank rendering area
    let tank_width = inner.width as usize;
    let tank_height = inner.height as usize;
    
    // Create a 2D buffer for the tank characters
    let mut buffer: Vec<Vec<Span>> = vec![vec![Span::raw(" "); tank_width]; tank_height];

    // Theme Colors
    let theme = app.get_current_theme();
    
    // Adjust for night time (dimming)
    let (substrate_color, plant_color, bubble_color) = if is_night {
         // Simple dimming logic: if theme is classic, use hardcoded night colors
         // Otherwise, maybe just use theme colors but dimmed? 
         // For now, let's respect the theme colors but maybe swap to darker variants if it's default
         if theme.name == "Classic" {
             (Color::Rgb(50, 50, 60), Color::Rgb(30, 70, 30), Color::Rgb(100, 100, 130))
         } else {
             (theme.substrate_color, theme.plant_color, theme.water_color)
         }
    } else {
        (theme.substrate_color, theme.plant_color, theme.water_color)
    };
    
    for y in 0..tank_height {
        for x in 0..tank_width {
             if y == tank_height - 1 {
                buffer[y][x] = Span::styled(theme.substrate_char.to_string(), Style::default().fg(substrate_color));
            } else {
                // Growing corner plants (left and right)
                let left_height = app.save_data.left_plant_height as usize;
                let right_height = app.save_data.right_plant_height as usize;
                
                // Left plant growth (x < 3)
                if x < 3 && y >= tank_height.saturating_sub(1 + left_height) && y < tank_height - 1 {
                    buffer[y][x] = Span::styled("Y", Style::default().fg(plant_color));
                }
                // Right plant growth (x > width - 4)
                else if x > tank_width - 4 && y >= tank_height.saturating_sub(1 + right_height) && y < tank_height - 1 {
                    buffer[y][x] = Span::styled("Y", Style::default().fg(plant_color));
                }
                // Surface bubbles
                else if y == 0 && x % 15 == 0 && app.animation_frame % 60 < 30 && !is_night && !app.save_data.is_frozen {
                     buffer[y][x] = Span::styled("°", Style::default().fg(bubble_color));
                } else if app.save_data.is_frozen && y == tank_height / 2 && x == tank_width / 2 {
                     // Nothing specifically, maybe freeze overlay logic later
                }
            }
        }
    }

    // Render Decorations (Background Layer)
    for deco in &app.save_data.decorations {
        let sprite_lines = deco.get_sprite();
        let sprite_height = sprite_lines.len();
        
        // anchor to bottom (above substrate which is last line)
        // substrate is at tank_height - 1
        // so object bottom is tank_height - 2
        let base_y = (tank_height - 1).saturating_sub(sprite_height);
        
        // Keep X relative
        let base_x = (deco.position.0 * (tank_width - 15) as f32).round() as usize; 
        
        for (offset_y, line) in sprite_lines.iter().enumerate() {
            let y = base_y + offset_y;
            if y >= tank_height { continue; }
            
            let mut current_x = base_x;
            for char in line.chars() {
                if current_x < tank_width {
                    // Use a subtle color for decorations
                     let color = if is_night {
                         // Slightly visible at night
                         match deco.deco_type {
                             crate::models::DecorationType::Plant => Color::Green, // Keep green but maybe it will look dark on black
                             _ => Color::Gray,
                         }
                     } else {
                         match deco.deco_type {
                             crate::models::DecorationType::Plant => theme.plant_color,
                             crate::models::DecorationType::Rock => Color::Gray,
                             crate::models::DecorationType::Castle => Color::White,
                             crate::models::DecorationType::Skull => Color::White,
                         }
                     };
                    
                    buffer[y][current_x] = Span::styled(char.to_string(), Style::default().fg(color));
                    current_x += 1;
                }
            }
        }
    }

    // Render Fish
    if !app.save_data.fish.is_empty() {
        for fish in &app.save_data.fish {
            if !fish.alive {
                continue;
            }
            
            // Calculate base position (top-left of sprite)
            let base_y = (fish.position.1 * (tank_height - 2) as f32).round() as usize;
            let base_x = (fish.position.0 * (tank_width - 5) as f32).round() as usize;
            
            let sprite_lines = FishSprite::from_fish(fish, app.animation_frame);
            
            for (offset_y, line) in sprite_lines.iter().enumerate() {
                let y = base_y + offset_y;
                if y >= tank_height { continue; }
                
                let mut current_x = base_x;
                for char in line.chars() {
                    if current_x < tank_width {
                        // Use Species specific color or Theme default?
                        // Let's use Theme default for special themes like Matrix/Retro
                        let color = if theme.name == "Matrix" || theme.name == "Retro Amber" || theme.name == "Zen Garden" {
                            theme.fish_default_color
                        } else {
                            if matches!(fish.stage, crate::models::GrowthStage::Fry) {
                                 Color::White 
                             } else {
                                 Color::Yellow // Default per species ideally, but keep simple for now
                             }
                        };
                         
                        buffer[y][current_x] = Span::styled(char.to_string(), Style::default().fg(color));
                        current_x += 1;
                    }
                }
            }
        }
    } else {
        // Empty tank message
        let empty_msg = "Press 'N' to add fish (up to 3)!";
        let start_x = (tank_width.saturating_sub(empty_msg.len())) / 2;
        let y = tank_height / 2;
        
        for (i, char) in empty_msg.chars().enumerate() {
             if start_x + i < tank_width {
                 buffer[y][start_x + i] = Span::styled(char.to_string(), Style::default().fg(theme.title_color));
             }
        }
    }
    
    // Draw Frozen Overlay if needed
    if app.save_data.is_frozen {
        let msg = "❄️ FROZEN ❄️";
        let start_x = (tank_width.saturating_sub(msg.len())) / 2;
        let y = 1; 
        for (i, char) in msg.chars().enumerate() {
             if start_x + i < tank_width {
                 buffer[y][start_x + i] = Span::styled(char.to_string(), Style::default().fg(theme.water_color).add_modifier(Modifier::BOLD));
             }
        }
    }

    // Render Particles (Foreground)
    for p in &app.particles {
         let y = (p.y * (tank_height - 1) as f32).round() as usize;
         let x = (p.x * (tank_width - 1) as f32).round() as usize;
         
         // Don't render on substrate (last line)
         if y < tank_height - 1 && x < tank_width {
             // Light blue for bubbles
             buffer[y][x] = Span::styled(p.symbol.to_string(), Style::default().fg(Color::Cyan)); 
         }
    }
    
    // Render Algae Overlay (Dirty Glass)
    let algae_level = app.save_data.algae_level;
    if algae_level > 1.0 {
        let density = algae_level / 100.0; // 0.0 to 1.0
        
        for y in 0..tank_height-1 { // Don't cover substrate fully? or maybe yes
            for x in 0..tank_width {
                // Simple pseudo-random hash for static noise
                let seed = (x as u32).wrapping_mul(374761393).wrapping_add((y as u32).wrapping_mul(668265263));
                let rand_val = (seed % 100) as f32 / 100.0;
                
                if rand_val < density {
                    // Algae pixel!
                    // If density is high, use thicker chars
                    let char = if density > 0.6 && rand_val < density * 0.5 { "#" } else { "." };
                    
                    // Green slime color
                    buffer[y][x] = Span::styled(char, Style::default().fg(Color::Green));
                }
            }
        }
    }

    // OLD Shop Rendering was here - Moved to render()


    // Convert buffer to Lines
    let mut lines = Vec::new();
    for row in buffer {
        lines.push(Line::from(row));
    }

    let tank_content = Paragraph::new(lines);
    frame.render_widget(tank_content, inner);
}

fn render_stats(frame: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title("📊 Status")
        .style(Style::default().fg(Color::Green));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let mut lines = Vec::new();

    if app.save_data.fish.is_empty() {
        lines.push(Line::from("No fish in tank"));
        lines.push(Line::from(""));
        lines.push(Line::from("Press 'N' to add"));
        lines.push(Line::from("up to 10 fish!"));
    } else {
        // COMPACT VIEW for many fish
        if app.save_data.fish.len() > 4 {
             for fish in &app.save_data.fish {
                let gender_symbol = match fish.gender {
                    crate::models::Gender::Male => "♂",
                    crate::models::Gender::Female => "♀",
                };
                
                let health_color = if fish.health > 70.0 { Color::Green } else { Color::Red };
                
                // Status icon (check sleep/eat)
                let status_icon = match fish.state {
                    crate::models::FishState::Resting => " 💤", // Sleeping
                    crate::models::FishState::Eating => " 🍖",  // Eating
                    _ => "",
                };
                
                // One line per fish: [ICON] Name (S) H:99% Zzz
                if fish.alive {
                    lines.push(Line::from(vec![
                        Span::styled("🐟 ", Style::default().fg(Color::Yellow)),
                        Span::styled(format!("{} ", fish.name), Style::default().add_modifier(Modifier::BOLD)),
                        Span::raw(format!("({}) ", gender_symbol)),
                        Span::styled("❤", Style::default().fg(health_color)),
                        Span::raw(format!("{:.0}% ", fish.health)),
                        Span::styled("🍗", Style::default().fg(Color::Magenta)),
                        Span::raw(format!("{:.0}%{}", fish.hunger, status_icon)),
                    ]));
                } else {
                    lines.push(Line::from(vec![
                        Span::styled("💀 ", Style::default().fg(Color::DarkGray)),
                        Span::styled(format!("{} (Dec.)", fish.name), Style::default().fg(Color::DarkGray)),
                    ]));
                }
            }
             
             // Summary at bottom
             let alive = app.save_data.fish.iter().filter(|f| f.alive).count();
             lines.push(Line::from(""));
             lines.push(Line::from(format!("Pop: {}/{}", alive, app.save_data.fish.len())));
             
        } else {
            // DETAILED VIEW (Original)
            for (idx, fish) in app.save_data.fish.iter().enumerate() {
                if idx > 0 {
                    lines.push(Line::from(""));
                }
    
                let gender_symbol = match fish.gender {
                    crate::models::Gender::Male => "♂",
                    crate::models::Gender::Female => "♀",
                };
    
                lines.push(Line::from(vec![
                    Span::styled("🐟 ", Style::default().fg(Color::Yellow)),
                    Span::styled(&fish.name, Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(format!(" ({}) {}", fish.species.name(), gender_symbol)),
                ]));
    
                if fish.alive {
                    lines.push(Line::from(vec![
                        Span::raw(stat_color_indicator(fish.hunger)),
                        Span::raw(format!(" H{} ", draw_stat_bar(fish.hunger, 5))),
                        Span::raw(stat_color_indicator(fish.health)),
                        Span::raw(format!(" ❤️{}", draw_stat_bar(fish.health, 5))),
                    ]));
    
                    // Show warnings for this fish
                    let warnings = fish.get_warnings();
                    if !warnings.is_empty() {
                        for warning in warnings.iter().take(2) {
                            lines.push(Line::from(Span::styled(
                                warning.clone(),
                                Style::default().fg(Color::Red),
                            )));
                        }
                    }
                } else {
                    lines.push(Line::from(Span::styled(
                        "💀 Deceased",
                        Style::default().fg(Color::Red),
                    )));
                }
            }
            
            // Summary
            let alive = app.save_data.fish.iter().filter(|f| f.alive).count();
            lines.push(Line::from(""));
            lines.push(Line::from(format!("Alive: {}/{}", alive, app.save_data.fish.len())));
        }
    }

    // Water Quality Section
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled("💧 Water Quality", Style::default().fg(Color::Cyan))));
    
    let water = &app.save_data.water;
    
    // Purity
    let purity_color = if water.purity > 80.0 { Color::Green } 
                      else if water.purity > 50.0 { Color::Yellow } 
                      else { Color::Red };
    lines.push(Line::from(vec![
        Span::raw("Purity: "),
        Span::styled(format!("{:.1}%", water.purity), Style::default().fg(purity_color)),
    ]));
    lines.push(Line::from(draw_stat_bar(water.purity, 10)));
    
    // Temp & pH
    lines.push(Line::from(vec![
        Span::raw(format!("Temp: {:.1}°C  ", water.temperature)),
        Span::raw(format!("pH: {:.1}", water.ph)),
    ]));

    // Money and Equipment Section
    lines.push(Line::from("")); // Add a blank line before this section
    lines.push(Line::from(vec![
        Span::styled("💰 Money: ", Style::default().fg(Color::Yellow)),
        Span::styled(format!("${:.2}", app.save_data.money), Style::default().fg(Color::White)),
    ]));
    lines.push(Line::from("")); // Blank line between money and equipment
    lines.push(Line::from(Span::styled("🏗 Equipment", Style::default().fg(Color::Yellow))));
    lines.push(Line::from(vec![
        Span::styled("⚡Filter ", if app.save_data.equipment.has_filter { Style::default().fg(Color::Green) } else { Style::default().fg(Color::DarkGray) }),
        Span::styled("🌡️Heater ", if app.save_data.equipment.has_heater { Style::default().fg(Color::Red) } else { Style::default().fg(Color::DarkGray) }),
        Span::styled("🌿Plants", if app.save_data.equipment.has_plants { Style::default().fg(Color::Green) } else { Style::default().fg(Color::DarkGray) }),
    ]));

    // Notifications
    if !app.notifications.is_empty() {
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            "📢 Messages:",
            Style::default().fg(Color::Yellow),
        )));
        for notif in app.notifications.iter().rev().take(3) {
            lines.push(Line::from(Span::styled(
                notif.clone(),
                Style::default().fg(Color::Yellow),
            )));
        }
    }

    let stats_content = Paragraph::new(lines);
    frame.render_widget(stats_content, inner);
}

fn render_controls(frame: &mut Frame, app: &App, area: Rect) {
    let fish_count = app.save_data.fish.iter().filter(|f| f.alive).count();
    
    let freeze_text = if app.save_data.is_frozen { "[Z]Unfreeze" } else { "[Z]Freeze" };
    
    let controls_text = if fish_count > 0 {
        if app.save_data.fish.len() < 10 {
            format!("v0.9.5 [F]eed [N]ew [W]ater [S]crub [P]Shop [X]Remove {}", freeze_text)
        } else {
            format!("v0.9.5 [F]eed [W]ater [S]crub [P]Shop [X]Remove {}", freeze_text)
        }
    } else {
        format!("v0.9.5 [N]ew [P]Shop [X]Remove [R]estart [Q]uit {}", freeze_text)
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Gray));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let controls = Paragraph::new(controls_text)
        .style(Style::default().fg(Color::White));

    frame.render_widget(controls, inner);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
