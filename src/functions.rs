use super::vec3::Vec3;

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
    Vec3::new((
        v.x,
        v.z * angle.sin() + v.y * angle.cos(),
        v.z * angle.cos() - v.y * angle.sin(),
    ))
}
pub fn rotate_y(v: Vec3, angle: f64) -> Vec3 {
    Vec3::new((
        v.x * angle.cos() - v.z * angle.sin(),
        v.y,
        v.x * angle.sin() + v.z * angle.cos(),
    ))
}
pub fn rotate_z(v: Vec3, angle: f64) -> Vec3 {
    Vec3::new((
        v.x * angle.cos() - v.y * angle.sin(),
        v.x * angle.sin() + v.y * angle.cos(),
        v.z,
    ))
}

// pub struct Object {

// }


#[derive(Clone, Copy)]
pub struct Sphere {
    radius: f64,
    position: Vec3
}

impl Sphere {
    pub fn new(radius: f64, position: Vec3) -> Box<Sphere> {
        Box::new(Sphere { radius, position })
    }
}

#[derive(Clone, Copy)]
pub struct Cube {
    pub size: Vec3,
    pub position: Vec3,
}

impl Cube {
    pub fn new(size: Vec3, position: Vec3) -> Box<Cube> {
        Box::new(Cube{size, position})
    }
}
#[derive(Clone, Copy)]
pub struct Plane {
    pub normal: Vec3,
    pub position: Vec3
}
impl Plane {
    pub fn new(normal: Vec3, position: Vec3) -> Box<Plane> {
        Box::new(Plane{normal, position})
    }
}

pub(crate) trait Object: CloneObject + Send {
    fn intersect(&mut self, ro: Vec3, rd: Vec3) -> (f64, Vec3);
    fn get_reflection(&mut self, ro: Vec3, rd: Vec3, min_it: &mut f64, normal: &mut Vec3, albedo: &mut f64);
}

pub(crate) trait CloneObject {
    fn clone_obj(&self) -> Box<dyn Object>;
}

impl<T> CloneObject for T
where
    T: 'static + Object + Clone,
{
    fn clone_obj(&self) -> Box<dyn Object> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Object> {
    fn clone(&self) -> Self {
        self.clone_obj()
    }
}

impl Object for Sphere {
    fn intersect(&mut self, ro: Vec3, rd: Vec3) -> (f64, Vec3) {
        // Quadratic equation ax^2 + bx + c = 0
        // Omit some coefficients below to reduce calculation complexity
        // let a = rd.dot(rd); // a = 1 : omit
        let b = ro.dot(rd); //  * 2.0 : will cancel in the end
        let c = ro.dot(ro) - self.radius * self.radius;
        let mut h = b * b - c; // * 4.0 * a : a=1, 4.0 cancels with 2.0*2.0
        if h < 0.0 {
            // No roots, ray misses the sphere
            return (-1.0, Vec3::new(0.0));
        }
        h = h.sqrt();
        // - b ± sqrt(b - 4ac)
        (-h - b, Vec3::new(0.0)) //, h - b : we don't use second root //  roots should be devided by (2.0*a), which cancels coefficients above
    }
    fn get_reflection(&mut self, ro: Vec3, rd: Vec3, min_it: &mut f64, normal: &mut Vec3, _albedo: &mut f64) {
        let (intersection, _) = self.intersect(ro - self.position, rd);
        if intersection > 0.0 && intersection < *min_it {
            let it_point = ro - self.position + rd * intersection;
            *min_it = intersection;
            *normal = it_point.norm();
        }
    }
}
impl Object for Cube {
    fn intersect(&mut self, ro: Vec3, rd: Vec3) -> (f64, Vec3) {
        let m = Vec3::new(1.0) / rd;
        let n = m * ro;
        let k = m.abs() * self.size;
        let t1 = -n - k;
        let t2 = -n + k;
        let tn = t1.x.max(t1.y).max(t1.z);
        let tf = t2.x.min(t2.y).min(t2.z);
        if tn >= tf || tf < 0.0 {
            return (-1.0, Vec3::new(0.0))
        }
        let yzx = Vec3::new((t1.y, t1.z, t1.x));
        let zxy = Vec3::new((t1.z, t1.x, t1.y));
        let normal = -rd.sign() * yzx.step(t1) * zxy.step(t1);
        (tn, normal) // tf : we don't use second point
    }
    fn get_reflection(&mut self, ro: Vec3, rd: Vec3, min_it: &mut f64, normal: &mut Vec3, _albedo: &mut f64) {
        let (intersection, n) = self.intersect(ro - self.position, rd);
        if intersection > 0.0 && intersection < *min_it {
            *min_it = intersection;
            *normal = n;
        }
    }
}

impl Object for Plane {
    fn intersect(&mut self, ro: Vec3, rd: Vec3) -> (f64, Vec3) {
        (-ro.dot(self.normal) / rd.dot(self.normal), -self.normal)
    }
    fn get_reflection(&mut self, ro: Vec3, rd: Vec3, min_it: &mut f64, normal: &mut Vec3, albedo: &mut f64){
        let (intersection, n) = self.intersect(ro - self.position, rd);
        if intersection > 0.0 && intersection < *min_it {
            *min_it = intersection;
            *normal = n;
            *albedo = 0.5;
        }
    }
}
