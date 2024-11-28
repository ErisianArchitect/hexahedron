use bytemuck::NoUninit;
use std::slice::Iter;
use std::iter::Cloned;

// The html_colors macro is in colors.rs
// It was just really big and I didn't want it to clutter up this code.
include!("colors.rs");
// This is the table that converts u8 to f32 as a normalized value between 0 and 1 where 0u8 is 0 and 255u8 is 1.
include!("byte_to_f32.rs");

#[inline]
const fn hex_byte(byte: u8) -> [u8; 2] {
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
        b'a'..=b'f' => b'W',
        b'A'..=b'F' => b'7',
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
const fn normalized_byte(byte: u8) -> f32 {
    BYTE_TO_F32[byte as usize]
}

macro_rules! color_enum {
    ($([
        $readable_name:literal
        $pascal_name:ident
        $const_name:ident
        $var_name:ident
        $lower_name:ident
        $upper_name:ident
        $rgb_hex:literal
        RGB($r:literal, $g:literal, $b:literal)
    ])+) => {
        #[repr(u8)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, NoUninit)]
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

    pub const fn from_name(name: &str) -> Option<Color> {
        macro_rules! search {
            ($field:ident) => {
                recursive_color_binary_search(name, 0, HTML_COLORS.len(), std::mem::offset_of!(ColorEntry, $field))
            };
        }
        let Some(index) = (match get_name_case(name) {
            NameCase::Unknown => None,
            NameCase::Readable => search!(readable_name),
            NameCase::Pascal => search!(pascal_name),
            NameCase::LowerSnake => search!(lower_snake),
            NameCase::UpperSnake => search!(upper_snake),
            NameCase::Lowercase => search!(lower_name),
            NameCase::Uppercase => search!(upper_name),
        }) else {
            return None;
        };
        Some(Color::SORTED_COLORS[index])
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
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, NoUninit)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

macro_rules! rgb_color_consts {
    ($([
        $readable_name:literal
        $pascal_name:ident
        $const_name:ident
        $var_name:ident
        $lower_name:ident
        $upper_name:ident
        $rgb_hex:literal
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
    pub const fn as_bytes(&self) -> &[u8; 3] {
        unsafe {
            &*(self as *const Self).cast()
        }
    }

    #[inline]
    pub fn as_mut_bytes(&mut self) -> &mut [u8; 3] {
        unsafe {
            &mut *(self as *mut Self).cast()
        }
    }

    #[inline]
    pub const fn to_tuple(self) -> (u8, u8, u8) {
        (self.r, self.g, self.b)
    }

    #[inline]
    pub const fn to_vec3(self) -> glam::Vec3 {
        glam::Vec3::new(
            normalized_byte(self.r),
            normalized_byte(self.g),
            normalized_byte(self.b),
        )
    }

    #[inline]
    pub const fn to_vec4(self) -> glam::Vec4 {
        glam::Vec4::new(
            normalized_byte(self.r),
            normalized_byte(self.g),
            normalized_byte(self.b),
            1.0,
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

    pub const fn from_name(name: &str) -> Option<Self> {
        let Some(color) = Color::from_name(name) else {
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
        $pascal_name:ident
        $const_name:ident
        $var_name:ident
        $lower_name:ident
        $upper_name:ident
        $rgb_hex:literal
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
    pub const fn as_bytes(&self) -> &[u8; 4] {
        unsafe {
            &*(self as *const Self).cast()
        }
    }

    #[inline]
    pub fn as_mut_bytes(&mut self) -> &mut [u8; 4] {
        unsafe {
            &mut *(self as *mut Self).cast()
        }
    }

    #[inline]
    pub const fn to_tuple(self) -> (u8, u8, u8, u8) {
        (self.r, self.g, self.b, self.a)
    }

    #[inline]
    pub const fn to_vec4(self) -> glam::Vec4 {
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

    pub const fn from_name(name: &str) -> Option<Self> {
        let Some(color) = Color::from_name(name) else {
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
        let g = normalized_byte(self.level);
        glam::Vec3::new(
            g,
            g,
            g,
        )
    }

    #[inline]
    pub const fn to_vec4(self) -> glam::Vec4 {
        let g = normalized_byte(self.level);
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

impl ColorEntry {
    #[cfg(debug_assertions)]
    pub const fn string_memory(&self) -> usize {
        self.readable_name.len() +
        self.pascal_name.len() +
        self.upper_snake.len() +
        self.lower_snake.len() +
        self.lower_name.len() +
        self.upper_name.len() +
        self.hex.len()
    }

    #[cfg(debug_assertions)]
    pub const fn memory_usage(&self) -> usize {
        self.string_memory() + std::mem::size_of::<Self>()
    }
}

macro_rules! color_table {
    ($([
        $readable_name:literal
        $pascal_name:ident
        $upper_snake_name:ident
        $lower_snake_name:ident
        $lowercase_name:ident
        $uppercase_name:ident
        $rgb_hex:literal
        RGB($r:literal, $g:literal, $b:literal)
    ])+) => {
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

struct CaseFlags {
    has_space: bool,
    has_under: bool,
    has_lower: bool,
    has_upper: bool,
}

impl CaseFlags {
    const fn mark_space(self) -> Self {
        Self {
            has_space: true,
            has_under: self.has_under,
            has_lower: self.has_lower,
            has_upper: self.has_upper,
        }
    }

    const fn mark_under(self) -> Self {
        Self {
            has_space: self.has_space,
            has_under: true,
            has_lower: self.has_lower,
            has_upper: self.has_upper,
        }
    }

    const fn mark_lower(self) -> Self {
        Self {
            has_space: self.has_space,
            has_under: self.has_under,
            has_lower: true,
            has_upper: self.has_upper,
        }
    }

    const fn mark_upper(self) -> Self {
        Self {
            has_space: self.has_space,
            has_under: self.has_under,
            has_lower: self.has_lower,
            has_upper: true,
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum NameCase {
    #[default]
    Unknown,
    Readable,
    Pascal,
    LowerSnake,
    UpperSnake,
    Lowercase,
    Uppercase,
}

const fn recursive_get_case(name: &[u8], case_flags: CaseFlags, index: usize) -> Option<CaseFlags> {
    if index >= name.len() {
        return Some(case_flags);
    }
    let case_flags = match name[index] {
        b' ' => case_flags.mark_space(),
        b'_' => case_flags.mark_under(),
        b'a'..=b'z' => case_flags.mark_lower(),
        b'A'..=b'Z' => case_flags.mark_upper(),
        _ => return None,
    };
    recursive_get_case(name, case_flags, index + 1)
}

const fn get_name_case(name: &str) -> NameCase {
    let case_flags = recursive_get_case(name.as_bytes(), CaseFlags {
        has_space: false,
        has_under: false,
        has_lower: false,
        has_upper: false,
    }, 0);
    let Some(case_flags) = case_flags else {
        return NameCase::Unknown
    };
    match (
        case_flags.has_space,
        case_flags.has_under,
        case_flags.has_lower,
        case_flags.has_upper
    ) {
        (true, false, true, true) => NameCase::Readable,
        (false, false, true, false) => NameCase::Lowercase,
        (false, true, true, false) => NameCase::LowerSnake,
        (false, false, false, true) => NameCase::Uppercase,
        (false, true, false, true) => NameCase::UpperSnake,
        (false, false, true, true) => NameCase::Pascal,
        _ => NameCase::Unknown,
    }
}

const fn cmp_byte(a: u8, b: u8) -> std::cmp::Ordering {
    use std::cmp::Ordering;
    if a < b {
        Ordering::Less
    } else if a > b {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}

const fn recursive_cmp_next(a: &[u8], b: &[u8], index: usize) -> std::cmp::Ordering {
    use std::cmp::Ordering;
    match (a.len() == index, b.len() == index) {
        (false, false) => match cmp_byte(a[index], b[index]) {
            Ordering::Equal => recursive_cmp_next(a, b, index + 1),
            cmp => cmp,
        }
        (false, true) => Ordering::Greater,
        (true, false) => Ordering::Less,
        (true, true) => Ordering::Equal,
    }
}

const fn const_str_cmp_ascii(a: &str, b: &str) -> std::cmp::Ordering {
    recursive_cmp_next(a.as_bytes(), b.as_bytes(), 0)
}

const fn recursive_color_binary_search(name: &str, low: usize, high: usize, field_offset: usize) -> Option<usize> {
    use std::cmp::Ordering;
    if low == high {
        return None;
    }
    let mid = low + (high - low) / 2;
    let field = unsafe {
        let ptr = &HTML_COLORS[mid] as *const ColorEntry;
        let ptr = ptr.byte_add(field_offset).cast::<&'static str>();
        *ptr as &'static str
    };
    match const_str_cmp_ascii(name, field) {
        Ordering::Equal => Some(mid),
        Ordering::Less => recursive_color_binary_search(name, low, mid, field_offset),
        Ordering::Greater => recursive_color_binary_search(name, mid + 1, high, field_offset),
    }
}

#[cfg(test)]
mod testing_sandbox {
    // TODO: Remove this sandbox when it is no longer in use.
    use super::*;
    
    #[test]
    fn sandbox() {
        
        // const CB: Rgba = Color::CornflowerBlue.rgb().transparent();
        let memory = HTML_COLORS.iter().map(|entry| {
            entry.readable_name.len() +
            entry.pascal_name.len() +
            entry.upper_snake.len() +
            entry.lower_snake.len() +
            entry.lower_name.len() +
            entry.upper_name.len() +
            entry.hex.len()
        }).sum::<usize>() + std::mem::size_of_val(&HTML_COLORS);
        println!("Memory: {memory}");
        // let color = find_color("cornflower_blue").expect("Failed to find color.");
        // println!("{color}");
        // println!("Color: {COLOR}");
        // let bytes = COLOR.as_bytes();
        // println!("{bytes:#?}");
        // println!("Memory: {memory}");
        // println!("{}", std::mem::size_of::<[u8; 4]>());
        // println!("\"{}\" = #{}", Color::CornflowerBlue.readable_name(), Color::CornflowerBlue.hex());
        // println!("{}", Color::Black == Rgb::gray(0));
        let colors = [
            Rgb::BLACK,
            Rgb::WHITE,
            Rgb::RED,
            Rgb::LIME,
            Rgb::BLUE,
        ];
        let buf = unsafe {
            let ptr: *const u8 = (&colors as *const Rgb).cast();
            std::slice::from_raw_parts(ptr, colors.len() * std::mem::size_of::<Rgb>())
        };
        println!("{}", buf.len());
    }
}