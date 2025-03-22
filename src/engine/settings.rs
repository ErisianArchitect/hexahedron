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

impl From<(u32, u32)> for Resolution {
    fn from(value: (u32, u32)) -> Self {
        match (value.0, value.1) {
            (640, 480) => Resolution::SD,
            (1280, 720) => Resolution::HD,
            (1920, 1080) => Resolution::HD,
            (2560, 1440) => Resolution::QHD,
            (3480, 2160) => Resolution::Ultra,
            (7680, 4320) => Resolution::EightK,
            (width, height) => Resolution::Custom(PhysicalSize::new(width, height))
        }
    }
}

impl From<PhysicalSize<u32>> for Resolution {
    fn from(value: PhysicalSize<u32>) -> Self {
        Self::from((value.width, value.height))
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Fullscreen {
    #[default]
    Exclusive,
    Borderless,
}

#[derive(Debug, Clone)]
pub struct EngineSettings<T = String, R = Resolution, F = Option<u32>> {
    pub prefered_present_mode: wgpu::PresentMode,
    pub max_framerate: F,
    pub preferred_resolution: R,
    pub fullscreen: bool,
    pub title: T,
}

impl Default for EngineSettings {
    fn default() -> Self {
        Self {
            prefered_present_mode: wgpu::PresentMode::Fifo,
            preferred_resolution: Resolution::Window,
            max_framerate: None,
            fullscreen: false,
            title: "Hexahedron Game".to_owned(),
        }
    }
}

impl<T: Into<String>, R: Into<Resolution>, F: Into<Option<u32>>> EngineSettings<T, R, F> {
    pub fn resolve(self) -> EngineSettings<String, Resolution, Option<u32>> {
        EngineSettings {
            prefered_present_mode: self.prefered_present_mode,
            fullscreen: self.fullscreen,
            preferred_resolution: self.preferred_resolution.into(),
            title: self.title.into(),
            max_framerate: self.max_framerate.into(),
        }
    }
}