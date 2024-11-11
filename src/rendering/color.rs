use bytemuck::NoUninit;

macro_rules! html_colors {
    ($macro:path) => {
        $macro!{
["Alice Blue"               AliceBlue               ALICE_BLUE                  alice_blue                  "F0F8FF"    "F0F8FFFF"  RGB(240, 248, 255)  ]
["Antique White"            AntiqueWhite            ANTIQUE_WHITE               antique_white               "FAEBD7"    "FAEBD7FF"  RGB(250, 235, 215)  ]
["Aqua"                     Aqua                    AQUA                        aqua                        "00FFFF"    "00FFFFFF"  RGB(0, 255, 255)    ]
["Aquamarine"               Aquamarine              AQUAMARINE                  aquamarine                  "7FFFD4"    "7FFFD4FF"  RGB(127, 255, 212)  ]
["Azure"                    Azure                   AZURE                       azure                       "F0FFFF"    "F0FFFFFF"  RGB(240, 255, 255)  ]
["Beige"                    Beige                   BEIGE                       beige                       "F5F5DC"    "F5F5DCFF"  RGB(245, 245, 220)  ]
["Bisque"                   Bisque                  BISQUE                      bisque                      "FFE4C4"    "FFE4C4FF"  RGB(255, 228, 196)  ]
["Black"                    Black                   BLACK                       black                       "000000"    "000000FF"  RGB(0, 0, 0)        ]
["Blanched Almond"          BlanchedAlmond          BLANCHED_ALMOND             blanched_almond             "FFEBCD"    "FFEBCDFF"  RGB(255, 235, 205)  ]
["Blue"                     Blue                    BLUE                        blue                        "0000FF"    "0000FFFF"  RGB(0, 0, 255)      ]
["Blue Violet"              BlueViolet              BLUE_VIOLET                 blue_violet                 "8A2BE2"    "8A2BE2FF"  RGB(138, 43, 226)   ]
["Brown"                    Brown                   BROWN                       brown                       "A52A2A"    "A52A2AFF"  RGB(165, 42, 42)    ]
["Burly Wood"               BurlyWood               BURLY_WOOD                  burly_wood                  "DEB887"    "DEB887FF"  RGB(222, 184, 135)  ]
["Cadet Blue"               CadetBlue               CADET_BLUE                  cadet_blue                  "5F9EA0"    "5F9EA0FF"  RGB(95, 158, 160)   ]
["Chartreuse"               Chartreuse              CHARTREUSE                  chartreuse                  "7FFF00"    "7FFF00FF"  RGB(127, 255, 0)    ]
["Chocolate"                Chocolate               CHOCOLATE                   chocolate                   "D2691E"    "D2691EFF"  RGB(210, 105, 30)   ]
["Coral"                    Coral                   CORAL                       coral                       "FF7F50"    "FF7F50FF"  RGB(255, 127, 80)   ]
["Cornflower Blue"          CornflowerBlue          CORNFLOWER_BLUE             cornflower_blue             "6495ED"    "6495EDFF"  RGB(100, 149, 237)  ]
["Cornsilk"                 Cornsilk                CORNSILK                    cornsilk                    "FFF8DC"    "FFF8DCFF"  RGB(255, 248, 220)  ]
["Crimson"                  Crimson                 CRIMSON                     crimson                     "DC143C"    "DC143CFF"  RGB(220, 20, 60)    ]
["Cyan"                     Cyan                    CYAN                        cyan                        "00FFFF"    "00FFFFFF"  RGB(0, 255, 255)    ]
["Dark Blue"                DarkBlue                DARK_BLUE                   dark_blue                   "00008B"    "00008BFF"  RGB(0, 0, 139)      ]
["Dark Cyan"                DarkCyan                DARK_CYAN                   dark_cyan                   "008B8B"    "008B8BFF"  RGB(0, 139, 139)    ]
["Dark Goldenrod"           DarkGoldenrod           DARK_GOLDENROD              dark_goldenrod              "B8860B"    "B8860BFF"  RGB(184, 134, 11)   ]
["Dark Gray"                DarkGray                DARK_GRAY                   dark_gray                   "A9A9A9"    "A9A9A9FF"  RGB(169, 169, 169)  ]
["Dark Green"               DarkGreen               DARK_GREEN                  dark_green                  "006400"    "006400FF"  RGB(0, 100, 0)      ]
["Dark Khaki"               DarkKhaki               DARK_KHAKI                  dark_khaki                  "BDB76B"    "BDB76BFF"  RGB(189, 183, 107)  ]
["Dark Magenta"             DarkMagenta             DARK_MAGENTA                dark_magenta                "8B008B"    "8B008BFF"  RGB(139, 0, 139)    ]
["Dark Olive Green"         DarkOliveGreen          DARK_OLIVE_GREEN            dark_olive_green            "556B2F"    "556B2FFF"  RGB(85, 107, 47)    ]
["Dark Orange"              DarkOrange              DARK_ORANGE                 dark_orange                 "FF8C00"    "FF8C00FF"  RGB(255, 140, 0)    ]
["Dark Orchid"              DarkOrchid              DARK_ORCHID                 dark_orchid                 "9932CC"    "9932CCFF"  RGB(153, 50, 204)   ]
["Dark Red"                 DarkRed                 DARK_RED                    dark_red                    "8B0000"    "8B0000FF"  RGB(139, 0, 0)      ]
["Dark Salmon"              DarkSalmon              DARK_SALMON                 dark_salmon                 "E9967A"    "E9967AFF"  RGB(233, 150, 122)  ]
["Dark Sea Green"           DarkSeaGreen            DARK_SEA_GREEN              dark_sea_green              "8FBC8F"    "8FBC8FFF"  RGB(143, 188, 143)  ]
["Dark Slate Blue"          DarkSlateBlue           DARK_SLATE_BLUE             dark_slate_blue             "483D8B"    "483D8BFF"  RGB(72, 61, 139)    ]
["Dark Slate Gray"          DarkSlateGray           DARK_SLATE_GRAY             dark_slate_gray             "2F4F4F"    "2F4F4FFF"  RGB(47, 79, 79)     ]
["Dark Turquoise"           DarkTurquoise           DARK_TURQUOISE              dark_turquoise              "00CED1"    "00CED1FF"  RGB(0, 206, 209)    ]
["Dark Violet"              DarkViolet              DARK_VIOLET                 dark_violet                 "9400D3"    "9400D3FF"  RGB(148, 0, 211)    ]
["Deep Pink"                DeepPink                DEEP_PINK                   deep_pink                   "FF1493"    "FF1493FF"  RGB(255, 20, 147)   ]
["Deep Sky Blue"            DeepSkyBlue             DEEP_SKY_BLUE               deep_sky_blue               "00BFFF"    "00BFFFFF"  RGB(0, 191, 255)    ]
["Dim Gray"                 DimGray                 DIM_GRAY                    dim_gray                    "696969"    "696969FF"  RGB(105, 105, 105)  ]
["Dodger Blue"              DodgerBlue              DODGER_BLUE                 dodger_blue                 "1E90FF"    "1E90FFFF"  RGB(30, 144, 255)   ]
["Fire Brick"               FireBrick               FIRE_BRICK                  fire_brick                  "B22222"    "B22222FF"  RGB(178, 34, 34)    ]
["Floral White"             FloralWhite             FLORAL_WHITE                floral_white                "FFFAF0"    "FFFAF0FF"  RGB(255, 250, 240)  ]
["Forest Green"             ForestGreen             FOREST_GREEN                forest_green                "228B22"    "228B22FF"  RGB(34, 139, 34)    ]
["Fuchsia"                  Fuchsia                 FUCHSIA                     fuchsia                     "FF00FF"    "FF00FFFF"  RGB(255, 0, 255)    ]
["Gainsboro"                Gainsboro               GAINSBORO                   gainsboro                   "DCDCDC"    "DCDCDCFF"  RGB(220, 220, 220)  ]
["Ghost White"              GhostWhite              GHOST_WHITE                 ghost_white                 "F8F8FF"    "F8F8FFFF"  RGB(248, 248, 255)  ]
["Gold"                     Gold                    GOLD                        gold                        "FFD700"    "FFD700FF"  RGB(255, 215, 0)    ]
["Goldenrod"                Goldenrod               GOLDENROD                   goldenrod                   "DAA520"    "DAA520FF"  RGB(218, 165, 32)   ]
["Gray"                     Gray                    GRAY                        gray                        "808080"    "808080FF"  RGB(128, 128, 128)  ]
["Green"                    Green                   GREEN                       green                       "008000"    "008000FF"  RGB(0, 128, 0)      ]
["Green Yellow"             GreenYellow             GREEN_YELLOW                green_yellow                "ADFF2F"    "ADFF2FFF"  RGB(173, 255, 47)   ]
["Honeydew"                 Honeydew                HONEYDEW                    honeydew                    "F0FFF0"    "F0FFF0FF"  RGB(240, 255, 240)  ]
["Hot Pink"                 HotPink                 HOT_PINK                    hot_pink                    "FF69B4"    "FF69B4FF"  RGB(255, 105, 180)  ]
["Indian Red"               IndianRed               INDIAN_RED                  indian_red                  "CD5C5C"    "CD5C5CFF"  RGB(205, 92, 92)    ]
["Indigo"                   Indigo                  INDIGO                      indigo                      "4B0082"    "4B0082FF"  RGB(75, 0, 130)     ]
["Ivory"                    Ivory                   IVORY                       ivory                       "FFFFF0"    "FFFFF0FF"  RGB(255, 255, 240)  ]
["Khaki"                    Khaki                   KHAKI                       khaki                       "F0E68C"    "F0E68CFF"  RGB(240, 230, 140)  ]
["Lavender"                 Lavender                LAVENDER                    lavender                    "E6E6FA"    "E6E6FAFF"  RGB(230, 230, 250)  ]
["Lavender Blush"           LavenderBlush           LAVENDER_BLUSH              lavender_blush              "FFF0F5"    "FFF0F5FF"  RGB(255, 240, 245)  ]
["Lawn Green"               LawnGreen               LAWN_GREEN                  lawn_green                  "7CFC00"    "7CFC00FF"  RGB(124, 252, 0)    ]
["Lemon Chiffon"            LemonChiffon            LEMON_CHIFFON               lemon_chiffon               "FFFACD"    "FFFACDFF"  RGB(255, 250, 205)  ]
["Light Blue"               LightBlue               LIGHT_BLUE                  light_blue                  "ADD8E6"    "ADD8E6FF"  RGB(173, 216, 230)  ]
["Light Coral"              LightCoral              LIGHT_CORAL                 light_coral                 "F08080"    "F08080FF"  RGB(240, 128, 128)  ]
["Light Cyan"               LightCyan               LIGHT_CYAN                  light_cyan                  "E0FFFF"    "E0FFFFFF"  RGB(224, 255, 255)  ]
["Light Goldenrod Yellow"   LightGoldenrodYellow    LIGHT_GOLDENROD_YELLOW      light_goldenrod_yellow      "FAFAD2"    "FAFAD2FF"  RGB(250, 250, 210)  ]
["Light Gray"               LightGray               LIGHT_GRAY                  light_gray                  "D3D3D3"    "D3D3D3FF"  RGB(211, 211, 211)  ]
["Light Green"              LightGreen              LIGHT_GREEN                 light_green                 "90EE90"    "90EE90FF"  RGB(144, 238, 144)  ]
["Light Pink"               LightPink               LIGHT_PINK                  light_pink                  "FFB6C1"    "FFB6C1FF"  RGB(255, 182, 193)  ]
["Light Salmon"             LightSalmon             LIGHT_SALMON                light_salmon                "FFA07A"    "FFA07AFF"  RGB(255, 160, 122)  ]
["Light Sea Green"          LightSeaGreen           LIGHT_SEA_GREEN             light_sea_green             "20B2AA"    "20B2AAFF"  RGB(32, 178, 170)   ]
["Light Sky Blue"           LightSkyBlue            LIGHT_SKY_BLUE              light_sky_blue              "87CEFA"    "87CEFAFF"  RGB(135, 206, 250)  ]
["Light Slate Gray"         LightSlateGray          LIGHT_SLATE_GRAY            light_slate_gray            "778899"    "778899FF"  RGB(119, 136, 153)  ]
["Light Steel Blue"         LightSteelBlue          LIGHT_STEEL_BLUE            light_steel_blue            "B0C4DE"    "B0C4DEFF"  RGB(176, 196, 222)  ]
["Light Yellow"             LightYellow             LIGHT_YELLOW                light_yellow                "FFFFE0"    "FFFFE0FF"  RGB(255, 255, 224)  ]
["Lime"                     Lime                    LIME                        lime                        "00FF00"    "00FF00FF"  RGB(0, 255, 0)      ]
["Lime Green"               LimeGreen               LIME_GREEN                  lime_green                  "32CD32"    "32CD32FF"  RGB(50, 205, 50)    ]
["Linen"                    Linen                   LINEN                       linen                       "FAF0E6"    "FAF0E6FF"  RGB(250, 240, 230)  ]
["Magenta"                  Magenta                 MAGENTA                     magenta                     "FF00FF"    "FF00FFFF"  RGB(255, 0, 255)    ]
["Maroon"                   Maroon                  MAROON                      maroon                      "800000"    "800000FF"  RGB(128, 0, 0)      ]
["Medium Aquamarine"        MediumAquamarine        MEDIUM_AQUAMARINE           medium_aquamarine           "66CDAA"    "66CDAAFF"  RGB(102, 205, 170)  ]
["Medium Blue"              MediumBlue              MEDIUM_BLUE                 medium_blue                 "0000CD"    "0000CDFF"  RGB(0, 0, 205)      ]
["Medium Orchid"            MediumOrchid            MEDIUM_ORCHID               medium_orchid               "BA55D3"    "BA55D3FF"  RGB(186, 85, 211)   ]
["Medium Purple"            MediumPurple            MEDIUM_PURPLE               medium_purple               "9370DB"    "9370DBFF"  RGB(147, 112, 219)  ]
["Medium Sea Green"         MediumSeaGreen          MEDIUM_SEA_GREEN            medium_sea_green            "3CB371"    "3CB371FF"  RGB(60, 179, 113)   ]
["Medium Slate Blue"        MediumSlateBlue         MEDIUM_SLATE_BLUE           medium_slate_blue           "7B68EE"    "7B68EEFF"  RGB(123, 104, 238)  ]
["Medium Spring Green"      MediumSpringGreen       MEDIUM_SPRING_GREEN         medium_spring_green         "00FA9A"    "00FA9AFF"  RGB(0, 250, 154)    ]
["Medium Turquoise"         MediumTurquoise         MEDIUM_TURQUOISE            medium_turquoise            "48D1CC"    "48D1CCFF"  RGB(72, 209, 204)   ]
["Medium Violet Red"        MediumVioletRed         MEDIUM_VIOLET_RED           medium_violet_red           "C71585"    "C71585FF"  RGB(199, 21, 133)   ]
["Midnight Blue"            MidnightBlue            MIDNIGHT_BLUE               midnight_blue               "191970"    "191970FF"  RGB(25, 25, 112)    ]
["Mint Cream"               MintCream               MINT_CREAM                  mint_cream                  "F5FFFA"    "F5FFFAFF"  RGB(245, 255, 250)  ]
["Misty Rose"               MistyRose               MISTY_ROSE                  misty_rose                  "FFE4E1"    "FFE4E1FF"  RGB(255, 228, 225)  ]
["Moccasin"                 Moccasin                MOCCASIN                    moccasin                    "FFE4B5"    "FFE4B5FF"  RGB(255, 228, 181)  ]
["Navajo White"             NavajoWhite             NAVAJO_WHITE                navajo_white                "FFDEAD"    "FFDEADFF"  RGB(255, 222, 173)  ]
["Navy"                     Navy                    NAVY                        navy                        "000080"    "000080FF"  RGB(0, 0, 128)      ]
["Old Lace"                 OldLace                 OLD_LACE                    old_lace                    "FDF5E6"    "FDF5E6FF"  RGB(253, 245, 230)  ]
["Olive"                    Olive                   OLIVE                       olive                       "808000"    "808000FF"  RGB(128, 128, 0)    ]
["Olive Drab"               OliveDrab               OLIVE_DRAB                  olive_drab                  "6B8E23"    "6B8E23FF"  RGB(107, 142, 35)   ]
["Orange"                   Orange                  ORANGE                      orange                      "FFA500"    "FFA500FF"  RGB(255, 165, 0)    ]
["Orange Red"               OrangeRed               ORANGE_RED                  orange_red                  "FF4500"    "FF4500FF"  RGB(255, 69, 0)     ]
["Orchid"                   Orchid                  ORCHID                      orchid                      "DA70D6"    "DA70D6FF"  RGB(218, 112, 214)  ]
["Pale Goldenrod"           PaleGoldenrod           PALE_GOLDENROD              pale_goldenrod              "EEE8AA"    "EEE8AAFF"  RGB(238, 232, 170)  ]
["Pale Green"               PaleGreen               PALE_GREEN                  pale_green                  "98FB98"    "98FB98FF"  RGB(152, 251, 152)  ]
["Pale Turquoise"           PaleTurquoise           PALE_TURQUOISE              pale_turquoise              "AFEEEE"    "AFEEEEFF"  RGB(175, 238, 238)  ]
["Pale Violet Red"          PaleVioletRed           PALE_VIOLET_RED             pale_violet_red             "DB7093"    "DB7093FF"  RGB(219, 112, 147)  ]
["Papaya Whip"              PapayaWhip              PAPAYA_WHIP                 papaya_whip                 "FFEFD5"    "FFEFD5FF"  RGB(255, 239, 213)  ]
["Peach Puff"               PeachPuff               PEACH_PUFF                  peach_puff                  "FFDAB9"    "FFDAB9FF"  RGB(255, 218, 185)  ]
["Peru"                     Peru                    PERU                        peru                        "CD853F"    "CD853FFF"  RGB(205, 133, 63)   ]
["Pink"                     Pink                    PINK                        pink                        "FFC0CB"    "FFC0CBFF"  RGB(255, 192, 203)  ]
["Plum"                     Plum                    PLUM                        plum                        "DDA0DD"    "DDA0DDFF"  RGB(221, 160, 221)  ]
["Powder Blue"              PowderBlue              POWDER_BLUE                 powder_blue                 "B0E0E6"    "B0E0E6FF"  RGB(176, 224, 230)  ]
["Purple"                   Purple                  PURPLE                      purple                      "800080"    "800080FF"  RGB(128, 0, 128)    ]
["Red"                      Red                     RED                         red                         "FF0000"    "FF0000FF"  RGB(255, 0, 0)      ]
["Rosy Brown"               RosyBrown               ROSY_BROWN                  rosy_brown                  "BC8F8F"    "BC8F8FFF"  RGB(188, 143, 143)  ]
["Royal Blue"               RoyalBlue               ROYAL_BLUE                  royal_blue                  "4169E1"    "4169E1FF"  RGB(65, 105, 225)   ]
["Saddle Brown"             SaddleBrown             SADDLE_BROWN                saddle_brown                "8B4513"    "8B4513FF"  RGB(139, 69, 19)    ]
["Salmon"                   Salmon                  SALMON                      salmon                      "FA8072"    "FA8072FF"  RGB(250, 128, 114)  ]
["Sandy Brown"              SandyBrown              SANDY_BROWN                 sandy_brown                 "F4A460"    "F4A460FF"  RGB(244, 164, 96)   ]
["Sea Green"                SeaGreen                SEA_GREEN                   sea_green                   "2E8B57"    "2E8B57FF"  RGB(46, 139, 87)    ]
["Seashell"                 Seashell                SEASHELL                    seashell                    "FFF5EE"    "FFF5EEFF"  RGB(255, 245, 238)  ]
["Sienna"                   Sienna                  SIENNA                      sienna                      "A0522D"    "A0522DFF"  RGB(160, 82, 45)    ]
["Silver"                   Silver                  SILVER                      silver                      "C0C0C0"    "C0C0C0FF"  RGB(192, 192, 192)  ]
["Sky Blue"                 SkyBlue                 SKY_BLUE                    sky_blue                    "87CEEB"    "87CEEBFF"  RGB(135, 206, 235)  ]
["Slate Blue"               SlateBlue               SLATE_BLUE                  slate_blue                  "6A5ACD"    "6A5ACDFF"  RGB(106, 90, 205)   ]
["Slate Gray"               SlateGray               SLATE_GRAY                  slate_gray                  "708090"    "708090FF"  RGB(112, 128, 144)  ]
["Snow"                     Snow                    SNOW                        snow                        "FFFAFA"    "FFFAFAFF"  RGB(255, 250, 250)  ]
["Spring Green"             SpringGreen             SPRING_GREEN                spring_green                "00FF7F"    "00FF7FFF"  RGB(0, 255, 127)    ]
["Steel Blue"               SteelBlue               STEEL_BLUE                  steel_blue                  "4682B4"    "4682B4FF"  RGB(70, 130, 180)   ]
["Tan"                      Tan                     TAN                         tan                         "D2B48C"    "D2B48CFF"  RGB(210, 180, 140)  ]
["Teal"                     Teal                    TEAL                        teal                        "008080"    "008080FF"  RGB(0, 128, 128)    ]
["Thistle"                  Thistle                 THISTLE                     thistle                     "D8BFD8"    "D8BFD8FF"  RGB(216, 191, 216)  ]
["Tomato"                   Tomato                  TOMATO                      tomato                      "FF6347"    "FF6347FF"  RGB(255, 99, 71)    ]
["Turquoise"                Turquoise               TURQUOISE                   turquoise                   "40E0D0"    "40E0D0FF"  RGB(64, 224, 208)   ]
["Violet"                   Violet                  VIOLET                      violet                      "EE82EE"    "EE82EEFF"  RGB(238, 130, 238)  ]
["Wheat"                    Wheat                   WHEAT                       wheat                       "F5DEB3"    "F5DEB3FF"  RGB(245, 222, 179)  ]
["White"                    White                   WHITE                       white                       "FFFFFF"    "FFFFFFFF"  RGB(255, 255, 255)  ]
["White Smoke"              WhiteSmoke              WHITE_SMOKE                 white_smoke                 "F5F5F5"    "F5F5F5FF"  RGB(245, 245, 245)  ]
["Yellow"                   Yellow                  YELLOW                      yellow                      "FFFF00"    "FFFF00FF"  RGB(255, 255, 0)    ]
["Yellow Green"             YellowGreen             YELLOW_GREEN                yellow_green                "9ACD32"    "9ACD32FF"  RGB(154, 205, 50)   ]
        }
    };
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
    };
}

impl Color {
    #[inline]
    pub const fn index(self) -> usize {
        self as usize
    }

    #[inline]
    pub const fn camel_name(self) -> &'static str {
        HTML_COLORS[self.index()].camel_name
    }

    #[inline]
    pub const fn const_name(self) -> &'static str {
        HTML_COLORS[self.index()].const_name
    }

    #[inline]
    pub const fn var_name(self) -> &'static str {
        HTML_COLORS[self.index()].var_name
    }

    #[inline]
    pub const fn rgb(self) -> Rgb {
        HTML_COLORS[self.index()].rgb
    }

    #[inline]
    pub const fn readable_name(self) -> &'static str {
        HTML_COLORS[self.index()].readable_name
    }

    #[inline]
    pub const fn from_byte(byte: u8) -> Option<Self> {
        let index = byte as usize;
        if index < HTML_COLORS.len() {
            Some(HTML_COLORS[index].color)
        } else {
            None
        }
    }

    #[inline]
    pub const fn to_byte(self) -> u8 {
        self as u8
    }

    pub fn iter() -> impl Iterator<Item = Self> {
        HTML_COLORS.iter().map(|entry| entry.color)
    }
}

html_colors!(color_enum);

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
        Rgba::new(self.r, self.g, self.b, 255)
    }

    #[inline]
    pub const fn transparent(self) -> Rgba {
        Rgba::new(self.r, self.g, self.b, 0)
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
        Self::new(gray, gray, gray, gray)
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
    pub const WHITE: Self = Self::new(255);
    pub const BLACK: Self = Self::new(0);

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
    pub color: Color,
    pub rgb: Rgb,
}

impl ColorEntry {
    pub const fn new(
        readable_name: &'static str,
        camel_name: &'static str,
        const_name: &'static str,
        var_name: &'static str,
        color: Color,
        rgb: Rgb
    ) -> Self {
        Self {
            readable_name,
            camel_name,
            const_name,
            var_name,
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
                ColorEntry::new($readable_name, stringify!($camel_name), stringify!($const_name), stringify!($var_name), Color::$camel_name, Rgb::new($r, $g, $b)),
            )+
        ];
    };
}

html_colors!(color_table);

#[cfg(test)]
mod testing_sandbox {
    // TODO: Remove this sandbox when it is no longer in use.
    use super::*;
    #[test]
    fn sandbox() {
        const CB: &'static str = Color::CornflowerBlue.readable_name();
        println!("{}", CB);
    }
}