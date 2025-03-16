
/* What should a Scene do?
- Load/Unload
- Transition
- Pause/Resume
- Update Input
- Update
- Render
*/

use std::rc::Rc;
use std::cell::RefCell;

use super::{Engine, EventPropagation, frames::frame_info::FrameInfo};


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

    fn minimized_changed(
        &mut self,
        engine: &Engine,
        minimized: bool,
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
    ) -> EventPropagation {
        EventPropagation::Propagate
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

    fn begin_fixed_update(
        &mut self,
        engine: &Engine,
        frame: &FrameInfo,
    ) {}

    fn fixed_update(
        &mut self,
        engine: &Engine,
        frame: &FrameInfo,
    ) {}

    fn end_fixed_update(
        &mut self,
        engine: &Engine,
        frame: &FrameInfo,
    ) {}

    fn begin_update(
        &mut self,
        engine: &Engine,
        frame: &FrameInfo,
    ) {}

    fn update(
        &mut self,
        engine: &Engine,
        frame: &FrameInfo,
    ) {}

    fn end_update(
        &mut self,
        engine: &Engine,
        frame: &FrameInfo,
    ) {}

    fn begin_render(
        &mut self,
        engine: &Engine,
        frame: &FrameInfo,
    ) {}

    fn render(
        &mut self,
        engine: &Engine,
        frame: &FrameInfo,
    ) -> Result<(), wgpu::SurfaceError> {
        Ok(())
    }

    fn end_render(
        &mut self,
        engine: &Engine,
        frame: &FrameInfo,
    ) {}
}

#[derive(Clone)]
pub struct SharedScene {
    scene: Rc<RefCell<dyn Scene + 'static>>,
}

impl<T: Scene + 'static> From<T> for SharedScene {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl SharedScene {
    pub fn new<S: Scene + 'static>(scene: S) -> Self {
        Self {
            scene: Rc::new(RefCell::new(scene)),
        }
    }

    pub fn load(
        &self,
        engine: &Engine,
    ) {
        let mut scene = self.scene.borrow_mut();
        scene.load(engine);
    }

    pub fn unload(
        &self,
        engine: &Engine,
    ) {
        let mut scene = self.scene.borrow_mut();
        scene.unload(engine);
    }

    pub fn resize(
        &self,
        engine: &Engine,
        new_size: winit::dpi::PhysicalSize<u32>,
    ) {
        let mut scene = self.scene.borrow_mut();
        scene.resize(engine, new_size);
    }

    pub fn focus_changed(
        &self,
        engine: &Engine,
        focused: bool,
    ) {
        let mut scene = self.scene.borrow_mut();
        scene.focus_changed(engine, focused);
    }

    pub fn close_requested(
        &self,
        engine: &Engine,
    ) -> bool {
        let mut scene = self.scene.borrow_mut();
        scene.close_requested(engine)
    }

    pub fn process_event(
        &self,
        engine: &Engine,
        event: &winit::event::Event<()>,
    ) {
        let mut scene = self.scene.borrow_mut();
        scene.process_event(engine, event);
    }

    pub fn process_window_event(
        &self,
        engine: &Engine,
        event: &winit::event::WindowEvent,
    ) -> EventPropagation {
        let mut scene = self.scene.borrow_mut();
        scene.process_window_event(engine, event)
    }

    pub fn begin_frame(
        &self,
        engine: &Engine,
        frame_index: u64,
    ) {
        let mut scene = self.scene.borrow_mut();
        scene.begin_frame(engine, frame_index);
    }

    pub fn end_frame(
        &self,
        engine: &Engine,
        frame_index: u64,
    ) {
        let mut scene = self.scene.borrow_mut();
        scene.end_frame(engine, frame_index);
    }

    pub fn begin_update(
        &self,
        engine: &Engine,
        frame: &FrameInfo,
    ) {
        let mut scene = self.scene.borrow_mut();
        scene.begin_update(engine, frame);
    }

    pub fn update(
        &self,
        engine: &Engine,
        frame: &FrameInfo,
    ) {
        let mut scene = self.scene.borrow_mut();
        scene.update(engine, frame);
    }

    pub fn end_update(
        &self,
        engine: &Engine,
        frame: &FrameInfo,
    ) {
        let mut scene = self.scene.borrow_mut();
        scene.end_update(engine, frame);
    }

    pub fn begin_render(
        &self,
        engine: &Engine,
        frame: &FrameInfo,
    ) {
        let mut scene = self.scene.borrow_mut();
        scene.begin_render(engine, frame);
    }

    pub fn render(
        &self,
        engine: &Engine,
        frame: &FrameInfo,
    ) -> Result<(), wgpu::SurfaceError> {
        let mut scene = self.scene.borrow_mut();
        scene.render(engine, frame)
    }

    pub fn end_render(
        &self,
        engine: &Engine,
        frame: &FrameInfo,
    ) {
        let mut scene = self.scene.borrow_mut();
        scene.end_render(engine, frame);
    }
}