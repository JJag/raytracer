use ray::Ray;
use std::f32;
use vec3::Vec3;
use util;

pub struct Camera {
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    pub origin: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f32,
}

impl Camera {
    pub fn new(
        look_from: Vec3,
        look_at: Vec3,
        view_up: Vec3,
        vfov_deg: f32,
        aspect: f32,
        aperture: f32,
        focus_dist: f32,
    ) -> Self {
        let lens_radius = aperture / 2.0;
        let theta = vfov_deg * f32::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = half_height * aspect;

        // camera space axes
        let w = (look_from - look_at).normalize();
        let u = Vec3::cross(view_up, w).normalize();
        let v = Vec3::cross(w, u);

        let origin = look_from;

        let horizontal = u * (half_width * 2.0 * focus_dist);
        let vertical = v * (half_height * 2.0 * focus_dist);
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w * focus_dist;

        Camera {
            lower_left_corner,
            horizontal,
            vertical,
            origin,
            u,
            v,
            w,
            lens_radius,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let rd = random_on_unit_disc() * self.lens_radius;
        let offset = self.u * rd.x + self.v * rd.y;
        let origin = self.origin;
        let direction = self.lower_left_corner + u * self.horizontal + v * self.vertical - origin;
        Ray::new(origin + offset, direction - offset)
    }
}

fn random_on_unit_disc() -> Vec3 {
    loop {
        let v = Vec3::new(util::rand_f32(), util::rand_f32(), 0.0);
        let v = v * 2.0 - Vec3::new(1.0, 1.0, 0.0);
        if v.squared_length() < 1.0 {
            return v;
        }
    }
}
