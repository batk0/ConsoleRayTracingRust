pub mod functions;
pub mod vec2;
pub mod vec3;
use functions::*;
use std::time::Instant;
use vec2::Vec2;
use vec3::Vec3;
extern crate console;
use console::Term;
extern crate crossterm;
use std::io::stdout;
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    queue,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
    style::Print, execute,
};

fn main() {
    // let mut width = 120usize;
    // let mut height = 30usize;
    let mut out = stdout();
    let (height, width) = Term::buffered_stdout().size();
    let (height, width) = (height as usize, width as usize);
    queue!(out, Hide, EnterAlternateScreen);
    let aspect = width as f64/ height as f64;
    let pixel_aspect = 11.0 / 24.0;
    let gradient = " .:;!/|({$&%@".as_bytes();
    let gradient_size = gradient.len() - 1;
    let mut screen = vec![' ' as u8; width * height];
    for t in 0..10000 {
        let ts = Instant::now();
        // Main loop
        let light = Vec3::new((-0.5, 0.5, -1.0)).norm();
        let sphere_pos = Vec3::new((0.0, 3.0, 0.0));
        for j in 0..height {
            for i in 0..width {
                let mut uv = Vec2::new((i, j)) / Vec2::new((width, height)) * 2.0 - 1.0;
                uv.x *= aspect * pixel_aspect;
                let mut ro = Vec3::new((-6.0, 0.0, 0.0));
                let mut rd = Vec3::new((2.0, uv)).norm();
                ro = rotate_y(ro, 0.25);
                rd = rotate_y(rd, 0.25);
                ro = rotate_z(ro, t as f64 * 0.01);
                rd = rotate_z(rd, t as f64 * 0.01);
                let mut diff = 1.0;
                for _k in 0..5 {
                    let mut min_it = 99999.0;
                    let intersection = sphere(ro - sphere_pos, rd, 1.0);
                    let mut n = Vec3::new(0.0);
                    let mut albedo = 1.0;
                    if intersection.x > 0.0 {
                        let it_point = ro - sphere_pos + rd * intersection.x;
                        min_it = intersection.x;
                        n = it_point.norm();
                    }
                    let (intersection, cube_n) = cube(ro, rd, Vec3::new(1.0));
                    if intersection.x > 0.0 && intersection.x < min_it {
                        min_it = intersection.x;
                        n = cube_n;
                    }
                    let intersection = Vec2::new(plane(ro, rd, Vec3::new((0.0, 0.0, -1.0)), 1.0));
                    if intersection.x > 0.0 && intersection.x < min_it {
                        min_it = intersection.x;
                        n = Vec3::new((0.0, 0.0, -1.0));
                        albedo = 0.5;
                    }
                    if min_it < 99999.0 {
                        diff *= (n.dot(light) * 0.5 + 0.5) * albedo;
                        ro = ro + rd * (min_it - 0.01);
                        rd = reflect(rd, n);
                    } else {
                        break
                    }
                }
                let mut color = (diff * 20.0) as usize;
                color = color.clamp(0,gradient_size);
                let pixel = gradient[color];
                if screen[i+j*width] != pixel {
                    screen[i+j*width] = pixel;
                    execute!(out, MoveTo(i  as u16, j as u16), Print(String::from_utf8_lossy(&[pixel])));
                }
            }
            // screen[(j+1) * width] = '\n' as u8;
        }
        // Get FPS
        let ts_new = Instant::now();
        queue!(out, MoveTo(0, height as u16), Print("FPS: "), Print(1000000 / ts_new.saturating_duration_since(ts).as_micros()), Print(" "));
    }
    queue!(out, Show, LeaveAlternateScreen);
}
