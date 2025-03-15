use winit::dpi::PhysicalSize;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Resolution {
    /// Inherit resolution from the window.
    #[default]
    Window,
    /// 480p (640x480)
    SD,
    /// 720p (1280x720)
    HD,
    /// 1080p (1920x1080)
    FHD,
    /// 1440p (2560x1440)
    QHD,
    /// 4K (3480x2160)
    Ultra,
    /// 8K (7680x4320)
    EightK,
    Custom(PhysicalSize<u32>),
}

impl Resolution {
    pub fn size(self, window: &winit::window::Window) -> PhysicalSize<u32> {
        match self {
            Resolution::Window => window.inner_size(),
            Resolution::SD => PhysicalSize::new(640, 480),
            Resolution::HD => PhysicalSize::new(1280, 720),
            Resolution::FHD => PhysicalSize::new(1920, 1080),
            Resolution::QHD => PhysicalSize::new(2560, 1440),
            Resolution::Ultra => PhysicalSize::new(3480, 2160),
            Resolution::EightK => PhysicalSize::new(7680, 4320),
            Resolution::Custom(physical_size) => physical_size,
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Fullscreen {
    #[default]
    Exclusive,
    Borderless,
}

#[derive(Debug, Clone)]
pub struct EngineSettings {
    pub vsync: bool,
    pub max_framerate: Option<u32>,
    pub preferred_resolution: Resolution,
    pub fullscreen: bool,
    pub title: String,
}

impl Default for EngineSettings {
    fn default() -> Self {
        Self {
            vsync: true,
            preferred_resolution: Resolution::Window,
            max_framerate: None,
            fullscreen: false,
            title: String::from("Hexahedron Game"),
        }
    }
}