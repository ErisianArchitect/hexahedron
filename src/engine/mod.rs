

/*
Okay, so the engine should handle everything basically.
So it needs to manage "Scenes"
*/

use std::sync::{Arc, Mutex};

use hashbrown::HashMap;
use scene::SharedScene;

pub mod messaging;
pub mod scene;
pub mod scene_graph;

pub struct Engine<'a> {
    pub surface: wgpu::Surface<'a>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub render_pipeline: wgpu::RenderPipeline,
    pub window: winit::window::Window,
    pub monitor: winit::monitor::MonitorHandle,
    // Scenes
    pub scenes: Arc<Mutex<HashMap<String, SharedScene>>>,
}

pub struct EngineBuilder {
    inner_size: Option<winit::dpi::Size>,
    title: Option<String>,
}

/* TODO List
- Implement frame pacing.
*/

impl<'a> Engine<'a> {
    pub fn new<T, S>(
        title: T,
        inner_size: Option<S>,
    ) -> Self
    where
        T: Into<String>,
        S: Into<winit::dpi::Size>,
    {
        todo!()
    }

    pub fn run(&self) {
        
    }
}