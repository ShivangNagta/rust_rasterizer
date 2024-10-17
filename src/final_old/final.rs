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
struct Point3D {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Clone, Copy)]
struct Vertex {
    position: Point3D,
    color: Color,
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
    let scale = 200.0;
    let x = (point.x * scale + width as f32 / 2.0) as i32;
    let y = (-point.y * scale + height as f32 / 2.0) as i32;
    (x, y)
}

fn interpolate_color(c1: Color, c2: Color, t: f32) -> Color {
    Color::RGB(
        (c1.r as f32 * (1.0 - t) + c2.r as f32 * t) as u8,
        (c1.g as f32 * (1.0 - t) + c2.g as f32 * t) as u8,
        (c1.b as f32 * (1.0 - t) + c2.b as f32 * t) as u8,
    )
}

fn darken_color(color: Color, factor: f32) -> Color {
    Color::RGB(
        (color.r as f32 * factor) as u8,
        (color.g as f32 * factor) as u8,
        (color.b as f32 * factor) as u8,
    )
}

fn draw_interpolated_line(canvas: &mut WindowCanvas, x1: i32, y1: i32, x2: i32, y2: i32, c1: Color, c2: Color, z1: f32, z2: f32, resolution: i32) {
    let dx = (x2 - x1).abs();
    let dy = (y2 - y1).abs();
    let sx = if x1 < x2 { 1 } else { -1 };
    let sy = if y1 < y2 { 1 } else { -1 };
    let mut err = dx - dy;

    let mut x = x1;
    let mut y = y1;

    let total_distance = ((x2 - x1).pow(2) + (y2 - y1).pow(2)) as f32;

    loop {
        let t = ((x - x1).pow(2) + (y - y1).pow(2)) as f32 / total_distance;
        let color = interpolate_color(c1, c2, t);
        let z = z1 * (1.0 - t) + z2 * t;
        let shadow_factor = (1.0 + z).max(0.0).min(1.0);
        let shadowed_color = darken_color(color, shadow_factor);
        canvas.set_draw_color(shadowed_color);
        canvas.fill_rect(Rect::new(x - x % resolution, y - y % resolution, resolution as u32, resolution as u32)).unwrap();

        if x == x2 && y == y2 {
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

fn fill_triangle(canvas: &mut WindowCanvas, v1: &Vertex, v2: &Vertex, v3: &Vertex, width: u32, height: u32, resolution: i32) {
    let (x1, y1) = project_to_2d(&v1.position, width, height);
    let (x2, y2) = project_to_2d(&v2.position, width, height);
    let (x3, y3) = project_to_2d(&v3.position, width, height);

    // Sort vertices by y-coordinate
    let mut vertices = vec![(x1, y1, v1), (x2, y2, v2), (x3, y3, v3)];
    vertices.sort_by_key(|&(_, y, _)| y);

    let [(x1, y1, v1), (x2, y2, v2), (x3, y3, v3)] = [vertices[0], vertices[1], vertices[2]];

    // Compute slopes
    let slope_1_3 = if y3 != y1 { (x3 - x1) as f32 / (y3 - y1) as f32 } else { 0.0 };
    let slope_1_2 = if y2 != y1 { (x2 - x1) as f32 / (y2 - y1) as f32 } else { 0.0 };
    let slope_2_3 = if y3 != y2 { (x3 - x2) as f32 / (y3 - y2) as f32 } else { 0.0 };

    // Rasterize the upper part of the triangle
    let mut x_start = x1 as f32;
    let mut x_end = x1 as f32;

    for y in (y1..=y2).step_by(resolution as usize) {
        let t1 = if y3 != y1 { (y - y1) as f32 / (y3 - y1) as f32 } else { 1.0 };
        let t2 = if y2 != y1 { (y - y1) as f32 / (y2 - y1) as f32 } else { 1.0 };

        let color_start = interpolate_color(v1.color, v3.color, t1);
        let color_end = interpolate_color(v1.color, v2.color, t2);

        let z_start = v1.position.z * (1.0 - t1) + v3.position.z * t1;
        let z_end = v1.position.z * (1.0 - t2) + v2.position.z * t2;

        draw_interpolated_line(canvas, x_start as i32, y, x_end as i32, y, color_start, color_end, z_start, z_end, resolution);

        x_start += slope_1_3 * resolution as f32;
        x_end += slope_1_2 * resolution as f32;
    }

    // Rasterize the lower part of the triangle
    x_end = x2 as f32;

    for y in (y2..=y3).step_by(resolution as usize) {
        let t1 = if y3 != y1 { (y - y1) as f32 / (y3 - y1) as f32 } else { 1.0 };
        let t2 = if y3 != y2 { (y - y2) as f32 / (y3 - y2) as f32 } else { 1.0 };

        let color_start = interpolate_color(v1.color, v3.color, t1);
        let color_end = interpolate_color(v2.color, v3.color, t2);

        let z_start = v1.position.z * (1.0 - t1) + v3.position.z * t1;
        let z_end = v2.position.z * (1.0 - t2) + v3.position.z * t2;

        draw_interpolated_line(canvas, x_start as i32, y, x_end as i32, y, color_start, color_end, z_start, z_end, resolution);

        x_start += slope_1_3 * resolution as f32;
        x_end += slope_2_3 * resolution as f32;
    }
}

fn draw_shadow(canvas: &mut WindowCanvas, vertices: &[Vertex], width: u32, height: u32, resolution: i32, rotation_angle: f32) {
    let shadow_vertices: Vec<Vertex> = vertices.iter().map(|v| {
        let rotated = rotate_y(&v.position, rotation_angle);
        Vertex {
            position: Point3D { x: rotated.x, y: -1.0, z: rotated.z },
            color: Color::RGB(0, 0, 0),  // Shadow color (black)
        }
    }).collect();

    fill_triangle(canvas, &shadow_vertices[0], &shadow_vertices[1], &shadow_vertices[2], width, height, resolution);
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
        .window("Rust Rasterizer with Grid Lines", width, height)
        .position_centered()
        .build()
        .expect("Could not initialize video subsystem");

    let mut canvas = window.into_canvas().present_vsync()
        .build()
        .expect("Could not make a canvas");

    let mut event_pump = sdl_context.event_pump()?;
    let mut rotation_slider = Slider::new(50, 50, 200, 10, 0.0, 2.0 * PI);
    let mut resolution_slider = Slider::new(50, 100, 200, 10, 1.0, 50.0);
    let mut rotation_angle: f32 = rotation_slider.value;
    let mut resolution: i32 = resolution_slider.value as i32;

    let vertices = [
        Vertex { position: Point3D { x: 0.0, y: 1.3, z: 0.0 }, color: Color::RGB(255, 0, 0) },
        Vertex { position: Point3D { x: -1.2, y: -1.0, z: 0.5 }, color: Color::RGB(0, 255, 0) },
        Vertex { position: Point3D { x: 1.2, y: -1.3, z: -0.5 }, color: Color::RGB(0, 0, 255) },
    ];

    'running: loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        // Draw grid
        draw_grid(&mut canvas, width, height, resolution);

        // Draw shadow
        draw_shadow(&mut canvas, &vertices, width, height, resolution, rotation_angle);

        // Draw triangle
        let rotated_vertices: Vec<Vertex> = vertices.iter().map(|v| Vertex {
            position: rotate_y(&v.position, rotation_angle),
            color: v.color,
        }).collect();

        fill_triangle(&mut canvas, &rotated_vertices[0], &rotated_vertices[1], &rotated_vertices[2], width, height, resolution);

        rotation_slider.render(&mut canvas);
        resolution_slider.render(&mut canvas);

        canvas.present();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                }
                _ => {
                    rotation_slider.handle_event(&event);
                    resolution_slider.handle_event(&event);
                }
            }
        }

        rotation_angle = rotation_slider.value;
        resolution = resolution_slider.value as i32;
        ::std::thread::sleep(Duration::from_millis(16));
    }

    Ok(())
}
