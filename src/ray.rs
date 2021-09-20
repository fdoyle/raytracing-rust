use crate::vec3::{Point3, Vec3};

struct Ray {
    pub origin: Point3,
    pub dir: Vec3
}

impl Ray {
    fn at(&self, t: f32) -> Point3 {
        return *self.origin + dir * t
    }
}