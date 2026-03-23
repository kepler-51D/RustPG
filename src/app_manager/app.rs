use std::sync::Arc;

use crate::app_manager::state::State;
use glam::Vec3;
use instant::Instant;
use winit::{
    application::ApplicationHandler,
    event::{DeviceEvent, KeyEvent, WindowEvent},
    event_loop::ActiveEventLoop,
    keyboard::PhysicalKey,
    window::Window,
};

pub struct App {
    // pub render_manager: RenderManager,
    // pub world_manager: Option<WorldManager>,
    pub state: Option<State>,
    pub last_render_time: Instant,
}

impl App {
    pub fn new() -> Self {
        Self {
            // render_manager: RenderManager::new(Vec3::ZERO, 10),
            // world_manager: None,
            state: None,
            last_render_time: instant::Instant::now(),
        }
    }
}
impl ApplicationHandler<State> for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = Window::default_attributes();
        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());

        let state = pollster::block_on(State::new(window)).unwrap();
        self.state = Some(state);

        //if let Some(ref state_ref) = self.state {
        //    self.world_manager = Some(
        //        WorldManager::new(state_ref, 2, 1)
        //    );
        //}
    }

    #[allow(unused_mut)]
    fn user_event(&mut self, _event_loop: &ActiveEventLoop, mut event: State) {
        self.state = Some(event);
    }
    fn device_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _device_id: winit::event::DeviceId,
        event: DeviceEvent,
    ) {
        let current_state = match &mut self.state {
            Some(canvas) => canvas,
            None => return,
        };
        match event {
            DeviceEvent::MouseMotion { delta } => {
                current_state
                    .camera_controller
                    .handle_mouse(delta.0, delta.1);
            }
            DeviceEvent::Added => {},
            DeviceEvent::Removed => {},
            DeviceEvent::MouseWheel { delta } => {},
            DeviceEvent::Motion { axis, value } => {},
            DeviceEvent::Button { button, state } => {},
            DeviceEvent::Key(raw_key_event) => {},
        }
        // let _ = current_state.window.set_cursor_position(LogicalPosition::new(100.0f32,100.0f32));
    }
    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        let current_state = match &mut self.state {
            Some(canvas) => canvas,
            None => return,
        };
        match event {
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(code),
                        state: key_state,
                        ..
                    },
                ..
            } => {
                current_state.handle_key(event_loop, code, key_state.is_pressed());
                current_state.input(&event);
            }
            WindowEvent::RedrawRequested => {
                let now = instant::Instant::now();
                let dt = now - self.last_render_time;
                self.last_render_time = now;
                current_state.update(dt);
                //current_state.render_model(&current_state.obj_model);
                // current_state.render_chunk_manager();
                current_state.render_vertices();
                //current_state.render_vertices();
                //      self.world_manager.as_mut().unwrap().render_world(current_state);
            }
            _ => {}
        }
    }
    // ...
}
