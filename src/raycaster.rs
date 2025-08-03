use crate::*;
use std::f64::consts::PI;

pub struct RayCaster {
    pub fov: f64,
    pub max_depth: f64,
}

impl RayCaster {
    pub fn new() -> Self {
        Self {
            fov: PI / 3.0, // 60 degrees
            max_depth: 20.0,
        }
    }

    pub fn render(&self, buffer: &mut Vec<u32>, player: &Player, map: &Map, effects: &Effects, window_width: usize, window_height: usize) {
        let half_fov = self.fov / 2.0;
        let angle_step = self.fov / window_width as f64;
        
        for x in 0..window_width {
            let ray_angle = player.angle - half_fov + (x as f64 * angle_step);
            let (distance, wall_type, hit_side) = self.cast_ray(player.x, player.y, ray_angle, map);
            
            if distance < self.max_depth {
                self.draw_wall_slice(buffer, x, distance, wall_type, hit_side, map, effects, ray_angle, player, window_width, window_height);
            } else {
                self.draw_background_slice(buffer, x, window_width, window_height);
            }
        }
    }

    fn cast_ray(&self, start_x: f64, start_y: f64, angle: f64, map: &Map) -> (f64, CellType, bool) {
        let dx = angle.cos();
        let dy = angle.sin();
        
        // DDA algorithm for fast ray casting
        let mut map_x = start_x as i32;
        let mut map_y = start_y as i32;
        
        let delta_dist_x = (1.0 / dx).abs();
        let delta_dist_y = (1.0 / dy).abs();
        
        let (step_x, mut side_dist_x) = if dx < 0.0 {
            (-1, (start_x - map_x as f64) * delta_dist_x)
        } else {
            (1, (map_x as f64 + 1.0 - start_x) * delta_dist_x)
        };
        
        let (step_y, mut side_dist_y) = if dy < 0.0 {
            (-1, (start_y - map_y as f64) * delta_dist_y)
        } else {
            (1, (map_y as f64 + 1.0 - start_y) * delta_dist_y)
        };
        
        let mut hit = false;
        let mut side = false; // false if x-side, true if y-side
        let mut iterations = 0;
        const MAX_ITERATIONS: i32 = 100; // Prevent infinite loops
        
        // Perform DDA
        while !hit && iterations < MAX_ITERATIONS {
            iterations += 1;
            
            // Jump to next map square, either in x-direction, or in y-direction
            if side_dist_x < side_dist_y {
                side_dist_x += delta_dist_x;
                map_x += step_x;
                side = false;
            } else {
                side_dist_y += delta_dist_y;
                map_y += step_y;
                side = true;
            }
            
            // Check if ray has hit a wall
            if map_x < 0 || map_y < 0 || map_x >= map.width as i32 || map_y >= map.height as i32 {
                return (self.max_depth, CellType::Empty, false);
            }
            
            if map.is_wall(map_x as usize, map_y as usize) {
                hit = true;
            }
        }
        
        // If we hit max iterations without finding a wall, return max depth
        if iterations >= MAX_ITERATIONS {
            return (self.max_depth, CellType::Empty, false);
        }
        
        // Calculate distance
        let perp_wall_dist = if !side {
            (map_x as f64 - start_x + (1.0 - step_x as f64) / 2.0) / dx
        } else {
            (map_y as f64 - start_y + (1.0 - step_y as f64) / 2.0) / dy
        };
        
        let cell_type = map.get_cell(map_x as usize, map_y as usize);
        (perp_wall_dist, cell_type, side)
    }

    fn draw_wall_slice(&self, buffer: &mut Vec<u32>, x: usize, distance: f64, wall_type: CellType, hit_side: bool, map: &Map, effects: &Effects, ray_angle: f64, player: &Player, window_width: usize, window_height: usize) {
        // Calculate wall height based on distance
        let wall_height = (window_height as f64 / distance).min(window_height as f64);
        let wall_start = ((window_height as f64 - wall_height) / 2.0) as usize;
        let wall_end = (wall_start + wall_height as usize).min(window_height);
        
        // Get base wall color
        let base_color = map.get_wall_color(wall_type);
        
        // Calculate lighting with flashlight effect
        let mut lighting = if hit_side { 0.7 } else { 1.0 };
        
        // Apply flashlight effect
        let angle_diff = ray_angle - player.angle;
        let flashlight_intensity = effects.get_flashlight_intensity(angle_diff);
        lighting *= flashlight_intensity;
        
        // Extract RGB components and apply lighting
        let r = ((base_color >> 16) & 0xFF) as f64;
        let g = ((base_color >> 8) & 0xFF) as f64;
        let b = (base_color & 0xFF) as f64;
        
        let lit_r = (r * lighting) as u32;
        let lit_g = (g * lighting) as u32;
        let lit_b = (b * lighting) as u32;
        
        let final_color = (lit_r << 16) | (lit_g << 8) | lit_b;
        
        let ceiling_color = 0x87CEEB; // Sky blue
        let floor_color = 0x404040; // Dark gray
        
        // Optimized single loop for entire column
        for y in 0..window_height {
            let pixel_index = y * window_width + x;
            
            if y < wall_start {
                // Ceiling
                buffer[pixel_index] = ceiling_color;
            } else if y < wall_end {
                // Wall with texture
                let wall_y = y - wall_start;
                let texture_y = (wall_y as f64 / wall_height * 64.0) as usize % 64;
                let texture_color = self.apply_wall_texture(final_color, texture_y, wall_type);
                buffer[pixel_index] = texture_color;
            } else {
                // Floor
                buffer[pixel_index] = floor_color;
            }
        }
    }

    fn draw_background_slice(&self, buffer: &mut Vec<u32>, x: usize, window_width: usize, window_height: usize) {
        for y in 0..window_height {
            if y < window_height / 2 {
                buffer[y * window_width + x] = 0x87CEEB; // Sky blue ceiling
            } else {
                buffer[y * window_width + x] = 0x404040; // Dark gray floor
            }
        }
    }

    fn apply_wall_texture(&self, base_color: u32, _texture_y: usize, _wall_type: CellType) -> u32 {
        base_color
    }

    fn get_ceiling_color(&self, y: usize, effects: &Effects, window_height: usize) -> u32 {
        let base_color = 0x87CEEB; // Sky blue
        
        if effects.fog_enabled {
            let fog_factor = (y as f64 / (window_height as f64 / 2.0)).min(1.0);
            let fog_color = 0x708090; // Slate gray fog
            self.blend_colors(base_color, fog_color, fog_factor * 0.3)
        } else {
            base_color
        }
    }

    fn get_floor_color(&self, y: usize, effects: &Effects, window_height: usize) -> u32 {
        let base_color = 0x404040; // Dark gray
        
        if effects.fog_enabled {
            let fog_factor = ((window_height - y) as f64 / (window_height as f64 / 2.0)).min(1.0);
            let fog_color = 0x708090; // Slate gray fog
            self.blend_colors(base_color, fog_color, fog_factor * 0.3)
        } else {
            base_color
        }
    }

    fn blend_colors(&self, color1: u32, color2: u32, factor: f64) -> u32 {
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
}