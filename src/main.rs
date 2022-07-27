pub mod functions;
pub mod vec2;
pub mod vec3;
use functions::*;
use std::{time::Instant, process::exit};
use vec2::Vec2;
use vec3::Vec3;
use console::Term;
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
    _ = queue!(stdout(), Hide, EnterAlternateScreen);
    ctrlc::set_handler(move || {
        println!("received Ctrl+C!");
        _ = execute!(stdout(), Show, LeaveAlternateScreen);
        exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    let (height, width) = Term::buffered_stdout().size();
    let (height, width) = (height as usize, width as usize);
    let aspect = width as f64/ height as f64;
    let pixel_aspect = 11.0 / 24.0;
    let gradient = " .:;!/|({%@$&".as_bytes();
    // let gradient = " .'`,-^\"_:;!><i?][}{1)(|\\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$".as_bytes();
    let gradient_size = gradient.len() - 1;
    let mut screen = vec![vec![' ' as u8; width]; height];
    let ts_start = Instant::now();
    let sphere1_pos = Vec3::new((0.0, 3.0, 0.0));
    let sphere2_pos = Vec3::new((3.0, 0.0, 0.0));
    let sphere3_pos = Vec3::new((0.0, -3.0, 0.0));
    let sphere4_pos = Vec3::new((-3.0, 0.0, 0.0));
    let cube_pos = Vec3::new((0.0, 0.0, -1.0));
    loop {
        // Main loop
        let ts = Instant::now();
        let t = ts.saturating_duration_since(ts_start).as_millis()/10;
        let light = Vec3::new((-0.5, 0.5, -1.0)).norm();
        for j in 0..height {
            for i in 0..width {
                let mut uv = Vec2::new((i, j)) / Vec2::new((width, height)) * 2.0 - 1.0;
                uv.x *= aspect * pixel_aspect;
                let mut ro = Vec3::new((-10.0, 0.0, 0.0));
                let mut rd = Vec3::new((2.0, uv)).norm();
                ro = rotate_y(ro, 0.25);
                rd = rotate_y(rd, 0.25);
                ro = rotate_z(ro, t as f64 * 0.01);
                rd = rotate_z(rd, t as f64 * 0.01);
                let mut diff = 1.0;
                for _k in 0..5 {
                    let mut min_it = 99999.0;
                    let mut n = Vec3::new(0.0);
                    let mut albedo = 1.0;
                    intersect_sphere(ro, rd, sphere1_pos, 1.0, &mut min_it, &mut n);
                    intersect_sphere(ro, rd, sphere2_pos, 1.0, &mut min_it, &mut n);
                    intersect_sphere(ro, rd, sphere3_pos, 1.0, &mut min_it, &mut n);
                    intersect_sphere(ro, rd, sphere4_pos, 1.0, &mut min_it, &mut n);
                    intersec_cube(ro, rd, cube_pos, &mut min_it, &mut n);
                    intersec_plane(ro, rd, Vec3::new(1.0), &mut min_it, &mut n, &mut albedo);
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
                screen[j][i] = pixel;
                
            }
            _ = execute!(stdout(), MoveTo(0, j as u16), Print(String::from_utf8_lossy(&screen[j])));
            // screen[(j+1) * width] = '\n' as u8;
        }
        // Get FPS
        let ts_new = Instant::now();        
        _ = queue!(stdout(), MoveTo(0, height as u16), Print("FPS: "), Print(1000000 / ts_new.saturating_duration_since(ts).as_micros()), Print(" "));
    }
}

fn intersec_plane(ro: Vec3, rd: Vec3, pos: Vec3, min_it: &mut f64, n: &mut Vec3, albedo: &mut f64) {
    let intersection = Vec2::new(plane(ro - pos, rd, Vec3::new((0.0, 0.0, -1.0)), 1.0));
    if intersection.x > 0.0 && intersection.x < *min_it {
        *min_it = intersection.x;
        *n = Vec3::new((0.0, 0.0, -1.0));
        *albedo = 0.2;
    }
}

fn intersec_cube(ro: Vec3, rd: Vec3, pos: Vec3, min_it: &mut f64, n: &mut Vec3) {
    let (intersection, cube_n) = cube(ro - pos, rd, Vec3::new(1.0));
    if intersection.x > 0.0 && intersection.x < *min_it {
        *min_it = intersection.x;
        *n = cube_n;
    }
}

fn intersect_sphere(ro: Vec3, rd: Vec3, pos: Vec3, r: f64, min_it: &mut f64, n: &mut Vec3) {
    let intersection = sphere(ro - pos, rd, r);
    if intersection.x > 0.0 && intersection.x < *min_it {
        let it_point = ro - pos + rd * intersection.x;
        *min_it = intersection.x;
        *n = it_point.norm();
    }
}
