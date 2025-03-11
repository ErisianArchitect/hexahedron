pub mod bit;
pub mod axis;
pub mod axis_flags;

use bit::GetBit;
use glam::{IVec2, IVec3, IVec4, Vec3};

/// Wraps coordinates within `W`x`H` and returns the index within that space.  
/// This is used for getting the index within a 1-D array with WxH elements.
#[inline]
pub const fn index2<const W: i32, const H: i32>(x: i32, y: i32) -> usize {
    let x = x.rem_euclid(W);
    let y = y.rem_euclid(H);
    (y * W + x) as usize
}

/// Wraps coordinates within `W`x`H`x`D` and returns the index within that space.  
/// This is used for getting the index within a 1-D array with WxHxD elements.
#[inline]
pub const fn index3<const W: i32, const H: i32, const D: i32>(x: i32, y: i32, z: i32) -> usize {
    let x = x.rem_euclid(W);
    let y = y.rem_euclid(H);
    let z = z.rem_euclid(D);
    (y * W*D + z * W + x) as usize
}

#[inline]
pub const fn index2_16(x: i32, y: i32) -> usize {
    let x = (x & 15) as usize;
    let y = (y & 15) as usize;
    y << 4 | x
}

#[inline]
pub const fn index3_16(x: i32, y: i32, z: i32) -> usize {
    let x = (x & 15) as usize;
    let y = (y & 15) as usize;
    let z = (z & 15) as usize;
    y << 8 | z << 4 | x
}

#[inline]
pub const fn index2_32(x: i32, y: i32) -> usize {
    let x = (x & 31) as usize;
    let y = (y & 31) as usize;
    y << 5 | x
}

#[inline]
pub const fn index3_32(x: i32, y: i32, z: i32) -> usize {
    let x = (x & 31) as usize;
    let y = (y & 31) as usize;
    let z = (z & 31) as usize;
    y << 10 | z << 5 | x
}

#[inline]
pub const fn index2_64(x: i32, y: i32) -> usize {
    let x = (x & 63) as usize;
    let y = (y & 63) as usize;
    y << 6 | x
}

#[inline]
pub const fn index3_64(x: i32, y: i32, z: i32) -> usize {
    let x = (x & 63) as usize;
    let y = (y & 63) as usize;
    let z = (z & 63) as usize;
    y << 12 | z << 6 | x
}

/// Returns (min, max).
#[inline]
pub fn minmax<T: PartialOrd>(a: T, b: T) -> (T, T) {
    if a <= b { (a, b) } else { (b, a) }
}

#[inline]
pub fn f32_not_zero(value: f32) -> bool {
    value != 0.0 && value != -0.0
}

#[inline]
pub fn f32_is_zero(value: f32) -> bool {
    value == 0.0 || value == -0.0
}

#[inline]
pub fn f64_not_zero(value: f64) -> bool {
    value != 0.0 && value != -0.0
}

#[inline]
pub fn f64_is_zero(value: f64) -> bool {
    value == 0.0 || value == -0.0
}

/// Returns `Some(t)` where t is the normalized distance between the min and max.
/// So if the min and max were 5 and 10 and you wanted to check the value of
/// 7.5, you would expect to get a result of `Some(0.5)` because 7.5 is halfway
/// between 5 and 10.
pub fn check_between_f32(value: f32, min: f32, max: f32) -> Option<f32> {
    debug_assert!(min < max, "min must be less than max.");
    if value < min || value > max {
        None
    } else {
        let diff = max - min;
        let mult = 1.0 / diff;
        let value_in = value - min;
        Some(value_in * mult)
    }
}

/// Returns `Some(t)` where t is the normalized distance between the min and max.
/// So if the min and max were 5 and 10 and you wanted to check the value of
/// 7.5, you would expect to get a result of `Some(0.5)` because 7.5 is halfway
/// between 5 and 10.
pub fn check_between_f64(value: f64, min: f64, max: f64) -> Option<f64> {
    debug_assert!(min < max, "min must be less than max.");
    if value < min || value > max {
        None
    } else {
        let diff = max - min;
        let mult = 1.0 / diff;
        let value_in = value - min;
        Some(value_in * mult)
    }
}

/// Calculate the normal of a triangle.
pub fn calculate_tri_normal(tri: &[Vec3]) -> Vec3 {
    debug_assert_eq!(tri.len(), 3);
    let a = tri[1] - tri[0];
    let b = tri[2] - tri[0];
    let nx = a.y * b.z - a.z * b.y;
    let ny = a.z * b.x - a.x * b.z;
    let nz = a.x * b.y - a.y * b.x;
    Vec3::new(nx, ny, nz).normalize()
}

pub fn quadratic_bezier2(points: &[glam::Vec2], t: f32) -> glam::Vec2 {
    debug_assert_eq!(points.len(), 3, "Must have exactly 3 points.");
    let ab = points[0].lerp(points[1], t);
    let bc = points[1].lerp(points[2], t);
    ab.lerp(bc, t)
}

pub fn cubic_bezier2(points: &[glam::Vec2], t: f32) -> glam::Vec2 {
    debug_assert_eq!(points.len(), 4, "Must have exactly 4 points.");
    let ab = points[0].lerp(points[1], t);
    let bc = points[1].lerp(points[2], t);
    let cd = points[2].lerp(points[3], t);
    let ab_bc = ab.lerp(bc, t);
    let bc_cd = bc.lerp(cd, t);
    ab_bc.lerp(bc_cd, t)
}

pub fn quadratic_bezier3(points: &[glam::Vec3], t: f32) -> glam::Vec3 {
    debug_assert_eq!(points.len(), 3, "Must have exactly 3 points.");
    let ab = points[0].lerp(points[1], t);
    let bc = points[1].lerp(points[2], t);
    ab.lerp(bc, t)
}

pub fn cubic_bezier3(points: &[glam::Vec3], t: f32) -> glam::Vec3 {
    debug_assert_eq!(points.len(), 4, "Must have exactly 4 points.");
    let ab = points[0].lerp(points[1], t);
    let bc = points[1].lerp(points[2], t);
    let cd = points[2].lerp(points[3], t);
    let ab_bc = ab.lerp(bc, t);
    let bc_cd = bc.lerp(cd, t);
    ab_bc.lerp(bc_cd, t)
}

#[inline]
pub fn checkerboard1<T: GetBit>(x: T) -> bool {
    x.get_bit(0)
}

#[inline]
pub fn checkerboard2<T: GetBit>(x: T, y: T) -> bool {
    x.get_bit(0) ^
    y.get_bit(0)
}

#[inline]
pub fn checkerboard3<T: GetBit>(x: T, y: T, z: T) -> bool {
    x.get_bit(0) ^
    y.get_bit(0) ^
    z.get_bit(0)
}

#[inline]
pub fn checkerboard4<T: GetBit>(x: T, y: T, z: T, w: T) -> bool {
    x.get_bit(0) ^
    y.get_bit(0) ^
    z.get_bit(0) ^
    w.get_bit(0)
}

pub trait Checkerboard {
    fn checkerboard(self) -> bool;
}

impl<T: GetBit> Checkerboard for T {
    #[inline]
    fn checkerboard(self) -> bool {
        checkerboard1(self)
    }
}

impl<T: GetBit> Checkerboard for (T, T) {
    
    #[inline]
    fn checkerboard(self) -> bool {
        checkerboard2(self.0, self.1)
    }
}

impl<T: GetBit> Checkerboard for (T, T, T) {
    #[inline]
    fn checkerboard(self) -> bool {
        checkerboard3(self.0, self.1, self.2)
    }
}

impl<T: GetBit> Checkerboard for (T, T, T, T) {
    #[inline]
    fn checkerboard(self) -> bool {
        checkerboard4(self.0, self.1, self.2, self.3)
    }
}

impl<T: GetBit> Checkerboard for [T; 1] {
    #[inline]
    fn checkerboard(self) -> bool {
        let [x] = self;
        checkerboard1(x)
    }
}

impl<T: GetBit> Checkerboard for [T; 2] {
    #[inline]
    fn checkerboard(self) -> bool {
        let [x, y] = self;
        checkerboard2(x, y)
    }
}

impl<T: GetBit> Checkerboard for [T; 3] {
    #[inline]
    fn checkerboard(self) -> bool {
        let [x, y, z] = self;
        checkerboard3(x, y, z)
    }
}

impl<T: GetBit> Checkerboard for [T; 4] {
    #[inline]
    fn checkerboard(self) -> bool {
        let [x, y, z, w] = self;
        checkerboard4(x, y, z, w)
    }
}

impl Checkerboard for IVec2 {
    #[inline]
    fn checkerboard(self) -> bool {
        checkerboard2(self.x, self.y)
    }
}

impl Checkerboard for IVec3 {
    #[inline]
    fn checkerboard(self) -> bool {
        checkerboard3(self.x, self.y, self.z)
    }
}

impl Checkerboard for IVec4 {
    #[inline]
    fn checkerboard(self) -> bool {
        checkerboard4(self.x, self.y, self.z, self.w)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn minmax_test() {
        assert_eq!(minmax(4, 2), (2, 4));
        assert_eq!(minmax(2, 4), (2, 4));
        assert_eq!(minmax(4, 4), (4, 4));
        assert_eq!(minmax(3.14, 2.15), (2.15, 3.14));
    }
}