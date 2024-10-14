use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use std::time::Duration;

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

#[derive(Clone, Copy)]
struct Vertex {
    x: i32,
    y: i32,
    color: Color,
}

fn interpolate_color(c1: Color, c2: Color, t: f32) -> Color {
    Color::RGB(
        (c1.r as f32 * (1.0 - t) + c2.r as f32 * t) as u8,
        (c1.g as f32 * (1.0 - t) + c2.g as f32 * t) as u8,
        (c1.b as f32 * (1.0 - t) + c2.b as f32 * t) as u8,
    )
}

fn draw_triangle(canvas: &mut WindowCanvas, v1: Vertex, v2: Vertex, v3: Vertex, grid_size: i32) {
    let mut vertices = vec![v1, v2, v3];
    vertices.sort_by(|a, b| a.y.cmp(&b.y));
    let [v1, v2, v3] = vertices.as_slice() else { return };

    let x1 = v1.x as f32;
    let x2 = v2.x as f32;
    let x3 = v3.x as f32;
    let y1 = v1.y as f32;
    let y2 = v2.y as f32;
    let y3 = v3.y as f32;

    // Interpolate through the first half of the triangle
    for y in (v1.y..=v2.y).step_by(grid_size as usize) {
        let t1 = if y2 != y1 { (y as f32 - y1) / (y2 - y1) } else { 1.0 };
        let t2 = if y3 != y1 { (y as f32 - y1) / (y3 - y1) } else { 1.0 };

        let start_x = x1 + (x2 - x1) * t1;
        let end_x = x1 + (x3 - x1) * t2;

        let start_color = interpolate_color(v1.color, v2.color, t1);
        let end_color = interpolate_color(v1.color, v3.color, t2);

        draw_horizontal_line(canvas, y, start_x as i32, end_x as i32, start_color, end_color, grid_size);
    }

    // Interpolate through the second half of the triangle
    for y in (v2.y + 1..=v3.y).step_by(grid_size as usize) {
        let t1 = if y3 != y2 { (y as f32 - y2) / (y3 - y2) } else { 1.0 };
        let t2 = if y3 != y1 { (y as f32 - y1) / (y3 - y1) } else { 1.0 };

        let start_x = x2 + (x3 - x2) * t1;
        let end_x = x1 + (x3 - x1) * t2;

        let start_color = interpolate_color(v2.color, v3.color, t1);
        let end_color = interpolate_color(v1.color, v3.color, t2);

        draw_horizontal_line(canvas, y, start_x as i32, end_x as i32, start_color, end_color, grid_size);
    }
}

fn draw_horizontal_line(canvas: &mut WindowCanvas, y: i32, start_x: i32, end_x: i32, start_color: Color, end_color: Color, grid_size: i32) {
    let left_x = start_x.min(end_x);
    let right_x = start_x.max(end_x);

    for x in (left_x..=right_x).step_by(grid_size as usize) {
        let t = if right_x != left_x {
            (x - left_x) as f32 / (right_x - left_x) as f32
        } else {
            0.0
        };
        let color = interpolate_color(start_color, end_color, t);
        canvas.set_draw_color(color);
        canvas.fill_rect(Rect::new(x, y, grid_size as u32, grid_size as u32)).unwrap();
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

fn main() -> Result<(), String> {
    let width = 800;
    let height = 600;
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Rust Rasterizer with Color Interpolation and Grid", width, height)
        .position_centered()
        .build()
        .expect("Could not initialize video subsystem");

    let mut canvas = window.into_canvas().present_vsync()
        .build()
        .expect("Could not make a canvas");

    let mut event_pump = sdl_context.event_pump()?;
    let mut slider = Slider::new(50, 50, 200, 10, 1.0, 60.0);
    let mut grid_size: i32 = slider.value as i32;

    let v1 = Vertex { x: 500, y: 50, color: Color::RGB(255, 0, 0) };   // Red
    let v2 = Vertex { x: 150, y: 450, color: Color::RGB(0, 255, 0) };   // Green
    let v3 = Vertex { x: 600, y: 500, color: Color::RGB(0, 0, 255) };   // Blue

    'running: loop {
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();

        render_grid(&mut canvas, Color::RGB(230, 230, 230), grid_size, width, height);
        draw_triangle(&mut canvas, v1, v2, v3, grid_size);
        slider.render(&mut canvas);

        canvas.present();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                }
                _ => {
                    slider.handle_event(&event);
                }
            }
        }

        grid_size = slider.value as i32;
        ::std::thread::sleep(Duration::from_millis(16));
    }

    Ok(())
}