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

    let mut canvas = window.into_canvas().present_vsync()
        .build()
        .expect("Could not make a canvas");

    let mut event_pump = sdl_context.event_pump()?;
    // let mut rotation_slider = Slider::new(50, 50, 200, 10, 0.0, 2.0 * PI);
    let mut resolution_slider = Slider::new(50, 100, 200, 10, 25.0, 1.0, 50.0);
    // let mut rotation_angle: f32 = rotation_slider.value;
    let mut resolution: i32 = resolution_slider.value as i32;



    'running: loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        // Draw grid
        draw_grid(&mut canvas, width, height, resolution);

        
        // rotation_slider.render(&mut canvas);
        resolution_slider.render(&mut canvas);

        canvas.present();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                }
                _ => {
                    // rotation_slider.handle_event(&event);
                    resolution_slider.handle_event(&event);
                }
            }
        }

        // rotation_angle = rotation_slider.value;
        resolution = resolution_slider.value as i32;
        ::std::thread::sleep(Duration::from_millis(16));
    }

    Ok(())
}
