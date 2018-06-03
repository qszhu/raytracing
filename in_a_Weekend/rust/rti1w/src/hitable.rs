use vec3::Vec3;
use ray::Ray;

pub struct HitRecord {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3
}

/*************************************************************************************************/
pub trait Hitable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

/*************************************************************************************************/
pub struct HitableList {
    pub list: Vec<Box<Hitable>>
}

impl HitableList {
    pub fn new(list: Vec<Box<Hitable>>) -> Self {
        HitableList { list }
    }
}

impl Hitable for HitableList {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut best: Option<HitRecord> = None;
        for child in self.list.iter() {
            if let Some(hit) = child.hit(r, t_min, t_max) {
                if best.is_none() || hit.t < best.as_ref()?.t { best = Some(hit) }
            }
        }
        best
    }
}

/*************************************************************************************************/
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Self {
        Sphere { center, radius }
    }
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = r.origin - self.center;
        let a = r.direction.dot(r.direction);
        let b = oc.dot(r.direction);
        let c = oc.dot(oc) - self.radius*self.radius;
        let discriminant = b*b - a*c;
        let hit = |t: f32| -> Option<HitRecord> {
            if t <= t_min || t >= t_max { return None }
            let p = r.point_at(t);
            return Some(HitRecord { t, p, normal: (p - self.center) / self.radius })
        };
        if discriminant > 0. {
            let h = hit((-b - discriminant.sqrt())/a);
            if h.is_some() { return h }
            let h = hit((-b + discriminant.sqrt())/a);
            if h.is_some() { return h }
        }
        None
    }
}
