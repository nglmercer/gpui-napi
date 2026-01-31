use crate::renderer::window_manager::types::*;
use softbuffer::{Context, Surface};
use std::collections::HashMap;
use std::num::NonZeroU32;
use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::dpi::{LogicalPosition, LogicalSize};
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::{WindowAttributes, WindowId, WindowLevel};

/// The application that runs in the event loop
pub struct WindowManagerApp {
    pub state: SharedState,
    pub windows: HashMap<WindowId, ManagedWindow>,
}

impl WindowManagerApp {
    pub fn new(state: SharedState) -> Self {
        Self {
            state,
            windows: HashMap::new(),
        }
    }

    pub fn process_commands(&mut self, event_loop: &ActiveEventLoop) {
        let commands = {
            let mut state = self.state.lock().expect("Lock poisoned");
            std::mem::take(&mut state.pending_commands)
        };

        for cmd in commands {
            match cmd {
                WindowCommand::CreateWindow {
                    id,
                    width,
                    height,
                    title,
                    x,
                    y,
                    always_on_top,
                } => {
                    self.create_window(event_loop, id, width, height, title, x, y, always_on_top);
                }
                WindowCommand::Present { window_id } => {
                    self.request_redraw(window_id);
                }
                WindowCommand::SetPosition { window_id, x, y } => {
                    self.set_window_position(window_id, x, y);
                }
                WindowCommand::SetAlwaysOnTop {
                    window_id,
                    always_on_top,
                } => {
                    self.set_window_always_on_top(window_id, always_on_top);
                }
                WindowCommand::SetTitle { window_id, title } => {
                    self.set_window_title(window_id, title);
                }
                WindowCommand::CloseWindow { window_id } => {
                    self.close_window_by_id(window_id, event_loop);
                }
                _ => {}
            }
        }

        // Check for redraw requests
        let windows_to_redraw: Vec<u64> = {
            let state = self.state.lock().expect("Lock poisoned");
            state
                .windows
                .iter()
                .filter(|(_, ws)| ws.needs_redraw)
                .map(|(id, _)| *id)
                .collect()
        };

        for window_id in windows_to_redraw {
            self.request_redraw(window_id);
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn create_window(
        &mut self,
        event_loop: &ActiveEventLoop,
        id: u64,
        width: u32,
        height: u32,
        title: String,
        x: Option<i32>,
        y: Option<i32>,
        always_on_top: bool,
    ) {
        let mut window_attrs = WindowAttributes::default()
            .with_title(&title)
            .with_inner_size(LogicalSize::new(width, height));

        if always_on_top {
            window_attrs = window_attrs.with_window_level(WindowLevel::AlwaysOnTop);
        }

        if let (Some(x_pos), Some(y_pos)) = (x, y) {
            window_attrs = window_attrs.with_position(LogicalPosition::new(x_pos, y_pos));
        }

        let window = Arc::new(
            event_loop
                .create_window(window_attrs)
                .expect("Failed to create window"),
        );

        let context = Context::new(window.clone()).expect("Failed to create context");
        let surface = Surface::new(&context, window.clone()).expect("Failed to create surface");

        let winit_id = window.id();

        self.windows.insert(
            winit_id,
            ManagedWindow {
                window: window.clone(),
                surface,
                _context: context,
                state_id: id,
            },
        );

        // Update the pre-registered window state with the actual winit_id
        let mut state = self.state.lock().expect("Lock poisoned");

        if let Some(window_state) = state.windows.get_mut(&id) {
            // Window was pre-registered, just update the winit_id
            window_state.winit_id = Some(winit_id);
        } else {
            // Fallback: create window state if not pre-registered (shouldn't happen)
            let pixel_count = (width * height) as usize;
            let pixel_buffer = vec![0xFF000000u32; pixel_count];

            state.windows.insert(
                id,
                WindowState {
                    width,
                    height,
                    pixel_buffer,
                    needs_redraw: true,
                    title,
                    x,
                    y,
                    always_on_top,
                    winit_id: Some(winit_id),
                },
            );
        }

        window.request_redraw();
    }

    fn render_window(&mut self, window_id: WindowId) {
        if let Some(managed) = self.windows.get_mut(&window_id) {
            let state = self.state.lock().expect("Lock poisoned");

            if let Some(window_state) = state.windows.get(&managed.state_id) {
                let width =
                    NonZeroU32::new(window_state.width.max(1)).expect("Width should not be zero");
                let height =
                    NonZeroU32::new(window_state.height.max(1)).expect("Height should not be zero");

                // Clone the pixel buffer to release the lock before surface operations
                let pixel_buffer = window_state.pixel_buffer.clone();

                drop(state); // Release lock before surface operations

                managed
                    .surface
                    .resize(width, height)
                    .expect("Failed to resize surface");
                let mut buffer = managed.surface.buffer_mut().expect("Failed to get buffer");

                let pixel_count = pixel_buffer.len();
                let buffer_len = buffer.len();

                for i in 0..pixel_count.min(buffer_len) {
                    buffer[i] = pixel_buffer[i];
                }

                buffer.present().unwrap_or_else(|e| {
                    eprintln!("Failed to present buffer: {}", e);
                });
            }
        }
    }

    fn request_redraw(&self, window_id: u64) {
        for managed in self.windows.values() {
            if managed.state_id == window_id {
                managed.window.request_redraw();
                break;
            }
        }
    }

    fn set_window_position(&self, window_id: u64, x: i32, y: i32) {
        for managed in self.windows.values() {
            if managed.state_id == window_id {
                managed
                    .window
                    .set_outer_position(LogicalPosition::new(x, y));
                break;
            }
        }
    }

    fn set_window_always_on_top(&self, window_id: u64, always_on_top: bool) {
        for managed in self.windows.values() {
            if managed.state_id == window_id {
                let level = if always_on_top {
                    WindowLevel::AlwaysOnTop
                } else {
                    WindowLevel::Normal
                };
                // Note: set_window_level is not available in all winit versions
                // This would require recreating the window or using platform-specific APIs
                let _ = level;
                break;
            }
        }
    }

    fn set_window_title(&self, window_id: u64, title: String) {
        for managed in self.windows.values() {
            if managed.state_id == window_id {
                managed.window.set_title(&title);
                break;
            }
        }
    }

    fn close_window_by_id(&mut self, window_id: u64, _event_loop: &ActiveEventLoop) {
        let mut to_remove = None;
        for (id, managed) in &self.windows {
            if managed.state_id == window_id {
                to_remove = Some(*id);
                break;
            }
        }

        if let Some(id) = to_remove {
            self.windows.remove(&id);
            let mut state = self.state.lock().expect("Lock poisoned");
            state.windows.remove(&window_id);
        }
    }

    fn close_window(&mut self, window_id: WindowId, event_loop: &ActiveEventLoop) {
        self.windows.remove(&window_id);
        let mut state = self.state.lock().expect("Lock poisoned");

        // Find and remove by winit_id
        let mut id_to_remove = None;
        for (id, ws) in &state.windows {
            if ws.winit_id == Some(window_id) {
                id_to_remove = Some(*id);
                break;
            }
        }
        if let Some(id) = id_to_remove {
            state.windows.remove(&id);
        }

        if state.windows.is_empty() {
            state.should_exit = true;
            event_loop.exit();
        }
    }
}

impl ApplicationHandler for WindowManagerApp {
    fn resumed(&mut self, _event_loop: &ActiveEventLoop) {}

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                self.close_window(window_id, event_loop);
            }
            WindowEvent::RedrawRequested => {
                // Mark the window as needing redraw in shared state
                if let Some(managed) = self.windows.get(&window_id) {
                    let mut state = self.state.lock().expect("Lock poisoned");
                    if let Some(window_state) = state.windows.get_mut(&managed.state_id) {
                        window_state.needs_redraw = false;
                    }
                }
                self.render_window(window_id);
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        self.process_commands(event_loop);

        // Check if we should exit
        let should_exit = {
            let state = self.state.lock().expect("Lock poisoned");
            state.should_exit
        };

        if should_exit {
            event_loop.exit();
        }
    }
}
