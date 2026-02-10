use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::app::App;
use crate::models::FishState;
use crate::utils::{draw_stat_bar, stat_color_indicator, FishSprite, TankElements};

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
            Constraint::Percentage(70),  // Tank view
            Constraint::Percentage(30),  // Stats panel
        ])
        .split(chunks[0]);

    render_tank(frame, app, main_chunks[0]);
    render_stats(frame, app, main_chunks[1]);
    render_controls(frame, app, chunks[1]);
}

fn render_tank(frame: &mut Frame, app: &App, area: Rect) {
    let (hour, minute) = app.get_game_time();
    let is_night = app.is_night();
    
    // Time indicator with emoji
    let time_emoji = if is_night { "🌙" } else { "🌞" };
    let time_str = format!("{} {:02}:{:02}", time_emoji, hour, minute);
    
    let block = Block::default()
        .title(vec![
            Span::raw("🐠 "),
            Span::styled("Fish", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw("Tank "),
            Span::styled(time_str, Style::default().fg(if is_night { Color::Blue } else { Color::Yellow })),
        ])
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Cyan));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    // Tank rendering area
    // Tank rendering area
    let tank_width = inner.width as usize;
    let tank_height = inner.height as usize;
    
    // Create a 2D buffer for the tank characters
    let mut buffer: Vec<Vec<Span>> = vec![vec![Span::raw(" "); tank_width]; tank_height];

    // Initialize buffer with background based on time
    let (substrate_color, plant_color, bubble_color) = if is_night {
        (Color::Rgb(50, 50, 60), Color::Rgb(30, 70, 30), Color::Rgb(100, 100, 130))
    } else {
        (Color::Rgb(100, 100, 100), Color::Green, Color::Cyan)
    };
    
    for y in 0..tank_height {
        for x in 0..tank_width {
             if y == tank_height - 1 {
                buffer[y][x] = Span::styled("▓", Style::default().fg(substrate_color));
            } else if y == tank_height - 2 && (x < 3 || x > tank_width - 4) {
                 buffer[y][x] = Span::styled("Y", Style::default().fg(plant_color));
            } else if y == 0 && x % 15 == 0 && app.animation_frame % 60 < 30 && !is_night && !app.save_data.is_frozen {
                 buffer[y][x] = Span::styled("°", Style::default().fg(bubble_color));
            } else if app.save_data.is_frozen && y == tank_height / 2 && x == tank_width / 2 {
                 // Nothing specifically, maybe freeze overlay logic later
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
                        // Color based on species/status? For now Yellow default
                         let color = if matches!(fish.stage, crate::models::GrowthStage::Fry) {
                             Color::White 
                         } else {
                             Color::Yellow
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
                 buffer[y][start_x + i] = Span::raw(char.to_string());
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
                 buffer[y][start_x + i] = Span::styled(char.to_string(), Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD));
             }
        }
    }

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
        lines.push(Line::from("up to 3 goldfish!"));
    } else {
        // Show each fish
        for (idx, fish) in app.save_data.fish.iter().enumerate() {
            if idx > 0 {
                lines.push(Line::from(""));
            }

            lines.push(Line::from(vec![
                Span::styled("🐟 ", Style::default().fg(Color::Yellow)),
                Span::styled(&fish.name, Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(format!(" ({})", fish.species.name())),
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

    // Equipment Section
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled("⚙️ Equipment", Style::default().fg(Color::Cyan))));
    
    let eq = &app.save_data.equipment;
    let mut eq_spans = Vec::new();
    
    if eq.has_filter {
        eq_spans.push(Span::styled("⚡Filter ", Style::default().fg(Color::Green)));
    } else {
        eq_spans.push(Span::styled("Filter ", Style::default().fg(Color::DarkGray)));
    }
    
    if eq.has_heater {
        eq_spans.push(Span::styled("🌡️Heater ", Style::default().fg(Color::Red)));
    } else {
        eq_spans.push(Span::styled("Heater ", Style::default().fg(Color::DarkGray)));
    }
    
    if eq.has_plants {
        eq_spans.push(Span::styled("🌿Plants", Style::default().fg(Color::Green)));
    } else {
        eq_spans.push(Span::styled("Plants", Style::default().fg(Color::DarkGray)));
    }
    
    lines.push(Line::from(eq_spans));

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
    
    let controls_text = if fish_count > 0 {
        if app.save_data.fish.len() < 3 {
            "v0.6 [F]eed [N]ew [W]ater [E]quip [R]estart [Z]Freeze [C]lear [Q]uit"
        } else {
            "v0.6 [F]eed [W]ater [E]quip [R]estart [Z]Freeze [C]lear [Q]uit"
        }
    } else {
        "v0.6 [N]ew Fish [R]estart [Z]Freeze [C]lear [Q]uit"
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
