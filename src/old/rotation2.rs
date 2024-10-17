use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use std::time::Duration;
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

struct Point3D {
    x: f32,
    y: f32,
    z: f32,
}

fn rotate_y(point: &Point3D, angle: f32) -> Point3D {
    let cos_a = angle.cos();
    let sin_a = angle.sin();
    Point3D {
        x: point.x * cos_a + point.z * sin_a,
        y: point.y,
        z: -point.x * sin_a + point.z * cos_a,
    }
}

fn project_to_2d(point: &Point3D, width: u32, height: u32) -> (i32, i32) {
    let scale = 200.0; // Adjust this value to change the projection scale
    let x = (point.x * scale + width as f32 / 2.0) as i32;
    let y = (-point.y * scale + height as f32 / 2.0) as i32;
    (x, y)
}


impl Slider {
    fn new(x: i32, y: i32, width: i32, height: i32, min_value: f32, max_value: f32) -> Self {
        Slider {
            x,
            y,
            width,
            height,
            value: min_value,
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

fn render_grid(canvas: &mut WindowCanvas, color: Color, size: i32, width: u32, height: u32) {
    canvas.set_draw_color(color);
    for i in 0..((height as i32) / size + 1) {
        canvas.fill_rect(Rect::new(0, size * i, width, 1)).expect("Could not draw rect");
    }
    for i in 0..((width as i32) / size + 1) {
        canvas.fill_rect(Rect::new(size * i, 0, 1, height)).expect("Could not draw rect");
    }
}

fn bresenham_line(canvas: &mut WindowCanvas, x0: i32, y0: i32, x1: i32, y1: i32, grid_size: i32) {
    let mut x0 = x0 / grid_size;
    let mut y0 = y0 / grid_size;
    let x1 = x1 / grid_size;
    let y1 = y1 / grid_size;
    let dx = (x1 - x0).abs();
    let dy = (y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx - dy;

    loop {
        draw_filled_rect(canvas, x0 * grid_size, y0 * grid_size, grid_size);
        
        if x0 == x1 && y0 == y1 {
            break;
        }

        let err2 = err * 2;
        if err2 > -dy {
            err -= dy;
            x0 += sx;
        }
        if err2 < dx {
            err += dx;
            y0 += sy;
        }
    }
}

fn draw_triangle(canvas: &mut WindowCanvas, grid_size: i32, angle: f32, width: u32, height: u32) {
    let vertices = [
        Point3D { x: 0.0, y: 1.0, z: 0.0 },
        Point3D { x: -1.0, y: -1.0, z: 0.5 },
        Point3D { x: 1.0, y: -1.0, z: -0.5 },
    ];

    let rotated_vertices: Vec<Point3D> = vertices.iter().map(|v| rotate_y(v, angle)).collect();
    let projected_vertices: Vec<(i32, i32)> = rotated_vertices.iter()
        .map(|v| project_to_2d(v, width, height))
        .collect();

    for i in 0..3 {
        let (x0, y0) = projected_vertices[i];
        let (x1, y1) = projected_vertices[(i + 1) % 3];
        bresenham_line(canvas, x0, y0, x1, y1, grid_size);
    }
}

fn draw_filled_rect(canvas: &mut WindowCanvas, x: i32, y: i32, size: i32) {
    canvas.set_draw_color(Color::RGB(255, 0, 0));
    canvas.fill_rect(Rect::new(x, y, size as u32, size as u32)).expect("Could not draw rect");
}

fn main() -> Result<(), String> {
    let width = 800;
    let height = 600;
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Rust Rasterizer with Slider and Rotation", width, height)
        .position_centered()
        .build()
        .expect("Could not initialize video subsystem");

    let mut canvas = window.into_canvas().present_vsync()
        .build()
        .expect("Could not make a canvas");

    let mut event_pump = sdl_context.event_pump()?;
    let mut grid_slider = Slider::new(50, 50, 200, 10, 1.0, 80.0);
    let mut rotation_slider = Slider::new(50, 100, 200, 10, 0.0, 2.0 * PI);
    let mut grid_size: i32 = grid_slider.value as i32;
    let mut rotation_angle: f32 = rotation_slider.value;

    'running: loop {
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();

        render_grid(&mut canvas, Color::RGB(230, 230, 230), grid_size, width, height);
        draw_triangle(&mut canvas, grid_size, rotation_angle, width, height);
        grid_slider.render(&mut canvas);
        rotation_slider.render(&mut canvas);

        canvas.present();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                }
                _ => {
                    grid_slider.handle_event(&event);
                    rotation_slider.handle_event(&event);
                }
            }
        }

        grid_size = grid_slider.value as i32;
        rotation_angle = rotation_slider.value;
        ::std::thread::sleep(Duration::from_millis(16));
    }

    Ok(())
}