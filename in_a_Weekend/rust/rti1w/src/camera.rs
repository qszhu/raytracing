use std::f32;

use vec3::Vec3;
use ray::Ray;

pub struct Camera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
}

impl Camera {
    pub fn new(lookfrom: Vec3, lookat: Vec3, vup: Vec3, vfov: f32, aspect: f32) -> Camera {
        let theta = vfov*f32::consts::PI/180.0;
        let half_height = (theta/2.0).tan();
        let half_width = aspect * half_height;
        let w = Vec3::unit(lookfrom - lookat);
        let u = Vec3::unit(vup.cross(w));
        let v = w.cross(u);
        let origin = lookfrom;
        Camera {
            lower_left_corner: origin - half_width*u - half_height*v - w,
            horizontal: 2.0*half_width*u,
            vertical: 2.0*half_height*v,
            origin
        }
    }
    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        return Ray::new(self.origin, self.lower_left_corner + s*self.horizontal + t*self.vertical - self.origin);
    }
}
