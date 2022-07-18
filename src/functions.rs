use super::{vec2::Vec2, vec3::Vec3};

pub fn clamp(value: f64, fmin: f64, fmax: f64) -> f64 {
    value.min(fmax).max(fmin)
}

pub fn sign(value: f64) -> f64 {
    ((value > 0.0) as i8 - (value < 0.0) as i8) as f64
}

pub fn step(edge: f64, x: f64) -> f64 {
    ((x > edge) as i8) as f64
}

pub fn reflect(rd: Vec3, n: Vec3) -> Vec3 {
    rd - n * 2.0 * n.dot(rd)
}
pub fn rotate_x(v: Vec3, angle: f64) -> Vec3 {
    // let mut r = v.clone();
    // r.z = v.z * angle.cos() - v.y * angle.sin();
    // r.y = v.z * angle.sin() + v.y * angle.cos();
    // r
    Vec3::new((
        v.x,
        v.z * angle.sin() + v.y * angle.cos(),
        v.z * angle.cos() - v.y * angle.sin(),
    ))
}
pub fn rotate_y(v: Vec3, angle: f64) -> Vec3 {
    // let mut r = v.clone();
    // r.x = v.x * angle.cos() - v.z * angle.sin();
    // r.z = v.x * angle.sin() + v.z * angle.cos();
    // r
    Vec3::new((
        v.x * angle.cos() - v.z * angle.sin(),
        v.y,
        v.x * angle.sin() + v.z * angle.cos(),
    ))
}
pub fn rotate_z(v: Vec3, angle: f64) -> Vec3 {
    // let mut r = v.clone();
    // r.x = v.x * angle.cos() - v.y * angle.sin();
    // r.y = v.x * angle.sin() + v.y * angle.cos();
    // r
    Vec3::new((
        v.x * angle.cos() - v.y * angle.sin(),
        v.x * angle.sin() + v.y * angle.cos(),
        v.z,
    ))
}
pub fn sphere(ro: Vec3, rd: Vec3, r: f64) -> Vec2 {
    let b = ro.dot(rd);
    let c = ro.dot(ro) - r * r;
    let mut h = b * b - c;
    if h < 0.0 {
        return Vec2::new(-1.0);
    }
    h = h.sqrt();
    Vec2::new((-h - b, h - b))
}
pub fn cube(ro: Vec3, rd: Vec3, size: Vec3) -> (Vec2, Vec3) {
    let m = Vec3::new(1.0) / rd;
    let n = m * ro;
    let k = m.abs() * size;
    let t1 = -n - k;
    let t2 = -n + k;
    let tn = t1.x.max(t1.y).max(t1.z);
    let tf = t2.x.min(t2.y).min(t2.z);
    if tn > tf || tf < 0.0 {
        return (Vec2::new(-1.0), Vec3::new(0.0));
    }
    let yzx = Vec3::new((t1.y, t1.z, t1.x));
    let zxy = Vec3::new((t1.z, t1.x, t1.y));
    (
        Vec2::new((tn, tf)),
        -rd.sign() * yzx.step(t1) * zxy.step(t1),
    )
}
pub fn plane(ro: Vec3, rd: Vec3, p: Vec3, w: f64) -> f64 {
    -(ro.dot(p) + w) / rd.dot(p)
}
