use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::Sdl;
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

fn main() -> Result<(), String> {
    let width = 800;
    let height = 600;

    // Initialize SDL2
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("SDL2 Slider Example", width, height)
        .position_centered()
        .build()
        .expect("Failed to create window");

    let mut canvas = window
        .into_canvas()
        .build()
        .expect("Failed to create canvas");

    let mut event_pump = sdl_context.event_pump()?;

    // Create a slider
    let mut slider = Slider::new(100, 300, 600, 20, 0.0, 100.0);

    // Main event loop
    'running: loop {
        // Poll for events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => slider.handle_event(&event),
            }
        }

        // Clear the canvas
        canvas.set_draw_color(Color::RGB(30, 30, 30));
        canvas.clear();

        // Render the slider
        slider.render(&mut canvas);

        // Present the canvas
        canvas.present();

        // Delay to control frame rate
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
