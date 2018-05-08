use material::Material;
use ray::Ray;
use std::f32;
use std::rc::Rc;
use vec3::Vec3;

pub struct Hit {
    pub t: f32,
    pub p: Vec3,
    pub n: Vec3,
    pub material: Rc<Material>,
}

pub trait Hitable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<Hit>;
}

pub struct Sphere {
    pub center: Vec3,
    pub r: f32,
    pub material: Rc<Material>,
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let radius = self.r;
        let center = self.center;
        let oc = r.origin - center;
        let a = Vec3::dot(r.direction, r.direction);
        let b = 2.0 * Vec3::dot(oc, r.direction);
        let c = Vec3::dot(oc, oc) - radius * radius;
        let delta = b * b - 4.0 * a * c;

        if delta < 0.0 {
            None
        } else {
            let delta_sqrt = delta.sqrt();
            let mut t = (-b - delta_sqrt) / (2.0 * a);
            if t <= t_min || t >= t_max {
                t = (-b + delta_sqrt) / (2.0 * a);
            }
            if t <= t_min || t >= t_max {
                return None;
            }
            let p = r.point_at(t);
            let n = (p - center).normalize() * self.r.signum();
            let material = self.material.clone();
            let hit = Hit { t, p, n, material };
            Some(hit)
        }
    }
}

impl<T: Hitable> Hitable for Vec<T> {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let mut best_hit: Option<Hit> = None;
        let mut best_t = f32::MAX;
        for obj in self {
            if let Some(hit) = obj.hit(r, t_min, t_max) {
                if hit.t < best_t {
                    best_t = hit.t;
                    best_hit = Some(hit);
                }
            }
        }
        best_hit
    }
}
