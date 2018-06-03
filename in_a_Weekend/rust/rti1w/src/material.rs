extern crate rand;

use rand::Rng;

use vec3::Vec3;
use ray::Ray;
use hitable::HitRecord;

pub struct ScatterRecord {
    pub attenuation: Vec3,
    pub scattered: Ray
}

/*************************************************************************************************/
pub trait Material {
    fn scatter(&self, r_in: &Ray, hit: &HitRecord) -> Option<ScatterRecord>;
}

/*************************************************************************************************/
pub struct Lambertian {
    albedo: Vec3
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Lambertian {
        Lambertian { albedo }
    }
}

fn random_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();
    let mut r = || -> f32 { rng.gen() };
    loop {
        let p = 2.0*Vec3::new(r(), r(), r()) - Vec3::new(1.,1.,1.);
        if p.dot(p) < 1.0 { return p }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, hit: &HitRecord) -> Option<ScatterRecord> {
        let target = hit.p + hit.normal + random_in_unit_sphere();
        return Some(ScatterRecord {
            attenuation: self.albedo,
            scattered: Ray::new(hit.p, target-hit.p)
        })
    }
}

/*************************************************************************************************/
pub struct Metal {
    albedo: Vec3,
    fuzz: f32
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f32) -> Metal {
        Metal { albedo, fuzz: fuzz.min(1.) }
    }
}

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0*v.dot(n)*n
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, hit: &HitRecord) -> Option<ScatterRecord> {
        let reflected = reflect(Vec3::unit(r_in.direction), hit.normal);
        let scattered = Ray::new(hit.p, reflected + self.fuzz*random_in_unit_sphere());
        if scattered.direction.dot(hit.normal) > 0. {
            Some(ScatterRecord {
                attenuation: self.albedo,
                scattered
            })
        } else {
            None
        }
    }
}

/*************************************************************************************************/
fn refrect(v: Vec3, n: Vec3, ni_over_nt: f32) -> Option<Vec3> {
    let uv = Vec3::unit(v);
    let dt = uv.dot(n);
    let discriminant = 1.0 - ni_over_nt*ni_over_nt*(1.0-dt*dt);
    if discriminant > 0.0 {
        Some(ni_over_nt*(uv - n*dt) - n*discriminant.sqrt())
    } else {
        None
    }
}

fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let mut r0 = (1.0-ref_idx) / (1.0+ref_idx);
    r0 *= r0;
    return r0 + (1.0-r0)*(1.0-cosine).powf(5.0);
}

pub struct Dielectric {
    pub ref_idx: f32
}

impl Dielectric {
    pub fn new(ref_idx: f32) -> Dielectric {
        Dielectric { ref_idx }
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, hit: &HitRecord) -> Option<ScatterRecord> {
        let reflected = reflect(r_in.direction, hit.normal);
        let attenuation = Vec3::new(1.0, 1.0, 1.0);
        let outward_normal;
        let ni_over_nt;
        let reflect_prob;
        let cosine;
        if r_in.direction.dot(hit.normal) > 0.0 {
            outward_normal = -hit.normal;
            ni_over_nt = self.ref_idx;
            cosine = self.ref_idx * r_in.direction.dot(hit.normal) / r_in.direction.length();
        } else {
            outward_normal = hit.normal;
            ni_over_nt = 1.0 / self.ref_idx;
            cosine = -r_in.direction.dot(hit.normal) / r_in.direction.length();
        }
        if let Some(refracted) = refrect(r_in.direction, outward_normal, ni_over_nt) {
            reflect_prob = schlick(cosine, self.ref_idx);
            if rand::random::<f32>() < reflect_prob {
                Some(ScatterRecord { attenuation, scattered: Ray::new(hit.p, reflected) })
            } else {
                Some(ScatterRecord { attenuation, scattered: Ray::new(hit.p, refracted) })
            }
        } else {
            Some(ScatterRecord { attenuation, scattered: Ray::new(hit.p, reflected) })
        }
    }
}
