use crate::*;
use minifb::{Key, Window};
use std::time::Instant;

#[derive(Clone, Copy, PartialEq)]
pub enum GameState {
    Splash,
    LevelSelect,
    Playing,
    Success,
    GameOver,
}

pub struct Game {
    pub state: GameState,
    pub player: Player,
    pub map: Map,
    pub raycaster: RayCaster,
    pub audio: AudioSystem,
    pub ui: UI,
    pub effects: Effects,
    pub buffer: Vec<u32>,
    pub current_level: usize,
    pub player_health: i32,
    pub splash_timer: f64,
    pub level_complete: bool,
    pub mouse_sensitivity: f64,
    pub last_mouse_x: f64,
    pub last_keys: std::collections::HashSet<Key>,
    pub wall_damage_cooldown: f64,
    pub hazard_damage_cooldown: f64,
}

impl Game {
    pub fn new() -> Self {
        let map = Map::new(0);
        let player = Player::new(1.5, 1.5, 0.0);
        let raycaster = RayCaster::new();
        let audio = AudioSystem::new();
        let ui = UI::new();
        let effects = Effects::new();
        
        Self {
            state: GameState::Splash,
            player,
            map,
            raycaster,
            audio,
            ui,
            effects,
            buffer: vec![0; WIDTH * HEIGHT],
            current_level: 0,
            player_health: 100,
            splash_timer: 0.0,
            level_complete: false,
            mouse_sensitivity: 0.002,
            last_mouse_x: WIDTH as f64 / 2.0,
            last_keys: std::collections::HashSet::new(),
            wall_damage_cooldown: 0.0,
            hazard_damage_cooldown: 0.0,
        }
    }

    pub fn update(&mut self, delta_time: f64, window: &Window, mouse_pos: (f32, f32)) {
        // Helper function to check if key was just pressed
        let key_just_pressed = |key: Key, last_keys: &std::collections::HashSet<Key>| {
            window.is_key_down(key) && !last_keys.contains(&key)
        };
        
        match self.state {
            GameState::Splash => {
                self.splash_timer += delta_time;
                if self.splash_timer > 3.0 || key_just_pressed(Key::Space, &self.last_keys) {
                    self.state = GameState::LevelSelect;
                    self.audio.play_menu_music();
                }
            },
            GameState::LevelSelect => {
                if key_just_pressed(Key::Key1, &self.last_keys) {
                    self.start_level(0);
                } else if key_just_pressed(Key::Key2, &self.last_keys) {
                    self.start_level(1);
                } else if key_just_pressed(Key::Key3, &self.last_keys) {
                    self.start_level(2);
                }
            },
            GameState::Playing => {
                self.update_gameplay(delta_time, window, mouse_pos);
            },
            GameState::Success => {
                if key_just_pressed(Key::Space, &self.last_keys) {
                    self.state = GameState::LevelSelect;
                    self.audio.play_menu_music();
                }
            },
            GameState::GameOver => {
                if key_just_pressed(Key::R, &self.last_keys) {
                    self.restart_level();
                } else if key_just_pressed(Key::M, &self.last_keys) {
                    self.state = GameState::LevelSelect;
                    self.audio.play_menu_music();
                }
            },
        }
        
        // Update key tracking
        self.last_keys.clear();
        for key in [Key::Space, Key::Key1, Key::Key2, Key::Key3, Key::R, Key::M, Key::F].iter() {
            if window.is_key_down(*key) {
                self.last_keys.insert(*key);
            }
        }
        
        self.effects.update(delta_time);
    }

    fn update_gameplay(&mut self, delta_time: f64, window: &Window, mouse_pos: (f32, f32)) {
        // Mouse rotation (horizontal only) - mouse_pos.0 is already the delta
        let mouse_delta_x = mouse_pos.0 as f64;
        self.player.angle += mouse_delta_x;

        // Player movement
        let mut move_x = 0.0;
        let mut move_y = 0.0;
        
        if window.is_key_down(Key::W) || window.is_key_down(Key::Up) {
            move_x += self.player.angle.cos();
            move_y += self.player.angle.sin();
        }
        if window.is_key_down(Key::S) || window.is_key_down(Key::Down) {
            move_x -= self.player.angle.cos();
            move_y -= self.player.angle.sin();
        }
        if window.is_key_down(Key::A) || window.is_key_down(Key::Left) {
            move_x += (self.player.angle - PI/2.0).cos();
            move_y += (self.player.angle - PI/2.0).sin();
        }
        if window.is_key_down(Key::D) || window.is_key_down(Key::Right) {
            move_x += (self.player.angle + PI/2.0).cos();
            move_y += (self.player.angle + PI/2.0).sin();
        }

        // Normalize movement vector
        let move_length = (move_x * move_x + move_y * move_y).sqrt();
        if move_length > 0.0 {
            move_x /= move_length;
            move_y /= move_length;
            
            let hit_wall = self.player.update(delta_time, move_x, move_y, &self.map);
            
            // Wall collision damage with cooldown
            if hit_wall && self.wall_damage_cooldown <= 0.0 {
                let damage = 5; // Fixed damage per collision
                self.player_health -= damage;
                self.effects.trigger_damage_effect();
                self.wall_damage_cooldown = 0.5; // 0.5 second cooldown
                
                if self.player_health <= 0 {
                    self.state = GameState::GameOver;
                    self.audio.play_death_sound();
                }
            }
            
            if move_length > 0.1 {
                self.audio.play_footstep();
            }
        }

        // Optimized level completion check
        let player_grid_x = self.player.x as usize;
        let player_grid_y = self.player.y as usize;
        
        // Simple check - just see if player is on exit cell
        let mut found_exit = false;
        if self.map.is_exit(player_grid_x, player_grid_y) {
            found_exit = true;
        }
        
        if found_exit {
            self.level_complete = true;
            self.state = GameState::Success;
            self.audio.play_success_sound();
        }

        // Damage from hazards (orange zones) with cooldown
        if self.map.is_hazard(self.player.x as usize, self.player.y as usize) && self.hazard_damage_cooldown <= 0.0 {
            let damage = 10; // Fixed damage per hazard tick
            self.player_health -= damage;
            self.effects.trigger_damage_effect();
            self.hazard_damage_cooldown = 0.3; // 0.3 second cooldown (faster than wall damage)
            
            if self.player_health <= 0 {
                self.state = GameState::GameOver;
                self.audio.play_death_sound();
            }
        }

        // Toggle flashlight
        if window.is_key_down(Key::F) && !self.last_keys.contains(&Key::F) {
            self.effects.toggle_flashlight();
            println!("Flashlight toggled: {}", self.effects.flashlight_enabled);
        }
        
        // Update effects
        self.effects.update(delta_time);
        
        // Update wall damage cooldown
        if self.wall_damage_cooldown > 0.0 {
            self.wall_damage_cooldown -= delta_time;
        }
        
        // Update hazard damage cooldown
        if self.hazard_damage_cooldown > 0.0 {
            self.hazard_damage_cooldown -= delta_time;
        }
    }

    fn start_level(&mut self, level: usize) {
        self.current_level = level;
        self.map = Map::new(level);
        self.player = Player::new(1.5, 1.5, 0.0);
        self.player_health = 100;
        self.level_complete = false;
        self.state = GameState::Playing;
        self.audio.play_game_music();
        self.effects.reset();
    }

    fn restart_level(&mut self) {
        self.start_level(self.current_level);
    }

    pub fn render(&mut self, window_width: usize, window_height: usize) -> &Vec<u32> {
        // Resize buffer if window dimensions changed
        let required_size = window_width * window_height;
        if self.buffer.len() != required_size {
            self.buffer.resize(required_size, 0);
        }
        
        // Clear buffer
        for pixel in &mut self.buffer {
            *pixel = 0;
        }

        match self.state {
            GameState::Splash => {
                self.ui.render_splash_screen(&mut self.buffer, window_width, window_height);
            },
            GameState::LevelSelect => {
                self.ui.render_level_select(&mut self.buffer, window_width, window_height);
            },
            GameState::Playing => {
                // Render 3D view
                self.raycaster.render(&mut self.buffer, &self.player, &self.map, &self.effects, window_width, window_height);
                
                // Render optimized minimap
                self.ui.render_minimap(&mut self.buffer, &self.player, &self.map, window_width, window_height);
                
                // Render HUD
                self.ui.render_hud(&mut self.buffer, self.player_health, window_width, window_height);
                
                // Apply visual effects for damage feedback
                self.effects.apply_effects(&mut self.buffer);
                
                // Apply flashlight overlay effect
                self.effects.apply_flashlight_overlay(&mut self.buffer, &self.player, window_width, window_height);
            },
            GameState::Success => {
                self.ui.render_success_screen(&mut self.buffer, window_width, window_height);
            },
            GameState::GameOver => {
                self.ui.render_game_over_screen(&mut self.buffer, window_width, window_height);
            },
        }

        &self.buffer
    }
}