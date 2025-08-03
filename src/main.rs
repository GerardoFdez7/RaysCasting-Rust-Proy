use minifb::{Key, Window, WindowOptions, MouseMode, CursorStyle};
use std::time::{Duration, Instant};
use std::f64::consts::PI;
use rand::Rng;

mod game;
mod player;
mod map;
mod raycaster;
mod audio;
mod ui;
mod effects;

use game::*;
use player::*;
use map::*;
use raycaster::*;
use audio::*;
use ui::*;
use effects::*;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;
const FULLSCREEN_WIDTH: usize = 1600;
const FULLSCREEN_HEIGHT: usize = 900;

fn main() {
    let mut window_options = WindowOptions::default();
    let mut is_fullscreen = false;
    
    let mut window = Window::new(
        "Ray Casting Game - Rust",
        WIDTH,
        HEIGHT,
        window_options,
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });
    
    let mut game = Game::new();
    let mut last_time = Instant::now();
    let mut fps_counter = 0;
    let mut fps_timer = Instant::now();
    let mut last_mouse_pos = (WIDTH as f32 / 2.0, HEIGHT as f32 / 2.0);
    let mut cursor_hidden = false;
    let mut mouse_captured = false;
    let mut last_f11_state = false;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let current_time = Instant::now();
        let delta_time = current_time.duration_since(last_time).as_secs_f64();
        last_time = current_time;

        // FPS counter
        fps_counter += 1;
        if fps_timer.elapsed() >= Duration::from_secs(1) {
            println!("FPS: {}", fps_counter);
            fps_counter = 0;
            fps_timer = Instant::now();
        }
        
        // Handle fullscreen toggle with F11
        let f11_pressed = window.is_key_down(Key::F11);
        if f11_pressed && !last_f11_state {
            is_fullscreen = !is_fullscreen;
            
            // Recreate window with new fullscreen state
            let mut new_options = WindowOptions::default();
            if is_fullscreen {
                new_options.borderless = true;
                new_options.title = false;
                new_options.resize = false;
            }
            
            // Use large resolution for fullscreen
            let (new_width, new_height) = if is_fullscreen {
                (1920, 1080)
            } else {
                (WIDTH, HEIGHT)
            };
            
            // Create new window
            if let Ok(new_window) = Window::new(
                "Ray Casting Game - Rust",
                new_width,
                new_height,
                new_options,
            ) {
                window = new_window;
                println!("Toggled fullscreen: {}", is_fullscreen);
            }
        }
        last_f11_state = f11_pressed;

        // Handle cursor visibility and mouse capture based on game state
        if matches!(game.state, GameState::Playing) {
            // Hide cursor and capture mouse during gameplay
            if !cursor_hidden {
                window.set_cursor_visibility(false);
                cursor_hidden = true;
                mouse_captured = true;
                // Initialize mouse position to center of screen
                let (render_width, render_height) = if is_fullscreen {
                    let size = window.get_size();
                    (size.0, size.1)
                } else {
                    (WIDTH, HEIGHT)
                };
                last_mouse_pos = (render_width as f32 / 2.0, render_height as f32 / 2.0);
            }
            
            // Get current mouse position and calculate delta
            let current_mouse_pos = window.get_mouse_pos(MouseMode::Pass).unwrap_or(last_mouse_pos);
            
            // Calculate mouse delta for FPS-style control
            let mouse_delta_x = current_mouse_pos.0 - last_mouse_pos.0;
            let mouse_delta_y = current_mouse_pos.1 - last_mouse_pos.1;
            
            // Apply sensitivity to mouse movement
            let sensitivity = 0.00005;
            let mouse_input = (mouse_delta_x * sensitivity, mouse_delta_y * sensitivity);
            
            game.update(delta_time, &window, mouse_input);
            
            // Update last mouse position
            last_mouse_pos = current_mouse_pos;
            
            // Keep mouse centered for continuous movement
            let (render_width, render_height) = if is_fullscreen {
                let size = window.get_size();
                (size.0, size.1)
            } else {
                (WIDTH, HEIGHT)
            };
            let center_x = render_width as f32 / 2.0;
            let center_y = render_height as f32 / 2.0;
            
            // If mouse moved too far from center, reset it (prevents edge hitting)
            let distance_from_center = ((current_mouse_pos.0 - center_x).powi(2) + (current_mouse_pos.1 - center_y).powi(2)).sqrt();
            if distance_from_center > 100.0 {
                last_mouse_pos = (center_x, center_y);
            }
        } else {
            // Show normal cursor when not playing
            if cursor_hidden {
                window.set_cursor_visibility(true);
                cursor_hidden = false;
                mouse_captured = false;
            }
            
            // Normal mouse handling for menus
            let mouse_pos = window.get_mouse_pos(MouseMode::Pass).unwrap_or(last_mouse_pos);
            game.update(delta_time, &window, (0.0, 0.0));
            last_mouse_pos = mouse_pos;
        }
        
        // Get actual window dimensions for rendering
        let (render_width, render_height) = if is_fullscreen {
            // Get actual window size after fullscreen creation
            let size = window.get_size();
            (size.0, size.1)
        } else {
            (WIDTH, HEIGHT)
        };
        let buffer = game.render(render_width, render_height);
        
        window.update_with_buffer(&buffer, render_width, render_height).unwrap();
    }
}