
/* What should a Scene do?
- Load/Unload
- Transition
- Pause/Resume
- Update Input
- Update
- Render
*/

use std::any::Any;
use std::{rc::Rc, sync::atomic::AtomicBool};
use std::cell::{RefCell, UnsafeCell};

use super::{Engine, EventPropagation, frames::frame_info::FrameInfo};


#[allow(unused)]
pub trait Scene: Any {

    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;

    fn load(
        &mut self,
        engine: &Engine,
    ) {}

    fn unload(
        &mut self,
        engine: &Engine,
    ) {}

    fn on_active_changed(
        &mut self,
        engine: &Engine,
        active: bool,
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

struct SceneCell {
    scene: Box<UnsafeCell<dyn Scene + 'static>>,
    active: AtomicBool,
}

impl SceneCell {
    #[inline]
    pub fn new<S: Scene + 'static>(scene: S, active: bool) -> Self {
        Self {
            scene: Box::new(UnsafeCell::new(scene)),
            active: AtomicBool::new(active),
        }
    }

    #[inline]
    fn get_scene_mut(&self) -> &mut dyn Scene {
        unsafe {
            self.scene.get().as_mut().unwrap()
        }
    }

    #[inline]
    fn set_active(&self, active: bool) {
        self.active.store(active, std::sync::atomic::Ordering::Relaxed);
    }
}

#[derive(Clone)]
pub struct SharedScene {
    cell: Rc<SceneCell>,
}

impl<T: Scene + 'static> From<T> for SharedScene {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl SharedScene {
    #[inline]
    pub fn new<S: Scene + 'static + Sized>(scene: S) -> Self {
        Self {
            cell: Rc::new(SceneCell::new(scene, true)),
        }
    }

    #[inline]
    pub fn set_active(&self, engine: &Engine, active: bool) {
        self.cell.set_active(active);
        self.on_active_changed(engine, active);
    }

    #[inline]
    fn on_active_changed(&self, engine: &Engine, active: bool) {
        self.cell.get_scene_mut().on_active_changed(engine, active);
    }

    #[inline]
    pub fn load(
        &self,
        engine: &Engine,
    ) {
        self.cell.get_scene_mut().load(engine);
    }

    #[inline]
    pub fn unload(
        &self,
        engine: &Engine,
    ) {
        self.cell.get_scene_mut().unload(engine);
    }

    #[inline]
    pub fn resize(
        &self,
        engine: &Engine,
        new_size: winit::dpi::PhysicalSize<u32>,
    ) {
        self.cell.get_scene_mut().resize(engine, new_size);
    }

    #[inline]
    pub fn focus_changed(
        &self,
        engine: &Engine,
        focused: bool,
    ) {
        self.cell.get_scene_mut().focus_changed(engine, focused);
    }

    #[inline]
    pub fn close_requested(
        &self,
        engine: &Engine,
    ) -> bool {
        self.cell.get_scene_mut().close_requested(engine)
    }

    #[inline]
    pub fn process_event(
        &self,
        engine: &Engine,
        event: &winit::event::Event<()>,
    ) {
        self.cell.get_scene_mut().process_event(engine, event);
    }

    #[inline]
    pub fn process_window_event(
        &self,
        engine: &Engine,
        event: &winit::event::WindowEvent,
    ) -> EventPropagation {
        self.cell.get_scene_mut().process_window_event(engine, event)
    }

    #[inline]
    pub fn begin_frame(
        &self,
        engine: &Engine,
        frame_index: u64,
    ) {
        self.cell.get_scene_mut().begin_frame(engine, frame_index);
    }

    #[inline]
    pub fn end_frame(
        &self,
        engine: &Engine,
        frame_index: u64,
    ) {
        self.cell.get_scene_mut().end_frame(engine, frame_index);
    }

    #[inline]
    pub fn begin_update(
        &self,
        engine: &Engine,
        frame: &FrameInfo,
    ) {
        self.cell.get_scene_mut().begin_update(engine, frame);
    }

    #[inline]
    pub fn update(
        &self,
        engine: &Engine,
        frame: &FrameInfo,
    ) {
        self.cell.get_scene_mut().update(engine, frame);
    }

    #[inline]
    pub fn end_update(
        &self,
        engine: &Engine,
        frame: &FrameInfo,
    ) {
        self.cell.get_scene_mut().end_update(engine, frame);
    }

    #[inline]
    pub fn begin_render(
        &self,
        engine: &Engine,
        frame: &FrameInfo,
    ) {
        self.cell.get_scene_mut().begin_render(engine, frame);
    }

    #[inline]
    pub fn render(
        &self,
        engine: &Engine,
        frame: &FrameInfo,
    ) -> Result<(), wgpu::SurfaceError> {
        self.cell.get_scene_mut().render(engine, frame)
    }

    #[inline]
    pub fn end_render(
        &self,
        engine: &Engine,
        frame: &FrameInfo,
    ) {
        self.cell.get_scene_mut().end_render(engine, frame);
    }

    #[inline]
    pub fn cast<S: Scene>(&self) -> Option<&S> {
        unsafe {
            let scene_obj: &dyn Scene = &*self.cell.scene.get();
            scene_obj.as_any().downcast_ref()
        }
    }

    #[inline]
    pub unsafe fn cast_mut<S: Scene>(&self) -> Option<&mut S> {
        unsafe {
            let scene_obj: &mut dyn Scene = &mut *self.cell.scene.get();
            scene_obj.as_any_mut().downcast_mut()
        }
    }
}

#[cfg(test)]
mod tests {
    use std::mem::MaybeUninit;

    use super::*;
    #[test]
    fn scene_test() {
        struct TestScene {
            value: String,
        }
        impl Scene for TestScene {
            fn as_any(&self) -> &dyn Any {
                self
            }

            fn as_any_mut(&mut self) -> &mut dyn Any {
                self
            }

            fn load(
                    &mut self,
                    engine: &Engine,
                ) {
                self.value = "hello, world".to_owned();
            }

            fn unload(
                    &mut self,
                    engine: &Engine,
                ) {
                println!("{}", self.value);
            }
        }
        let scene = SharedScene::new(TestScene { value: String::new() });
        unsafe {
            let engine = MaybeUninit::uninit().assume_init();
            scene.load(&engine);
            scene.unload(&engine);
            unsafe {
                scene.cast_mut::<TestScene>().unwrap().value = "this is a test.".to_owned();
            }
            scene.unload(&engine);
            std::mem::forget(engine);
        }
    }
}