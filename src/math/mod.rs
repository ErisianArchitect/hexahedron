pub mod bit;
pub mod axis;
pub mod axis_flags;

use bit::GetBit;
use glam::{IVec2, IVec3, IVec4, Vec3};

#[inline]
pub const fn index2<const W: i32>(x: i32, y: i32) -> usize {
    let x = x.rem_euclid(W);
    let y = y.rem_euclid(W);
    (y * W + x) as usize
}

#[inline]
pub const fn index3<const W: i32>(x: i32, y: i32, z: i32) -> usize {
    let x = x.rem_euclid(W);
    let y = y.rem_euclid(W);
    let z = z.rem_euclid(W);
    (y * W*W + z * W + x) as usize
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
    assert_eq!(tri.len(), 3);
    let a = tri[1] - tri[0];
    let b = tri[2] - tri[0];
    let nx = a.y * b.z - a.z * b.y;
    let ny = a.z * b.x - a.x * b.z;
    let nz = a.x * b.y - a.y * b.x;
    Vec3::new(nx, ny, nz).normalize()
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