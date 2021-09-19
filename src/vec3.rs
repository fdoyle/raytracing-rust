use std::ops;
use std::fs::File;
use std::io::Write;


pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

pub type Point3 = Vec3;
pub type Color = Vec3;

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 {
            x,
            y,
            z,
        }
    }

    pub fn length(&self) -> f32 {
        return self.length_squared().sqrt();
    }

    pub fn length_squared(&self) -> f32 {
        return self.x * self.x + self.y * self.y + self.z * self.z;
    }

    pub fn print(&self) {
        println!("Vec3 {},{},{}", self.x, self.y, self.z)
    }

    pub fn dot(self, rhs: Vec3) -> f32 {
        return self.x * rhs.x + self.y * rhs.y + self.z * rhs.z;
    }

    pub fn cross(self, rhs: Vec3) -> Vec3 {
        return Vec3 {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        };
    }

    pub fn unit_vector(self) -> Vec3 {
        let length = self.length();
        return self / length;
    }
}


//top part of vec3 class, from tutorial
impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        return Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        };
    }
}

impl ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}


impl ops::MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl ops::DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}


//vec3 utility functions
impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        return Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        };
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        return Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        };
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        return Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        };
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Self::Output {
        return Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        };
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f32) -> Self::Output {
        return Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        };
    }
}

impl Color {
    pub fn write(self, mut file: &File) -> std::io::Result<()> {
        let x:i32 = (255.99999 * self.x) as i32;
        let y:i32 = (255.99999 * self.y) as i32;
        let z:i32 = (255.99999 * self.z) as i32;
        file.write_fmt(format_args!("{} {} {}\n", x,y,z))
    }
}