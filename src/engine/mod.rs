// TODO: Remove this eventually
#![allow(unused)]


/*
Okay, so the engine should handle everything basically.
So it needs to manage "Scenes"
*/

pub mod messaging;
pub mod scene;
pub mod scene_graph;
// pub mod engine;
pub mod settings;
pub mod frames;
pub mod render_frame;

// std
use std::{cell::RefCell, io::Write, rc::Rc, sync::{Arc, Mutex}, time::{Duration, Instant}};
use chrono::{DateTime, Local};
use frames::framespace::{FrameSpace, Spacing};
// std extensions
use hashbrown::HashMap;
use settings::Resolution;
// external
use winit::{
    dpi::{PhysicalPosition, PhysicalSize, Position}, event::{ElementState, Event, KeyEvent, WindowEvent}, event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget}, keyboard::KeyCode, monitor::MonitorHandle, window::{Window, WindowBuilder}
};
use wgpu::{
    Surface,
    Device,
    Queue,
    SurfaceConfiguration,
};
// internal
use hexcore::{extensions::*, time::timer::Timer};
use hexmath::average::{
    AverageBuffer,
    AvgBuffer,
};
use self::{frames::{frame_info::{FrameInfo, FrameTime}, framespace::FrameLimiter}, scene::SharedScene, settings::EngineSettings};

type RcCell<T> = Rc<RefCell<T>>;

pub fn rc_cell<T>(value: T) -> RcCell<T> {
    Rc::new(RefCell::new(value))
}

#[derive(Debug, Default, Clone)]
pub struct WindowState {
    pub fullscreen: bool,
    pub focused: bool,
    pub minimized: bool,
}

pub struct GameLoopState {
    pub frame: FrameInfo,
    pub refresh_rate: Duration,
    pub start_time: DateTime<chrono::Local>,
}

pub struct Engine<'a> {
    // Window Fields
    /// The [Window] reference for this [Engine] instance.
    pub window: &'a Window,
    pub monitor: MonitorHandle,
    pub size: PhysicalSize<u32>,
    pub window_state: Rc<RefCell<WindowState>>,
    // WGPU Fields
    pub device: Device,
    pub queue: Queue,
    pub surface: Surface<'a>,
    pub config: SurfaceConfiguration,
    // Engine Fields
    pub scenes: (),
    pub engine_settings: Rc<RefCell<EngineSettings>>,
    pub loop_state: Rc<RefCell<GameLoopState>>,
    gamepad: gilrs::Gilrs,
}

#[derive(Debug, Clone, Copy)]
pub struct Control<'a> {
    control_flow: &'a EventLoopWindowTarget<()>,
    request_exit: &'a std::sync::atomic::AtomicBool,
}

impl<'a> Control<'a> {
    pub fn new(
        control_flow: &'a EventLoopWindowTarget<()>,
        request_exit: &'a std::sync::atomic::AtomicBool,
    ) -> Self {
        Self {
            control_flow,
            request_exit,
        }
    }

    pub fn exit(&self) {
        self.control_flow.exit();
    }

    pub fn request_exit(&self) {
        self.request_exit.store(true, std::sync::atomic::Ordering::Relaxed);
    }
}

pub struct EngineRunner<'a> {
    engine: Option<Engine<'a>>,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EventPropagation {
    #[default]
    Propagate,
    Halt,
}

impl EventPropagation {
    #[inline]
    pub const fn should_propagate(self) -> bool {
        matches!(self, Self::Propagate)
    }

    #[inline]
    pub const fn should_halt(self) -> bool {
        matches!(self, Self::Halt)
    }
}

/* TODO List
- Implement frame pacing.
*/

impl<'a> Engine<'a> {
    fn build(
        window: &'a Window,
        settings: EngineSettings,
        loop_state: GameLoopState,
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
        let present_mode = surface_caps
            .present_modes
            .iter()
            .cloned()
            .find(|mode| mode.eq(&settings.prefered_present_mode))
            .unwrap_or(wgpu::PresentMode::Fifo);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode,
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &config);
        let window_state = WindowState {
            fullscreen: window.fullscreen().is_some(),
            focused: window.has_focus(),
            minimized: {
                let size = window.inner_size();
                size.width == 0 && size.height == 0
            },
        };
        Self {
            window,
            monitor,
            size,
            window_state: rc_cell(window_state),
            device,
            queue,
            surface,
            config,
            scenes: (),
            engine_settings: rc_cell(settings),
            loop_state: rc_cell(loop_state),
            gamepad: gilrs::Gilrs::new().expect("Failed to create Gilrs instance."),
        }
    }

    pub fn run<T: Into<String>, R: Into<Resolution>, F: Into<Option<u32>>>(
        settings: EngineSettings<T, R, F>,
    ) {
        #![allow(unused)]
        let settings = settings.resolve();
        let mut event_loop = EventLoop::new().unwrap();
        event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
        let window = WindowBuilder::new()
            .with_title(&settings.title)
            .with_min_inner_size(PhysicalSize::new(240, 120))
            .build(&event_loop).unwrap();
        if settings.fullscreen {
            window.set_fullscreen(Some(winit::window::Fullscreen::Borderless(window.current_monitor())));
        } else {
            window.request_inner_size(settings.preferred_resolution.size(&window));
            let screen_size = window.current_monitor().unwrap().size();
            let window_size = window.outer_size();
            let window_half_size = PhysicalSize::new(window_size.width / 2, window_size.height / 2);
            let screen_half_size = PhysicalSize::new(screen_size.width / 2, screen_size.height / 2);
            let center_point = PhysicalPosition::new(
                screen_half_size.width - window_half_size.width,
                screen_half_size.height - window_half_size.height,
            );
            window.set_outer_position(center_point);
        }
        // println!("Inner Size: {:?}", window.inner_size());
        // Get the monitor refresh rate, convert it to a Duration. This duration, if it exists, is then used
        // as the seed to the frametime averager.
        let monitor_refresh_time = if let Some(refresh_mhz) = window.primary_monitor().unwrap().refresh_rate_millihertz() {
            let frame_time = Duration::from_secs(1000) / refresh_mhz;
            log::info!("Monitor Refresh Rate: {refresh_mhz}");
            log::info!("Monitor Refresh Frame Time: {frame_time:?}");
            Some(frame_time)
        } else {
            None
        };
        let mut frametime_averager = hexmath::average::AverageBuffer::<Duration>::new(15, monitor_refresh_time);
        let mut frame_timer = hexcore::time::timer::Timer::start();
        let mut loop_timer = hexcore::time::timer::Timer::start();
        let mut frame_limiter = FrameLimiter::start_new(settings.max_framerate.map(|fps| Duration::from_secs(1) / fps));

        let mut frame_space = FrameSpace {
            last_fixed_time: Instant::now(),
            fixed_timestep: Duration::from_secs(1) / 120,
            last_frame_time: Instant::now(),
            frame_spacing: if matches!(
                settings.prefered_present_mode,
                wgpu::PresentMode::Fifo
                | wgpu::PresentMode::Mailbox
            ) {
                Spacing::Fixed(monitor_refresh_time.unwrap_or_else(|| Duration::from_secs(1) / 60))
            } else {
                match settings.max_framerate {
                    Some(framerate) => {
                        Spacing::Fixed(Duration::from_secs(1) / framerate)
                    }
                    None => Spacing::Immediate,
                }
            },
            average_frame_time: AverageBuffer::new(20, None),
        };
        
        let mut frame = FrameInfo {
            index: 0,
            // TODO: Adjust average_fps and delta_time according to refresh rate.
            average_frame_time: FrameTime::new(frametime_averager.average()),
            delta_time: FrameTime::new(Duration::ZERO),
            run_time: Duration::ZERO,
        };
        let start_time = Instant::now();
        let loop_state = GameLoopState {
            frame,
            refresh_rate: monitor_refresh_time.unwrap_or_else(|| Duration::from_secs(1) / 60),
            start_time: Local::now(),
        };

        let mut engine = Engine::build(&window, settings, loop_state);
        // if engine.engine_settings.borrow().vsync {
        //     engine.config.present_mode = wgpu::PresentMode::Fifo;
        //     engine.surface.configure(&engine.device, &engine.config);
        //     let timing = engine.vsync_timer().expect("Failed to perform VSync render.");
        //     engine.loop_state.borrow_mut().refresh_rate = timing;
        //     engine.config.present_mode = wgpu::PresentMode::Mailbox;
        //     engine.surface.configure(&engine.device, &engine.config);
        // }
        event_loop.run(move |event, control_flow| {
            let request_exit = std::sync::atomic::AtomicBool::new(false);
            let control = Control::new(control_flow, &request_exit);
            // Process Event in Engine
            engine.process_event(&event, control);
            while let Some(event) = engine.gamepad.next_event() {
                if engine.process_gamepad_event(&event, control).should_propagate() {
                    engine.scenes_process_gamepad_event(&event, control);
                }
            }
            if !engine.window_state.borrow().minimized {
                engine.window.request_redraw();
            }
            // Main event match
            match event {
                // Idle event loop, request redraw if the window is focused.
                Event::WindowEvent { window_id, ref event }
                if window_id == engine.window.id() && engine.process_window_event(event, Control::new(control_flow, &request_exit)).should_propagate() => {
                    engine.scenes_process_window_event(event, control);
                    match event {
                        WindowEvent::CloseRequested if engine.close_requested() => {
                            control_flow.exit();
                        }
                        WindowEvent::RedrawRequested if !engine.window_state.borrow().minimized => {
                            // {
                            //     let mut engine = &mut engine;
                            //     frame_space.fixed_updates(move |timestep| {
                            //         engine.begin_fixed_update(control, timestep);
                            //         engine.fixed_update(control, timestep);
                            //         engine.end_fixed_update(control, timestep);
                            //     });
                            // }
                            // {
                            //     let mut engine = &mut engine;
                            //     let mut frametime_averager = &mut frametime_averager;
                            //     let mut frame_timer = &mut frame_timer;
                            //     frame_space.frame(move || {
                            //         let frame_time = frame_timer.time();
                            //         let run_time = start_time.elapsed();
                            //         engine.loop_state.borrow_mut().frame.run_time = run_time;
                            //         engine.loop_state.borrow_mut().frame.delta_time = FrameTime::new(frame_time);
                            //         let average_frame_time = frametime_averager.push(frame_time);
                            //         engine.loop_state.borrow_mut().frame.average_frame_time = FrameTime::new(average_frame_time);
                            //         engine.begin_update(control);
                            //         engine.update(control);
                            //         engine.end_update(control);
                            //         engine.begin_render(control);
                            //         engine.render(control).expect("Render Failed :(");
                            //         engine.end_render(control);
                            //         engine.loop_state.borrow_mut().frame.index += 1;
                            //     });
                            // }
                        }
                        &WindowEvent::Resized(new_size) => {
                            if new_size.width == 0 && new_size.height == 0 {
                                let was_minimized = std::mem::replace(&mut engine.window_state.borrow_mut().minimized, true);
                                if !was_minimized {
                                    // Set control flow to "Wait" so that it doesn't keep polling while the
                                    // window is minimized.
                                    control_flow.set_control_flow(winit::event_loop::ControlFlow::Wait);
                                    engine.minimized_changed(true);
                                }
                            } else {
                                let was_minimized = std::mem::replace(&mut engine.window_state.borrow_mut().minimized, false);
                                if was_minimized {
                                    // Now that the window is no longer minimized, set the control flow
                                    // back to poll.
                                    control_flow.set_control_flow(winit::event_loop::ControlFlow::Poll);
                                    engine.minimized_changed(false);
                                    engine.window.request_redraw();
                                }
                                engine.resized(new_size);
                            }
                        }
                        &WindowEvent::Focused(focus) => {
                            if focus {
                                frame_timer.reset();
                            }
                            engine.window_state.borrow_mut().focused = focus;
                            engine.focus_changed(focus);
                            engine.window.request_redraw();
                        }
                        &WindowEvent::Moved(_) => {
                            engine.window.request_redraw();
                        }
                        _ => (),
                    }
                }
                _ => (),
            }
            if !engine.window_state.borrow().minimized {
                {
                    let mut engine = &mut engine;
                    frame_space.fixed_updates(move |timestep| {
                        engine.begin_fixed_update(control, timestep);
                        engine.fixed_update(control, timestep);
                        engine.end_fixed_update(control, timestep);
                    });
                }
                {
                    let mut engine = &mut engine;
                    let mut frametime_averager = &mut frametime_averager;
                    let mut frame_timer = &mut frame_timer;
                    frame_space.frame(move || {
                        let frame_time = frame_timer.time();
                        let run_time = start_time.elapsed();
                        engine.loop_state.borrow_mut().frame.run_time = run_time;
                        engine.loop_state.borrow_mut().frame.delta_time = FrameTime::new(frame_time);
                        let average_frame_time = frametime_averager.push(frame_time);
                        engine.loop_state.borrow_mut().frame.average_frame_time = FrameTime::new(average_frame_time);
                        engine.begin_update(control);
                        engine.update(control);
                        engine.end_update(control);
                        engine.begin_render(control);
                        engine.render(control).expect("Render Failed :(");
                        engine.end_render(control);
                        engine.loop_state.borrow_mut().frame.index += 1;
                    });
                }
            }
            // End main event match
            // Check if exit was requested.
            if request_exit.load(std::sync::atomic::Ordering::Relaxed)
            && engine.close_requested() {
                control_flow.exit();
            } else if !engine.window_state.borrow().minimized {
                engine.window.request_redraw();
            }
            // spin_sleep::sleep(Duration::from_micros(100));
        }).expect("Me event loop broken :(");
    }

    pub fn set_fullscreen(&self, fullscreen: bool) {
        self.window.set_fullscreen(if fullscreen {
            Some(winit::window::Fullscreen::Borderless(Some(self.monitor.clone())))
        } else {
            None
        });
        self.window_state.borrow_mut().fullscreen = fullscreen;
    }

    pub fn toggle_fullscreen(&self) {
        let fullscreen = self.is_fullscreen();
        self.window.set_fullscreen(if fullscreen {
            None
        } else {
            Some(winit::window::Fullscreen::Borderless(Some(self.monitor.clone())))
        });
    }

    pub fn is_fullscreen(&self) -> bool {
        self.window.fullscreen().is_some()
    }

    fn focus_changed(&mut self, focus: bool) {
        // TODO Propagate event to scenes
    }

    fn resized(&mut self, new_size: PhysicalSize<u32>) {
        self.size = new_size;
        self.config.width = new_size.width;
        self.config.height = new_size.height;
        self.surface.configure(&self.device, &self.config);
        // TODO Propagate event to scenes.
    }

    fn minimized_changed(&mut self, minimized: bool) {
        // TODO Propagate event to scenes
    }

    fn process_event(&mut self, event: &Event<()>, control: Control<'_>) {
        // TODO Propagate event to scenes
    }

    fn process_gamepad_event(&mut self, event: &gilrs::Event, control: Control<'_>) -> EventPropagation {
        EventPropagation::Propagate
    }

    fn scenes_process_gamepad_event(&mut self, event: &gilrs::Event, control: Control<'_>) {

    }

    fn scenes_process_window_event(&mut self, event: &WindowEvent, control: Control<'_>) {

    }

    fn process_window_event(&mut self, event: &WindowEvent, control: Control<'_>) -> EventPropagation {
        match event {
            WindowEvent::KeyboardInput { device_id, event, is_synthetic } => {
                match event.physical_key {
                    winit::keyboard::PhysicalKey::Code(key_code) => match key_code {
                        KeyCode::F11 if !event.repeat && event.state == ElementState::Pressed => {
                            let fullscreen = self.window_state.borrow().fullscreen;
                            self.set_fullscreen(!fullscreen);
                            return EventPropagation::Halt;
                        }
                        // TODO This is for testing purposes. Remove later.
                        KeyCode::Escape if event.state == ElementState::Pressed => {
                            control.request_exit();
                            return EventPropagation::Halt;
                        }
                        _ => (),
                    },
                    _ => (),
                }
                if event.state.is_pressed() {
                    if let Some(text) = event.logical_key.to_text() {
                        print!("{text}");
                        std::io::stdout().flush().unwrap();
                    }
                }
            }
            _ => (),
        }
        EventPropagation::Propagate
    }

    fn close_requested(&mut self) -> bool {
        true
    }

    fn begin_update(&mut self, control: Control<'_>) {

    }

    fn update(&mut self, control: Control<'_>) {
        const DELAYS: [u64; 6] = [3, 5, 8, 6, 4, 13];
        // println!("Average FPS: {:.0} {}", self.loop_state.frame.average_frame_time.fps(), self.loop_state.frame.index);
        if self.frame_index() >= 20 {
            let framerate = FrameTime::new(self.loop_state.borrow().refresh_rate);
            let fps = self.loop_state.borrow().frame.average_frame_time.fps();
            if (framerate.fps() - fps).abs() > 0.5 {
                println!("Framerate +/- 1 ({:.0}) FPS: {:.0}", framerate.fps(), fps);
            }
        }
    }

    fn end_update(&mut self, control: Control<'_>) {

    }

    fn begin_fixed_update(&mut self, control: Control<'_>, timestep: Duration) {

    }

    fn fixed_update(&mut self, control: Control<'_>, timestep: Duration) {

    }

    fn end_fixed_update(&mut self, control: Control<'_>, timestep: Duration) {

    }

    fn begin_render(&mut self, control: Control<'_>) {

    }

    fn render(&mut self, control: Control<'_>) -> Result<Duration, wgpu::SurfaceError> {
        let start_time = Instant::now();
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Hexahedron Engine Render Encoder"),
        });
        // Clear Render Pass
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Hexahedron Engine Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.0, g: 0.0, b: 0.0, a: 1.0
                        }),
                        store: wgpu::StoreOp::Store,
                    }
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });
        }
        self.queue.submit(std::iter::once(encoder.finish()));
        let duration = start_time.elapsed();
        // Artificial Delay
        std::thread::sleep(Duration::from_millis(5));
        output.present();
        Ok(duration)
    }

    fn end_render(&mut self, control: Control<'_>) {

    }

    pub fn frame_index(&self) -> u64 {
        self.loop_state.borrow().frame.index
    }

    pub fn average_frame_time(&self) -> FrameTime {
        self.loop_state.borrow().frame.average_frame_time
    }

    pub fn delta_time(&self) -> FrameTime {
        self.loop_state.borrow().frame.delta_time
    }

    pub fn start_time(&self) -> DateTime<Local> {
        self.loop_state.borrow().start_time
    }

}