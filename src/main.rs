pub mod functions;
pub mod vec2;
pub mod vec3;
use console::Term;
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    execute, queue,
    style::Print,
};
use std::{
    process::exit, 
    time::Instant,
    io::stdout,
    thread,
    sync::mpsc,
};
use functions::*;
use vec2::Vec2;
use vec3::Vec3;

struct Row {
    line: Vec<u8>,
    n: usize,
}

#[derive(Clone)]
struct RowParams {
    width: usize,
    height: usize,
    aspect: f64,
    pixel_aspect: f64,
    objects: Vec<Box<dyn Object>>,
    light: Vec3,
}

const GRADIENT: &[u8] = " .:;!/|({%@$&".as_bytes();
// let gradient = " .'`,-^\"_:;!><i?][}{1)(|\\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$".as_bytes();
const GRADIENT_SIZE: usize = GRADIENT.len() - 1;

fn main() {
    let mut fps = 0;
    queue!(stdout(), Hide).expect("Cannot hide cursor");
    ctrlc::set_handler(move || {
        println!("received Ctrl+C!");
        execute!(stdout(), Show).expect("Cannot show cursor");
        exit(0);
    }).expect("Error setting Ctrl-C handler");

    let (height, width) = Term::buffered_stdout().size();
    let (height, width) = (height as usize, width as usize);
    // let (height, width) = (90 as usize, 320 as usize);
    let aspect = width as f64 / height as f64;
    let pixel_aspect = 11.0 / 24.0;
    let light = Vec3::new((-0.5, 0.5, -1.0)).norm();
    let objects: Vec<Box<dyn Object>> = vec![
        Sphere::new(1.0, Vec3::new((0.0, 3.0, 0.0))),
        Sphere::new(1.0, Vec3::new((3.0, 0.0, 0.0))),
        Sphere::new(1.0, Vec3::new((0.0, -3.0, 0.0))),
        Sphere::new(1.0, Vec3::new((-3.0, 0.0, 0.0))),
        Cube::new(Vec3::new(1.0), Vec3::new((0.0, 0.0, -1.0))),
        Plane::new(Vec3::new((0.0, 0.0, 1.0)), Vec3::new((0.0, 0.0, 2.0))),
    ];
    let ts_start = Instant::now();
    let common_row_params = RowParams { width, height, aspect, pixel_aspect, objects, light };
    let (tx, rx): (mpsc::SyncSender<Row>, mpsc::Receiver<Row>) = mpsc::sync_channel(1);
    thread::spawn(move || {
        loop {
            draw_row(rx.recv().expect("Cannot recv"))
        }
    });
    loop {
        // Main loop
        let ts = Instant::now();
        let t = ts.saturating_duration_since(ts_start).as_secs_f64();
        // common_row_params.t = t;
        let mut rows = Vec::new();
        for j in 0..height {
            let rp = common_row_params.clone();
            let tx = tx.clone();
            rows.push(thread::spawn(move || {
                // rp.j = j;
                tx.send(render_row(rp, j, t)).expect("Cannot send Row");
            }));            
        }
        for row in rows {
            row.join().expect("Child thread paniced");
        }
    // Get FPS
        let ts_new = Instant::now();
        fps = (fps + 1000000 / ts_new.saturating_duration_since(ts).as_micros()) / 2;
        tx.send(Row{line: format!("FPS: {} ", fps).as_bytes().to_vec(), n: height}).expect("Cannot send FPS");
    }
}

fn draw_row(row: Row) {
    queue!(
        stdout(),
        MoveTo(0, row.n as u16),
        Print(String::from_utf8_lossy(&row.line))
    ).expect("Cannot print line");
}

fn render_row(mut params: RowParams, j: usize, t: f64) -> Row {
    let mut row = Row{line: vec![' ' as u8; params.width], n: j};
    for i in 0..params.width {
        let mut uv = Vec2::new((i, j)) / Vec2::new((params.width, params.height)) * 2.0 - 1.0;
        uv.x *= params.aspect * params.pixel_aspect;
        let mut ro = Vec3::new((-10.0, 0.0, 0.0));
        let mut rd = Vec3::new((2.0, uv)).norm();
        ro = rotate_y(ro, 0.25);
        rd = rotate_y(rd, 0.25);
        ro = rotate_z(ro, t);
        rd = rotate_z(rd, t);
        let mut diff = 1.0;
        for _k in 0..5 {
            let mut min_it = 99999.0;
            let mut n = Vec3::new(0.0);
            let mut albedo = 1.0;
            for obj in params.objects.iter_mut() {
                obj.get_reflection(ro, rd, &mut min_it, &mut n, &mut albedo);
            };
            if min_it < 99999.0 {
                diff *= (n.dot(params.light) * 0.5 + 0.5) * albedo;
                ro = ro + rd * (min_it - 0.01);
                rd = reflect(rd, n);
            } else {
                break;
            }
        }
        let mut color = (diff * 20.0) as usize;
        color = color.clamp(0, GRADIENT_SIZE);
        let pixel = GRADIENT[color];
        row.line[i] = pixel;
    };
    row
}
