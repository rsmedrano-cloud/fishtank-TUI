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
    let block = Block::default()
        .borders(Borders::ALL)
        .title("🐟 Fishtank")
        .style(Style::default().fg(Color::White));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let tank_width = inner.width as usize;
    let tank_height = inner.height as usize;
    let mut lines = Vec::new();

    if !app.save_data.fish.is_empty() {
        for y in 0..tank_height {
            let mut line_content = String::new();
            let mut x = 0;

            while x < tank_width {
                let fx = (x as f32) / (tank_width as f32);
                let fy = (y as f32) / (tank_height as f32);

                let mut found_fish = false;
                
                // Check all fish
                for fish in &app.save_data.fish {
                    if !fish.alive {
                        continue;
                    }
                    let fish_dist = ((fish.position.0 - fx).powi(2) + (fish.position.1 - fy).powi(2)).sqrt();
                    if fish_dist < 0.03 {  // Tighter detection
                        let sprite = FishSprite::from_fish(fish, app.animation_frame);
                        line_content.push_str(sprite);
                        x += sprite.len();  // Skip ahead by sprite length
                        found_fish = true;
                        break;
                    }
                }

                if !found_fish {
                    if y == tank_height - 1 {
                        line_content.push('▓');
                    } else if y == tank_height - 2 && (x < 3 || x > tank_width - 4) {
                        line_content.push('Y');
                    } else if y == 0 && x % 15 == 0 && app.animation_frame % 60 < 30 {
                        line_content.push('°');
                    } else {
                        line_content.push(' ');
                    }
                    x += 1;
                }
            }

            lines.push(Line::from(line_content));
        }
    } else {
        let empty_y = tank_height / 2;
        for y in 0..tank_height {
            if y == empty_y {
                lines.push(Line::from(format!("{:^width$}", "Press 'N' to add fish (up to 3)!", width = tank_width)));
            } else if y == tank_height - 1 {
                lines.push(Line::from("▓".repeat(tank_width)));
            } else {
                lines.push(Line::from(" ".repeat(tank_width)));
            }
        }
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
            "[F]eed  [N]ew Fish  [R]estart  [Q]uit"
        } else {
            "[F]eed  [R]estart  [Q]uit"
        }
    } else {
        "[N]ew Fish  [R]estart  [Q]uit"
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
