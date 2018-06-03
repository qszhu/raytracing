use std::f32;
use std::ops::*;

/*************************************************************************************************/
#[derive(Debug, PartialEq)]
pub struct Color { pub r: u8, pub g: u8, pub b: u8 }

impl Color {
    pub fn from_vec3(v: &Vec3) -> Self {
        fn u(f: f32) -> u8 { (255.99*f) as u8 }
        Color { r: u(v.x), g: u(v.y), b: u(v.z) }
    }
}

/*************************************************************************************************/
#[derive(Debug, Clone, Copy)]
pub struct Vec3 { pub x: f32, pub y: f32, pub z: f32 }

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3 { x, y, z }
    }
    pub fn zero() -> Self {
        Vec3 { x: 0., y: 0., z: 0. }
    }
    pub fn dot(&self, other: Self) -> f32 { self.x*other.x + self.y*other.y + self.z*other.z }
    pub fn cross(&self, other: Self) -> Self {
        Vec3 {
            x: self.y*other.z - self.z*other.y,
            y: self.z*other.x - self.x*other.z,
            z: self.x*other.y - self.y*other.x
        }
    }
    pub fn length(&self) -> f32 { self.dot(*self).sqrt() }
    pub fn unit(v: Vec3) -> Vec3 { v / v.length() }
}

// v1 = v2
impl PartialEq for Vec3 {
    fn eq(&self, other: &Vec3) -> bool {
        (self.x-other.x).abs() <= f32::EPSILON &&
        (self.y-other.y).abs() <= f32::EPSILON &&
        (self.z-other.z).abs() <= f32::EPSILON
    }
}
impl Eq for Vec3 {}

// v1 + v2
impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, other: Self) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

// v1 += v2
impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        *self = Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

// -v
impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self{
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z
        }
    }
}

// v1 - v2
impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
}

// v1 -= v2
impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Vec3) {
        *self = Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
}

// v1 * v2
impl Mul for Vec3 {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z
        }
    }
}

// v1 *= v2
impl MulAssign for Vec3 {
    fn mul_assign(&mut self, other: Vec3) {
        *self = Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z
        }
    }
}

// v * a
impl Mul<f32> for Vec3 {
    type Output = Vec3;
    fn mul(self, other: f32) -> Vec3 {
        Vec3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other
        }
    }
}

// v *= a
impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, other: f32) {
        *self = Vec3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other
        }
    }
}

// a * v
impl Mul<Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self * other.x,
            y: self * other.y,
            z: self * other.z
        }
    }
}

// v1 / v2
impl Div for Vec3 {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        Vec3 {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z
        }
    }
}

// v1 /= v2
impl DivAssign for Vec3 {
    fn div_assign(&mut self, other: Vec3) {
        *self = Vec3 {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z
        }
    }
}

// v / a
impl Div<f32> for Vec3 {
    type Output = Vec3;
    fn div(self, other: f32) -> Vec3 {
        Vec3 {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other
        }
    }
}

// v /= a
impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, other: f32) {
        *self = Vec3 {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other
        }
    }
}

#[cfg(test)]
mod tests {
    use std::f32;
    use super::*;

    #[test]
    fn test_color() {
        let c = Color::from_vec3(&Vec3::zero());
        assert_eq!(c, Color { r: 0, g: 0, b: 0 });

        let c = Color::from_vec3(&Vec3::new(1.,1.,1.));
        assert_eq!(c, Color { r: 255, g: 255, b: 255 });
    }

    #[test]
    fn test_create() {
        let a = Vec3::new(1., 2., 3.);
        assert_eq!(a.x, 1.);
        assert_eq!(a.y, 2.);
        assert_eq!(a.z, 3.);
    }

    #[test]
    fn test_create_zero() {
        let a = Vec3::zero();
        assert_eq!(a.x, 0.);
        assert_eq!(a.y, 0.);
        assert_eq!(a.z, 0.);
    }

    #[test]
    fn test_length() {
        let a = Vec3::new(1., 2., 3.);
        assert!((a.length()-14_f32.sqrt()).abs() <= f32::EPSILON);
    }

    #[test]
    fn test_unit() {
        let a = Vec3::new(2., 0., 0.);
        assert_eq!(Vec3::unit(a), Vec3::new(1., 0., 0.));
    }

    #[test]
    fn test_dot() {
        let a = Vec3::new(1., 2., 3.);
        let b = Vec3::new(4., 5., 6.);
        assert_eq!(a.dot(b), 32.);
    }

    #[test]
    fn test_cross() {
        let a = Vec3::new(1., 2., 3.);
        let b = Vec3::new(4., 5., 6.);
        assert_eq!(a.cross(b), Vec3::new(-3., 6., -3.));
    }

    #[test]
    fn test_add() {
        let a = Vec3::new(1., 2., 3.);
        let b = Vec3::new(4., 5., 6.);
        assert_eq!(a+b, Vec3::new(5., 7., 9.));
    }

    #[test]
    fn test_add_assign() {
        let mut a = Vec3::new(1., 2., 3.);
        let b = Vec3::new(4., 5., 6.);
        a += b;
        assert_eq!(a, Vec3::new(5., 7., 9.));
    }

    #[test]
    fn test_neg() {
        let a = Vec3::new(1., 2., 3.);
        assert_eq!(-a, Vec3::new(-1., -2., -3.));
    }

    #[test]
    fn test_sub() {
        let a = Vec3::new(1., 2., 3.);
        let b = Vec3::new(4., 5., 6.);
        assert_eq!(a-b, Vec3::new(-3., -3., -3.));
    }

    #[test]
    fn test_sub_assign() {
        let mut a = Vec3::new(1., 2., 3.);
        let b = Vec3::new(4., 5., 6.);
        a -= b;
        assert_eq!(a, Vec3::new(-3., -3., -3.));
    }

    #[test]
    fn test_mul() {
        let a = Vec3::new(1., 2., 3.);
        let b = Vec3::new(4., 5., 6.);
        assert_eq!(a*b, Vec3::new(4., 10., 18.));
    }

    #[test]
    fn test_mul_assign() {
        let mut a = Vec3::new(1., 2., 3.);
        let b = Vec3::new(4., 5., 6.);
        a *= b;
        assert_eq!(a, Vec3::new(4., 10., 18.));
    }

    #[test]
    fn test_mul_scalar() {
        let a = Vec3::new(1., 2., 3.);
        assert_eq!(a*2., Vec3::new(2., 4., 6.));
    }

    #[test]
    fn test_mul_assign_scalar() {
        let mut a = Vec3::new(1., 2., 3.);
        a *= 2.;
        assert_eq!(a, Vec3::new(2., 4., 6.));
    }

    #[test]
    fn test_mul_scalar_l() {
        let a = Vec3::new(1., 2., 3.);
        assert_eq!(2.*a, Vec3::new(2., 4., 6.));
    }

    #[test]
    fn test_div() {
        let a = Vec3::new(1., 2., 3.);
        let b = Vec3::new(4., 5., 6.);
        assert_eq!(a/b, Vec3::new(0.25, 0.4, 0.5));
    }

    #[test]
    fn test_div_assign() {
        let mut a = Vec3::new(1., 2., 3.);
        let b = Vec3::new(4., 5., 6.);
        a /= b;
        assert_eq!(a, Vec3::new(0.25, 0.4, 0.5));
    }

    #[test]
    fn test_div_scalar() {
        let a = Vec3::new(1., 2., 3.);
        assert_eq!(a/2., Vec3::new(0.5, 1., 1.5));
    }

    #[test]
    fn test_div_assign_scalar() {
        let mut a = Vec3::new(1., 2., 3.);
        a /= 2.;
        assert_eq!(a, Vec3::new(0.5, 1., 1.5));
    }

}
