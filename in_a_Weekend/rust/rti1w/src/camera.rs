extern crate rand;

use std::f32;
use rand::Rng;

use vec3::Vec3;
use ray::Ray;

pub struct Camera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
    pub lens_radius: f32
}

fn random_in_unit_disk() -> Vec3 {
    let mut rng = rand::thread_rng();
    let mut r = || -> f32 { rng.gen() };
    loop {
        let p = 2.0*Vec3::new(r(),r(),0.) - Vec3::new(1.,1.,0.);
        if p.dot(p) < 1. { return p }
    }
}

impl Camera {
    pub fn new(lookfrom: Vec3, lookat: Vec3, vup: Vec3, vfov: f32, aspect: f32, aperture: f32, focus_dist: f32) -> Camera {
        let lens_radius = aperture / 2.;
        let theta = vfov*f32::consts::PI/180.0;
        let half_height = (theta/2.0).tan();
        let half_width = aspect * half_height;
        let w = Vec3::unit(lookfrom - lookat);
        let u = Vec3::unit(vup.cross(w));
        let v = w.cross(u);
        let origin = lookfrom;
        Camera {
            lower_left_corner: origin - half_width*focus_dist*u - half_height*focus_dist*v - focus_dist*w,
            horizontal: 2.0*half_width*focus_dist*u,
            vertical: 2.0*half_height*focus_dist*v,
            origin,
            u, v, w, lens_radius
        }
    }
    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        let rd = self.lens_radius*random_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;
        return Ray::new(self.origin + offset, self.lower_left_corner + s*self.horizontal + t*self.vertical - self.origin - offset);
    }
}
