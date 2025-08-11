use minifb::{Key, Window, WindowOptions};
use std::time::Instant;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

fn main() {
    let mut window = Window::new(
        "Ilustración del Oso - Demo",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let start_time = Instant::now();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let elapsed = start_time.elapsed().as_secs_f64();
        
        // Limpiar buffer con fondo azul oscuro
        for pixel in buffer.iter_mut() {
            *pixel = 0x001122;
        }

        // Dibujar el oso en el centro
        draw_bear(&mut buffer, WIDTH / 2, HEIGHT / 2, elapsed);
        
        // Dibujar título
        draw_simple_text(&mut buffer, "OSO HECHO CON CIRCULOS", WIDTH / 2 - 120, 50, 0xFFFFFF);
        draw_simple_text(&mut buffer, "Presiona ESC para salir", WIDTH / 2 - 100, HEIGHT - 50, 0xCCCCCC);

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}

fn draw_circle(buffer: &mut Vec<u32>, center_x: i32, center_y: i32, radius: i32, color: u32) {
    for y in (center_y - radius).max(0)..(center_y + radius + 1).min(HEIGHT as i32) {
        for x in (center_x - radius).max(0)..(center_x + radius + 1).min(WIDTH as i32) {
            let dx = x - center_x;
            let dy = y - center_y;
            let distance_sq = dx * dx + dy * dy;
            
            if distance_sq <= radius * radius {
                let pixel_index = (y as usize) * WIDTH + (x as usize);
                if pixel_index < buffer.len() {
                    buffer[pixel_index] = color;
                }
            }
        }
    }
}

fn draw_bear(buffer: &mut Vec<u32>, center_x: usize, center_y: usize, time: f64) {
    let cx = center_x as i32;
    let cy = center_y as i32;
    
    // Colores del oso
    let brown = 0x8B4513;      // Marrón
    let dark_brown = 0x654321; // Marrón oscuro
    let black = 0x000000;      // Negro
    let pink = 0xFFB6C1;       // Rosa claro
    let white = 0xFFFFFF;      // Blanco
    
    // Animación sutil - el oso "respira"
    let breath = (time * 1.5).sin() * 2.0;
    
    // Cuerpo principal (círculo grande)
    draw_circle(buffer, cx, cy + 10, (30.0 + breath) as i32, brown);
    
    // Cabeza (círculo mediano)
    draw_circle(buffer, cx, cy - 25, (22.0 + breath * 0.5) as i32, brown);
    
    // Orejas externas (dos círculos pequeños)
    draw_circle(buffer, cx - 15, cy - 40, 10, dark_brown);
    draw_circle(buffer, cx + 15, cy - 40, 10, dark_brown);
    
    // Orejas internas (círculos más pequeños y rosados)
    draw_circle(buffer, cx - 15, cy - 40, 6, pink);
    draw_circle(buffer, cx + 15, cy - 40, 6, pink);
    
    // Ojos (dos círculos blancos con pupilas negras)
    draw_circle(buffer, cx - 8, cy - 30, 4, white);
    draw_circle(buffer, cx + 8, cy - 30, 4, white);
    draw_circle(buffer, cx - 8, cy - 30, 2, black);
    draw_circle(buffer, cx + 8, cy - 30, 2, black);
    
    // Hocico (círculo beige)
    draw_circle(buffer, cx, cy - 18, 6, 0xF5DEB3);
    
    // Nariz (círculo negro pequeño)
    draw_circle(buffer, cx, cy - 20, 3, black);
    
    // Boca (dos pequeños círculos para simular sonrisa)
    draw_circle(buffer, cx - 3, cy - 14, 1, black);
    draw_circle(buffer, cx + 3, cy - 14, 1, black);
    
    // Patas delanteras (dos círculos medianos)
    draw_circle(buffer, cx - 20, cy + 25, 8, brown);
    draw_circle(buffer, cx + 20, cy + 25, 8, brown);
    
    // Patas traseras (dos círculos pequeños)
    draw_circle(buffer, cx - 12, cy + 40, 6, dark_brown);
    draw_circle(buffer, cx + 12, cy + 40, 6, dark_brown);
    
    // Barriga (círculo más claro)
    draw_circle(buffer, cx, cy + 5, 15, 0xD2B48C);
}

fn draw_simple_text(buffer: &mut Vec<u32>, text: &str, x: usize, y: usize, color: u32) {
    // Implementación muy simple de texto usando píxeles
    let char_width = 8;
    let char_height = 12;
    
    for (i, ch) in text.chars().enumerate() {
        let char_x = x + i * char_width;
        if char_x >= WIDTH { break; }
        
        // Dibujar un rectángulo simple para cada carácter
        for dy in 0..char_height {
            for dx in 0..6 {
                let px = char_x + dx;
                let py = y + dy;
                if px < WIDTH && py < HEIGHT {
                    // Patrón simple basado en el carácter
                    let pattern = match ch {
                        'A'..='Z' | 'a'..='z' => (dx + dy) % 3 == 0,
                        '0'..='9' => (dx * dy) % 4 == 0,
                        ' ' => false,
                        _ => dx % 2 == 0 && dy % 2 == 0,
                    };
                    
                    if pattern {
                        buffer[py * WIDTH + px] = color;
                    }
                }
            }
        }
    }
}
