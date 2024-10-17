use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::WindowCanvas;
use std::time::{Duration, Instant};
use std::f32::consts::PI;


struct Slider {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    value: f32,
    min_value: f32,
    max_value: f32,
    dragging: bool,
}

impl Slider {
    fn new(x: i32, y: i32, width: i32, height: i32,value: f32, min_value: f32, max_value: f32) -> Self {
        Slider {
            x,
            y,
            width,
            height,
            value,
            min_value,
            max_value,
            dragging: false,
        }
    }


    fn render(&self, canvas: &mut WindowCanvas) {
        canvas.set_draw_color(Color::RGB(200, 200, 200));
        canvas.fill_rect(Rect::new(self.x, self.y, self.width as u32, self.height as u32)).unwrap();

        let knob_x = (self.value - self.min_value) / (self.max_value - self.min_value) * (self.width as f32) + self.x as f32 - (self.height as f32 / 2.0);
        canvas.set_draw_color(Color::RGB(100, 100, 255));
        canvas.fill_rect(Rect::new(knob_x as i32, self.y - (self.height / 2), self.height as u32, self.height as u32)).unwrap();
    }

    fn handle_event(&mut self, event: &Event) {
        match event {
            Event::MouseButtonDown { x, y, .. } if *x >= self.x && *x <= (self.x + self.width) && *y >= self.y - (self.height / 2) && *y <= (self.y + self.height / 2) => {
                self.dragging = true;
                self.update_value(*x);
            }
            Event::MouseButtonUp { .. } => {
                self.dragging = false;
            }
            Event::MouseMotion { x, .. } if self.dragging => {
                self.update_value(*x);
            }
            _ => {}
        }
    }

    fn update_value(&mut self, mouse_x: i32) {
        let relative_x = mouse_x - self.x;
        self.value = (relative_x as f32 / self.width as f32) * (self.max_value - self.min_value) + self.min_value;
        self.value = self.value.clamp(self.min_value, self.max_value);
    }
}

fn draw_grid(canvas: &mut WindowCanvas, width: u32, height: u32, resolution: i32) {
    canvas.set_draw_color(Color::RGB(50, 50, 50)); // Dark gray color for the grid

    // Draw vertical lines
    for x in (0..width as i32).step_by(resolution as usize) {
        canvas.draw_line((x, 0), (x, height as i32)).unwrap();
    }

    // Draw horizontal lines
    for y in (0..height as i32).step_by(resolution as usize) {
        canvas.draw_line((0, y), (width as i32, y)).unwrap();
    }
}

struct Vertex {
    x: f32,
    y: f32,
    z: f32,
}

struct Point3D {
    vertex: Vertex,
    color: Color,
}


fn interpolate_color(c1: Color, c2: Color , t: f32) -> Color {
    let r = (c1.r as f32 * (1.0 - t) + c2.r as f32 * t) as u8;
    let g = (c1.g as f32 * (1.0 - t) + c2.g as f32 * t) as u8;
    let b = (c1.b as f32 * (1.0 - t) + c2.b as f32 * t) as u8;
    Color::RGB(r, g, b)

}

fn draw_interpolated_line(canvas: &mut WindowCanvas, x1: i32, y1: i32, x2: i32, y2: i32, c1: Color, c2: Color, z1: f32, z2: f32, resolution: i32) {
    let dx = (x2 - x1).abs();
    let dy = (y2 - y1).abs();
    let sx = if x1 < x2 { resolution } else { -resolution };
    let sy = if y1 < y2 { resolution } else { -resolution };
    let mut err = dx - dy;

    let mut x = x1;
    let mut y = y1;

    let total_distance = ((x2 - x1).pow(2) + (y2 - y1).pow(2)) as f32;

    loop {
        let t = ((x - x1).pow(2) + (y - y1).pow(2)) as f32 / total_distance;
        let color = interpolate_color(c1, c2, t);
        // let z = z1 * (1.0 - t) + z2 * t;
        // let shadow_factor = (1.0 + z).max(0.0).min(1.0);
        // let shadowed_color = darken_color(color, shadow_factor);
        canvas.set_draw_color(color);
        canvas.fill_rect(Rect::new(x - x % resolution, y - y % resolution, resolution as u32, resolution as u32)).unwrap();

        // Break when reaching the target (or when you're close enough, adjust to ensure rounding)
        if (x - x2).abs() < resolution && (y - y2).abs() < resolution {
            break;
        }

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

fn main() -> Result<(), String> {
    let width = 800;
    let height = 600;
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Rust Rasterizer", width, height)
        .position_centered()
        .build()
        .expect("Could not initialize video subsystem");

    let mut canvas = window.into_canvas()
        .build()
        .expect("Could not make a canvas");

    let mut event_pump = sdl_context.event_pump()?;
    let mut resolution_slider = Slider::new(50, 100, 200, 10, 25.0, 1.0, 50.0);
    let mut resolution: i32 = resolution_slider.value as i32;

    // Initialize FPS tracking
    let mut last_time = Instant::now();
    let mut frame_count = 0;

    let vertices = [ { Point3D {
        vertex: Vertex { x: 600.0, y: 50.0, z: 0.0 },
        color: Color::RGB(255, 0, 0),
    } }, { Point3D {
        vertex: Vertex { x: 50.0, y: 400.0, z: 0.5 },
        color: Color::RGB(0, 255, 0),
    } }, { Point3D {
        vertex: Vertex { x: 400.0, y: 500.0, z: -0.5 },
        color: Color::RGB(0, 0, 255),
    }} ];

    'running: loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        // Draw grid and lines
        draw_grid(&mut canvas, width, height, resolution);

        for i in 0..3 {
            let point1 = &vertices[i];
            let point2 = &vertices[(i + 1) % 3];
            draw_interpolated_line(&mut canvas, 
                 point1.vertex.x as i32,
                 point1.vertex.y as i32,
                 point2.vertex.x as i32,
                 point2.vertex.y as i32, 
                 point1.color, point2.color, point1.vertex.z, point2.vertex.z, resolution);
        }

        resolution_slider.render(&mut canvas);

        // Measure frame time and calculate FPS
        frame_count += 1;
        let now = Instant::now();
        let duration = now.duration_since(last_time);
        if duration.as_secs_f32() >= 1.0 {
            let fps = frame_count as f32 / duration.as_secs_f32();
            println!("FPS: {:.2}", fps);  // You can display this value on the screen using text rendering instead of printing
            frame_count = 0;
            last_time = now;
        }

        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                }
                _ => {
                    resolution_slider.handle_event(&event);
                }
            }
        }

        resolution = resolution_slider.value as i32;
        canvas.present();

        // Limit to ~60 FPS
        // ::std::thread::sleep(Duration::from_millis(16));
    }

    Ok(())
}
