use std::ops::{Add, Div, Mul, Sub};

#[derive(Copy, Clone)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Add for Vec2 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl Sub for Vec2 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
impl Sub<f64> for Vec2 {
    type Output = Self;
    fn sub(self, rhs: f64) -> Self {
        Self {
            x: self.x - rhs,
            y: self.y - rhs,
        }
    }
}
impl Mul for Vec2 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}
impl Mul<f64> for Vec2 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}
impl Div for Vec2 {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

pub trait IntoVec2 {
    fn into(self) -> Vec2;
}

impl Vec2 {
    pub fn new<T>(args: T) -> Self
        where T: IntoVec2
    {
        args.into()
    }
    // pub fn from_f64(v: f64) -> Self {
    //     Self { x: v, y: v}
    // }
    // pub fn from_xy(x: f64, y: f64) -> Self {
    //     Self {x: x, y: y}
    // }
    pub fn length(self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

impl IntoVec2 for f64 {
    fn into(self) -> Vec2 {
       Vec2 { x: self, y: self} 
    }
}

impl IntoVec2 for (f64,f64) {
    fn into(self) -> Vec2 {
       Vec2 { x: self.0, y: self.1} 
    }
}

impl IntoVec2 for (usize, usize) {
    fn into(self) -> Vec2 {
       Vec2 { x: self.0 as f64, y: self.1 as f64} 
    }
}
