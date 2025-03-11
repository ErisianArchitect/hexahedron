
/* What should a Scene do?
- Load/Unload
- Transition
- Pause/Resume
- Update Input
- Update
- Render
*/

use std::sync::{Arc, Mutex};

use super::Engine;


#[allow(unused)]
pub trait Scene {

    fn load(
        &mut self,
        engine: &Engine,
    ) {}

    fn unload(
        &mut self,
        engine: &Engine,
    ) {}

    fn resize(
        &mut self,
        engine: &Engine,
        new_size: winit::dpi::PhysicalSize<u32>,
    ) {}

    fn focus_changed(
        &mut self,
        engine: &Engine,
        focused: bool,
    ) {}

    fn close_requested(
        &mut self,
        engine: &Engine,
    ) -> bool {
        true
    }

    fn process_event(
        &mut self,
        engine: &Engine,
        event: &winit::event::Event<()>,
    ) {}

    fn process_window_event(
        &mut self,
        engine: &Engine,
        event: &winit::event::WindowEvent,
    ) -> bool {
        false
    }

    fn begin_frame(
        &mut self,
        engine: &Engine,
        frame_index: u64,
    ) {}

    fn end_frame(
        &mut self,
        engine: &Engine,
        frame_index: u64,
    ) {}

    fn begin_update(
        &mut self,
        engine: &Engine,
        frame_index: u64,
    ) {}

    fn update(
        &mut self,
        engine: &Engine,
        frame_index: u64,
    ) {}

    fn end_update(
        &mut self,
        engine: &Engine,
        frame_index: u64,
    ) {}

    fn begin_render(
        &mut self,
        engine: &Engine,
        frame_index: u64,
    ) {}

    fn render(
        &mut self,
        engine: &Engine,
        frame_index: u64,
    ) -> Result<(), wgpu::SurfaceError> {
        Ok(())
    }

    fn end_render(
        &mut self,
        engine: &Engine,
        frame_index: u64,
    ) {}
}

#[derive(Clone)]
pub struct SharedScene {
    scene: Arc<Mutex<dyn Scene + 'static>>,
}

const SCENE_LOCK_FAILURE: &'static str = "Failed to lock scene.";

impl<T: Scene + 'static> From<T> for SharedScene {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl SharedScene {
    pub fn new<S: Scene + 'static>(scene: S) -> Self {
        Self {
            scene: Arc::new(Mutex::new(scene)),
        }
    }

    pub fn load(
        &self,
        engine: &Engine,
    ) {
        let mut scene_lock = self.scene.lock().expect(SCENE_LOCK_FAILURE);
        scene_lock.load(engine);
    }

    pub fn unload(
        &self,
        engine: &Engine,
    ) {
        let mut scene_lock = self.scene.lock().expect(SCENE_LOCK_FAILURE);
        scene_lock.unload(engine);
    }

    pub fn resize(
        &self,
        engine: &Engine,
        new_size: winit::dpi::PhysicalSize<u32>,
    ) {
        let mut scene_lock = self.scene.lock().expect(SCENE_LOCK_FAILURE);
        scene_lock.resize(engine, new_size);
    }

    pub fn focus_changed(
        &self,
        engine: &Engine,
        focused: bool,
    ) {
        let mut scene_lock = self.scene.lock().expect(SCENE_LOCK_FAILURE);
        scene_lock.focus_changed(engine, focused);
    }

    pub fn close_requested(
        &self,
        engine: &Engine,
    ) -> bool {
        let mut scene_lock = self.scene.lock().expect(SCENE_LOCK_FAILURE);
        scene_lock.close_requested(engine)
    }

    pub fn process_event(
        &self,
        engine: &Engine,
        event: &winit::event::Event<()>,
    ) {
        let mut scene_lock = self.scene.lock().expect(SCENE_LOCK_FAILURE);
        scene_lock.process_event(engine, event);
    }

    pub fn process_window_event(
        &self,
        engine: &Engine,
        event: &winit::event::WindowEvent,
    ) -> bool {
        let mut scene_lock = self.scene.lock().expect(SCENE_LOCK_FAILURE);
        scene_lock.process_window_event(engine, event)
    }

    pub fn begin_frame(
        &self,
        engine: &Engine,
        frame_index: u64,
    ) {
        let mut scene_lock = self.scene.lock().expect(SCENE_LOCK_FAILURE);
        scene_lock.begin_frame(engine, frame_index);
    }

    pub fn end_frame(
        &self,
        engine: &Engine,
        frame_index: u64,
    ) {
        let mut scene_lock = self.scene.lock().expect(SCENE_LOCK_FAILURE);
        scene_lock.end_frame(engine, frame_index);
    }

    pub fn begin_update(
        &self,
        engine: &Engine,
        frame_index: u64,
    ) {
        let mut scene_lock = self.scene.lock().expect(SCENE_LOCK_FAILURE);
        scene_lock.begin_update(engine, frame_index);
    }

    pub fn update(
        &self,
        engine: &Engine,
        frame_index: u64,
    ) {
        let mut scene_lock = self.scene.lock().expect(SCENE_LOCK_FAILURE);
        scene_lock.update(engine, frame_index);
    }

    pub fn end_update(
        &self,
        engine: &Engine,
        frame_index: u64,
    ) {
        let mut scene_lock = self.scene.lock().expect(SCENE_LOCK_FAILURE);
        scene_lock.end_update(engine, frame_index);
    }

    pub fn begin_render(
        &self,
        engine: &Engine,
        frame_index: u64,
    ) {
        let mut scene_lock = self.scene.lock().expect(SCENE_LOCK_FAILURE);
        scene_lock.begin_render(engine, frame_index);
    }

    pub fn render(
        &self,
        engine: &Engine,
        frame_index: u64,
    ) -> Result<(), wgpu::SurfaceError> {
        let mut scene_lock = self.scene.lock().expect(SCENE_LOCK_FAILURE);
        scene_lock.render(engine, frame_index)
    }

    pub fn end_render(
        &self,
        engine: &Engine,
        frame_index: u64,
    ) {
        let mut scene_lock = self.scene.lock().expect(SCENE_LOCK_FAILURE);
        scene_lock.end_render(engine, frame_index);
    }
}