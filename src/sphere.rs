use crate::vec3::{Point3, Vec3};
use crate::ray::Ray;
use crate::hittable::{Hittable, HitRecord};
use std::borrow::Borrow;

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(cen: Point3, r: f64) -> Sphere {
        Sphere {
            center: cen,
            radius: r,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.dir.length_squared();
        let half_b = oc.dot(ray.dir);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None
        }
        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }
        let p = ray.at(root);
        let outward_normal = (p - self.center) / self.radius;

        let mut hr = HitRecord{
            t: root,
            p,
            normal: (p - self.center) / self.radius,
            front_face: false
        };
        hr.set_face_normal(ray, outward_normal);
        return Some(hr)
    }
}