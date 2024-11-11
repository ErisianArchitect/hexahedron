use bytemuck::NoUninit;
use std::slice::Iter;
use std::iter::Cloned;

// The html_colors macro is in colors.rs
// It was just really big and I didn't want it to clutter up this code.
include!("colors.rs");

#[inline]
pub const fn hex_byte(byte: u8) -> [u8; 2] {
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

#[inline]
fn byte_lerp(a: u8, b: u8, t: f32) -> u8 {
    if a == b {
        return a;
    }
    let a = a as f32;
    let b = b as f32;
    let result = a + (b - a) * t;
    result as u8
}

#[inline]
pub fn normalized_byte(byte: u8) -> f32 {
    byte as f32 / 255.0
}

macro_rules! color_enum {
    ($([
        $readable_name:literal
        $camel_name:ident
        $const_name:ident
        $var_name:ident
        $rgb_hex:literal
        $rgba_hex:literal
        RGB($r:literal, $g:literal, $b:literal)
    ])+) => {
        #[repr(u8)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, NoUninit)]
        pub enum Color {
            $(
                $camel_name,
            )*
        }

        impl Color {
            const SORTED_COLORS: [Color; 140] = [
                $(
                    Color::$camel_name,
                )*
            ];
        }
    };
}

impl Color {
    #[inline]
    const fn index(self) -> usize {
        self as usize
    }

    /// Gets the name in a `Human Readable Format`.
    #[inline]
    pub const fn readable_name(self) -> &'static str {
        HTML_COLORS[self.index()].readable_name
    }

    /// Gets the `CamelCase` name.
    #[inline]
    pub const fn camel_name(self) -> &'static str {
        HTML_COLORS[self.index()].camel_name
    }

    /// Gets the `UPPER_SNAKE_CASE` name.
    #[inline]
    pub const fn const_name(self) -> &'static str {
        HTML_COLORS[self.index()].const_name
    }

    /// Gets the `snake_case` name.
    #[inline]
    pub const fn var_name(self) -> &'static str {
        HTML_COLORS[self.index()].var_name
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

    #[inline]
    pub const fn transparent(self) -> Rgba {
        self.rgb().transparent()
    }

    #[inline]
    pub const fn opaque(self) -> Rgba {
        self.rgb().rgba()
    }

    /// Gets [Rgba] value.
    #[inline]
    pub const fn rgba(self) -> Rgba {
        self.opaque()
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
}

html_colors!(color_enum);

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, NoUninit)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

macro_rules! rgb_color_consts {
    ($([
        $readable_name:literal
        $camel_name:ident
        $const_name:ident
        $var_name:ident
        $rgb_hex:literal
        $rgba_hex:literal
        RGB($r:literal, $g:literal, $b:literal)
    ])+) => {
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
    pub const fn from_tuple((r,g,b): (u8, u8, u8)) -> Self {
        Self{r,g,b}
    }

    #[inline]
    pub const fn to_bytes(self) -> [u8; 3] {
        [self.r, self.g, self.b]
    }

    #[inline]
    pub const fn to_tuple(self) -> (u8, u8, u8) {
        (self.r, self.g, self.b)
    }

    #[inline]
    pub fn to_vec3(self) -> glam::Vec3 {
        glam::Vec3::new(
            normalized_byte(self.r),
            normalized_byte(self.g),
            normalized_byte(self.b),
        )
    }

    pub fn hex(self) -> String {
        let mut hex = String::with_capacity(6);
        let r = hex_byte(self.r);
        let g = hex_byte(self.g);
        let b = hex_byte(self.b);
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

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, NoUninit)]
pub struct Rgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8
}

macro_rules! rgba_color_consts {
    ($([
        $readable_name:literal
        $camel_name:ident
        $const_name:ident
        $var_name:ident
        $rgb_hex:literal
        $rgba_hex:literal
        RGB($r:literal, $g:literal, $b:literal)
    ])+) => {
        impl Rgba {
            $(
                pub const $const_name: Self = Rgba::new($r, $g, $b, 255);
            )+
        }
    };
}

html_colors!(rgba_color_consts);

impl Rgba {
    pub const     TRANSPARENTWHITE: Self = Rgba::WHITE.transparent();
    pub const     TRANSPARENTBLACK: Self = Rgba::BLACK.transparent();
    
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
    pub const fn from_tuple((r,g,b,a): (u8, u8, u8, u8)) -> Self {
        Self{r,g,b,a}
    }

    #[inline]
    pub const fn to_bytes(self) -> [u8; 4] {
        [self.r, self.g, self.b, self.a]
    }

    #[inline]
    pub const fn to_tuple(self) -> (u8, u8, u8, u8) {
        (self.r, self.g, self.b, self.a)
    }

    #[inline]
    pub fn to_vec4(self) -> glam::Vec4 {
        glam::Vec4::new(
            normalized_byte(self.r),
            normalized_byte(self.g),
            normalized_byte(self.b),
            normalized_byte(self.a),
        )
    }

    pub fn hex(self) -> String {
        let mut hex = String::with_capacity(8);
        let r = hex_byte(self.r);
        let g = hex_byte(self.g);
        let b = hex_byte(self.b);
        let a = hex_byte(self.a);
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

impl From<Rgb> for Rgba {
    #[inline]
    fn from(value: Rgb) -> Self {
        value.rgba()
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
        write!(f, "Color::{}", self.camel_name())
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
    pub camel_name: &'static str,
    pub const_name: &'static str,
    pub var_name: &'static str,
    pub hex: &'static str,
    pub color: Color,
    pub rgb: Rgb,
}

impl ColorEntry {
    pub const fn new(
        readable_name: &'static str,
        camel_name: &'static str,
        const_name: &'static str,
        var_name: &'static str,
        hex: &'static str,
        color: Color,
        rgb: Rgb
    ) -> Self {
        Self {
            readable_name,
            camel_name,
            const_name,
            var_name,
            hex,
            color,
            rgb,
        }
    }
}

macro_rules! color_table {
    ($([
        $readable_name:literal
        $camel_name:ident
        $const_name:ident
        $var_name:ident
        $rgb_hex:literal
        $rgba_hex:literal
        RGB($r:literal, $g:literal, $b:literal)
    ])+) => {
        pub const HTML_COLORS: [ColorEntry; 140] = [
            $(
                ColorEntry::new(
                    $readable_name,
                    stringify!($camel_name),
                    stringify!($const_name),
                    stringify!($var_name),
                    $rgb_hex,
                    Color::$camel_name,
                    Rgb::new($r, $g, $b)
                ),
            )+
        ];
    };
}

html_colors!(color_table);

enum NameCase {
    Unknown,
    Readable,
    Camel,
    LowerSnake,
    UpperSnake,
}

fn get_name_case(name: &str) -> NameCase {
    // has lowercase letter
    let mut has_lower = false;
    // has uppercase letter
    let mut has_upper = false;
    // has underscore
    let mut has_under = false;
    // has space
    let mut has_space = false;
    for chr in name.chars() {
        match chr {
            ' ' => {
                has_space = true;
            }
            '_' => {
                has_under = true;
            }
            'a'..='z' => {
                has_lower = true;
            }
            'A'..='Z' => {
                has_upper = true;
            }
            _ => {
                return NameCase::Unknown;
            }
        }
    }
    match (has_space, has_under, has_lower, has_upper) {
        (true, false, true, true) => NameCase::Readable,
        (false, false, true, true) => NameCase::Camel,
        (false, _, true, false) => NameCase::LowerSnake,
        (false, _, false, true) => NameCase::UpperSnake,
        _ => NameCase::Unknown,
    }
}

pub fn find_color(name: &str) -> Option<Color> {
    Some(Color::SORTED_COLORS[match get_name_case(name) {
        NameCase::Unknown => return None,
        NameCase::Readable => HTML_COLORS.binary_search_by_key(&name, |entry| entry.readable_name).ok()?,
        NameCase::Camel => HTML_COLORS.binary_search_by_key(&name, |entry| entry.camel_name).ok()?,
        NameCase::LowerSnake => HTML_COLORS.binary_search_by_key(&name, |entry| entry.var_name).ok()?,
        NameCase::UpperSnake => HTML_COLORS.binary_search_by_key(&name, |entry| entry.const_name).ok()?,
    }])
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

#[cfg(test)]
mod testing_sandbox {
    // TODO: Remove this sandbox when it is no longer in use.
    use super::*;
    #[test]
    fn sandbox() {
        // const CB: Rgba = Color::CornflowerBlue.rgb().transparent();
        let _memory = HTML_COLORS.iter().map(|entry| {
            entry.camel_name.len() +
            entry.const_name.len() +
            entry.hex.len() +
            entry.readable_name.len() +
            entry.var_name.len()
        }).sum::<usize>() + std::mem::size_of_val(&HTML_COLORS);
        let color = find_color("cornflower_blue").expect("Failed to find color.");
        println!("{color}");
        // println!("Memory: {memory}");
        // println!("{}", std::mem::size_of::<[u8; 4]>());
        // println!("\"{}\" = #{}", Color::CornflowerBlue.readable_name(), Color::CornflowerBlue.hex());
        // println!("{}", Color::Black == Rgb::gray(0));
    }
}