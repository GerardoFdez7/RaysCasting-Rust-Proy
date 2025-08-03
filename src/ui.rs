use crate::*;
use std::time::Instant;

pub struct UI {
    pub start_time: Instant,
    pub animated_sprite_frame: usize,
    pub animation_timer: f64,
}

impl UI {
    pub fn new() -> Self {
        Self {
            start_time: Instant::now(),
            animated_sprite_frame: 0,
            animation_timer: 0.0,
        }
    }

    pub fn render_splash_screen(&mut self, buffer: &mut Vec<u32>, window_width: usize, window_height: usize) {
        // Clear with dark background
        for pixel in buffer.iter_mut() {
            *pixel = 0x001122;
        }

        // Animated title
        let elapsed = self.start_time.elapsed().as_secs_f64();
        let pulse = (elapsed * 2.0).sin().abs();
        let title_color = self.interpolate_color(0x4444FF, 0x8888FF, pulse);
        
        // Draw title "RAY CASTER"
        self.draw_text(buffer, "RAY CASTER", window_width / 2 - 80, window_height / 2 - 60, title_color, 3, window_width, window_height);
        self.draw_text(buffer, "RUST EDITION", window_width / 2 - 60, window_height / 2 - 20, 0xFFFFFF, 2, window_width, window_height);
        
        // Instructions
        self.draw_text(buffer, "Press SPACE to continue", window_width / 2 - 100, window_height / 2 + 40, 0xCCCCCC, 1, window_width, window_height);
        
        // Animated sprite (spinning cube)
        self.render_animated_sprite(buffer, window_width / 2, window_height / 2 + 100, elapsed, window_width, window_height);
    }

    pub fn render_level_select(&self, buffer: &mut Vec<u32>, window_width: usize, window_height: usize) {
        // Clear with gradient background
        for y in 0..window_height {
            for x in 0..window_width {
                let gradient = (y as f64 / window_height as f64 * 255.0) as u32;
                buffer[y * window_width + x] = gradient << 8; // Green gradient
            }
        }

        // Title
        self.draw_text(buffer, "SELECT LEVEL", window_width / 2 - 70, 100, 0xFFFFFF, 3, window_width, window_height);
        
        // Level options
        self.draw_text(buffer, "1 - Beginner Maze", window_width / 2 - 80, 200, 0xFFFF44, 2, window_width, window_height);
        self.draw_text(buffer, "2 - Stone Fortress", window_width / 2 - 90, 250, 0x44FFFF, 2, window_width, window_height);
        self.draw_text(buffer, "3 - Metal Labyrinth", window_width / 2 - 95, 300, 0xFF44FF, 2, window_width, window_height);
        
        // Instructions
        self.draw_text(buffer, "Press 1, 2, or 3 to select", window_width / 2 - 120, 400, 0xCCCCCC, 1, window_width, window_height);
    }

    pub fn render_success_screen(&self, buffer: &mut Vec<u32>, window_width: usize, window_height: usize) {
        // Clear with golden background
        for pixel in buffer.iter_mut() {
            *pixel = 0x332200;
        }

        // Victory message
        self.draw_text(buffer, "LEVEL COMPLETE!", window_width / 2 - 90, window_height / 2 - 60, 0xFFD700, 3, window_width, window_height);
        self.draw_text(buffer, "Congratulations!", window_width / 2 - 80, window_height / 2 - 20, 0xFFFFFF, 2, window_width, window_height);
        
        // Instructions
        self.draw_text(buffer, "Press SPACE for level select", window_width / 2 - 120, window_height / 2 + 40, 0xCCCCCC, 1, window_width, window_height);
        
        // Draw victory stars
        for i in 0..10 {
            let x = window_width / 2 + ((i as f64 * 0.628).cos() * 100.0) as usize;
            let y = window_height / 2 + ((i as f64 * 0.628).sin() * 50.0) as usize;
            self.draw_star(buffer, x, y, 0xFFD700, window_width, window_height);
        }
    }

    pub fn render_game_over_screen(&self, buffer: &mut Vec<u32>, window_width: usize, window_height: usize) {
        // Clear with dark red background
        for pixel in buffer.iter_mut() {
            *pixel = 0x220000;
        }

        // Game over message
        self.draw_text(buffer, "GAME OVER", window_width / 2 - 60, window_height / 2 - 60, 0xFF4444, 3, window_width, window_height);
        self.draw_text(buffer, "You have died!", window_width / 2 - 70, window_height / 2 - 20, 0xFFFFFF, 2, window_width, window_height);
        
        // Instructions
        self.draw_text(buffer, "Press R to restart", window_width / 2 - 80, window_height / 2 + 20, 0xCCCCCC, 1, window_width, window_height);
        self.draw_text(buffer, "Press M for menu", window_width / 2 - 75, window_height / 2 + 50, 0xCCCCCC, 1, window_width, window_height);
    }

    pub fn render_minimap(&self, buffer: &mut Vec<u32>, player: &Player, map: &Map, window_width: usize, window_height: usize) {
        let minimap_size = 120;
        let minimap_x = window_width - minimap_size - 10;
        let minimap_y = 10;
        let cell_size = minimap_size / map.width.max(map.height);
        
        // Draw map cells with different colors for each wall type
        for y in 0..map.height {
            for x in 0..map.width {
                let cell = map.get_cell(x, y);
                let color = match cell {
                    CellType::Empty => 0x000000,
                    CellType::Wall1 => 0xFF4444, // Red walls
                    CellType::Wall2 => 0x4444FF, // Blue walls
                    CellType::Wall3 => 0x44FF44, // Green walls
                    CellType::Wall4 => 0xFFFF44, // Yellow walls
                    CellType::Exit => 0xFF00FF,   // Magenta exit
                    CellType::Hazard => 0xFF8800, // Orange hazard
                };
                
                // Draw cell
                for dy in 0..cell_size {
                    for dx in 0..cell_size {
                        let px = minimap_x + x * cell_size + dx;
                        let py = minimap_y + y * cell_size + dy;
                        if px < window_width && py < window_height {
                            buffer[py * window_width + px] = color;
                        }
                    }
                }
            }
        }
        
        // Draw player as arrow showing direction
        let player_x = minimap_x + (player.x * cell_size as f64) as usize;
        let player_y = minimap_y + (player.y * cell_size as f64) as usize;
        
        // Draw arrow pointing in player's direction
        let cos_angle = player.angle.cos();
        let sin_angle = player.angle.sin();
        
        // Arrow tip (front)
        let tip_x = player_x as i32 + (cos_angle * 6.0) as i32;
        let tip_y = player_y as i32 + (sin_angle * 6.0) as i32;
        
        // Arrow base corners
        let base_left_x = player_x as i32 + (cos_angle * -3.0 - sin_angle * 2.0) as i32;
        let base_left_y = player_y as i32 + (sin_angle * -3.0 + cos_angle * 2.0) as i32;
        let base_right_x = player_x as i32 + (cos_angle * -3.0 + sin_angle * 2.0) as i32;
        let base_right_y = player_y as i32 + (sin_angle * -3.0 - cos_angle * 2.0) as i32;
        
        // Draw arrow lines
        self.draw_line(buffer, player_x, player_y, tip_x as usize, tip_y as usize, 0xFFFF00, window_width, window_height); // Yellow tip
        self.draw_line(buffer, player_x, player_y, base_left_x as usize, base_left_y as usize, 0xFFFFFF, window_width, window_height); // White base
        self.draw_line(buffer, player_x, player_y, base_right_x as usize, base_right_y as usize, 0xFFFFFF, window_width, window_height); // White base
        self.draw_line(buffer, base_left_x as usize, base_left_y as usize, base_right_x as usize, base_right_y as usize, 0xFFFFFF, window_width, window_height); // White base
        
        // Draw center dot
        if player_x < window_width && player_y < window_height {
            buffer[player_y * window_width + player_x] = 0xFF0000; // Red center
        }
    }

    pub fn render_hud(&mut self, buffer: &mut Vec<u32>, health: i32, window_width: usize, window_height: usize) {
        // Health bar
        let health_bar_width = 200;
        let health_bar_height = 20;
        let health_bar_x = 10;
        let health_bar_y = window_height - 40;
        
        // Health bar rendering
        let health_width = (health_bar_width as f64 * (health as f64 / 100.0)) as usize;
        let health_color = if health > 60 {
            0x44FF44
        } else if health > 30 {
            0xFFFF44
        } else {
            0xFF4444
        };
        
        // Single pass health bar rendering
        for y in health_bar_y..health_bar_y + health_bar_height {
            if y >= window_height { break; }
            let row_start = y * window_width + health_bar_x;
            
            for x in 0..health_bar_width {
                if health_bar_x + x >= window_width { break; }
                
                let color = if y == health_bar_y || y == health_bar_y + health_bar_height - 1 ||
                              x == 0 || x == health_bar_width - 1 {
                    0x444444 // Border
                } else if x >= 2 && x - 2 < health_width && y > health_bar_y + 1 && y < health_bar_y + health_bar_height - 2 {
                    health_color // Health fill
                } else {
                    0x444444 // Background
                };
                
                buffer[row_start + x] = color;
            }
        }
        
        // Health text
        self.draw_text(buffer, &format!("Health: {}", health), health_bar_x, health_bar_y - 25, 0xFFFFFF, 1, window_width, window_height);
        
        // Controls help
        self.draw_text(buffer, "WASD: Move | Mouse: Look | F: Flashlight", 10, 10, 0xCCCCCC, 1, window_width, window_height);
        
        // Draw crosshair in center of screen
        self.draw_crosshair(buffer, window_width, window_height);
        
        // Update animated sprite
        self.animation_timer += 0.016;
        if self.animation_timer > 0.2 {
            self.animated_sprite_frame = (self.animated_sprite_frame + 1) % 4;
            self.animation_timer = 0.0;
        }
        
        // Draw animated sprite in corner
        self.render_animated_sprite(buffer, window_width - 50, window_height - 50, self.animation_timer * 10.0, window_width, window_height);
    }

    fn render_animated_sprite(&self, buffer: &mut Vec<u32>, x: usize, y: usize, time: f64, window_width: usize, window_height: usize) {
        let size = 20;
        let rotation = time * 2.0;
        
        // Draw rotating square
        for dy in -(size as i32)/2..=(size as i32)/2 {
            for dx in -(size as i32)/2..=(size as i32)/2 {
                // Rotate point
                let cos_r = rotation.cos();
                let sin_r = rotation.sin();
                let rx = dx as f64 * cos_r - dy as f64 * sin_r;
                let ry = dx as f64 * sin_r + dy as f64 * cos_r;
                
                if rx.abs() < 8.0 && ry.abs() < 8.0 {
                    let px = (x as i32 + rx as i32) as usize;
                    let py = (y as i32 + ry as i32) as usize;
                    
                    if px < window_width && py < window_height {
                        let color = self.interpolate_color(0xFF4444, 0x4444FF, (time * 0.5).sin().abs());
                        buffer[py * window_width + px] = color;
                    }
                }
            }
        }
    }

    fn draw_text(&self, buffer: &mut Vec<u32>, text: &str, x: usize, y: usize, color: u32, scale: usize, window_width: usize, window_height: usize) {
        let font = self.get_simple_font();
        let char_width = 8 * scale;
        let _char_height = 8 * scale;
        
        for (i, ch) in text.chars().enumerate() {
            let char_x = x + i * char_width;
            if char_x >= window_width { break; }
            
            let char_data = font.get(&ch).unwrap_or(&[0u8; 8]);
            
            for row in 0..8 {
                for col in 0..8 {
                    if (char_data[row] >> (7 - col)) & 1 == 1 {
                        for sy in 0..scale {
                            for sx in 0..scale {
                                let px = char_x + col * scale + sx;
                                let py = y + row * scale + sy;
                                if px < window_width && py < window_height {
                                    buffer[py * window_width + px] = color;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn draw_star(&self, buffer: &mut Vec<u32>, x: usize, y: usize, color: u32, window_width: usize, window_height: usize) {
        let points = [
            (0, -8), (2, -2), (8, -2), (3, 2), (5, 8),
            (0, 5), (-5, 8), (-3, 2), (-8, -2), (-2, -2)
        ];
        
        for i in 0..points.len() {
            let (x1, y1) = points[i];
            let (x2, y2) = points[(i + 1) % points.len()];
            self.draw_line(buffer, 
                (x as i32 + x1) as usize, (y as i32 + y1) as usize,
                (x as i32 + x2) as usize, (y as i32 + y2) as usize, color, window_width, window_height);
        }
    }
    
    fn draw_crosshair(&self, buffer: &mut Vec<u32>, window_width: usize, window_height: usize) {
        let center_x = window_width / 2;
        let center_y = window_height / 2;
        let crosshair_size = 10;
        let crosshair_thickness = 1;
        let crosshair_color = 0xFFFFFF; // White crosshair
        
        // Draw horizontal line
        for x in (center_x - crosshair_size)..(center_x + crosshair_size + 1) {
            for thickness in 0..crosshair_thickness + 1 {
                if x < window_width && (center_y + thickness) < window_height {
                    buffer[(center_y + thickness) * window_width + x] = crosshair_color;
                }
                if x < window_width && center_y >= thickness {
                    buffer[(center_y - thickness) * window_width + x] = crosshair_color;
                }
            }
        }
        
        // Draw vertical line
        for y in (center_y - crosshair_size)..(center_y + crosshair_size + 1) {
            for thickness in 0..crosshair_thickness + 1 {
                if (center_x + thickness) < window_width && y < window_height {
                    buffer[y * window_width + (center_x + thickness)] = crosshair_color;
                }
                if center_x >= thickness && y < window_height {
                    buffer[y * window_width + (center_x - thickness)] = crosshair_color;
                }
            }
        }
        
        // Draw center dot
        if center_x < window_width && center_y < window_height {
            buffer[center_y * window_width + center_x] = crosshair_color;
        }
    }

    fn draw_line(&self, buffer: &mut Vec<u32>, x1: usize, y1: usize, x2: usize, y2: usize, color: u32, window_width: usize, window_height: usize) {
        let dx = (x2 as i32 - x1 as i32).abs();
        let dy = (y2 as i32 - y1 as i32).abs();
        let sx = if x1 < x2 { 1 } else { -1 };
        let sy = if y1 < y2 { 1 } else { -1 };
        let mut err = dx - dy;
        
        let mut x = x1 as i32;
        let mut y = y1 as i32;
        
        loop {
            if x >= 0 && x < window_width as i32 && y >= 0 && y < window_height as i32 {
                buffer[y as usize * window_width + x as usize] = color;
            }
            
            if x == x2 as i32 && y == y2 as i32 { break; }
            
            let e2 = 2 * err;
            if e2 > -dy {
                err -= dy;
                x += sx;
            }
            if e2 < dx {
                err += dx;
                y += sy;
            }
        }
    }

    fn interpolate_color(&self, color1: u32, color2: u32, factor: f64) -> u32 {
        let r1 = ((color1 >> 16) & 0xFF) as f64;
        let g1 = ((color1 >> 8) & 0xFF) as f64;
        let b1 = (color1 & 0xFF) as f64;
        
        let r2 = ((color2 >> 16) & 0xFF) as f64;
        let g2 = ((color2 >> 8) & 0xFF) as f64;
        let b2 = (color2 & 0xFF) as f64;
        
        let r = (r1 * (1.0 - factor) + r2 * factor) as u32;
        let g = (g1 * (1.0 - factor) + g2 * factor) as u32;
        let b = (b1 * (1.0 - factor) + b2 * factor) as u32;
        
        (r << 16) | (g << 8) | b
    }

    fn get_simple_font(&self) -> std::collections::HashMap<char, [u8; 8]> {
        let mut font = std::collections::HashMap::new();
        
        // Simple 8x8 bitmap font (partial implementation)
        font.insert(' ', [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
        font.insert('A', [0x18, 0x3C, 0x66, 0x66, 0x7E, 0x66, 0x66, 0x00]);
        font.insert('B', [0x7C, 0x66, 0x66, 0x7C, 0x66, 0x66, 0x7C, 0x00]);
        font.insert('C', [0x3C, 0x66, 0x60, 0x60, 0x60, 0x66, 0x3C, 0x00]);
        font.insert('D', [0x78, 0x6C, 0x66, 0x66, 0x66, 0x6C, 0x78, 0x00]);
        font.insert('E', [0x7E, 0x60, 0x60, 0x7C, 0x60, 0x60, 0x7E, 0x00]);
        font.insert('F', [0x7E, 0x60, 0x60, 0x7C, 0x60, 0x60, 0x60, 0x00]);
        font.insert('G', [0x3C, 0x66, 0x60, 0x6E, 0x66, 0x66, 0x3C, 0x00]);
        font.insert('H', [0x66, 0x66, 0x66, 0x7E, 0x66, 0x66, 0x66, 0x00]);
        font.insert('I', [0x3C, 0x18, 0x18, 0x18, 0x18, 0x18, 0x3C, 0x00]);
        font.insert('L', [0x60, 0x60, 0x60, 0x60, 0x60, 0x60, 0x7E, 0x00]);
        font.insert('M', [0x63, 0x77, 0x7F, 0x6B, 0x63, 0x63, 0x63, 0x00]);
        font.insert('N', [0x66, 0x76, 0x7E, 0x7E, 0x6E, 0x66, 0x66, 0x00]);
        font.insert('O', [0x3C, 0x66, 0x66, 0x66, 0x66, 0x66, 0x3C, 0x00]);
        font.insert('P', [0x7C, 0x66, 0x66, 0x7C, 0x60, 0x60, 0x60, 0x00]);
        font.insert('R', [0x7C, 0x66, 0x66, 0x7C, 0x6C, 0x66, 0x66, 0x00]);
        font.insert('S', [0x3C, 0x66, 0x60, 0x3C, 0x06, 0x66, 0x3C, 0x00]);
        font.insert('T', [0x7E, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x00]);
        font.insert('U', [0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x3C, 0x00]);
        font.insert('V', [0x66, 0x66, 0x66, 0x66, 0x66, 0x3C, 0x18, 0x00]);
        font.insert('W', [0x63, 0x63, 0x63, 0x6B, 0x7F, 0x77, 0x63, 0x00]);
        font.insert('Y', [0x66, 0x66, 0x66, 0x3C, 0x18, 0x18, 0x18, 0x00]);
        
        // Numbers
        font.insert('0', [0x3C, 0x66, 0x6E, 0x76, 0x66, 0x66, 0x3C, 0x00]);
        font.insert('1', [0x18, 0x38, 0x18, 0x18, 0x18, 0x18, 0x7E, 0x00]);
        font.insert('2', [0x3C, 0x66, 0x06, 0x0C, 0x30, 0x60, 0x7E, 0x00]);
        font.insert('3', [0x3C, 0x66, 0x06, 0x1C, 0x06, 0x66, 0x3C, 0x00]);
        
        // Special characters
        font.insert('!', [0x18, 0x18, 0x18, 0x18, 0x00, 0x18, 0x18, 0x00]);
        font.insert(':', [0x00, 0x18, 0x18, 0x00, 0x18, 0x18, 0x00, 0x00]);
        font.insert('-', [0x00, 0x00, 0x00, 0x7E, 0x00, 0x00, 0x00, 0x00]);
        
        font
    }
}