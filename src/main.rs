use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use std::time::{Duration, Instant};
use std::f32::consts::PI;
use std::collections::HashMap;

// Custom UI : A slider for giving dynamic changes
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

// Custom UI : A structure to store interpolated points from Bresenham's line algorithm
// Hashmap : { y: [(x, color)] }
struct InterpolatedPoints {
    points: HashMap<i32, Vec<(i32, Color)>>,
}
impl InterpolatedPoints {
    fn new() -> Self {
        Self { points: HashMap::new() }
    }

    fn add_point(&mut self, x: i32, y: i32, color: Color) {
        self.points.entry(y).or_insert_with(Vec::new).push((x, color));
    }

    fn get_min_max_x(&self, y: i32) -> Option<((i32, Color), (i32, Color))> {
        self.points.get(&y).map(|xs| {
            let min_point = xs.iter().min_by_key(|(x, _)| x).unwrap();
            let max_point = xs.iter().max_by_key(|(x, _)| x).unwrap();
            (*min_point, *max_point)
        })
    }
}


// A background grid to help visualize the 2d space
fn draw_grid(canvas: &mut WindowCanvas, width: u32, height: u32, resolution: i32) {
    canvas.set_draw_color(Color::RGB(50, 50, 50));
    for x in (0..width as i32).step_by(resolution as usize) {
        canvas.draw_line((x, 0), (x, height as i32)).unwrap();
    }
    for y in (0..height as i32).step_by(resolution as usize) {
        canvas.draw_line((0, y), (width as i32, y)).unwrap();
    }
}

// Position struct
struct Vertex {
    x: i32,
    y: i32,
    z: i32,
}
// Position and color struct
struct Point3D {
    vertex: Vertex,
    color: Color,
}

// 2d rotation about y-axis
fn rotate_y(point: &Vertex, angle: f32) -> Vertex {
    let cos_a = angle.cos();
    let sin_a = angle.sin();
    Vertex {
        x: (point.x as f32 * cos_a + point.z as f32 * sin_a) as i32,
        y: point.y,
        z: (-point.x as f32 * sin_a + point.z as f32 * cos_a) as i32,
    }
}

// Simple Linear Interpolation for color
fn interpolate_color(c1: Color, c2: Color , t: f32) -> Color {
    let r = (c1.r as f32 * (1.0 - t) + c2.r as f32 * t) as u8;
    let g = (c1.g as f32 * (1.0 - t) + c2.g as f32 * t) as u8;
    let b = (c1.b as f32 * (1.0 - t) + c2.b as f32 * t) as u8;
    Color::RGB(r, g, b)
}

// Drawing a line using Bresenham's line algorithm and interpolating colors based on the two vertices
fn draw_bresenham_line(interpolated_points: &mut InterpolatedPoints, canvas: &mut WindowCanvas, x1: i32, y1: i32, x2: i32, y2: i32, c1: Color, c2: Color, _z1: i32, _z2: i32, resolution: i32 ) {
    
    
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
        interpolated_points.add_point(x - x % resolution, y - y % resolution, color);
        
        // let z = z1 * (1.0 - t) + z2 * t;
        // let shadow_factor = (1.0 + z).max(0.0).min(1.0);
        // let shadowed_color = darken_color(color, shadow_factor);
        canvas.set_draw_color(color);
        canvas.fill_rect(Rect::new(x - x % resolution, y - y % resolution, resolution as u32, resolution as u32)).unwrap();


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

// Drawing a horizontal line using Bresenham's line algorithm and interpolating colors based on the two vertices
// It does not store the points in the hashmap, as it is only for filling the triangle
fn draw_horizontal_line(canvas: &mut WindowCanvas, x1: i32, y1: i32, x2: i32, y2: i32, c1: Color, c2: Color, _z1: i32, _z2: i32, resolution: i32 ) {
    
    
    let dx = (x2 - x1).abs();
    let dy = (y2 - y1).abs();
    let sx = if x1 < x2 { 1 } else { -1 };
    let sy = if y1 < y2 { 1 } else { -1 };
    let mut err = dx - dy;

    let mut x = x1;
    let mut y = y1;

    let total_distance = ((x2 - x1).pow(2) + (y2 - y1).pow(2)) as f32;

    loop {
        
        let t = if total_distance == 0.0 {0.0} else {((x - x1).pow(2) + (y - y1).pow(2)) as f32 / total_distance};
        let color = interpolate_color(c1, c2, t);
        
        
        // let z = z1 * (1.0 - t) + z2 * t;
        // let shadow_factor = (1.0 + z).max(0.0).min(1.0);
        // let shadowed_color = darken_color(color, shadow_factor);
        canvas.set_draw_color(color);
        canvas.fill_rect(Rect::new(x - x % resolution, y - y % resolution, resolution as u32, resolution as u32)).unwrap();

        
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

// Filling the triangle using the interpolated points
fn fill_triangle(
    canvas: &mut WindowCanvas,
    v0: &Point3D,
    v1: &Point3D,
    v2: &Point3D,
    resolution: i32,
    interpolated_points: &mut InterpolatedPoints,
) {
    let mut min_y = v0.vertex.y.min(v1.vertex.y).min(v2.vertex.y);
    let mut max_y = v0.vertex.y.max(v1.vertex.y).max(v2.vertex.y);
    min_y -= min_y % resolution;
    max_y -= max_y % resolution;

    for y in (min_y..=max_y).step_by(resolution as usize) {
        if let Some(((x_min, color_min), (x_max, color_max))) = interpolated_points.get_min_max_x(y) {          
            draw_horizontal_line(canvas, x_min, y, x_max, y , color_min, color_max, 0, 0, resolution);
        } else {
            eprintln!("Warning: No min/max x found for y = {}", y);
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
        .present_vsync()
        .build()
        .expect("Could not make a canvas");

    let mut event_pump = sdl_context.event_pump()?;
    let mut resolution_slider = Slider::new(50, 100, 200, 10, 25.0, 1.0, 50.0);
    let mut rotation_slider = Slider::new(50, 50, 200, 10,0.0,  0.0, 2.0 * PI);
    let mut rotation_angle: f32 = rotation_slider.value;
    let mut resolution: i32 = resolution_slider.value as i32;

    //FPS tracking
    let mut last_time = Instant::now();
    let mut frame_count = 0;

    let original_vertices = vec![
        Point3D {
            vertex: Vertex { x: 0, y: -250, z: 0 },
            color: Color::RGB(255, 0, 0),
        },
        Point3D {
            vertex: Vertex { x: -300, y: 100, z: 0 },
            color: Color::RGB(0, 255, 0),
        },
        Point3D {
            vertex: Vertex { x: 350, y: 200, z: 0 },
            color: Color::RGB(0, 0, 255),
        }
    ];

    let mut interpolated_points = InterpolatedPoints::new();
    let center_x = width as i32 / 2;
    let center_y = height as i32 / 2;

    'running: loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        draw_grid(&mut canvas, width, height, resolution);

        let rotated_vertices: Vec<Point3D> = original_vertices.iter().map(|v| {
            let rotated = rotate_y(&v.vertex, rotation_angle);
            Point3D {
                vertex: Vertex {
                    x: rotated.x + center_x,
                    y: rotated.y + center_y,
                    z: rotated.z,
                },
                color: v.color,
            }
        }).collect();

        for i in 0..3 {
            let point1 = &rotated_vertices[i];
            let point2 = &rotated_vertices[(i + 1) % 3];
            draw_bresenham_line(&mut interpolated_points, &mut canvas, 
                 point1.vertex.x,
                 point1.vertex.y,
                 point2.vertex.x,
                 point2.vertex.y, 
                 point1.color, point2.color, point1.vertex.z, point2.vertex.z, resolution);
        }

        fill_triangle(&mut canvas, &rotated_vertices[0], &rotated_vertices[1], &rotated_vertices[2], resolution, &mut interpolated_points);

        resolution_slider.render(&mut canvas);
        rotation_slider.render(&mut canvas);

        // Measure frame time and calculate FPS
        frame_count += 1;
        let now = Instant::now();
        let duration = now.duration_since(last_time);
        if duration.as_secs_f32() >= 1.0 {
            let fps = frame_count as f32 / duration.as_secs_f32();
            println!("FPS: {:.2}", fps);
            frame_count = 0;
            last_time = now;
        }

        // Event Handling
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                }
                _ => {
                    resolution_slider.handle_event(&event);
                    rotation_slider.handle_event(&event);
                    interpolated_points = InterpolatedPoints::new();
                    
                }
            }
        }

        resolution = resolution_slider.value as i32;
        rotation_angle = rotation_slider.value;
        canvas.present();

        // Limit to ~60 FPS
        ::std::thread::sleep(Duration::from_millis(16));
    }

    Ok(())
}
