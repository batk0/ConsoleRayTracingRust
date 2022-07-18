use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Copy, Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}
impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}
impl Mul for Vec3 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}
impl Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}
impl Div for Vec3 {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}
impl Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, rhs: f64) -> Self {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}
impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

pub trait IntoVec3 {
    fn into(self) -> Vec3;
}

impl IntoVec3 for f64 {
    fn into(self) -> Vec3 {
        Vec3 {
            x: self,
            y: self,
            z: self,
        }
    }
}

impl IntoVec3 for (f64, super::vec2::Vec2) {
    fn into(self) -> Vec3 {
        Vec3 {
            x: self.0,
            y: self.1.x,
            z: self.1.y,
        }
    }
}

impl IntoVec3 for (f64, f64, f64) {
    fn into(self) -> Vec3 {
        Vec3 {
            x: self.0,
            y: self.1,
            z: self.2,
        }
    }
}

impl Vec3 {
    pub fn new<T>(args: T) -> Self
    where
        T: IntoVec3,
    {
        args.into()
    }
    // pub fn from_f64(v: f64) -> Self {
    //     Self { x: v, y: v, z: v}
    // }
    // pub fn from_xyz(x: f64, y: f64, z: f64) -> Self {
    //     Self {x: x, y: y, z: z}
    // }
    pub fn length(self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
    pub fn norm(self) -> Self {
        self / self.length()
    }
    pub fn dot(self, rhs: Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
    pub fn abs(self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
            z: self.z.abs(),
        }
    }
    pub fn sign(self) -> Self {
        Self {
            x: super::functions::sign(self.x),
            y: super::functions::sign(self.y),
            z: super::functions::sign(self.z),
        }
    }
    pub fn step(self, v: Self) -> Self {
        Self {
            x: super::functions::step(self.x, v.x),
            y: super::functions::step(self.y, v.y),
            z: super::functions::step(self.z, v.z),
        }    
    }
}
