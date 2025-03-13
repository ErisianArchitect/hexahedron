// std
use std::{sync::{Arc, Mutex}, time::{Duration, Instant}};
// std extensions
use hashbrown::HashMap;
// external
use winit::{
    dpi::PhysicalSize, event_loop::EventLoop, monitor::MonitorHandle, window::{Window, WindowBuilder}
};
use wgpu::{
    Surface,
    Device,
    Queue,
    SurfaceConfiguration,
};
// internal
use hexcore::extensions::*;
use hexmath::average::{
    AverageBuffer,
    AvgBuffer,
};
use super::{frames::frame_info::FrameInfo, scene::SharedScene, settings::EngineSettings};

pub struct Engine<'a> {
    // Window Fields
    /// The [Window] reference for this [Engine] instance.
    pub window: &'a Window,
    pub monitor: MonitorHandle,
    pub size: PhysicalSize<u32>,
    // WGPU Fields
    pub surface: Surface<'a>,
    pub device: Device,
    pub queue: Queue,
    pub config: SurfaceConfiguration,
    // Engine Fields
    pub scenes: (),
    pub engine_settings: EngineSettings,
}

pub struct EngineRunner<'a> {
    engine: Option<Engine<'a>>,
}

/* TODO List
- Implement frame pacing.
*/

impl<'a> Engine<'a> {
    pub fn new(
        window: &'a Window,
        settings: EngineSettings,
    ) -> Self {
        // Create window and EventLoop
        // Create WGPU context

        let monitor = window.current_monitor().unwrap();
        let size = window.inner_size();
        let aspect_ratio = size.width as f32 / size.height as f32;

        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        });

        let surface = instance.create_surface(window).unwrap();

        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        })).unwrap();

        let limits = wgpu::Limits {
            max_push_constant_size: 256,
            ..Default::default()
        };

        let (device, queue) = pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                required_features: wgpu::Features::PUSH_CONSTANTS,
                required_limits: limits,
                label: Some("WGPU Device"),
                memory_hints: wgpu::MemoryHints::Performance,
            },
            None,
        )).unwrap();

        // Surface Caps/Format
        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps.formats.iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            // enable vsync: (PresentMode::Fifo)
            present_mode: settings.vsync.select(
                wgpu::PresentMode::Fifo,
                wgpu::PresentMode::Immediate,
            ),
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        Self {
            window,
            monitor,
            size,
            surface,
            device,
            queue,
            config,
            scenes: (),
            engine_settings: settings,
        }
    }

    pub fn run(
        settings: EngineSettings,
    ) {
        #![allow(unused)]
        let mut event_loop = EventLoop::new().unwrap();
        event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);

        let window = WindowBuilder::new()
            .with_title(&settings.title)
            .build(&event_loop).unwrap();
        let mut engine = Engine::new(&window, settings);
        let refresh_rate = if let Some(refresh_mhz) = engine.monitor.refresh_rate_millihertz() {
            Some(Duration::from_millis(refresh_mhz as u64))
        } else {
            None
        };
        let mut frametime_averager = hexmath::average::AverageBuffer::<Duration>::new(60, refresh_rate);
        let mut frame = FrameInfo {
            frame_index: 0,
            // TODO: Adjust average_fps and delta_time according to refresh rate.
            average_fps: 1.0 / frametime_averager.average().as_secs_f64(),
            delta_time: Duration::ZERO,
            runtime: Duration::ZERO,
        };
        let start_time = Instant::now();
        
    }
}