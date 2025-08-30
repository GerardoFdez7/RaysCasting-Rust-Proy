use minifb::{Key, Window, WindowOptions};
use std::time::{Duration, Instant};
use std::f64::consts::PI;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

// 3D Vector structure
#[derive(Copy, Clone)]
struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
    
    fn dot(&self, other: &Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    
    fn normalize(&self) -> Self {
        let length = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        Self {
            x: self.x / length,
            y: self.y / length,
            z: self.z / length,
        }
    }
}

// Cube face structure
struct Face {
    vertices: [Vec3; 4],
    normal: Vec3,
    color: u32,
}

// Camera structure
struct Camera {
    position: Vec3,
    direction: Vec3,
    up: Vec3,
    fov: f64,
}

impl Camera {
    fn new() -> Self {
        Self {
            position: Vec3::new(0.0, 0.0, -3.0),
            direction: Vec3::new(0.0, 0.0, 1.0),
            up: Vec3::new(0.0, 1.0, 0.0),
            fov: PI / 3.0, // 60 degrees
        }
    }
    
    fn rotate_y(&mut self, angle: f64) {
        let cos_angle = angle.cos();
        let sin_angle = angle.sin();
        
        let new_x = self.direction.x * cos_angle + self.direction.z * sin_angle;
        let new_z = -self.direction.x * sin_angle + self.direction.z * cos_angle;
        
        self.direction.x = new_x;
        self.direction.z = new_z;
    }
    
    fn move_forward(&mut self, distance: f64) {
        self.position.x += self.direction.x * distance;
        self.position.y += self.direction.y * distance;
        self.position.z += self.direction.z * distance;
    }
}

// Cube structure
struct Cube {
    faces: Vec<Face>,
    rotation: f64,
}

impl Cube {
    fn new() -> Self {
        // Define cube vertices (1x1x1 cube centered at origin)
        let vertices = [
            Vec3::new(-0.5, -0.5, -0.5), // 0: back-bottom-left
            Vec3::new(0.5, -0.5, -0.5),  // 1: back-bottom-right
            Vec3::new(0.5, 0.5, -0.5),   // 2: back-top-right
            Vec3::new(-0.5, 0.5, -0.5),  // 3: back-top-left
            Vec3::new(-0.5, -0.5, 0.5),  // 4: front-bottom-left
            Vec3::new(0.5, -0.5, 0.5),   // 5: front-bottom-right
            Vec3::new(0.5, 0.5, 0.5),    // 6: front-top-right
            Vec3::new(-0.5, 0.5, 0.5),   // 7: front-top-left
        ];
        
        // Define faces with different colors
        let faces = vec![
            // Front face (red)
            Face {
                vertices: [vertices[4], vertices[5], vertices[6], vertices[7]],
                normal: Vec3::new(0.0, 0.0, 1.0),
                color: 0xFF0000, // Red
            },
            // Back face (green)
            Face {
                vertices: [vertices[1], vertices[0], vertices[3], vertices[2]],
                normal: Vec3::new(0.0, 0.0, -1.0),
                color: 0x00FF00, // Green
            },
            // Right face (blue)
            Face {
                vertices: [vertices[5], vertices[1], vertices[2], vertices[6]],
                normal: Vec3::new(1.0, 0.0, 0.0),
                color: 0x0000FF, // Blue
            },
            // Left face (yellow)
            Face {
                vertices: [vertices[0], vertices[4], vertices[7], vertices[3]],
                normal: Vec3::new(-1.0, 0.0, 0.0),
                color: 0xFFFF00, // Yellow
            },
            // Top face (cyan)
            Face {
                vertices: [vertices[7], vertices[6], vertices[2], vertices[3]],
                normal: Vec3::new(0.0, 1.0, 0.0),
                color: 0x00FFFF, // Cyan
            },
            // Bottom face (magenta)
            Face {
                vertices: [vertices[0], vertices[1], vertices[5], vertices[4]],
                normal: Vec3::new(0.0, -1.0, 0.0),
                color: 0xFF00FF, // Magenta
            },
        ];
        
        Self {
            faces,
            rotation: 0.0,
        }
    }
    
    fn update(&mut self, delta_time: f64) {
        // Rotate the cube
        self.rotation += delta_time * 0.5;
    }
    
    fn render(&self, buffer: &mut Vec<u32>, camera: &Camera) {
        // Clear buffer with black
        for pixel in buffer.iter_mut() {
            *pixel = 0x000000;
        }
        
        // Light direction (for diffuse lighting)
        let light_dir = Vec3::new(
            (self.rotation * 0.5).sin(),
            0.7,
            (self.rotation * 0.5).cos()
        ).normalize();
        
        // Render each face of the cube
        for face in &self.faces {
            // Apply rotation to face vertices and normal
            let cos_angle = self.rotation.cos();
            let sin_angle = self.rotation.sin();
            
            let mut rotated_vertices = [Vec3::new(0.0, 0.0, 0.0); 4];
            for (i, vertex) in face.vertices.iter().enumerate() {
                // Rotate around Y axis
                let rotated_x = vertex.x * cos_angle + vertex.z * sin_angle;
                let rotated_z = -vertex.x * sin_angle + vertex.z * cos_angle;
                
                // Rotate around X axis
                let rotated_y = vertex.y * cos_angle - rotated_z * sin_angle;
                let final_z = vertex.y * sin_angle + rotated_z * cos_angle;
                
                rotated_vertices[i] = Vec3::new(rotated_x, rotated_y, final_z);
            }
            
            // Calculate face normal after rotation
            let edge1 = Vec3::new(
                rotated_vertices[1].x - rotated_vertices[0].x,
                rotated_vertices[1].y - rotated_vertices[0].y,
                rotated_vertices[1].z - rotated_vertices[0].z,
            );
            
            let edge2 = Vec3::new(
                rotated_vertices[2].x - rotated_vertices[1].x,
                rotated_vertices[2].y - rotated_vertices[1].y,
                rotated_vertices[2].z - rotated_vertices[1].z,
            );
            
            // Cross product to get normal
            let normal = Vec3::new(
                edge1.y * edge2.z - edge1.z * edge2.y,
                edge1.z * edge2.x - edge1.x * edge2.z,
                edge1.x * edge2.y - edge1.y * edge2.x,
            ).normalize();
            
            // Backface culling - skip faces pointing away from camera
            let view_dir = Vec3::new(
                rotated_vertices[0].x - camera.position.x,
                rotated_vertices[0].y - camera.position.y,
                rotated_vertices[0].z - camera.position.z,
            ).normalize();
            
            if normal.dot(&view_dir) >= 0.0 {
                continue;
            }
            
            // Calculate diffuse lighting
            let light_intensity = (-normal.dot(&light_dir)).max(0.2); // Add ambient light of 0.2
            
            // Extract RGB components from base color
            let r = ((face.color >> 16) & 0xFF) as f64;
            let g = ((face.color >> 8) & 0xFF) as f64;
            let b = (face.color & 0xFF) as f64;
            
            // Apply lighting
            let lit_r = (r * light_intensity).min(255.0) as u32;
            let lit_g = (g * light_intensity).min(255.0) as u32;
            let lit_b = (b * light_intensity).min(255.0) as u32;
            
            let final_color = (lit_r << 16) | (lit_g << 8) | lit_b;
            
            // Project 3D vertices to 2D screen coordinates
            let mut screen_coords = [(0, 0); 4];
            for (i, vertex) in rotated_vertices.iter().enumerate() {
                // Translate vertex relative to camera
                let translated = Vec3::new(
                    vertex.x - camera.position.x,
                    vertex.y - camera.position.y,
                    vertex.z - camera.position.z,
                );
                
                // Skip vertices behind camera
                if translated.z <= 0.0 {
                    continue;
                }
                
                // Perspective projection
                let fov_factor = 1.0 / (camera.fov / 2.0).tan();
                let screen_x = ((translated.x / translated.z) * fov_factor + 1.0) * WIDTH as f64 / 2.0;
                let screen_y = ((translated.y / translated.z) * fov_factor + 1.0) * HEIGHT as f64 / 2.0;
                
                screen_coords[i] = (screen_x as i32, screen_y as i32);
            }
            
            // Draw the face as a filled polygon
            draw_filled_polygon(buffer, &screen_coords, final_color);
        }
    }
}

// Function to draw a filled polygon
fn draw_filled_polygon(buffer: &mut Vec<u32>, vertices: &[(i32, i32); 4], color: u32) {
    // Find bounding box
    let mut min_x = WIDTH as i32;
    let mut min_y = HEIGHT as i32;
    let mut max_x = 0;
    let mut max_y = 0;
    
    for &(x, y) in vertices {
        min_x = min_x.min(x);
        min_y = min_y.min(y);
        max_x = max_x.max(x);
        max_y = max_y.max(y);
    }
    
    // Clip to screen bounds
    min_x = min_x.max(0);
    min_y = min_y.max(0);
    max_x = max_x.min(WIDTH as i32 - 1);
    max_y = max_y.min(HEIGHT as i32 - 1);
    
    // Scan each pixel in the bounding box
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if point_in_polygon(x, y, vertices) {
                let pixel_index = y as usize * WIDTH + x as usize;
                if pixel_index < buffer.len() {
                    buffer[pixel_index] = color;
                }
            }
        }
    }
}

// Function to check if a point is inside a polygon using the ray casting algorithm
fn point_in_polygon(x: i32, y: i32, vertices: &[(i32, i32); 4]) -> bool {
    let mut inside = false;
    let n = vertices.len();
    
    for i in 0..n {
        let j = (i + 1) % n;
        let (xi, yi) = vertices[i];
        let (xj, yj) = vertices[j];
        
        let intersect = ((yi > y) != (yj > y)) && 
                        (x < (xj - xi) * (y - yi) / (yj - yi) + xi);
        
        if intersect {
            inside = !inside;
        }
    }
    
    inside
}

fn main() {
    // Create window
    let mut window = Window::new(
        "3D Cube with Diffuse Lighting",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("Failed to create window: {}", e);
    });
    
    // Limit to max ~60 fps
    window.limit_update_rate(Some(Duration::from_micros(16600)));
    
    // Create buffer for pixels
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    
    // Create cube and camera
    let mut cube = Cube::new();
    let mut camera = Camera::new();
    
    // Timing variables
    let mut last_time = Instant::now();
    let mut fps_counter = 0;
    let mut fps_timer = Instant::now();
    
    // Main loop
    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Calculate delta time
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
        
        // Handle keyboard input
        if window.is_key_down(Key::A) {
            camera.rotate_y(-1.0 * delta_time);
        }
        if window.is_key_down(Key::D) {
            camera.rotate_y(1.0 * delta_time);
        }
        if window.is_key_down(Key::W) {
            camera.move_forward(1.0 * delta_time);
        }
        if window.is_key_down(Key::S) {
            camera.move_forward(-1.0 * delta_time);
        }
        
        // Update cube
        cube.update(delta_time);
        
        // Render cube
        cube.render(&mut buffer, &camera);
        
        // Update window
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap_or_else(|e| {
                panic!("Failed to update window: {}", e);
            });
    }
}