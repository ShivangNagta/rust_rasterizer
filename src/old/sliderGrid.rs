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
    value: f32,        // Current value
    min_value: f32,    // Minimum value
    max_value: f32,    // Maximum value
    dragging: bool,    // Whether the slider is being dragged
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
        // Draw the slider track
        canvas.set_draw_color(Color::RGB(200, 200, 200));
        canvas.fill_rect(Rect::new(self.x, self.y, self.width as u32, self.height as u32)).unwrap();

        // Draw the slider knob
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
        // Clamp the value to min and max
        self.value = self.value.clamp(self.min_value, self.max_value);
    }
}

fn render(canvas: &mut WindowCanvas, color: Color, size: &i32, width: u32, height: u32) {
    canvas.set_draw_color(color);
    for i in 0..((height as i32) / size + 1) {
        canvas.fill_rect(Rect::new(0, size * i, width, 1))
            .expect("Could not draw rect");
    }
    for i in 0..((width as i32) / size + 1) {
        canvas.fill_rect(Rect::new(size * i, 0, 1, height))
            .expect("Could not draw rect");
    }
}

fn main() -> Result<(), String> {
    let width = 800;
    let height = 600;
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Rust Rasterizer with Slider", width, height)
        .position_centered()
        .build()
        .expect("Could not initialize video subsystem");

    let mut canvas = window.into_canvas().present_vsync() // Enable VSync
        .build()
        .expect("Could not make a canvas");

    let mut event_pump = sdl_context.event_pump()?;
    let mut size = 20; // Initial size for the grid
    let mut slider = Slider::new(600, 20, 180, 20, 1.0, 100.0); // Slider position and range

    'running: loop {
        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                _ => slider.handle_event(&event),
            }
        }

        // Set a clear background color
        canvas.set_draw_color(Color::RGB(30, 30, 30)); // Background color
        canvas.clear(); // Clear the canvas only once per frame

        // Render grid
        render(&mut canvas, Color::RGB(255, 255, 255), &size, width, height);

        // Render the slider
        slider.render(&mut canvas);

        // Update size based on slider value
        size = slider.value.round() as i32;

        // Present the rendered frame
        canvas.present();

        // Time management!
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
