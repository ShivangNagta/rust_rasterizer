use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use std::time::Duration;

fn render(canvas: &mut WindowCanvas, color: Color, size: &i32, width: u32, height: u32) {

    
    canvas.set_draw_color(color);
    for i in 0..(((height as i32) / size)+1){
        canvas.fill_rect(Rect::new(0, size*i, width, 1))
            .expect("Could not draw rect");
    }
    for i in 0..(((width as i32)/ size)+1){
        canvas.fill_rect(Rect::new(size*i, 0, 1, height))
            .expect("Could not draw rect");
    }
    canvas.present();
}

fn main() -> Result<(), String> {
    let width = 800;
    let height = 600;
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window("Rust Rasterizer", width, height)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window.into_canvas().build()
        .expect("could not make a canvas");

    let mut event_pump = sdl_context.event_pump()?;
    let mut size = 20;

    'running: loop {
        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                Event::KeyDown { keycode: Some(Keycode::W), .. } => {
                    size += 1;
                },
                Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                    size -= 1;
                },
                _ => {}
            }
        }
        canvas.set_draw_color(Color::RGB(100, 10, 10));
        canvas.clear();


        // Render
        render(&mut canvas, Color::RGB(255, 255, 255), &size, width, height);



        // Time management!
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}