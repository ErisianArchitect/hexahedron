use bytemuck::NoUninit;

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, NoUninit)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[inline]
pub fn normalized_byte(byte: u8) -> f32 {
    byte as f32 / 255.0
}

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

#[inline]
pub const fn hex_byte(byte: u8) -> [u8; 2] {
    [
        HEX_CHARS[(byte >> 4) as usize],
        HEX_CHARS[(byte & 0xF) as usize],
    ]
}

impl Rgb {
    /// `RGB(240, 248, 255)`  
    /// `#F0F8FF`
    pub const            ALICEBLUE: Self = Rgb::new(240, 248, 255);
    /// `RGB(250, 235, 215)`  
    /// `#FAEBD7`
    pub const         ANTIQUEWHITE: Self = Rgb::new(250, 235, 215);
    /// `RGB(0, 255, 255)`  
    /// `#00FFFF`
    pub const                 AQUA: Self = Rgb::new(  0, 255, 255);
    /// `RGB(127, 255, 212)`  
    /// `#7FFFD4`
    pub const           AQUAMARINE: Self = Rgb::new(127, 255, 212);
    /// `RGB(240, 255, 255)`  
    /// `#F0FFFF`
    pub const                AZURE: Self = Rgb::new(240, 255, 255);
    /// `RGB(245, 245, 220)`  
    /// `#F5F5DC`
    pub const                BEIGE: Self = Rgb::new(245, 245, 220);
    /// `RGB(255, 228, 196)`  
    /// `#FFE4C4`
    pub const               BISQUE: Self = Rgb::new(255, 228, 196);
    /// `RGB(0, 0, 0)`  
    /// `#000000`
    pub const                BLACK: Self = Rgb::new(  0,   0,   0);
    /// `RGB(255, 235, 205)`  
    /// `#FFEBCD`
    pub const       BLANCHEDALMOND: Self = Rgb::new(255, 235, 205);
    /// `RGB(0, 0, 255)`  
    /// `#0000FF`
    pub const                 BLUE: Self = Rgb::new(  0,   0, 255);
    /// `RGB(138, 43, 226)`  
    /// `#8A2BE2`
    pub const           BLUEVIOLET: Self = Rgb::new(138,  43, 226);
    /// `RGB(165, 42, 42)`  
    /// `#A52A2A`
    pub const                BROWN: Self = Rgb::new(165,  42,  42);
    /// `RGB(222, 184, 135)`  
    /// `#DEB887`
    pub const            BURLYWOOD: Self = Rgb::new(222, 184, 135);
    /// `RGB(95, 158, 160)`  
    /// `#5F9EA0`
    pub const            CADETBLUE: Self = Rgb::new( 95, 158, 160);
    /// `RGB(127, 255, 0)`  
    /// `#7FFF00`
    pub const           CHARTREUSE: Self = Rgb::new(127, 255,   0);
    /// `RGB(210, 105, 30)`  
    /// `#D2691E`
    pub const            CHOCOLATE: Self = Rgb::new(210, 105,  30);
    /// `RGB(255, 127, 80)`  
    /// `#FF7F50`
    pub const                CORAL: Self = Rgb::new(255, 127,  80);
    /// `RGB(100, 149, 237)`  
    /// `#6495ED`
    pub const       CORNFLOWERBLUE: Self = Rgb::new(100, 149, 237);
    /// `RGB(255, 248, 220)`  
    /// `#FFF8DC`
    pub const             CORNSILK: Self = Rgb::new(255, 248, 220);
    /// `RGB(220, 20, 60)`  
    /// `#DC143C`
    pub const              CRIMSON: Self = Rgb::new(220,  20,  60);
    /// `RGB(0, 255, 255)`  
    /// `#00FFFF`
    pub const                 CYAN: Self = Rgb::new(  0, 255, 255);
    /// `RGB(0, 0, 139)`  
    /// `#00008B`
    pub const             DARKBLUE: Self = Rgb::new(  0,   0, 139);
    /// `RGB(0, 139, 139)`  
    /// `#008B8B`
    pub const             DARKCYAN: Self = Rgb::new(  0, 139, 139);
    /// `RGB(184, 134, 11)`  
    /// `#B8860B`
    pub const        DARKGOLDENROD: Self = Rgb::new(184, 134,  11);
    /// `RGB(169, 169, 169)`  
    /// `#A9A9A9`
    pub const             DARKGRAY: Self = Rgb::new(169, 169, 169);
    /// `RGB(0, 100, 0)`  
    /// `#006400`
    pub const            DARKGREEN: Self = Rgb::new(  0, 100,   0);
    /// `RGB(189, 183, 107)`  
    /// `#BDB76B`
    pub const            DARKKHAKI: Self = Rgb::new(189, 183, 107);
    /// `RGB(139, 0, 139)`  
    /// `#8B008B`
    pub const          DARKMAGENTA: Self = Rgb::new(139,   0, 139);
    /// `RGB(85, 107, 47)`  
    /// `#556B2F`
    pub const       DARKOLIVEGREEN: Self = Rgb::new( 85, 107,  47);
    /// `RGB(255, 140, 0)`  
    /// `#FF8C00`
    pub const           DARKORANGE: Self = Rgb::new(255, 140,   0);
    /// `RGB(153, 50, 204)`  
    /// `#9932CC`
    pub const           DARKORCHID: Self = Rgb::new(153,  50, 204);
    /// `RGB(139, 0, 0)`  
    /// `#8B0000`
    pub const              DARKRED: Self = Rgb::new(139,   0,   0);
    /// `RGB(233, 150, 122)`  
    /// `#E9967A`
    pub const           DARKSALMON: Self = Rgb::new(233, 150, 122);
    /// `RGB(143, 188, 143)`  
    /// `#8FBC8F`
    pub const         DARKSEAGREEN: Self = Rgb::new(143, 188, 143);
    /// `RGB(72, 61, 139)`  
    /// `#483D8B`
    pub const        DARKSLATEBLUE: Self = Rgb::new( 72,  61, 139);
    /// `RGB(47, 79, 79)`  
    /// `#2F4F4F`
    pub const        DARKSLATEGRAY: Self = Rgb::new( 47,  79,  79);
    /// `RGB(0, 206, 209)`  
    /// `#00CED1`
    pub const        DARKTURQUOISE: Self = Rgb::new(  0, 206, 209);
    /// `RGB(148, 0, 211)`  
    /// `#9400D3`
    pub const           DARKVIOLET: Self = Rgb::new(148,   0, 211);
    /// `RGB(255, 20, 147)`  
    /// `#FF1493`
    pub const             DEEPPINK: Self = Rgb::new(255,  20, 147);
    /// `RGB(0, 191, 255)`  
    /// `#00BFFF`
    pub const          DEEPSKYBLUE: Self = Rgb::new(  0, 191, 255);
    /// `RGB(105, 105, 105)`  
    /// `#696969`
    pub const              DIMGRAY: Self = Rgb::new(105, 105, 105);
    /// `RGB(30, 144, 255)`  
    /// `#1E90FF`
    pub const           DODGERBLUE: Self = Rgb::new( 30, 144, 255);
    /// `RGB(178, 34, 34)`  
    /// `#B22222`
    pub const            FIREBRICK: Self = Rgb::new(178,  34,  34);
    /// `RGB(255, 250, 240)`  
    /// `#FFFAF0`
    pub const          FLORALWHITE: Self = Rgb::new(255, 250, 240);
    /// `RGB(34, 139, 34)`  
    /// `#228B22`
    pub const          FORESTGREEN: Self = Rgb::new( 34, 139,  34);
    /// `RGB(255, 0, 255)`  
    /// `#FF00FF`
    pub const              FUCHSIA: Self = Rgb::new(255,   0, 255);
    /// `RGB(220, 220, 220)`  
    /// `#DCDCDC`
    pub const            GAINSBORO: Self = Rgb::new(220, 220, 220);
    /// `RGB(248, 248, 255)`  
    /// `#F8F8FF`
    pub const           GHOSTWHITE: Self = Rgb::new(248, 248, 255);
    /// `RGB(255, 215, 0)`  
    /// `#FFD700`
    pub const                 GOLD: Self = Rgb::new(255, 215,   0);
    /// `RGB(218, 165, 32)`  
    /// `#DAA520`
    pub const            GOLDENROD: Self = Rgb::new(218, 165,  32);
    /// `RGB(128, 128, 128)`  
    /// `#808080`
    pub const                 GRAY: Self = Rgb::new(128, 128, 128);
    /// `RGB(0, 128, 0)`  
    /// `#008000`
    pub const                GREEN: Self = Rgb::new(  0, 128,   0);
    /// `RGB(173, 255, 47)`  
    /// `#ADFF2F`
    pub const          GREENYELLOW: Self = Rgb::new(173, 255,  47);
    /// `RGB(240, 255, 240)`  
    /// `#F0FFF0`
    pub const             HONEYDEW: Self = Rgb::new(240, 255, 240);
    /// `RGB(255, 105, 180)`  
    /// `#FF69B4`
    pub const              HOTPINK: Self = Rgb::new(255, 105, 180);
    /// `RGB(205, 92, 92)`  
    /// `#CD5C5C`
    pub const            INDIANRED: Self = Rgb::new(205,  92,  92);
    /// `RGB(75, 0, 130)`  
    /// `#4B0082`
    pub const               INDIGO: Self = Rgb::new( 75,   0, 130);
    /// `RGB(255, 255, 240)`  
    /// `#FFFFF0`
    pub const                IVORY: Self = Rgb::new(255, 255, 240);
    /// `RGB(240, 230, 140)`  
    /// `#F0E68C`
    pub const                KHAKI: Self = Rgb::new(240, 230, 140);
    /// `RGB(230, 230, 250)`  
    /// `#E6E6FA`
    pub const             LAVENDER: Self = Rgb::new(230, 230, 250);
    /// `RGB(255, 240, 245)`  
    /// `#FFF0F5`
    pub const        LAVENDERBLUSH: Self = Rgb::new(255, 240, 245);
    /// `RGB(124, 252, 0)`  
    /// `#7CFC00`
    pub const            LAWNGREEN: Self = Rgb::new(124, 252,   0);
    /// `RGB(255, 250, 205)`  
    /// `#FFFACD`
    pub const         LEMONCHIFFON: Self = Rgb::new(255, 250, 205);
    /// `RGB(173, 216, 230)`  
    /// `#ADD8E6`
    pub const            LIGHTBLUE: Self = Rgb::new(173, 216, 230);
    /// `RGB(240, 128, 128)`  
    /// `#F08080`
    pub const           LIGHTCORAL: Self = Rgb::new(240, 128, 128);
    /// `RGB(224, 255, 255)`  
    /// `#E0FFFF`
    pub const            LIGHTCYAN: Self = Rgb::new(224, 255, 255);
    /// `RGB(250, 250, 210)`  
    /// `#FAFAD2`
    pub const LIGHTGOLDENRODYELLOW: Self = Rgb::new(250, 250, 210);
    /// `RGB(211, 211, 211)`  
    /// `#D3D3D3`
    pub const            LIGHTGRAY: Self = Rgb::new(211, 211, 211);
    /// `RGB(144, 238, 144)`  
    /// `#90EE90`
    pub const           LIGHTGREEN: Self = Rgb::new(144, 238, 144);
    /// `RGB(255, 182, 193)`  
    /// `#FFB6C1`
    pub const            LIGHTPINK: Self = Rgb::new(255, 182, 193);
    /// `RGB(255, 160, 122)`  
    /// `#FFA07A`
    pub const          LIGHTSALMON: Self = Rgb::new(255, 160, 122);
    /// `RGB(32, 178, 170)`  
    /// `#20B2AA`
    pub const        LIGHTSEAGREEN: Self = Rgb::new( 32, 178, 170);
    /// `RGB(135, 206, 250)`  
    /// `#87CEFA`
    pub const         LIGHTSKYBLUE: Self = Rgb::new(135, 206, 250);
    /// `RGB(119, 136, 153)`  
    /// `#778899`
    pub const       LIGHTSLATEGRAY: Self = Rgb::new(119, 136, 153);
    /// `RGB(176, 196, 222)`  
    /// `#B0C4DE`
    pub const       LIGHTSTEELBLUE: Self = Rgb::new(176, 196, 222);
    /// `RGB(255, 255, 224)`  
    /// `#FFFFE0`
    pub const          LIGHTYELLOW: Self = Rgb::new(255, 255, 224);
    /// `RGB(0, 255, 0)`  
    /// `#00FF00`
    pub const                 LIME: Self = Rgb::new(  0, 255,   0);
    /// `RGB(50, 205, 50)`  
    /// `#32CD32`
    pub const            LIMEGREEN: Self = Rgb::new( 50, 205,  50);
    /// `RGB(250, 240, 230)`  
    /// `#FAF0E6`
    pub const                LINEN: Self = Rgb::new(250, 240, 230);
    /// `RGB(255, 0, 255)`  
    /// `#FF00FF`
    pub const              MAGENTA: Self = Rgb::new(255,   0, 255);
    /// `RGB(128, 0, 0)`  
    /// `#800000`
    pub const               MAROON: Self = Rgb::new(128,   0,   0);
    /// `RGB(102, 205, 170)`  
    /// `#66CDAA`
    pub const     MEDIUMAQUAMARINE: Self = Rgb::new(102, 205, 170);
    /// `RGB(0, 0, 205)`  
    /// `#0000CD`
    pub const           MEDIUMBLUE: Self = Rgb::new(  0,   0, 205);
    /// `RGB(186, 85, 211)`  
    /// `#BA55D3`
    pub const         MEDIUMORCHID: Self = Rgb::new(186,  85, 211);
    /// `RGB(147, 112, 219)`  
    /// `#9370DB`
    pub const         MEDIUMPURPLE: Self = Rgb::new(147, 112, 219);
    /// `RGB(60, 179, 113)`  
    /// `#3CB371`
    pub const       MEDIUMSEAGREEN: Self = Rgb::new( 60, 179, 113);
    /// `RGB(123, 104, 238)`  
    /// `#7B68EE`
    pub const      MEDIUMSLATEBLUE: Self = Rgb::new(123, 104, 238);
    /// `RGB(0, 250, 154)`  
    /// `#00FA9A`
    pub const    MEDIUMSPRINGGREEN: Self = Rgb::new(  0, 250, 154);
    /// `RGB(72, 209, 204)`  
    /// `#48D1CC`
    pub const      MEDIUMTURQUOISE: Self = Rgb::new( 72, 209, 204);
    /// `RGB(199, 21, 133)`  
    /// `#C71585`
    pub const      MEDIUMVIOLETRED: Self = Rgb::new(199,  21, 133);
    /// `RGB(25, 25, 112)`  
    /// `#191970`
    pub const         MIDNIGHTBLUE: Self = Rgb::new( 25,  25, 112);
    /// `RGB(245, 255, 250)`  
    /// `#F5FFFA`
    pub const            MINTCREAM: Self = Rgb::new(245, 255, 250);
    /// `RGB(255, 228, 225)`  
    /// `#FFE4E1`
    pub const            MISTYROSE: Self = Rgb::new(255, 228, 225);
    /// `RGB(255, 228, 181)`  
    /// `#FFE4B5`
    pub const             MOCCASIN: Self = Rgb::new(255, 228, 181);
    /// `RGB(255, 222, 173)`  
    /// `#FFDEAD`
    pub const          NAVAJOWHITE: Self = Rgb::new(255, 222, 173);
    /// `RGB(0, 0, 128)`  
    /// `#000080`
    pub const                 NAVY: Self = Rgb::new(  0,   0, 128);
    /// `RGB(253, 245, 230)`  
    /// `#FDF5E6`
    pub const              OLDLACE: Self = Rgb::new(253, 245, 230);
    /// `RGB(128, 128, 0)`  
    /// `#808000`
    pub const                OLIVE: Self = Rgb::new(128, 128,   0);
    /// `RGB(107, 142, 35)`  
    /// `#6B8E23`
    pub const            OLIVEDRAB: Self = Rgb::new(107, 142,  35);
    /// `RGB(255, 165, 0)`  
    /// `#FFA500`
    pub const               ORANGE: Self = Rgb::new(255, 165,   0);
    /// `RGB(255, 69, 0)`  
    /// `#FF4500`
    pub const            ORANGERED: Self = Rgb::new(255,  69,   0);
    /// `RGB(218, 112, 214)`  
    /// `#DA70D6`
    pub const               ORCHID: Self = Rgb::new(218, 112, 214);
    /// `RGB(238, 232, 170)`  
    /// `#EEE8AA`
    pub const        PALEGOLDENROD: Self = Rgb::new(238, 232, 170);
    /// `RGB(152, 251, 152)`  
    /// `#98FB98`
    pub const            PALEGREEN: Self = Rgb::new(152, 251, 152);
    /// `RGB(175, 238, 238)`  
    /// `#AFEEEE`
    pub const        PALETURQUOISE: Self = Rgb::new(175, 238, 238);
    /// `RGB(219, 112, 147)`  
    /// `#DB7093`
    pub const        PALEVIOLETRED: Self = Rgb::new(219, 112, 147);
    /// `RGB(255, 239, 213)`  
    /// `#FFEFD5`
    pub const           PAPAYAWHIP: Self = Rgb::new(255, 239, 213);
    /// `RGB(255, 218, 185)`  
    /// `#FFDAB9`
    pub const            PEACHPUFF: Self = Rgb::new(255, 218, 185);
    /// `RGB(205, 133, 63)`  
    /// `#CD853F`
    pub const                 PERU: Self = Rgb::new(205, 133,  63);
    /// `RGB(255, 192, 203)`  
    /// `#FFC0CB`
    pub const                 PINK: Self = Rgb::new(255, 192, 203);
    /// `RGB(221, 160, 221)`  
    /// `#DDA0DD`
    pub const                 PLUM: Self = Rgb::new(221, 160, 221);
    /// `RGB(176, 224, 230)`  
    /// `#B0E0E6`
    pub const           POWDERBLUE: Self = Rgb::new(176, 224, 230);
    /// `RGB(128, 0, 128)`  
    /// `#800080`
    pub const               PURPLE: Self = Rgb::new(128,   0, 128);
    /// `RGB(255, 0, 0)`  
    /// `#FF0000`
    pub const                  RED: Self = Rgb::new(255,   0,   0);
    /// `RGB(188, 143, 143)`  
    /// `#BC8F8F`
    pub const            ROSYBROWN: Self = Rgb::new(188, 143, 143);
    /// `RGB(65, 105, 225)`  
    /// `#4169E1`
    pub const            ROYALBLUE: Self = Rgb::new( 65, 105, 225);
    /// `RGB(139, 69, 19)`  
    /// `#8B4513`
    pub const          SADDLEBROWN: Self = Rgb::new(139,  69,  19);
    /// `RGB(250, 128, 114)`  
    /// `#FA8072`
    pub const               SALMON: Self = Rgb::new(250, 128, 114);
    /// `RGB(244, 164, 96)`  
    /// `#F4A460`
    pub const           SANDYBROWN: Self = Rgb::new(244, 164,  96);
    /// `RGB(46, 139, 87)`  
    /// `#2E8B57`
    pub const             SEAGREEN: Self = Rgb::new( 46, 139,  87);
    /// `RGB(255, 245, 238)`  
    /// `#FFF5EE`
    pub const             SEASHELL: Self = Rgb::new(255, 245, 238);
    /// `RGB(160, 82, 45)`  
    /// `#A0522D`
    pub const               SIENNA: Self = Rgb::new(160,  82,  45);
    /// `RGB(192, 192, 192)`  
    /// `#C0C0C0`
    pub const               SILVER: Self = Rgb::new(192, 192, 192);
    /// `RGB(135, 206, 235)`  
    /// `#87CEEB`
    pub const              SKYBLUE: Self = Rgb::new(135, 206, 235);
    /// `RGB(106, 90, 205)`  
    /// `#6A5ACD`
    pub const            SLATEBLUE: Self = Rgb::new(106,  90, 205);
    /// `RGB(112, 128, 144)`  
    /// `#708090`
    pub const            SLATEGRAY: Self = Rgb::new(112, 128, 144);
    /// `RGB(255, 250, 250)`  
    /// `#FFFAFA`
    pub const                 SNOW: Self = Rgb::new(255, 250, 250);
    /// `RGB(0, 255, 127)`  
    /// `#00FF7F`
    pub const          SPRINGGREEN: Self = Rgb::new(  0, 255, 127);
    /// `RGB(70, 130, 180)`  
    /// `#4682B4`
    pub const            STEELBLUE: Self = Rgb::new( 70, 130, 180);
    /// `RGB(210, 180, 140)`  
    /// `#D2B48C`
    pub const                  TAN: Self = Rgb::new(210, 180, 140);
    /// `RGB(0, 128, 128)`  
    /// `#008080`
    pub const                 TEAL: Self = Rgb::new(  0, 128, 128);
    /// `RGB(216, 191, 216)`  
    /// `#D8BFD8`
    pub const              THISTLE: Self = Rgb::new(216, 191, 216);
    /// `RGB(255, 99, 71)`  
    /// `#FF6347`
    pub const               TOMATO: Self = Rgb::new(255,  99,  71);
    /// `RGB(64, 224, 208)`  
    /// `#40E0D0`
    pub const            TURQUOISE: Self = Rgb::new( 64, 224, 208);
    /// `RGB(238, 130, 238)`  
    /// `#EE82EE`
    pub const               VIOLET: Self = Rgb::new(238, 130, 238);
    /// `RGB(245, 222, 179)`  
    /// `#F5DEB3`
    pub const                WHEAT: Self = Rgb::new(245, 222, 179);
    /// `RGB(255, 255, 255)`  
    /// `#FFFFFF`
    pub const                WHITE: Self = Rgb::new(255, 255, 255);
    /// `RGB(245, 245, 245)`  
    /// `#F5F5F5`
    pub const           WHITESMOKE: Self = Rgb::new(245, 245, 245);
    /// `RGB(255, 255, 0)`  
    /// `#FFFF00`
    pub const               YELLOW: Self = Rgb::new(255, 255,   0);
    /// `RGB(154, 205, 50)`  
    /// `#9ACD32`
    pub const          YELLOWGREEN: Self = Rgb::new(154, 205,  50);
    
    #[inline]
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self{r,g,b}
    }

    #[inline]
    pub const fn rgba(self) -> Rgba {
        Rgba::new(self.r, self.g, self.b, 255)
    }

    #[inline]
    pub const fn with_alpha(self, alpha: u8) -> Rgba {
        Rgba::new(self.r, self.g, self.b, alpha)
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

impl Rgba {
    /// `RGBA(240, 248, 255, 255)`  
    /// `#F0F8FFFF`
    pub const            ALICEBLUE: Self = Rgba::new(240, 248, 255, 255);
    /// `RGBA(250, 235, 215, 255)`  
    /// `#FAEBD7FF`
    pub const         ANTIQUEWHITE: Self = Rgba::new(250, 235, 215, 255);
    /// `RGBA(0, 255, 255, 255)`  
    /// `#00FFFFFF`
    pub const                 AQUA: Self = Rgba::new(  0, 255, 255, 255);
    /// `RGBA(127, 255, 212, 255)`  
    /// `#7FFFD4FF`
    pub const           AQUAMARINE: Self = Rgba::new(127, 255, 212, 255);
    /// `RGBA(240, 255, 255, 255)`  
    /// `#F0FFFFFF`
    pub const                AZURE: Self = Rgba::new(240, 255, 255, 255);
    /// `RGBA(245, 245, 220, 255)`  
    /// `#F5F5DCFF`
    pub const                BEIGE: Self = Rgba::new(245, 245, 220, 255);
    /// `RGBA(255, 228, 196, 255)`  
    /// `#FFE4C4FF`
    pub const               BISQUE: Self = Rgba::new(255, 228, 196, 255);
    /// `RGBA(0, 0, 0, 255)`  
    /// `#000000FF`
    pub const                BLACK: Self = Rgba::new(  0,   0,   0, 255);
    /// `RGBA(255, 235, 205, 255)`  
    /// `#FFEBCDFF`
    pub const       BLANCHEDALMOND: Self = Rgba::new(255, 235, 205, 255);
    /// `RGBA(0, 0, 255, 255)`  
    /// `#0000FFFF`
    pub const                 BLUE: Self = Rgba::new(  0,   0, 255, 255);
    /// `RGBA(138, 43, 226, 255)`  
    /// `#8A2BE2FF`
    pub const           BLUEVIOLET: Self = Rgba::new(138,  43, 226, 255);
    /// `RGBA(165, 42, 42, 255)`  
    /// `#A52A2AFF`
    pub const                BROWN: Self = Rgba::new(165,  42,  42, 255);
    /// `RGBA(222, 184, 135, 255)`  
    /// `#DEB887FF`
    pub const            BURLYWOOD: Self = Rgba::new(222, 184, 135, 255);
    /// `RGBA(95, 158, 160, 255)`  
    /// `#5F9EA0FF`
    pub const            CADETBLUE: Self = Rgba::new( 95, 158, 160, 255);
    /// `RGBA(127, 255, 0, 255)`  
    /// `#7FFF00FF`
    pub const           CHARTREUSE: Self = Rgba::new(127, 255,   0, 255);
    /// `RGBA(210, 105, 30, 255)`  
    /// `#D2691EFF`
    pub const            CHOCOLATE: Self = Rgba::new(210, 105,  30, 255);
    /// `RGBA(255, 127, 80, 255)`  
    /// `#FF7F50FF`
    pub const                CORAL: Self = Rgba::new(255, 127,  80, 255);
    /// `RGBA(100, 149, 237, 255)`  
    /// `#6495EDFF`
    pub const       CORNFLOWERBLUE: Self = Rgba::new(100, 149, 237, 255);
    /// `RGBA(255, 248, 220, 255)`  
    /// `#FFF8DCFF`
    pub const             CORNSILK: Self = Rgba::new(255, 248, 220, 255);
    /// `RGBA(220, 20, 60, 255)`  
    /// `#DC143CFF`
    pub const              CRIMSON: Self = Rgba::new(220,  20,  60, 255);
    /// `RGBA(0, 255, 255, 255)`  
    /// `#00FFFFFF`
    pub const                 CYAN: Self = Rgba::new(  0, 255, 255, 255);
    /// `RGBA(0, 0, 139, 255)`  
    /// `#00008BFF`
    pub const             DARKBLUE: Self = Rgba::new(  0,   0, 139, 255);
    /// `RGBA(0, 139, 139, 255)`  
    /// `#008B8BFF`
    pub const             DARKCYAN: Self = Rgba::new(  0, 139, 139, 255);
    /// `RGBA(184, 134, 11, 255)`  
    /// `#B8860BFF`
    pub const        DARKGOLDENROD: Self = Rgba::new(184, 134,  11, 255);
    /// `RGBA(169, 169, 169, 255)`  
    /// `#A9A9A9FF`
    pub const             DARKGRAY: Self = Rgba::new(169, 169, 169, 255);
    /// `RGBA(0, 100, 0, 255)`  
    /// `#006400FF`
    pub const            DARKGREEN: Self = Rgba::new(  0, 100,   0, 255);
    /// `RGBA(189, 183, 107, 255)`  
    /// `#BDB76BFF`
    pub const            DARKKHAKI: Self = Rgba::new(189, 183, 107, 255);
    /// `RGBA(139, 0, 139, 255)`  
    /// `#8B008BFF`
    pub const          DARKMAGENTA: Self = Rgba::new(139,   0, 139, 255);
    /// `RGBA(85, 107, 47, 255)`  
    /// `#556B2FFF`
    pub const       DARKOLIVEGREEN: Self = Rgba::new( 85, 107,  47, 255);
    /// `RGBA(255, 140, 0, 255)`  
    /// `#FF8C00FF`
    pub const           DARKORANGE: Self = Rgba::new(255, 140,   0, 255);
    /// `RGBA(153, 50, 204, 255)`  
    /// `#9932CCFF`
    pub const           DARKORCHID: Self = Rgba::new(153,  50, 204, 255);
    /// `RGBA(139, 0, 0, 255)`  
    /// `#8B0000FF`
    pub const              DARKRED: Self = Rgba::new(139,   0,   0, 255);
    /// `RGBA(233, 150, 122, 255)`  
    /// `#E9967AFF`
    pub const           DARKSALMON: Self = Rgba::new(233, 150, 122, 255);
    /// `RGBA(143, 188, 143, 255)`  
    /// `#8FBC8FFF`
    pub const         DARKSEAGREEN: Self = Rgba::new(143, 188, 143, 255);
    /// `RGBA(72, 61, 139, 255)`  
    /// `#483D8BFF`
    pub const        DARKSLATEBLUE: Self = Rgba::new( 72,  61, 139, 255);
    /// `RGBA(47, 79, 79, 255)`  
    /// `#2F4F4FFF`
    pub const        DARKSLATEGRAY: Self = Rgba::new( 47,  79,  79, 255);
    /// `RGBA(0, 206, 209, 255)`  
    /// `#00CED1FF`
    pub const        DARKTURQUOISE: Self = Rgba::new(  0, 206, 209, 255);
    /// `RGBA(148, 0, 211, 255)`  
    /// `#9400D3FF`
    pub const           DARKVIOLET: Self = Rgba::new(148,   0, 211, 255);
    /// `RGBA(255, 20, 147, 255)`  
    /// `#FF1493FF`
    pub const             DEEPPINK: Self = Rgba::new(255,  20, 147, 255);
    /// `RGBA(0, 191, 255, 255)`  
    /// `#00BFFFFF`
    pub const          DEEPSKYBLUE: Self = Rgba::new(  0, 191, 255, 255);
    /// `RGBA(105, 105, 105, 255)`  
    /// `#696969FF`
    pub const              DIMGRAY: Self = Rgba::new(105, 105, 105, 255);
    /// `RGBA(30, 144, 255, 255)`  
    /// `#1E90FFFF`
    pub const           DODGERBLUE: Self = Rgba::new( 30, 144, 255, 255);
    /// `RGBA(178, 34, 34, 255)`  
    /// `#B22222FF`
    pub const            FIREBRICK: Self = Rgba::new(178,  34,  34, 255);
    /// `RGBA(255, 250, 240, 255)`  
    /// `#FFFAF0FF`
    pub const          FLORALWHITE: Self = Rgba::new(255, 250, 240, 255);
    /// `RGBA(34, 139, 34, 255)`  
    /// `#228B22FF`
    pub const          FORESTGREEN: Self = Rgba::new( 34, 139,  34, 255);
    /// `RGBA(255, 0, 255, 255)`  
    /// `#FF00FFFF`
    pub const              FUCHSIA: Self = Rgba::new(255,   0, 255, 255);
    /// `RGBA(220, 220, 220, 255)`  
    /// `#DCDCDCFF`
    pub const            GAINSBORO: Self = Rgba::new(220, 220, 220, 255);
    /// `RGBA(248, 248, 255, 255)`  
    /// `#F8F8FFFF`
    pub const           GHOSTWHITE: Self = Rgba::new(248, 248, 255, 255);
    /// `RGBA(255, 215, 0, 255)`  
    /// `#FFD700FF`
    pub const                 GOLD: Self = Rgba::new(255, 215,   0, 255);
    /// `RGBA(218, 165, 32, 255)`  
    /// `#DAA520FF`
    pub const            GOLDENROD: Self = Rgba::new(218, 165,  32, 255);
    /// `RGBA(128, 128, 128, 255)`  
    /// `#808080FF`
    pub const                 GRAY: Self = Rgba::new(128, 128, 128, 255);
    /// `RGBA(0, 128, 0, 255)`  
    /// `#008000FF`
    pub const                GREEN: Self = Rgba::new(  0, 128,   0, 255);
    /// `RGBA(173, 255, 47, 255)`  
    /// `#ADFF2FFF`
    pub const          GREENYELLOW: Self = Rgba::new(173, 255,  47, 255);
    /// `RGBA(240, 255, 240, 255)`  
    /// `#F0FFF0FF`
    pub const             HONEYDEW: Self = Rgba::new(240, 255, 240, 255);
    /// `RGBA(255, 105, 180, 255)`  
    /// `#FF69B4FF`
    pub const              HOTPINK: Self = Rgba::new(255, 105, 180, 255);
    /// `RGBA(205, 92, 92, 255)`  
    /// `#CD5C5CFF`
    pub const            INDIANRED: Self = Rgba::new(205,  92,  92, 255);
    /// `RGBA(75, 0, 130, 255)`  
    /// `#4B0082FF`
    pub const               INDIGO: Self = Rgba::new( 75,   0, 130, 255);
    /// `RGBA(255, 255, 240, 255)`  
    /// `#FFFFF0FF`
    pub const                IVORY: Self = Rgba::new(255, 255, 240, 255);
    /// `RGBA(240, 230, 140, 255)`  
    /// `#F0E68CFF`
    pub const                KHAKI: Self = Rgba::new(240, 230, 140, 255);
    /// `RGBA(230, 230, 250, 255)`  
    /// `#E6E6FAFF`
    pub const             LAVENDER: Self = Rgba::new(230, 230, 250, 255);
    /// `RGBA(255, 240, 245, 255)`  
    /// `#FFF0F5FF`
    pub const        LAVENDERBLUSH: Self = Rgba::new(255, 240, 245, 255);
    /// `RGBA(124, 252, 0, 255)`  
    /// `#7CFC00FF`
    pub const            LAWNGREEN: Self = Rgba::new(124, 252,   0, 255);
    /// `RGBA(255, 250, 205, 255)`  
    /// `#FFFACDFF`
    pub const         LEMONCHIFFON: Self = Rgba::new(255, 250, 205, 255);
    /// `RGBA(173, 216, 230, 255)`  
    /// `#ADD8E6FF`
    pub const            LIGHTBLUE: Self = Rgba::new(173, 216, 230, 255);
    /// `RGBA(240, 128, 128, 255)`  
    /// `#F08080FF`
    pub const           LIGHTCORAL: Self = Rgba::new(240, 128, 128, 255);
    /// `RGBA(224, 255, 255, 255)`  
    /// `#E0FFFFFF`
    pub const            LIGHTCYAN: Self = Rgba::new(224, 255, 255, 255);
    /// `RGBA(250, 250, 210, 255)`  
    /// `#FAFAD2FF`
    pub const LIGHTGOLDENRODYELLOW: Self = Rgba::new(250, 250, 210, 255);
    /// `RGBA(211, 211, 211, 255)`  
    /// `#D3D3D3FF`
    pub const            LIGHTGRAY: Self = Rgba::new(211, 211, 211, 255);
    /// `RGBA(144, 238, 144, 255)`  
    /// `#90EE90FF`
    pub const           LIGHTGREEN: Self = Rgba::new(144, 238, 144, 255);
    /// `RGBA(255, 182, 193, 255)`  
    /// `#FFB6C1FF`
    pub const            LIGHTPINK: Self = Rgba::new(255, 182, 193, 255);
    /// `RGBA(255, 160, 122, 255)`  
    /// `#FFA07AFF`
    pub const          LIGHTSALMON: Self = Rgba::new(255, 160, 122, 255);
    /// `RGBA(32, 178, 170, 255)`  
    /// `#20B2AAFF`
    pub const        LIGHTSEAGREEN: Self = Rgba::new( 32, 178, 170, 255);
    /// `RGBA(135, 206, 250, 255)`  
    /// `#87CEFAFF`
    pub const         LIGHTSKYBLUE: Self = Rgba::new(135, 206, 250, 255);
    /// `RGBA(119, 136, 153, 255)`  
    /// `#778899FF`
    pub const       LIGHTSLATEGRAY: Self = Rgba::new(119, 136, 153, 255);
    /// `RGBA(176, 196, 222, 255)`  
    /// `#B0C4DEFF`
    pub const       LIGHTSTEELBLUE: Self = Rgba::new(176, 196, 222, 255);
    /// `RGBA(255, 255, 224, 255)`  
    /// `#FFFFE0FF`
    pub const          LIGHTYELLOW: Self = Rgba::new(255, 255, 224, 255);
    /// `RGBA(0, 255, 0, 255)`  
    /// `#00FF00FF`
    pub const                 LIME: Self = Rgba::new(  0, 255,   0, 255);
    /// `RGBA(50, 205, 50, 255)`  
    /// `#32CD32FF`
    pub const            LIMEGREEN: Self = Rgba::new( 50, 205,  50, 255);
    /// `RGBA(250, 240, 230, 255)`  
    /// `#FAF0E6FF`
    pub const                LINEN: Self = Rgba::new(250, 240, 230, 255);
    /// `RGBA(255, 0, 255, 255)`  
    /// `#FF00FFFF`
    pub const              MAGENTA: Self = Rgba::new(255,   0, 255, 255);
    /// `RGBA(128, 0, 0, 255)`  
    /// `#800000FF`
    pub const               MAROON: Self = Rgba::new(128,   0,   0, 255);
    /// `RGBA(102, 205, 170, 255)`  
    /// `#66CDAAFF`
    pub const     MEDIUMAQUAMARINE: Self = Rgba::new(102, 205, 170, 255);
    /// `RGBA(0, 0, 205, 255)`  
    /// `#0000CDFF`
    pub const           MEDIUMBLUE: Self = Rgba::new(  0,   0, 205, 255);
    /// `RGBA(186, 85, 211, 255)`  
    /// `#BA55D3FF`
    pub const         MEDIUMORCHID: Self = Rgba::new(186,  85, 211, 255);
    /// `RGBA(147, 112, 219, 255)`  
    /// `#9370DBFF`
    pub const         MEDIUMPURPLE: Self = Rgba::new(147, 112, 219, 255);
    /// `RGBA(60, 179, 113, 255)`  
    /// `#3CB371FF`
    pub const       MEDIUMSEAGREEN: Self = Rgba::new( 60, 179, 113, 255);
    /// `RGBA(123, 104, 238, 255)`  
    /// `#7B68EEFF`
    pub const      MEDIUMSLATEBLUE: Self = Rgba::new(123, 104, 238, 255);
    /// `RGBA(0, 250, 154, 255)`  
    /// `#00FA9AFF`
    pub const    MEDIUMSPRINGGREEN: Self = Rgba::new(  0, 250, 154, 255);
    /// `RGBA(72, 209, 204, 255)`  
    /// `#48D1CCFF`
    pub const      MEDIUMTURQUOISE: Self = Rgba::new( 72, 209, 204, 255);
    /// `RGBA(199, 21, 133, 255)`  
    /// `#C71585FF`
    pub const      MEDIUMVIOLETRED: Self = Rgba::new(199,  21, 133, 255);
    /// `RGBA(25, 25, 112, 255)`  
    /// `#191970FF`
    pub const         MIDNIGHTBLUE: Self = Rgba::new( 25,  25, 112, 255);
    /// `RGBA(245, 255, 250, 255)`  
    /// `#F5FFFAFF`
    pub const            MINTCREAM: Self = Rgba::new(245, 255, 250, 255);
    /// `RGBA(255, 228, 225, 255)`  
    /// `#FFE4E1FF`
    pub const            MISTYROSE: Self = Rgba::new(255, 228, 225, 255);
    /// `RGBA(255, 228, 181, 255)`  
    /// `#FFE4B5FF`
    pub const             MOCCASIN: Self = Rgba::new(255, 228, 181, 255);
    /// `RGBA(255, 222, 173, 255)`  
    /// `#FFDEADFF`
    pub const          NAVAJOWHITE: Self = Rgba::new(255, 222, 173, 255);
    /// `RGBA(0, 0, 128, 255)`  
    /// `#000080FF`
    pub const                 NAVY: Self = Rgba::new(  0,   0, 128, 255);
    /// `RGBA(253, 245, 230, 255)`  
    /// `#FDF5E6FF`
    pub const              OLDLACE: Self = Rgba::new(253, 245, 230, 255);
    /// `RGBA(128, 128, 0, 255)`  
    /// `#808000FF`
    pub const                OLIVE: Self = Rgba::new(128, 128,   0, 255);
    /// `RGBA(107, 142, 35, 255)`  
    /// `#6B8E23FF`
    pub const            OLIVEDRAB: Self = Rgba::new(107, 142,  35, 255);
    /// `RGBA(255, 165, 0, 255)`  
    /// `#FFA500FF`
    pub const               ORANGE: Self = Rgba::new(255, 165,   0, 255);
    /// `RGBA(255, 69, 0, 255)`  
    /// `#FF4500FF`
    pub const            ORANGERED: Self = Rgba::new(255,  69,   0, 255);
    /// `RGBA(218, 112, 214, 255)`  
    /// `#DA70D6FF`
    pub const               ORCHID: Self = Rgba::new(218, 112, 214, 255);
    /// `RGBA(238, 232, 170, 255)`  
    /// `#EEE8AAFF`
    pub const        PALEGOLDENROD: Self = Rgba::new(238, 232, 170, 255);
    /// `RGBA(152, 251, 152, 255)`  
    /// `#98FB98FF`
    pub const            PALEGREEN: Self = Rgba::new(152, 251, 152, 255);
    /// `RGBA(175, 238, 238, 255)`  
    /// `#AFEEEEFF`
    pub const        PALETURQUOISE: Self = Rgba::new(175, 238, 238, 255);
    /// `RGBA(219, 112, 147, 255)`  
    /// `#DB7093FF`
    pub const        PALEVIOLETRED: Self = Rgba::new(219, 112, 147, 255);
    /// `RGBA(255, 239, 213, 255)`  
    /// `#FFEFD5FF`
    pub const           PAPAYAWHIP: Self = Rgba::new(255, 239, 213, 255);
    /// `RGBA(255, 218, 185, 255)`  
    /// `#FFDAB9FF`
    pub const            PEACHPUFF: Self = Rgba::new(255, 218, 185, 255);
    /// `RGBA(205, 133, 63, 255)`  
    /// `#CD853FFF`
    pub const                 PERU: Self = Rgba::new(205, 133,  63, 255);
    /// `RGBA(255, 192, 203, 255)`  
    /// `#FFC0CBFF`
    pub const                 PINK: Self = Rgba::new(255, 192, 203, 255);
    /// `RGBA(221, 160, 221, 255)`  
    /// `#DDA0DDFF`
    pub const                 PLUM: Self = Rgba::new(221, 160, 221, 255);
    /// `RGBA(176, 224, 230, 255)`  
    /// `#B0E0E6FF`
    pub const           POWDERBLUE: Self = Rgba::new(176, 224, 230, 255);
    /// `RGBA(128, 0, 128, 255)`  
    /// `#800080FF`
    pub const               PURPLE: Self = Rgba::new(128,   0, 128, 255);
    /// `RGBA(255, 0, 0, 255)`  
    /// `#FF0000FF`
    pub const                  RED: Self = Rgba::new(255,   0,   0, 255);
    /// `RGBA(188, 143, 143, 255)`  
    /// `#BC8F8FFF`
    pub const            ROSYBROWN: Self = Rgba::new(188, 143, 143, 255);
    /// `RGBA(65, 105, 225, 255)`  
    /// `#4169E1FF`
    pub const            ROYALBLUE: Self = Rgba::new( 65, 105, 225, 255);
    /// `RGBA(139, 69, 19, 255)`  
    /// `#8B4513FF`
    pub const          SADDLEBROWN: Self = Rgba::new(139,  69,  19, 255);
    /// `RGBA(250, 128, 114, 255)`  
    /// `#FA8072FF`
    pub const               SALMON: Self = Rgba::new(250, 128, 114, 255);
    /// `RGBA(244, 164, 96, 255)`  
    /// `#F4A460FF`
    pub const           SANDYBROWN: Self = Rgba::new(244, 164,  96, 255);
    /// `RGBA(46, 139, 87, 255)`  
    /// `#2E8B57FF`
    pub const             SEAGREEN: Self = Rgba::new( 46, 139,  87, 255);
    /// `RGBA(255, 245, 238, 255)`  
    /// `#FFF5EEFF`
    pub const             SEASHELL: Self = Rgba::new(255, 245, 238, 255);
    /// `RGBA(160, 82, 45, 255)`  
    /// `#A0522DFF`
    pub const               SIENNA: Self = Rgba::new(160,  82,  45, 255);
    /// `RGBA(192, 192, 192, 255)`  
    /// `#C0C0C0FF`
    pub const               SILVER: Self = Rgba::new(192, 192, 192, 255);
    /// `RGBA(135, 206, 235, 255)`  
    /// `#87CEEBFF`
    pub const              SKYBLUE: Self = Rgba::new(135, 206, 235, 255);
    /// `RGBA(106, 90, 205, 255)`  
    /// `#6A5ACDFF`
    pub const            SLATEBLUE: Self = Rgba::new(106,  90, 205, 255);
    /// `RGBA(112, 128, 144, 255)`  
    /// `#708090FF`
    pub const            SLATEGRAY: Self = Rgba::new(112, 128, 144, 255);
    /// `RGBA(255, 250, 250, 255)`  
    /// `#FFFAFAFF`
    pub const                 SNOW: Self = Rgba::new(255, 250, 250, 255);
    /// `RGBA(0, 255, 127, 255)`  
    /// `#00FF7FFF`
    pub const          SPRINGGREEN: Self = Rgba::new(  0, 255, 127, 255);
    /// `RGBA(70, 130, 180, 255)`  
    /// `#4682B4FF`
    pub const            STEELBLUE: Self = Rgba::new( 70, 130, 180, 255);
    /// `RGBA(210, 180, 140, 255)`  
    /// `#D2B48CFF`
    pub const                  TAN: Self = Rgba::new(210, 180, 140, 255);
    /// `RGBA(0, 128, 128, 255)`  
    /// `#008080FF`
    pub const                 TEAL: Self = Rgba::new(  0, 128, 128, 255);
    /// `RGBA(216, 191, 216, 255)`  
    /// `#D8BFD8FF`
    pub const              THISTLE: Self = Rgba::new(216, 191, 216, 255);
    /// `RGBA(255, 99, 71, 255)`  
    /// `#FF6347FF`
    pub const               TOMATO: Self = Rgba::new(255,  99,  71, 255);
    /// `RGBA(64, 224, 208, 255)`  
    /// `#40E0D0FF`
    pub const            TURQUOISE: Self = Rgba::new( 64, 224, 208, 255);
    /// `RGBA(238, 130, 238, 255)`  
    /// `#EE82EEFF`
    pub const               VIOLET: Self = Rgba::new(238, 130, 238, 255);
    /// `RGBA(245, 222, 179, 255)`  
    /// `#F5DEB3FF`
    pub const                WHEAT: Self = Rgba::new(245, 222, 179, 255);
    /// `RGBA(255, 255, 255, 255)`  
    /// `#FFFFFFFF`
    pub const                WHITE: Self = Rgba::new(255, 255, 255, 255);
    /// `RGBA(245, 245, 245, 255)`  
    /// `#F5F5F5FF`
    pub const           WHITESMOKE: Self = Rgba::new(245, 245, 245, 255);
    /// `RGBA(255, 255, 0, 255)`  
    /// `#FFFF00FF`
    pub const               YELLOW: Self = Rgba::new(255, 255,   0, 255);
    /// `RGBA(154, 205, 50, 255)`  
    /// `#9ACD32FF`
    pub const          YELLOWGREEN: Self = Rgba::new(154, 205,  50, 255);
    
    #[inline]
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self{r,g,b,a}
    }

    #[inline]
    pub const fn rgb(self) -> Rgb {
        Rgb::new(self.r, self.g, self.b)
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

pub const fn rgb(r: u8, g: u8, b: u8) -> Rgb {
    Rgb::new(r, g, b)
}

pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Rgba {
    Rgba::new(r,g,b,a)
}

#[cfg(test)]
mod testing_sandbox {
    // TODO: Remove this sandbox when it is no longer in use.
    use super::*;
    #[test]
    fn sandbox() {
        let color = Rgb::lerp(Rgb::ALICEBLUE, Rgb::AQUAMARINE, 0.5);
        println!("{}", Rgb::ALICEBLUE);
        println!("{}", Rgb::AQUAMARINE);
        println!("{color}");
    }
}