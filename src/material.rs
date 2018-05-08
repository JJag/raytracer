use vec3::Vec3;
use ray::Ray;
use hitable::Hit;
use util;

pub trait Material {
    fn scatter(&self, r: &Ray, h: &Hit) -> Option<(Ray, Vec3)>; // Vec3 here is color
}

pub struct Lambertian {
    pub albedo: Vec3,
}

impl Material for Lambertian {
    fn scatter(&self, _r: &Ray, h: &Hit) -> Option<(Ray, Vec3)> {
        let n = h.n;
        let p = h.p;

        let reflection_dir = n + random_in_unit_sphere();
        let reflected_ray = Ray::new(p, reflection_dir);
        Some((reflected_ray, self.albedo))
    }
}

pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f32,
}

impl Material for Metal {
    fn scatter(&self, r: &Ray, h: &Hit) -> Option<(Ray, Vec3)> {
        let n = h.n;
        let p = h.p;
        let distortion = random_in_unit_sphere() * self.fuzz;
        let reflection_dir = reflect(r.direction, n) + distortion;
        let reflected_ray = Ray::new(p, reflection_dir);
        let is_reflected_inward = Vec3::dot(reflected_ray.direction, n) < 0.0;
        if is_reflected_inward {
            None
        } else {
            Some((reflected_ray, self.albedo))
        }
    }
}

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - (n * 2.0 * Vec3::dot(v, n))
}

fn random_in_unit_sphere() -> Vec3 {
    loop {
        let v = Vec3::new(
            util::rand_f32(),
            util::rand_f32(),
            util::rand_f32(),
        );
        let v = v * 2.0 - Vec3::ONES;
        if v.squared_length() <= 1.0 {
            return v;
        }
    }
}

pub struct Glass {
    pub refraction_index: f32,
}

impl Material for Glass {
    fn scatter(&self, r: &Ray, h: &Hit) -> Option<(Ray, Vec3)> {
        let ref_idx = self.refraction_index;
        let attenuation = Vec3::new(1.0, 1.0, 1.0);

        let outward_normal: Vec3;
        let ni_over_nt;
        let cosine;
        if Vec3::dot(r.direction, h.n) > 0.0 {
            outward_normal = -h.n;
            ni_over_nt = ref_idx;
            cosine = ref_idx * Vec3::dot(r.direction, h.n) / r.direction.length();
        } else {
            outward_normal = h.n;
            ni_over_nt = 1.0 / ref_idx;
            cosine = -Vec3::dot(r.direction, h.n) / r.direction.length()
        }

        let reflect_prob = shlick(cosine, ref_idx);

        let scatter_dir;
        if util::rand_f32() < reflect_prob {
            scatter_dir = reflect(r.direction, h.n);
        } else {
            let refracted = refract(r.direction, outward_normal, ni_over_nt);
            scatter_dir = refracted.unwrap_or(reflect(r.direction, h.n));
        }
        let scatter_ray = Ray { origin: h.p, direction: scatter_dir };

        Some((scatter_ray, attenuation))
    }
}

fn refract(v: Vec3, n: Vec3, ni_over_nt: f32) -> Option<Vec3> {
    let uv = v.normalize();
    let dt = Vec3::dot(uv, n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if discriminant > 0.0 {
        Some((uv - n * dt) * ni_over_nt - n * discriminant.sqrt())
    } else {
        None
    }
}

fn shlick(cosine: f32, ref_idx: f32) -> f32 {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}
