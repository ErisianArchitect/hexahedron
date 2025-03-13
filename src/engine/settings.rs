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

#[derive(Debug, Clone)]
pub struct EngineSettings {
    pub vsync: bool,
    pub preferred_resolution: Resolution,
    pub fullscreen: bool,
    pub title: String,
}

impl Default for EngineSettings {
    fn default() -> Self {
        Self {
            vsync: true,
            preferred_resolution: Resolution::Window,
            fullscreen: false,
            title: String::from("Hexahedron Game"),
        }
    }
}