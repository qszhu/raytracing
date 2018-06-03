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
    pub fn new(albedo: Vec3) -> Self {
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
    pub fn new(albedo: Vec3, fuzz: f32) -> Self {
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
