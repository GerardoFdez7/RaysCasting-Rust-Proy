use crate::*;
use rand::Rng;

pub struct Effects {
    pub flashlight_enabled: bool,
    pub fog_enabled: bool,
    pub damage_effect_timer: f64,
    pub anxiety_effect_timer: f64,
    pub screen_shake_intensity: f64,
    pub screen_shake_timer: f64,
    pub vignette_intensity: f64,
    pub color_distortion: f64,
    pub noise_intensity: f64,
}

impl Effects {
    pub fn new() -> Self {
        Self {
            flashlight_enabled: false,
            fog_enabled: true,
            damage_effect_timer: 0.0,
            anxiety_effect_timer: 0.0,
            screen_shake_intensity: 0.0,
            screen_shake_timer: 0.0,
            vignette_intensity: 0.3,
            color_distortion: 0.0,
            noise_intensity: 0.0,
        }
    }

    pub fn update(&mut self, delta_time: f64) {
        // Update damage effect
        if self.damage_effect_timer > 0.0 {
            self.damage_effect_timer -= delta_time;
            self.color_distortion = (self.damage_effect_timer / 0.5).min(1.0);
        } else {
            self.color_distortion = 0.0;
        }

        // Update screen shake
        if self.screen_shake_timer > 0.0 {
            self.screen_shake_timer -= delta_time;
            self.screen_shake_intensity = (self.screen_shake_timer / 0.3).min(1.0) * 5.0;
        } else {
            self.screen_shake_intensity = 0.0;
        }
    }

    pub fn toggle_flashlight(&mut self) {
        self.flashlight_enabled = !self.flashlight_enabled;
    }

    pub fn trigger_damage_effect(&mut self) {
        self.damage_effect_timer = 0.5;
        self.screen_shake_timer = 0.3;
    }

    pub fn reset(&mut self) {
        self.flashlight_enabled = false;
        self.damage_effect_timer = 0.0;
        self.anxiety_effect_timer = 0.0;
        self.screen_shake_timer = 0.0;
        self.color_distortion = 0.0;
    }

    pub fn apply_effects(&self, buffer: &mut Vec<u32>) {
        // Apply simple red overlay when taking damage
        if self.color_distortion > 0.0 {
            self.apply_simple_damage_tint(buffer);
        }
    }
    
    fn apply_simple_damage_tint(&self, buffer: &mut Vec<u32>) {
        let intensity = (self.color_distortion * 100.0) as u32;
        for pixel in buffer.iter_mut() {
            let r = ((*pixel >> 16) & 0xFF) + intensity;
            let g = (*pixel >> 8) & 0xFF;
            let b = *pixel & 0xFF;
            *pixel = ((r.min(255)) << 16) | (g << 8) | b;
        }
    }

    fn apply_screen_shake(&self, buffer: &mut Vec<u32>, window_width: usize, window_height: usize) {
        let mut rng = rand::thread_rng();
        let shake_x = rng.gen_range(-self.screen_shake_intensity as i32..=self.screen_shake_intensity as i32);
        let shake_y = rng.gen_range(-self.screen_shake_intensity as i32..=self.screen_shake_intensity as i32);
        
        let mut new_buffer = vec![0u32; window_width * window_height];
        
        for y in 0..window_height {
            for x in 0..window_width {
                let src_x = (x as i32 - shake_x).max(0).min(window_width as i32 - 1) as usize;
                let src_y = (y as i32 - shake_y).max(0).min(window_height as i32 - 1) as usize;
                new_buffer[y * window_width + x] = buffer[src_y * window_width + src_x];
            }
        }
        
        buffer.copy_from_slice(&new_buffer);
    }

    fn apply_damage_tint(&self, buffer: &mut Vec<u32>) {
        for pixel in buffer.iter_mut() {
            let r = ((*pixel >> 16) & 0xFF) as f64;
            let g = ((*pixel >> 8) & 0xFF) as f64;
            let b = (*pixel & 0xFF) as f64;
            
            // Increase red channel and reduce others
            let new_r = (r + 100.0 * self.color_distortion).min(255.0) as u32;
            let new_g = (g * (1.0 - self.color_distortion * 0.5)) as u32;
            let new_b = (b * (1.0 - self.color_distortion * 0.5)) as u32;
            
            *pixel = (new_r << 16) | (new_g << 8) | new_b;
        }
    }

    fn apply_vignette(&self, buffer: &mut Vec<u32>, window_width: usize, window_height: usize) {
        let center_x = window_width as f64 / 2.0;
        let center_y = window_height as f64 / 2.0;
        let max_distance = (center_x * center_x + center_y * center_y).sqrt();
        
        // Optimized single loop with pre-calculated values
        for (i, pixel) in buffer.iter_mut().enumerate() {
            let x = (i % window_width) as f64;
            let y = (i / window_width) as f64;
            
            let dx = x - center_x;
            let dy = y - center_y;
            let distance_sq = dx * dx + dy * dy;
            let distance = distance_sq.sqrt();
            
            let vignette_factor = 1.0 - (distance / max_distance * self.vignette_intensity).min(1.0);
            
            if vignette_factor < 1.0 {
                let r = ((*pixel >> 16) & 0xFF) as f64;
                let g = ((*pixel >> 8) & 0xFF) as f64;
                let b = (*pixel & 0xFF) as f64;
                
                let new_r = (r * vignette_factor) as u32;
                let new_g = (g * vignette_factor) as u32;
                let new_b = (b * vignette_factor) as u32;
                
                *pixel = (new_r << 16) | (new_g << 8) | new_b;
            }
        }
    }

    fn apply_noise(&self, buffer: &mut Vec<u32>) {
        let mut rng = rand::thread_rng();
        
        for pixel in buffer.iter_mut() {
            if rng.gen::<f64>() < self.noise_intensity {
                let noise_value = rng.gen_range(-20..=20);
                
                let r = ((*pixel >> 16) & 0xFF) as i32;
                let g = ((*pixel >> 8) & 0xFF) as i32;
                let b = (*pixel & 0xFF) as i32;
                
                let new_r = (r + noise_value).max(0).min(255) as u32;
                let new_g = (g + noise_value).max(0).min(255) as u32;
                let new_b = (b + noise_value).max(0).min(255) as u32;
                
                *pixel = (new_r << 16) | (new_g << 8) | new_b;
            }
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

    pub fn get_flashlight_intensity(&self, angle_diff: f64) -> f64 {
        if !self.flashlight_enabled {
            return 1.0;
        }
        
        let flashlight_cone = std::f64::consts::PI / 4.0; // 45 degree cone
        if angle_diff.abs() < flashlight_cone {
            let cone_factor = 1.0 - (angle_diff.abs() / flashlight_cone);
            1.0 + cone_factor * 1.5 // Much brighter in flashlight beam
        } else {
            0.1 // Much darker outside beam
        }
    }
    
    // Flashlight overlay effect
    pub fn apply_flashlight_overlay(&self, buffer: &mut Vec<u32>, player: &Player, window_width: usize, window_height: usize) {
        if !self.flashlight_enabled {
            return;
        }
        
        let center_x = window_width as f64 / 2.0;
        let center_y = window_height as f64 / 2.0;
        let max_radius = (window_width.min(window_height) as f64 / 3.0); // Flashlight radius
        
        for y in 0..window_height {
            for x in 0..window_width {
                let dx = x as f64 - center_x;
                let dy = y as f64 - center_y;
                let distance = (dx * dx + dy * dy).sqrt();
                
                if distance < max_radius {
                    let intensity = 1.0 - (distance / max_radius);
                    let brightness = (intensity * 60.0) as u32; // White overlay intensity
                    
                    let pixel_index = y * window_width + x;
                    let current_pixel = buffer[pixel_index];
                    
                    let r = ((current_pixel >> 16) & 0xFF) + brightness;
                    let g = ((current_pixel >> 8) & 0xFF) + brightness;
                    let b = (current_pixel & 0xFF) + brightness;
                    
                    buffer[pixel_index] = ((r.min(255)) << 16) | ((g.min(255)) << 8) | (b.min(255));
                }
            }
        }
    }
}