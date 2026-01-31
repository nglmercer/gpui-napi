use napi::bindgen_prelude::*;
use napi_derive::napi;
use softbuffer::{Context, Surface};
use std::num::NonZeroU32;
use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{Window, WindowAttributes, WindowId};

/// Window renderer using winit and softbuffer
#[napi]
pub struct WindowRenderer {
    width: u32,
    height: u32,
    pixel_buffer: Vec<u32>,
}

#[napi]
impl WindowRenderer {
    /// Create a new window renderer with specified dimensions
    #[napi(constructor)]
    pub fn new(width: u32, height: u32) -> Self {
        let pixel_count = (width * height) as usize;
        Self {
            width,
            height,
            pixel_buffer: vec![0; pixel_count],
        }
    }

    /// Set a pixel at the specified coordinates with RGB values
    #[napi]
    pub fn set_pixel(&mut self, x: u32, y: u32, r: u8, g: u8, b: u8) {
        if x < self.width && y < self.height {
            let index = (y * self.width + x) as usize;
            // Store as ARGB (0xAARRGGBB) - alpha is always 255
            self.pixel_buffer[index] =
                ((255 as u32) << 24) | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);
        }
    }

    /// Fill the entire buffer with a color (RGB)
    #[napi]
    pub fn fill(&mut self, r: u8, g: u8, b: u8) {
        let color = ((255 as u32) << 24) | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);
        for pixel in &mut self.pixel_buffer {
            *pixel = color;
        }
    }

    /// Clear the buffer to black
    #[napi]
    pub fn clear(&mut self) {
        for pixel in &mut self.pixel_buffer {
            *pixel = 0xFF000000; // Black with full alpha
        }
    }

    /// Get the width of the window
    #[napi(getter)]
    pub fn get_width(&self) -> u32 {
        self.width
    }

    /// Get the height of the window
    #[napi(getter)]
    pub fn get_height(&self) -> u32 {
        self.height
    }

    /// Present the pixel buffer in a window
    /// This will create a window and display the rendered pixels
    #[napi]
    pub fn present(&self, title: String) -> Result<()> {
        let event_loop = EventLoop::new().map_err(|e| {
            napi::Error::new(
                napi::Status::GenericFailure,
                format!("Failed to create event loop: {}", e),
            )
        })?;

        let mut app = PixelWindowApp::new(
            self.width,
            self.height,
            title,
            self.pixel_buffer.clone(),
        );

        event_loop.run_app(&mut app).map_err(|e| {
            napi::Error::new(
                napi::Status::GenericFailure,
                format!("Event loop error: {}", e),
            )
        })?;

        Ok(())
    }
}

/// Simple window that displays a buffer
#[napi]
pub struct SimpleWindow {
    width: u32,
    height: u32,
    title: String,
}

#[napi]
impl SimpleWindow {
    #[napi(constructor)]
    pub fn new(width: u32, height: u32, title: String) -> Self {
        Self {
            width,
            height,
            title,
        }
    }

    /// Show the window with a static color
    #[napi]
    pub fn show(&self, r: u8, g: u8, b: u8) -> Result<()> {
        let event_loop = EventLoop::new().map_err(|e| {
            napi::Error::new(
                napi::Status::GenericFailure,
                format!("Failed to create event loop: {}", e),
            )
        })?;

        let color = ((255 as u32) << 24) | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);
        let mut app = SimpleWindowApp::new(self.width, self.height, self.title.clone(), color);

        event_loop.run_app(&mut app).map_err(|e| {
            napi::Error::new(
                napi::Status::GenericFailure,
                format!("Event loop error: {}", e),
            )
        })?;

        Ok(())
    }
}

struct SimpleWindowApp {
    width: u32,
    height: u32,
    title: String,
    color: u32,
    window: Option<Arc<Window>>,
    surface: Option<Surface<Arc<Window>, Arc<Window>>>,
    context: Option<Context<Arc<Window>>>,
}

impl SimpleWindowApp {
    fn new(width: u32, height: u32, title: String, color: u32) -> Self {
        Self {
            width,
            height,
            title,
            color,
            window: None,
            surface: None,
            context: None,
        }
    }

    fn render(&mut self) {
        if let (Some(window), Some(surface)) = (&self.window, &mut self.surface) {
            let size = window.inner_size();
            let width = NonZeroU32::new(size.width.max(1)).expect("Width should not be zero");
            let height = NonZeroU32::new(size.height.max(1)).expect("Height should not be zero");
            surface.resize(width, height).expect("Failed to resize surface");
            let mut buffer = surface.buffer_mut().expect("Failed to get buffer");
            // Fill with the specified color
            for pixel in buffer.iter_mut() {
                *pixel = self.color;
            }
            buffer.present().unwrap_or_else(|e| {
                eprintln!("Failed to present buffer: {}", e);
            });
        }
    }
}

impl ApplicationHandler for SimpleWindowApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = WindowAttributes::default()
            .with_title(&self.title)
            .with_inner_size(LogicalSize::new(self.width, self.height));

        let window = Arc::new(
            event_loop
                .create_window(window_attributes)
                .expect("Failed to create window"),
        );

        let context = Context::new(window.clone()).expect("Failed to create softbuffer context");
        let surface = Surface::new(&context, window.clone()).expect("Failed to create surface");

        self.window = Some(window);
        self.context = Some(context);
        self.surface = Some(surface);

        // Request initial redraw
        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                self.render();
            }
            _ => {}
        }
    }
}

/// Application for displaying a pixel buffer in a window
struct PixelWindowApp {
    width: u32,
    height: u32,
    title: String,
    pixel_buffer: Vec<u32>,
    window: Option<Arc<Window>>,
    surface: Option<Surface<Arc<Window>, Arc<Window>>>,
    context: Option<Context<Arc<Window>>>,
}

impl PixelWindowApp {
    fn new(width: u32, height: u32, title: String, pixel_buffer: Vec<u32>) -> Self {
        Self {
            width,
            height,
            title,
            pixel_buffer,
            window: None,
            surface: None,
            context: None,
        }
    }

    fn render(&mut self) {
        if let (Some(_window), Some(surface)) = (&self.window, &mut self.surface) {
            let width = NonZeroU32::new(self.width.max(1)).expect("Width should not be zero");
            let height = NonZeroU32::new(self.height.max(1)).expect("Height should not be zero");
            surface.resize(width, height).expect("Failed to resize surface");
            let mut buffer = surface.buffer_mut().expect("Failed to get buffer");

            // Copy pixel buffer to the surface buffer
            let pixel_count = (self.width * self.height) as usize;
            let buffer_len = buffer.len();

            for i in 0..pixel_count.min(buffer_len) {
                buffer[i] = self.pixel_buffer[i];
            }

            buffer.present().unwrap_or_else(|e| {
                eprintln!("Failed to present buffer: {}", e);
            });
        }
    }
}

impl ApplicationHandler for PixelWindowApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = WindowAttributes::default()
            .with_title(&self.title)
            .with_inner_size(LogicalSize::new(self.width, self.height));

        let window = Arc::new(
            event_loop
                .create_window(window_attributes)
                .expect("Failed to create window"),
        );

        let context = Context::new(window.clone()).expect("Failed to create softbuffer context");
        let surface = Surface::new(&context, window.clone()).expect("Failed to create surface");

        self.window = Some(window);
        self.context = Some(context);
        self.surface = Some(surface);

        // Request initial redraw
        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                self.render();
            }
            _ => {}
        }
    }
}
