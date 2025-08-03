use crate::map::Map;

pub struct Player {
    pub x: f64,
    pub y: f64,
    pub angle: f64,
    pub speed: f64,
    pub rotation_speed: f64,
}

impl Player {
    pub fn new(x: f64, y: f64, angle: f64) -> Self {
        Self {
            x,
            y,
            angle,
            speed: 3.0,
            rotation_speed: 2.0,
        }
    }

    pub fn update(&mut self, delta_time: f64, move_x: f64, move_y: f64, map: &Map) -> bool {
        let move_speed = self.speed * delta_time;
        
        // Calculate new position
        let new_x = self.x + move_x * move_speed;
        let new_y = self.y + move_y * move_speed;
        
        // Collision detection with walls
        let collision_margin = 0.1;
        let mut hit_wall = false;
        
        // Check X movement
        if !map.is_wall((new_x + collision_margin * move_x.signum()) as usize, self.y as usize) &&
           !map.is_wall((new_x - collision_margin * move_x.signum()) as usize, self.y as usize) {
            self.x = new_x;
        } else if move_x.abs() > 0.01 {
            hit_wall = true;
        }
        
        // Check Y movement
        if !map.is_wall(self.x as usize, (new_y + collision_margin * move_y.signum()) as usize) &&
           !map.is_wall(self.x as usize, (new_y - collision_margin * move_y.signum()) as usize) {
            self.y = new_y;
        } else if move_y.abs() > 0.01 {
            hit_wall = true;
        }
        
        // Keep angle in valid range
        while self.angle < 0.0 {
            self.angle += 2.0 * std::f64::consts::PI;
        }
        while self.angle >= 2.0 * std::f64::consts::PI {
            self.angle -= 2.0 * std::f64::consts::PI;
        }
        
        hit_wall
    }
    
    pub fn rotate(&mut self, delta_angle: f64) {
        self.angle += delta_angle;
    }
}