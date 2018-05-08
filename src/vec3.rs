use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Neg;
use std::ops::Sub;

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub const ZERO: Vec3 = Vec3 { x: 0.0, y: 0.0, z: 0.0 };
    pub const ONES: Vec3 = Vec3 { x: 1.0, y: 1.0, z: 1.0 };

    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }


    pub fn dot(a: Vec3, b: Vec3) -> f32 {
        a.x * b.x + a.y * b.y + a.z * b.z
    }

    pub fn cross(a: Vec3, b: Vec3) -> Vec3 {
        let x = a.y * b.z - a.z * b.y;
        let y = a.z * b.x - a.x * b.z;
        let z = a.x * b.y - a.y * b.x;
        Vec3 { x, y, z }
    }

    pub fn r(&self) -> f32 { self.x }
    pub fn g(&self) -> f32 { self.y }
    pub fn b(&self) -> f32 { self.z }


    pub fn squared_length(&self) -> f32 {
        let x = self.x;
        let y = self.y;
        let z = self.z;
        x * x + y * y + z * z
    }

    pub fn length(&self) -> f32 { self.squared_length().sqrt() }

    pub fn normalize(&self) -> Vec3 {
        let l = self.length();
        self.clone() / l
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3 {
        let x = self.x + other.x;
        let y = self.y + other.y;
        let z = self.z + other.z;
        Vec3 { x, y, z }
    }
}

impl AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3 {
        let x = self.x - other.x;
        let y = self.y - other.y;
        let z = self.z - other.z;
        Vec3 { x, y, z }
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;
    fn div(self, c: f32) -> Vec3 {
        let x = self.x / c;
        let y = self.y / c;
        let z = self.z / c;
        Vec3 { x, y, z }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;
    fn mul(self, c: f32) -> Vec3 {
        let x = self.x * c;
        let y = self.y * c;
        let z = self.z * c;
        Vec3 { x, y, z }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        let x = -self.x;
        let y = -self.y;
        let z = -self.z;
        Vec3 { x, y, z }
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        rhs * self
    }
}
