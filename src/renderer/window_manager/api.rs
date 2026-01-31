use napi::bindgen_prelude::*;
use napi::JsNumber;
use napi_derive::napi;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use winit::event_loop::EventLoop;

#[cfg(target_os = "linux")]
use winit::platform::x11::EventLoopBuilderExtX11;
use winit::event_loop::EventLoopBuilder;

use crate::renderer::window_manager::app::WindowManagerApp;
use crate::renderer::window_manager::types::*;
use crate::renderer::window_manager::utils::js_number_to_u64;

/// Window Manager that handles multiple windows in a non-blocking way
#[napi]
pub struct WindowManager {
    pub(crate) state: SharedState,
    pub(crate) next_window_id: Arc<Mutex<u64>>,
    pub(crate) _event_loop_handle: Option<thread::JoinHandle<()>>,
}

#[napi]
impl WindowManager {
    /// Create a new window manager
    #[napi(constructor)]
    pub fn new() -> Result<Self> {
        let state = Arc::new(Mutex::new(WindowManagerState {
            windows: HashMap::new(),
            pending_commands: Vec::new(),
            should_exit: false,
        }));

        let next_window_id = Arc::new(Mutex::new(1u64));

        Ok(Self {
            state,
            next_window_id,
            _event_loop_handle: None,
        })
    }

    /// Start the event loop (must be called before creating windows)
    #[napi]
    pub fn start(&mut self) -> Result<()> {
        let state = self.state.clone();

        let handle = thread::spawn(move || {
            // Create event loop with any_thread flag for Linux
            #[cfg(target_os = "linux")]
            let event_loop = EventLoopBuilder::new()
                .with_any_thread(true)
                .build()
                .expect("Failed to create event loop");

            #[cfg(not(target_os = "linux"))]
            let event_loop = EventLoop::new().expect("Failed to create event loop");

            let mut app = WindowManagerApp::new(state);
            let _ = event_loop.run(move |event, event_loop| {
                // winit 0.29 run takes 2 arguments: event and event_loop
                // We need to adapt this to our 3-argument handler
                use winit::event_loop::ControlFlow;
                
                let mut control_flow = ControlFlow::Wait;
                app.handle_event(event, event_loop, &mut control_flow);
            });
        });

        self._event_loop_handle = Some(handle);
        Ok(())
    }

    /// Create a new window and return its ID
    #[napi]
    pub fn create_window(&mut self, width: u32, height: u32, title: String) -> Result<u64> {
        self.create_window_with_options(width, height, title, None, None, false, false, true)
    }

    /// Create a new window with position options
    #[napi]
    pub fn create_window_with_position(
        &mut self,
        width: u32,
        height: u32,
        title: String,
        x: i32,
        y: i32,
    ) -> Result<u64> {
        self.create_window_with_options(width, height, title, Some(x), Some(y), false, false, true)
    }

    /// Create a new window with all options
    #[napi]
    #[allow(clippy::too_many_arguments)]
    pub fn create_window_with_options(
        &mut self,
        width: u32,
        height: u32,
        title: String,
        x: Option<i32>,
        y: Option<i32>,
        always_on_top: bool,
        transparent: bool,
        decorations: bool,
    ) -> Result<u64> {
        let id = {
            let mut counter = self
                .next_window_id
                .lock()
                .map_err(|_| napi::Error::new(napi::Status::GenericFailure, "Lock poisoned"))?;
            let id = *counter;
            *counter += 1;
            id
        };

        let mut state = self
            .state
            .lock()
            .map_err(|_| napi::Error::new(napi::Status::GenericFailure, "Lock poisoned"))?;

        // Pre-register the window in shared state so window_count() and window_exists() work immediately
        let pixel_count = (width * height) as usize;
        // ARGB format: AAAA AAAA RRRR RRRR GGGG GGGG BBBB BBBB
        // For transparent windows, initialize with fully transparent pixels (alpha = 0)
        // For opaque windows, initialize with opaque black (alpha = 255)
        let pixel_buffer = if transparent {
            vec![0x00000000u32; pixel_count] // Fully transparent ARGB (A=0, R=0, G=0, B=0)
        } else {
            vec![0xFF000000u32; pixel_count] // Opaque black ARGB (A=255, R=0, G=0, B=0)
        };

        state.windows.insert(
            id,
            WindowState {
                width,
                height,
                pixel_buffer,
                needs_redraw: true,
                title: title.clone(),
                x,
                y,
                always_on_top,
                transparent,
                decorations,
                winit_id: None, // Will be set when window is actually created
            },
        );

        state.pending_commands.push(WindowCommand::CreateWindow {
            id,
            width,
            height,
            title,
            x,
            y,
            always_on_top,
            transparent,
            decorations,
        });

        Ok(id)
    }

    /// Set a pixel in a window's buffer
    /// Sets the pixel to opaque (alpha = 255) with the given RGB values
    #[napi]
    pub fn set_pixel(
        &self,
        window_id: JsNumber,
        x: u32,
        y: u32,
        r: u8,
        g: u8,
        b: u8,
    ) -> Result<()> {
        let window_id = js_number_to_u64(window_id)?;
        // ARGB format: AAAA AAAA RRRR RRRR GGGG GGGG BBBB BBBB
        // Alpha is in the high byte (0xFF = fully opaque)
        let color = ((0xFF as u32) << 24) | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);

        let mut state = self
            .state
            .lock()
            .map_err(|_| napi::Error::new(napi::Status::GenericFailure, "Lock poisoned"))?;

        if let Some(window_state) = state.windows.get_mut(&window_id) {
            if x < window_state.width && y < window_state.height {
                let index = (y * window_state.width + x) as usize;
                window_state.pixel_buffer[index] = color;
                window_state.needs_redraw = true;
            }
        }

        Ok(())
    }

    /// Set a pixel with alpha (RGBA) in a window's buffer
    /// The alpha value controls transparency: 0 = fully transparent, 255 = fully opaque
    #[napi]
    #[allow(clippy::too_many_arguments)]
    pub fn set_pixel_rgba(
        &self,
        window_id: JsNumber,
        x: u32,
        y: u32,
        r: u8,
        g: u8,
        b: u8,
        a: u8,
    ) -> Result<()> {
        let window_id = js_number_to_u64(window_id)?;
        // ARGB format: AAAA AAAA RRRR RRRR GGGG GGGG BBBB BBBB
        // Alpha is in the high byte (bits 24-31)
        let color = ((a as u32) << 24) | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);

        let mut state = self
            .state
            .lock()
            .map_err(|_| napi::Error::new(napi::Status::GenericFailure, "Lock poisoned"))?;

        if let Some(window_state) = state.windows.get_mut(&window_id) {
            if x < window_state.width && y < window_state.height {
                let index = (y * window_state.width + x) as usize;
                window_state.pixel_buffer[index] = color;
                window_state.needs_redraw = true;
            }
        }

        Ok(())
    }

    /// Clear a window's buffer to a color with optional alpha
    /// If alpha is not provided, defaults to 255 (fully opaque)
    #[napi]
    pub fn clear(&self, window_id: JsNumber, r: u8, g: u8, b: u8, a: Option<u8>) -> Result<()> {
        let window_id = js_number_to_u64(window_id)?;
        // ARGB format: AAAA AAAA RRRR RRRR GGGG GGGG BBBB BBBB
        // Alpha defaults to 255 (fully opaque) if not provided
        let alpha = a.unwrap_or(0xFF);
        let color = ((alpha as u32) << 24) | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);

        let mut state = self
            .state
            .lock()
            .map_err(|_| napi::Error::new(napi::Status::GenericFailure, "Lock poisoned"))?;

        if let Some(window_state) = state.windows.get_mut(&window_id) {
            for pixel in &mut window_state.pixel_buffer {
                *pixel = color;
            }
            window_state.needs_redraw = true;
        }

        Ok(())
    }

    /// Clear a window's buffer to black
    #[napi]
    pub fn clear_black(&self, window_id: JsNumber) -> Result<()> {
        let window_id = js_number_to_u64(window_id)?;
        self.clear_inner(window_id, 0, 0, 0)
    }

    fn clear_inner(&self, window_id: u64, r: u8, g: u8, b: u8) -> Result<()> {
        // ARGB format: AAAA AAAA RRRR RRRR GGGG GGGG BBBB BBBB
        // clear_black always uses opaque black
        let color = ((0xFF as u32) << 24) | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);

        let mut state = self
            .state
            .lock()
            .map_err(|_| napi::Error::new(napi::Status::GenericFailure, "Lock poisoned"))?;

        if let Some(window_state) = state.windows.get_mut(&window_id) {
            for pixel in &mut window_state.pixel_buffer {
                *pixel = color;
            }
            window_state.needs_redraw = true;
        }

        Ok(())
    }

    /// Present/render a window's buffer
    #[napi]
    pub fn present(&self, window_id: JsNumber) -> Result<()> {
        let window_id = js_number_to_u64(window_id)?;
        let mut state = self
            .state
            .lock()
            .map_err(|_| napi::Error::new(napi::Status::GenericFailure, "Lock poisoned"))?;

        if let Some(window_state) = state.windows.get_mut(&window_id) {
            window_state.needs_redraw = true;
        }

        state
            .pending_commands
            .push(WindowCommand::Present { window_id });
        Ok(())
    }

    /// Set window position
    #[napi]
    pub fn set_position(&self, window_id: JsNumber, x: i32, y: i32) -> Result<()> {
        let window_id = js_number_to_u64(window_id)?;
        let mut state = self
            .state
            .lock()
            .map_err(|_| napi::Error::new(napi::Status::GenericFailure, "Lock poisoned"))?;

        if let Some(window_state) = state.windows.get_mut(&window_id) {
            window_state.x = Some(x);
            window_state.y = Some(y);
        }

        state
            .pending_commands
            .push(WindowCommand::SetPosition { window_id, x, y });
        Ok(())
    }

    /// Set always on top
    #[napi]
    pub fn set_always_on_top(&self, window_id: JsNumber, always_on_top: bool) -> Result<()> {
        let window_id = js_number_to_u64(window_id)?;
        let mut state = self
            .state
            .lock()
            .map_err(|_| napi::Error::new(napi::Status::GenericFailure, "Lock poisoned"))?;

        if let Some(window_state) = state.windows.get_mut(&window_id) {
            window_state.always_on_top = always_on_top;
        }

        state.pending_commands.push(WindowCommand::SetAlwaysOnTop {
            window_id,
            always_on_top,
        });
        Ok(())
    }

    /// Set window title
    #[napi]
    pub fn set_title(&self, window_id: JsNumber, title: String) -> Result<()> {
        let window_id = js_number_to_u64(window_id)?;
        let mut state = self
            .state
            .lock()
            .map_err(|_| napi::Error::new(napi::Status::GenericFailure, "Lock poisoned"))?;

        if let Some(window_state) = state.windows.get_mut(&window_id) {
            window_state.title = title.clone();
        }

        state
            .pending_commands
            .push(WindowCommand::SetTitle { window_id, title });
        Ok(())
    }

    /// Set window to ignore mouse/keyboard input (click-through)
    #[napi]
    pub fn set_ignore_input(&self, window_id: JsNumber, ignore: bool) -> Result<()> {
        let window_id = js_number_to_u64(window_id)?;
        let mut state = self
            .state
            .lock()
            .map_err(|_| napi::Error::new(napi::Status::GenericFailure, "Lock poisoned"))?;

        state
            .pending_commands
            .push(WindowCommand::SetIgnoreInput { window_id, ignore });
        Ok(())
    }

    /// Close a window
    #[napi]
    pub fn close_window(&self, window_id: JsNumber) -> Result<()> {
        let window_id = js_number_to_u64(window_id)?;
        let mut state = self
            .state
            .lock()
            .map_err(|_| napi::Error::new(napi::Status::GenericFailure, "Lock poisoned"))?;

        state
            .pending_commands
            .push(WindowCommand::CloseWindow { window_id });
        Ok(())
    }

    /// Check if a window exists
    #[napi]
    pub fn window_exists(&self, window_id: JsNumber) -> Result<bool> {
        let window_id = js_number_to_u64(window_id)?;
        let state = self
            .state
            .lock()
            .map_err(|_| napi::Error::new(napi::Status::GenericFailure, "Lock poisoned"))?;

        Ok(state.windows.contains_key(&window_id))
    }

    /// Get window count
    #[napi(getter)]
    pub fn window_count(&self) -> Result<u32> {
        let state = self
            .state
            .lock()
            .map_err(|_| napi::Error::new(napi::Status::GenericFailure, "Lock poisoned"))?;

        Ok(state.windows.len() as u32)
    }
}
