use serde::{Serialize, Deserialize};
use bytemuck::NoUninit;
use std::slice::Iter;
use std::iter::Cloned;

// The html_colors macro is in colors.rs
// It was just really big and I didn't want it to clutter up this code.
include!("colors.rs");
// This is the table that converts u8 to f32 as a normalized value between 0 and 1 where 0u8 is 0 and 255u8 is 1.
include!("byte_to_f32.rs");

#[inline]
const fn byte_hex(byte: u8) -> [u8; 2] {
    const HEX_CHARS: [u8; 16] = [
        b'0',
        b'1',
        b'2',
        b'3',
        b'4',
        b'5',
        b'6',
        b'7',
        b'8',
        b'9',
        b'A',
        b'B',
        b'C',
        b'D',
        b'E',
        b'F',
    ];
    [
        HEX_CHARS[(byte >> 4) as usize],
        HEX_CHARS[(byte & 0xF) as usize],
    ]
}

const fn hex_char_value(hex_char: u8) -> Option<u8> {
    Some(hex_char - match hex_char {
        b'0'..=b'9' => b'0',
        b'A'..=b'F' => b'7',
        b'a'..=b'f' => b'W',
        _ => return None,
    })
}

const fn byte_from_hex(hex: &[u8]) -> Option<u8> {
    match hex_char_value(hex[0]) {
        Some(first) if hex.len() == 1 => Some(first),
        Some(first) if hex.len() == 2 => match hex_char_value(hex[1]) {
            Some(second) => Some((first << 4) | second),
            _ => None,
        }
        _ => None,
    }
}

pub fn find_color(name: &str) -> Option<Color> {
    struct NameCmp<'a>(&'a str);
    impl<'a> NameCmp<'a> {
        fn cmp_str(&self, cmp_to: &str) -> std::cmp::Ordering {
            use std::cmp::Ordering;
            let mut achars = self.chars();
            let mut bchars = cmp_to.chars();
            loop {
                match (achars.next(), bchars.next()) {
                    (Some(a), Some(b)) => match a.cmp(&b) {
                        Ordering::Equal => continue,
                        ordering => return ordering,
                    }
                    (Some(_), None) => {
                        return Ordering::Greater;
                    }
                    (None, Some(_)) => {
                        return Ordering::Less;
                    }
                    (None, None) => {
                        return Ordering::Equal;
                    }
                }
            }
        }

        fn chars(&'a self) -> impl Iterator<Item = char> + 'a {
            #[inline]
            fn filter_fn(c: &char) -> bool {
                !matches!(*c, ' ' | '_')
            }
            #[inline]
            fn to_lower(c: char) -> char {
                c.to_ascii_lowercase()
            }
            self.0.chars().filter(filter_fn).map(to_lower)
        }
    }
    let name = NameCmp(name);
    let mut low = 0usize;
    let mut high = HTML_COLORS.len();
    while low < high {
        let mid = low + (high - low) / 2;
        let mid_name = HTML_COLORS[mid].lower_name;
        match name.cmp_str(mid_name) {
            std::cmp::Ordering::Less => high = mid,
            std::cmp::Ordering::Equal => return Some(HTML_COLORS[mid].color),
            std::cmp::Ordering::Greater => low = mid + 1,
        }
    }
    None
}

#[inline]
pub fn byte_lerp(a: u8, b: u8, t: f32) -> u8 {
    if a == b {
        return a;
    }
    if t <= 0.0 {
        a
    } else if t >= 1.0 {
        b
    } else {
        let a = a as f32;
        let b = b as f32;
        let result = a + (b - a) * t;
        result as u8
    }
}

#[inline]
pub const fn byte_scalar(byte: u8) -> f32 {
    BYTE_TO_F32[byte as usize]
}

macro_rules! color_enum {
    ($({
        $readable_name:literal
        $pascal_name:ident
        $const_name:ident
        $var_name:ident
        $lower_name:ident
        $upper_name:ident
        $rgb_hex:literal
        RGB($r:literal, $g:literal, $b:literal)
    })+) => {
        #[repr(u8)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, NoUninit, Serialize, Deserialize)]
        pub enum Color {
            $(
                $pascal_name,
            )*
        }

        impl Color {
            pub const SORTED_COLORS: [Color; 140] = [
                $(
                    Color::$pascal_name,
                )*
            ];
        }
    };
}

html_colors!(color_enum);

impl Color {
    #[inline]
    pub const fn index(self) -> usize {
        self as usize
    }

    /// Gets the name in a `Human Readable Format`.
    #[inline]
    pub const fn readable_name(self) -> &'static str {
        HTML_COLORS[self.index()].readable_name
    }

    /// Gets the `PascalCase` name.
    #[inline]
    pub const fn pascal_name(self) -> &'static str {
        HTML_COLORS[self.index()].pascal_name
    }

    /// Gets the `lower_snake_case` name.
    #[inline]
    pub const fn lower_snake_name(self) -> &'static str {
        HTML_COLORS[self.index()].lower_snake
    }

    /// Gets the `UPPER_SNAKE_CASE` name.
    #[inline]
    pub const fn upper_snake_name(self) -> &'static str {
        HTML_COLORS[self.index()].upper_snake
    }

    /// Gets the `lowercasename`.
    #[inline]
    pub const fn lowercase_name(self) -> &'static str {
        HTML_COLORS[self.index()].lower_name
    }

    /// Gets the `UPPERCASENAME`
    #[inline]
    pub const fn uppercase_name(self) -> &'static str {
        HTML_COLORS[self.index()].upper_name
    }

    /// Gets the hex value.
    #[inline]
    pub const fn hex(self) -> &'static str {
        HTML_COLORS[self.index()].hex
    }

    /// Gets [Rgb] value.
    #[inline]
    pub const fn rgb(self) -> Rgb {
        HTML_COLORS[self.index()].rgb
    }

    /// Gets [Rgba] value.
    #[inline]
    pub const fn rgba(self) -> Rgba {
        self.rgb().rgba()
    }

    #[inline]
    pub const fn transparent(self) -> Rgba {
        self.rgb().transparent()
    }

    #[inline]
    pub const fn opaque(self) -> Rgba {
        self.rgba()
    }

    #[inline]
    pub const fn from_byte(byte: u8) -> Option<Self> {
        let index = byte as usize;
        if index < HTML_COLORS.len() {
            Some(Color::SORTED_COLORS[index])
        } else {
            None
        }
    }

    #[inline]
    pub const fn to_byte(self) -> u8 {
        self as u8
    }

    #[inline]
    pub fn iter() -> Cloned<Iter<'static, Color>> {
        Color::SORTED_COLORS.iter().cloned()
    }

    #[inline]
    pub fn lerp(self, other: Self, t: f32) -> Rgb {
        self.rgb().lerp(other.rgb(), t)
    }

    #[inline]
    pub fn from_name(name: &str) -> Option<Color> {
        find_color(name)
    }
}

impl From<Color> for Rgb {
    #[inline]
    fn from(value: Color) -> Self {
        value.rgb()
    }
}

impl From<Color> for Rgba {
    #[inline]
    fn from(value: Color) -> Self {
        value.rgba()
    }
}

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, NoUninit, Serialize, Deserialize)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

macro_rules! rgb_color_consts {
    ($({
        $readable_name:literal
        $pascal_name:ident
        $const_name:ident
        $var_name:ident
        $lower_name:ident
        $upper_name:ident
        $rgb_hex:literal
        RGB($r:literal, $g:literal, $b:literal)
    })+) => {
        impl Rgb {
            $(
                pub const $const_name: Self = Rgb::new($r, $g, $b);
            )+
        }
    };
}

html_colors!(rgb_color_consts);

impl Rgb {
    #[inline]
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self{r,g,b}
    }

    #[inline]
    pub const fn gray(level: u8) -> Self {
        Self::new(level, level, level)
    }

    #[inline]
    pub const fn red(red: u8) -> Self {
        Self::new(red, 0, 0)
    }

    #[inline]
    pub const fn green(green: u8) -> Self {
        Self::new(0, green, 0)
    }

    #[inline]
    pub const fn blue(blue: u8) -> Self {
        Self::new(0, 0, blue)
    }

    #[inline]
    pub const fn rgba(self) -> Rgba {
        self.opaque()
    }

    #[inline]
    pub const fn transparent(self) -> Rgba {
        Rgba::new(self.r, self.g, self.b, 0)
    }

    #[inline]
    pub const fn opaque(self) -> Rgba {
        Rgba::new(self.r, self.g, self.b, 255)
    }

    #[inline]
    pub const fn with_alpha(self, alpha: u8) -> Rgba {
        Rgba::new(self.r, self.g, self.b, alpha)
    }

    #[inline]
    pub const fn is_gray(self) -> bool {
        self.r == self.g && self.g == self.b
    }

    #[inline]
    pub const fn from_bytes([r,g,b]: [u8; 3]) -> Self {
        Rgb{r,g,b}
    }

    #[inline]
    pub const fn to_bytes(self) -> [u8; 3] {
        [self.r, self.g, self.b]
    }

    #[inline]
    pub const fn as_bytes(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(std::ptr::from_ref(self).cast(), 3)
        }
    }

    #[inline]
    pub fn as_mut_bytes(&mut self) -> &mut [u8] {
        unsafe {
            std::slice::from_raw_parts_mut(std::ptr::from_mut(self).cast(), 3)
        }
    }

    #[inline]
    pub const fn from_tuple((r,g,b): (u8, u8, u8)) -> Self {
        Self{r,g,b}
    }

    #[inline]
    pub const fn to_tuple(self) -> (u8, u8, u8) {
        (self.r, self.g, self.b)
    }

    #[inline]
    pub fn from_vec3(rgb: glam::Vec3) -> Self {
        Self {
            r: byte_lerp(0, 255, rgb.x),
            g: byte_lerp(0, 255, rgb.y),
            b: byte_lerp(0, 255, rgb.z),
        }
    }

    #[inline]
    pub const fn to_vec3(self) -> glam::Vec3 {
        glam::Vec3::new(
            byte_scalar(self.r),
            byte_scalar(self.g),
            byte_scalar(self.b),
        )
    }

    #[inline]
    pub const fn to_vec4(self) -> glam::Vec4 {
        glam::Vec4::new(
            byte_scalar(self.r),
            byte_scalar(self.g),
            byte_scalar(self.b),
            1.0,
        )
    }

    pub fn hex(self) -> String {
        let mut hex = String::with_capacity(6);
        let r = byte_hex(self.r);
        let g = byte_hex(self.g);
        let b = byte_hex(self.b);
        hex.push(r[0] as char);
        hex.push(r[1] as char);
        hex.push(g[0] as char);
        hex.push(g[1] as char);
        hex.push(b[0] as char);
        hex.push(b[1] as char);
        hex
    }

    pub fn lerp(self, b: Self, t: f32) -> Self {
        let t = t.clamp(0.0, 1.0);
        Self::new(
            byte_lerp(self.r, b.r, t),
            byte_lerp(self.g, b.g, t),
            byte_lerp(self.b, b.b, t)
        )
    }

    /// `hex` must be a valid hex value with a length of 6.
    pub const fn from_hex(hex: &str) -> Option<Self> {
        let hex = hex.as_bytes();
        if hex.len() != 6 {
            return None;
        }
        let Some(r) = byte_from_hex(&[hex[0], hex[1]]) else {
            return None;
        };
        let Some(g) = byte_from_hex(&[hex[2], hex[3]]) else {
            return None;
        };
        let Some(b) = byte_from_hex(&[hex[4], hex[5]]) else {
            return None;
        };
        Some(Self::new(r,g,b))
    }

    pub fn from_name(name: &str) -> Option<Self> {
        let Some(color) = find_color(name) else {
            return None;
        };
        Some(color.rgb())
    }
}

impl From<[u8; 3]> for Rgb {
    #[inline]
    fn from(value: [u8; 3]) -> Self {
        Rgb::from_bytes(value)
    }
}

impl From<(u8, u8, u8)> for Rgb {
    #[inline]
    fn from(value: (u8, u8, u8)) -> Self {
        Rgb::from_tuple(value)
    }
}

impl Into<[u8; 3]> for Rgb {
    #[inline]
    fn into(self) -> [u8; 3] {
        self.to_bytes()
    }
}

impl Into<(u8, u8, u8)> for Rgb {
    #[inline]
    fn into(self) -> (u8, u8, u8) {
        self.to_tuple()
    }
}

impl Into<glam::Vec3> for Rgb {
    #[inline]
    fn into(self) -> glam::Vec3 {
        self.to_vec3()
    }
}

impl Into<glam::Vec4> for Rgb {
    #[inline]
    fn into(self) -> glam::Vec4 {
        self.to_vec4()
    }
}

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, NoUninit, Serialize, Deserialize)]
pub struct Rgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8
}

macro_rules! rgba_color_consts {
    ($({
        $readable_name:literal
        $pascal_name:ident
        $const_name:ident
        $var_name:ident
        $lower_name:ident
        $upper_name:ident
        $rgb_hex:literal
        RGB($r:literal, $g:literal, $b:literal)
    })+) => {
        impl Rgba {
            $(
                pub const $const_name: Self = Rgba::new($r, $g, $b, 255);
            )+
        }
    };
}

html_colors!(rgba_color_consts);

impl Rgba {
    pub const TRANSPARENTWHITE: Self = Rgba::WHITE.transparent();
    pub const TRANSPARENTBLACK: Self = Rgba::BLACK.transparent();
    
    #[inline]
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self{r,g,b,a}
    }

    #[inline]
    pub const fn gray(gray: u8) -> Self {
        Self::new(gray, gray, gray, 255)
    }

    #[inline]
    pub const fn red(red: u8) -> Self {
        Self::new(red, 0, 0, 255)
    }

    #[inline]
    pub const fn green(green: u8) -> Self {
        Self::new(0, green, 0, 255)
    }

    #[inline]
    pub const fn blue(blue: u8) -> Self {
        Self::new(0, 0, blue, 255)
    }

    #[inline]
    pub const fn transparent(mut self) -> Self {
        self.a = 0;
        self
    }

    #[inline]
    pub const fn opaque(mut self) -> Self {
        self.a = 255;
        self
    }

    #[inline]
    pub const fn transparent_gray(gray: u8) -> Self {
        Self::alpha_gray(gray, 0)
    }

    #[inline]
    pub const fn alpha_gray(gray: u8, alpha: u8) -> Self {
        Self::new(gray, gray, gray, alpha)
    }

    #[inline]
    pub const fn rgb(self) -> Rgb {
        Rgb::new(self.r, self.g, self.b)
    }

    #[inline]
    pub const fn is_gray(self) -> bool {
        self.r == self.g && self.g == self.b
    }

    /// Determines if this [Rgba] is fully opaque (alpha channel is 255).
    #[inline]
    pub const fn is_opaque(self) -> bool {
        self.a == 255
    }

    /// Determines if this [Rgba] is fully transparent (alpha channel is 0).
    #[inline]
    pub const fn is_transparent(self) -> bool {
        self.a == 0
    }

    #[inline]
    pub const fn from_bytes([r,g,b,a]: [u8; 4]) -> Self {
        Self{r,g,b,a}
    }

    #[inline]
    pub const fn to_bytes(self) -> [u8; 4] {
        [self.r, self.g, self.b, self.a]
    }

    #[inline]
    pub const fn as_bytes(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(std::ptr::from_ref(self).cast(), 4)
        }
    }

    #[inline]
    pub fn as_mut_bytes(&mut self) -> &mut [u8] {
        unsafe {
            std::slice::from_raw_parts_mut(std::ptr::from_mut(self).cast(), 4)
        }
    }

    #[inline]
    pub const fn from_tuple((r,g,b,a): (u8, u8, u8, u8)) -> Self {
        Self{r,g,b,a}
    }

    #[inline]
    pub const fn to_tuple(self) -> (u8, u8, u8, u8) {
        (self.r, self.g, self.b, self.a)
    }

    #[inline]
    pub fn from_vec4(rgba: glam::Vec4) -> Self {
        Self {
            r: byte_lerp(0, 255, rgba.x),
            g: byte_lerp(0, 255, rgba.y),
            b: byte_lerp(0, 255, rgba.z),
            a: byte_lerp(0, 255, rgba.w),
        }
    }

    #[inline]
    pub const fn to_vec4(self) -> glam::Vec4 {
        glam::Vec4::new(
            byte_scalar(self.r),
            byte_scalar(self.g),
            byte_scalar(self.b),
            byte_scalar(self.a),
        )
    }

    pub fn hex(self) -> String {
        let mut hex = String::with_capacity(8);
        let r = byte_hex(self.r);
        let g = byte_hex(self.g);
        let b = byte_hex(self.b);
        let a = byte_hex(self.a);
        hex.push(r[0] as char);
        hex.push(r[1] as char);
        hex.push(g[0] as char);
        hex.push(g[1] as char);
        hex.push(b[0] as char);
        hex.push(b[1] as char);
        hex.push(a[0] as char);
        hex.push(a[1] as char);
        hex
    }

    pub fn lerp(self, b: Self, t: f32) -> Self {
        let t = t.clamp(0.0, 1.0);
        Self::new(
            byte_lerp(self.r, b.r, t),
            byte_lerp(self.g, b.g, t),
            byte_lerp(self.b, b.b, t),
            byte_lerp(self.a, b.a, t),
        )
    }

    /// `hex` must be a valid hex value with a length of 6.
    pub const fn from_hex(hex: &str) -> Option<Self> {
        let hex = hex.as_bytes();
        if hex.len() != 8 {
            return None;
        }
        let Some(r) = byte_from_hex(&[hex[0], hex[1]]) else {
            return None;
        };
        let Some(g) = byte_from_hex(&[hex[2], hex[3]]) else {
            return None;
        };
        let Some(b) = byte_from_hex(&[hex[4], hex[5]]) else {
            return None;
        };
        let Some(a) = byte_from_hex(&[hex[6], hex[7]]) else {
            return None;
        };
        Some(Self::new(r,g,b,a))
    }

    pub fn from_name(name: &str) -> Option<Self> {
        let Some(color) = find_color(name) else {
            return None;
        };
        Some(color.rgba())
    }
}

impl From<[u8; 4]> for Rgba {
    #[inline]
    fn from(value: [u8; 4]) -> Self {
        Self::from_bytes(value)
    }
}

impl From<(u8, u8, u8, u8)> for Rgba {
    #[inline]
    fn from(value: (u8, u8, u8, u8)) -> Self {
        Self::from_tuple(value)
    }
}

impl From<Rgb> for Rgba {
    #[inline]
    fn from(value: Rgb) -> Self {
        value.rgba()
    }
}

impl Into<[u8; 4]> for Rgba {
    #[inline]
    fn into(self) -> [u8; 4] {
        self.to_bytes()
    }
}

impl Into<(u8, u8, u8, u8)> for Rgba {
    #[inline]
    fn into(self) -> (u8, u8, u8, u8) {
        self.to_tuple()
    }
}

impl Into<glam::Vec4> for Rgba {
    #[inline]
    fn into(self) -> glam::Vec4 {
        self.to_vec4()
    }
}

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, NoUninit)]
pub struct Gray {
    level: u8
}

impl Gray {
    pub const BLACK: Self = Self::new(0);
    pub const MID: Self = Self::new(128);
    pub const WHITE: Self = Self::new(255);

    #[inline]
    pub const fn new(level: u8) -> Self {
        Self {
            level
        }
    }

    #[inline]
    pub const fn rgb(self) -> Rgb {
        Rgb::gray(self.level)
    }

    #[inline]
    pub const fn rgba(self) -> Rgba {
        Rgba::gray(self.level)
    }

    #[inline]
    pub const fn to_vec3(self) -> glam::Vec3 {
        let g = byte_scalar(self.level);
        glam::Vec3::new(
            g,
            g,
            g,
        )
    }

    #[inline]
    pub const fn to_vec4(self) -> glam::Vec4 {
        let g = byte_scalar(self.level);
        glam::Vec4::new(
            g,
            g,
            g,
            1.0,
        )
    }
}

impl From<Gray> for Rgb {
    #[inline]
    fn from(value: Gray) -> Self {
        value.rgb()
    }
}

impl From<Gray> for Rgba {
    #[inline]
    fn from(value: Gray) -> Self {
        value.rgba()
    }
}

impl From<u8> for Gray {
    #[inline]
    fn from(value: u8) -> Self {
        Gray::new(value)
    }
}

impl Into<u8> for Gray {
    #[inline]
    fn into(self) -> u8 {
        self.level
    }
}

impl Into<glam::Vec3> for Gray {
    #[inline]
    fn into(self) -> glam::Vec3 {
        self.to_vec3()
    }
}

impl Into<glam::Vec4> for Gray {
    #[inline]
    fn into(self) -> glam::Vec4 {
        self.to_vec4()
    }
}

impl std::cmp::PartialEq<Color> for Rgb {
    fn eq(&self, other: &Color) -> bool {
        self.eq(&other.rgb())
    }

    fn ne(&self, other: &Color) -> bool {
        self.ne(&other.rgb())
    }
}

impl std::cmp::PartialEq<Rgb> for Color {
    fn eq(&self, other: &Rgb) -> bool {
        self.rgb().eq(other)
    }

    fn ne(&self, other: &Rgb) -> bool {
        self.rgb().ne(other)
    }
}

impl std::cmp::PartialEq<Color> for Rgba {
    fn eq(&self, other: &Color) -> bool {
        self.eq(&other.rgba())
    }

    fn ne(&self, other: &Color) -> bool {
        self.ne(&other.rgba())
    }
}

impl std::cmp::PartialEq<Rgba> for Color {
    fn eq(&self, other: &Rgba) -> bool {
        self.rgba().eq(other)
    }

    fn ne(&self, other: &Rgba) -> bool {
        self.rgba().ne(other)
    }
}

impl std::fmt::Display for Rgb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Rgb({}, {}, {})", self.r, self.g, self.b)
    }
}

impl std::fmt::Display for Rgba {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Rgba({}, {}, {}, {})", self.r, self.g, self.b, self.a)
    }
}

impl std::fmt::Display for Gray {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Gray({})", self.level)
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Color::{}", self.pascal_name())
    }
}

#[inline]
pub const fn rgb(r: u8, g: u8, b: u8) -> Rgb {
    Rgb::new(r, g, b)
}

#[inline]
pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Rgba {
    Rgba::new(r,g,b,a)
}

#[inline]
pub const fn gray(level: u8) -> Gray {
    Gray::new(level)
}

pub struct ColorEntry {
    pub readable_name: &'static str,
    pub pascal_name: &'static str,
    pub lower_snake: &'static str,
    pub upper_snake: &'static str,
    pub lower_name: &'static str,
    pub upper_name: &'static str,
    pub hex: &'static str,
    pub color: Color,
    pub rgb: Rgb,
}

impl ColorEntry {
    pub const fn new(
        readable_name: &'static str,
        pascal_name: &'static str,
        upper_snake: &'static str,
        lower_snake: &'static str,
        lower_name: &'static str,
        upper_name: &'static str,
        hex: &'static str,
        color: Color,
        rgb: Rgb
    ) -> Self {
        Self {
            readable_name,
            pascal_name,
            upper_snake,
            lower_snake,
            lower_name,
            upper_name,
            hex,
            color,
            rgb,
        }
    }
}

#[cfg(debug_assertions)]
impl ColorEntry {
    pub const fn string_memory(&self) -> usize {
        self.readable_name.len() +
        self.pascal_name.len() +
        self.upper_snake.len() +
        self.lower_snake.len() +
        self.lower_name.len() +
        self.upper_name.len() +
        self.hex.len()
    }

    pub const fn memory_usage(&self) -> usize {
        self.string_memory() + std::mem::size_of::<Self>()
    }
}

macro_rules! color_table {
    ($({
        $readable_name:literal
        $pascal_name:ident
        $upper_snake_name:ident
        $lower_snake_name:ident
        $lowercase_name:ident
        $uppercase_name:ident
        $rgb_hex:literal
        RGB($r:literal, $g:literal, $b:literal)
    })+) => {
        pub const HTML_COLORS: [ColorEntry; 140] = [
            $(
                ColorEntry::new(
                    $readable_name,
                    stringify!($pascal_name),
                    stringify!($upper_snake_name),
                    stringify!($lower_snake_name),
                    stringify!($lowercase_name),
                    stringify!($uppercase_name),
                    $rgb_hex,
                    Color::$pascal_name,
                    Rgb::new($r, $g, $b)
                ),
            )+
        ];
    };
}

html_colors!(color_table);