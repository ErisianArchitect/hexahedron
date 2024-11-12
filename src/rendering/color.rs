use bytemuck::NoUninit;
use std::slice::Iter;
use std::iter::Cloned;

// The html_colors macro is in colors.rs
// It was just really big and I didn't want it to clutter up this code.
include!("colors.rs");

const BYTE_TO_F32: [f32; 256] = [
    0e0,
    3.921569e-3,
    7.843138e-3,
    1.1764706e-2,
    1.5686275e-2,
    1.9607844e-2,
    2.3529412e-2,
    2.745098e-2,
    3.137255e-2,
    3.529412e-2,
    3.9215688e-2,
    4.3137256e-2,
    4.7058824e-2,
    5.0980393e-2,
    5.490196e-2,
    5.882353e-2,
    6.27451e-2,
    6.666667e-2,
    7.058824e-2,
    7.450981e-2,
    7.8431375e-2,
    8.235294e-2,
    8.627451e-2,
    9.019608e-2,
    9.411765e-2,
    9.803922e-2,
    1.01960786e-1,
    1.05882354e-1,
    1.0980392e-1,
    1.1372549e-1,
    1.1764706e-1,
    1.2156863e-1,
    1.254902e-1,
    1.2941177e-1,
    1.3333334e-1,
    1.3725491e-1,
    1.4117648e-1,
    1.4509805e-1,
    1.4901961e-1,
    1.5294118e-1,
    1.5686275e-1,
    1.6078432e-1,
    1.6470589e-1,
    1.6862746e-1,
    1.7254902e-1,
    1.764706e-1,
    1.8039216e-1,
    1.8431373e-1,
    1.882353e-1,
    1.9215687e-1,
    1.9607843e-1,
    2e-1,
    2.0392157e-1,
    2.0784314e-1,
    2.1176471e-1,
    2.1568628e-1,
    2.1960784e-1,
    2.2352941e-1,
    2.2745098e-1,
    2.3137255e-1,
    2.3529412e-1,
    2.3921569e-1,
    2.4313726e-1,
    2.4705882e-1,
    2.509804e-1,
    2.5490198e-1,
    2.5882354e-1,
    2.627451e-1,
    2.6666668e-1,
    2.7058825e-1,
    2.7450982e-1,
    2.784314e-1,
    2.8235295e-1,
    2.8627452e-1,
    2.901961e-1,
    2.9411766e-1,
    2.9803923e-1,
    3.019608e-1,
    3.0588236e-1,
    3.0980393e-1,
    3.137255e-1,
    3.1764707e-1,
    3.2156864e-1,
    3.254902e-1,
    3.2941177e-1,
    3.3333334e-1,
    3.372549e-1,
    3.4117648e-1,
    3.4509805e-1,
    3.4901962e-1,
    3.529412e-1,
    3.5686275e-1,
    3.6078432e-1,
    3.647059e-1,
    3.6862746e-1,
    3.7254903e-1,
    3.764706e-1,
    3.8039216e-1,
    3.8431373e-1,
    3.882353e-1,
    3.9215687e-1,
    3.9607844e-1,
    4e-1,
    4.0392157e-1,
    4.0784314e-1,
    4.117647e-1,
    4.1568628e-1,
    4.1960785e-1,
    4.2352942e-1,
    4.2745098e-1,
    4.3137255e-1,
    4.3529412e-1,
    4.392157e-1,
    4.4313726e-1,
    4.4705883e-1,
    4.509804e-1,
    4.5490196e-1,
    4.5882353e-1,
    4.627451e-1,
    4.6666667e-1,
    4.7058824e-1,
    4.745098e-1,
    4.7843137e-1,
    4.8235294e-1,
    4.862745e-1,
    4.9019608e-1,
    4.9411765e-1,
    4.9803922e-1,
    5.019608e-1,
    5.058824e-1,
    5.0980395e-1,
    5.137255e-1,
    5.176471e-1,
    5.2156866e-1,
    5.254902e-1,
    5.294118e-1,
    5.3333336e-1,
    5.372549e-1,
    5.411765e-1,
    5.4509807e-1,
    5.4901963e-1,
    5.529412e-1,
    5.568628e-1,
    5.6078434e-1,
    5.647059e-1,
    5.686275e-1,
    5.7254905e-1,
    5.764706e-1,
    5.803922e-1,
    5.8431375e-1,
    5.882353e-1,
    5.921569e-1,
    5.9607846e-1,
    6e-1,
    6.039216e-1,
    6.0784316e-1,
    6.117647e-1,
    6.156863e-1,
    6.1960787e-1,
    6.2352943e-1,
    6.27451e-1,
    6.313726e-1,
    6.3529414e-1,
    6.392157e-1,
    6.431373e-1,
    6.4705884e-1,
    6.509804e-1,
    6.54902e-1,
    6.5882355e-1,
    6.627451e-1,
    6.666667e-1,
    6.7058825e-1,
    6.745098e-1,
    6.784314e-1,
    6.8235296e-1,
    6.862745e-1,
    6.901961e-1,
    6.9411767e-1,
    6.9803923e-1,
    7.019608e-1,
    7.058824e-1,
    7.0980394e-1,
    7.137255e-1,
    7.176471e-1,
    7.2156864e-1,
    7.254902e-1,
    7.294118e-1,
    7.3333335e-1,
    7.372549e-1,
    7.411765e-1,
    7.4509805e-1,
    7.490196e-1,
    7.529412e-1,
    7.5686276e-1,
    7.607843e-1,
    7.647059e-1,
    7.6862746e-1,
    7.7254903e-1,
    7.764706e-1,
    7.8039217e-1,
    7.8431374e-1,
    7.882353e-1,
    7.921569e-1,
    7.9607844e-1,
    8e-1,
    8.039216e-1,
    8.0784315e-1,
    8.117647e-1,
    8.156863e-1,
    8.1960785e-1,
    8.235294e-1,
    8.27451e-1,
    8.3137256e-1,
    8.352941e-1,
    8.392157e-1,
    8.4313726e-1,
    8.4705883e-1,
    8.509804e-1,
    8.5490197e-1,
    8.5882354e-1,
    8.627451e-1,
    8.666667e-1,
    8.7058824e-1,
    8.745098e-1,
    8.784314e-1,
    8.8235295e-1,
    8.862745e-1,
    8.901961e-1,
    8.9411765e-1,
    8.980392e-1,
    9.019608e-1,
    9.0588236e-1,
    9.098039e-1,
    9.137255e-1,
    9.1764706e-1,
    9.2156863e-1,
    9.254902e-1,
    9.2941177e-1,
    9.3333334e-1,
    9.372549e-1,
    9.411765e-1,
    9.4509804e-1,
    9.490196e-1,
    9.529412e-1,
    9.5686275e-1,
    9.607843e-1,
    9.647059e-1,
    9.6862745e-1,
    9.72549e-1,
    9.764706e-1,
    9.8039216e-1,
    9.843137e-1,
    9.882353e-1,
    9.9215686e-1,
    9.9607843e-1,
    1e0,
];

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

// macro_rules! unwrap_or_return {
//     ($option:expr $(, $default:expr)?) => {
//         if let Some(value) = $option {
//             value
//         } else {
//             return $($default)?;
//         }
//     }
// }

// macro_rules! unwrap_or {
//     ($option:expr, $default:expr) => {
//         if let Some(value) = $option {
//             value
//         } else {
//             $default
//         }
//     }
// }

// macro_rules! expect {
//     ($option:expr $(, $msg:literal )?) => {
//         if let Some(value) = $option {
//             value
//         } else {
//             panic!($( $msg )?);
//         }
//     };
// }

// const HEX_BYTE: u8 = expect!(byte_from_hex("AF"));

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
        $camel_name:ident
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

    /// Gets the `CamelCase` name.
    #[inline]
    pub const fn camel_name(self) -> &'static str {
        HTML_COLORS[self.index()].camel_name
    }

    /// Gets the `UPPER_SNAKE_CASE` name.
    #[inline]
    pub const fn const_name(self) -> &'static str {
        HTML_COLORS[self.index()].upper_snake
    }

    /// Gets the `snake_case` name.
    #[inline]
    pub const fn var_name(self) -> &'static str {
        HTML_COLORS[self.index()].lower_snake
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
        let index = match get_name_case(name) {
            NameCase::Unknown => None,
            NameCase::Readable => search!(readable_name),
            NameCase::Camel => search!(camel_name),
            NameCase::LowerSnake => search!(lower_snake),
            NameCase::UpperSnake => search!(upper_snake),
            NameCase::Lowercase => search!(lower_name),
            NameCase::Uppercase => search!(upper_name),
        };
        let Some(index) = index else {
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
        $camel_name:ident
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
        camel_name: &'static str,
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
            camel_name,
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
    pub const fn string_memory(&self) -> usize {
        self.readable_name.len() +
        self.camel_name.len() +
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
    ($([
        $readable_name:literal
        $camel_name:ident
        $const_name:ident
        $var_name:ident
        $lower_name:ident
        $upper_name:ident
        $rgb_hex:literal
        RGB($r:literal, $g:literal, $b:literal)
    ])+) => {
        pub const HTML_COLORS: [ColorEntry; 140] = [
            $(
                ColorEntry::new(
                    $readable_name,
                    stringify!($camel_name),
                    stringify!($const_name),
                    stringify!($var_name),
                    stringify!($lower_name),
                    stringify!($upper_name),
                    $rgb_hex,
                    Color::$camel_name,
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
    Camel,
    LowerSnake,
    UpperSnake,
    Lowercase,
    Uppercase,
}

const fn get_case_recursive(name: &[u8], case_flags: CaseFlags, index: usize) -> Option<CaseFlags> {
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
    get_case_recursive(name, case_flags, index + 1)
}

const fn get_name_case(name: &str) -> NameCase {
    let case_flags = get_case_recursive(name.as_bytes(), CaseFlags {
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
        (false, false, true, true) => NameCase::Camel,
        _ => NameCase::Unknown,
    }
}

const fn cmp_byte(a: u8, b: u8) -> std::cmp::Ordering {
    if a < b {
        std::cmp::Ordering::Less
    } else if a > b {
        std::cmp::Ordering::Greater
    } else {
        std::cmp::Ordering::Equal
    }
}

const fn recursive_cmp_next(a: &[u8], b: &[u8], index: usize) -> std::cmp::Ordering {
    use std::cmp::Ordering;
    match (a.len() == index, b.len() == index) {
        (true, true) => Ordering::Equal,
        (false, true) => Ordering::Greater,
        (true, false) => Ordering::Less,
        (false, false) => match cmp_byte(a[index], b[index]) {
            Ordering::Equal => recursive_cmp_next(a, b, index + 1),
            cmp => cmp,
        }
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
            entry.camel_name.len() +
            entry.upper_snake.len() +
            entry.lower_snake.len() +
            entry.lower_name.len() +
            entry.upper_name.len() +
            entry.hex.len()
        }).sum::<usize>() + std::mem::size_of_val(&HTML_COLORS);
        println!("Memory: {memory}");
        // let color = find_color("cornflower_blue").expect("Failed to find color.");
        // println!("{color}");
        macro_rules! unwrap {
            ($option:expr) => {
                if let Some(value) = $option {
                    value
                } else {
                    panic!("Failed to unwrap.");
                }
            };
        }
        // const COLOR: Rgb = unwrap!(Rgb::from_name("Light Blue"));
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