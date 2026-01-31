use crate::renderer::window_manager::types::*;
use std::collections::HashMap;
use std::sync::Arc;
use winit::dpi::{LogicalPosition, LogicalSize};
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoopWindowTarget};
use winit::window::WindowId;

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

    pub fn handle_event(
        &mut self,
        event: Event<()>,
        event_loop: &EventLoopWindowTarget<()>,
        control_flow: &mut ControlFlow,
    ) {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                window_id,
                event: WindowEvent::CloseRequested,
            } => {
                self.close_window(window_id, event_loop);
            }
            Event::WindowEvent {
                window_id,
                event: WindowEvent::RedrawRequested,
            } => {
                self.render_window(window_id);
            }
            Event::WindowEvent {
                window_id,
                event: WindowEvent::Resized(_),
            } => {
                // Handle resize by redrawing
                if let Some(managed) = self.windows.get(&window_id) {
                    let mut state = self.state.lock().expect("Lock poisoned");
                    if let Some(window_state) = state.windows.get_mut(&managed.state_id) {
                        window_state.needs_redraw = true;
                    }
                }
            }
            Event::AboutToWait => {
                // Process commands when the event loop is about to wait
                // This ensures commands are processed even without window events
                self.process_commands(event_loop);

                // Check if we should exit
                let should_exit = {
                    let state = self.state.lock().expect("Lock poisoned");
                    state.should_exit
                };

                if should_exit {
                    std::process::exit(0);
                }
            }
            _ => {}
        }
    }

    pub fn process_commands(&mut self, event_loop: &EventLoopWindowTarget<()>) {
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
                    transparent,
                    decorations,
                } => {
                    self.create_window(
                        event_loop,
                        id,
                        width,
                        height,
                        title,
                        x,
                        y,
                        always_on_top,
                        transparent,
                        decorations,
                    );
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
                WindowCommand::SetIgnoreInput { window_id, ignore } => {
                    self.set_window_ignore_input(window_id, ignore);
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
        event_loop: &EventLoopWindowTarget<()>,
        id: u64,
        width: u32,
        height: u32,
        title: String,
        x: Option<i32>,
        y: Option<i32>,
        always_on_top: bool,
        transparent: bool,
        decorations: bool,
    ) {
        use winit::window::WindowBuilder;

        let window_builder = WindowBuilder::new()
            .with_title(&title)
            .with_inner_size(LogicalSize::new(width, height))
            .with_transparent(transparent)
            .with_decorations(decorations)
            .with_visible(true);

        let window = Arc::new(
            window_builder
                .build(event_loop)
                .expect("Failed to create window"),
        );

        // Note: always_on_top is not directly supported in winit 0.29 WindowBuilder
        // It would require platform-specific code or window level adjustments after creation
        let _ = always_on_top; // Silence unused warning

        if let (Some(x_pos), Some(y_pos)) = (x, y) {
            window.set_outer_position(LogicalPosition::new(x_pos, y_pos));
        }

        let winit_id = window.id();

        // Create a pixmap for CPU-based rendering
        let pixmap = tiny_skia::Pixmap::new(width, height)
            .expect("Failed to create pixmap");

        self.windows.insert(
            winit_id,
            ManagedWindow {
                window: window.clone(),
                pixmap,
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
            // ARGB format: AAAA AAAA RRRR RRRR GGGG GGGG BBBB BBBB
            let pixel_buffer = if transparent {
                vec![0x00000000u32; pixel_count] // Fully transparent
            } else {
                vec![0xFF000000u32; pixel_count] // Opaque black
            };

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
                    transparent,
                    decorations,
                    winit_id: Some(winit_id),
                },
            );
        }

        window.request_redraw();
    }

    fn render_window(&mut self, window_id: WindowId) {
        // Get mutable access to the managed window first
        let managed = match self.windows.get_mut(&window_id) {
            Some(m) => m,
            None => return,
        };
        
        // Get the pixel buffer data from shared state
        let (pixel_buffer, width, height, is_transparent) = {
            let state = self.state.lock().expect("Lock poisoned");

            if let Some(window_state) = state.windows.get(&managed.state_id) {
                (
                    window_state.pixel_buffer.clone(),
                    window_state.width,
                    window_state.height,
                    window_state.transparent,
                )
            } else {
                return;
            }
        };
        
        // Convert ARGB u32 pixels to RGBA u8 format for tiny-skia
        // tiny-skia expects [R, G, B, A] per pixel in a Vec<u8>
        let mut rgba_data = vec![0u8; (width * height * 4) as usize];
        
        for i in 0..pixel_buffer.len() {
            let argb = pixel_buffer[i];

            // Extract ARGB components
            let alpha = (argb >> 24) & 0xFF;
            let red = (argb >> 16) & 0xFF;
            let green = (argb >> 8) & 0xFF;
            let blue = argb & 0xFF;

            // Write RGBA to buffer
            let idx = i * 4;
            rgba_data[idx] = red as u8;
            rgba_data[idx + 1] = green as u8;
            rgba_data[idx + 2] = blue as u8;
            rgba_data[idx + 3] = alpha as u8;
        }

        // Update the pixmap with the new pixel data
        managed.pixmap.data_mut().copy_from_slice(&rgba_data);

        // Get the pixmap reference for presentation
        let pixmap_ref = &managed.pixmap;
        let window_clone = managed.window.clone();
        
        // Render to the window using X11
        present_to_window(&window_clone, pixmap_ref, is_transparent);
        
        // Reset needs_redraw flag after rendering
        let mut state = self.state.lock().expect("Lock poisoned");
        if let Some(window_state) = state.windows.get_mut(&managed.state_id) {
            window_state.needs_redraw = false;
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

    fn set_window_always_on_top(&self, window_id: u64, _always_on_top: bool) {
        // Note: set_always_on_top is not available in winit 0.29
        // This would require platform-specific code
        let _ = window_id;
    }

    fn set_window_title(&self, window_id: u64, title: String) {
        for managed in self.windows.values() {
            if managed.state_id == window_id {
                managed.window.set_title(&title);
                break;
            }
        }
    }

    fn set_window_ignore_input(&self, window_id: u64, ignore: bool) {
        for managed in self.windows.values() {
            if managed.state_id == window_id {
                // set_cursor_hittest(false) makes the window ignore mouse events (click-through)
                // set_cursor_hittest(true) restores normal mouse event handling
                if let Err(e) = managed.window.set_cursor_hittest(!ignore) {
                    eprintln!(
                        "Failed to set cursor hittest for window {}: {}",
                        window_id, e
                    );
                }
                break;
            }
        }
    }

    fn close_window_by_id(&mut self, window_id: u64, _event_loop: &EventLoopWindowTarget<()>) {
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

    fn close_window(&mut self, window_id: WindowId, event_loop: &EventLoopWindowTarget<()>) {
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

fn present_to_window(window: &Arc<winit::window::Window>, pixmap: &tiny_skia::Pixmap, _is_transparent: bool) {
    #[cfg(target_os = "linux")]
    {
        use std::ffi::c_void;
        use std::os::raw::{c_int, c_uint};
        use x11::xlib::{self, XOpenDisplay, XSync, XFlush, XWindowAttributes};
        use raw_window_handle::HasRawWindowHandle;
        use raw_window_handle::RawWindowHandle;

        // Get the raw window handle
        let raw_handle = window.raw_window_handle();
        
        if let RawWindowHandle::Xlib(xlib_handle) = raw_handle {
            unsafe {
                // Open X11 display
                let display = XOpenDisplay(std::ptr::null());
                if display.is_null() {
                    eprintln!("Failed to open X11 display");
                    return;
                }

                let x_window = xlib_handle.window as xlib::Window;

                // Get window attributes to determine the correct visual and depth
                let mut window_attrs: XWindowAttributes = std::mem::zeroed();
                if xlib::XGetWindowAttributes(display, x_window, &mut window_attrs) == 0 {
                    eprintln!("Failed to get window attributes");
                    xlib::XCloseDisplay(display);
                    return;
                }
                
                let visual = window_attrs.visual;
                let depth = window_attrs.depth as c_uint;

                // Get window dimensions
                let width = pixmap.width() as c_int;
                let height = pixmap.height() as c_int;

                // Get the pixmap data (RGBA format from tiny-skia)
                let rgba_data = pixmap.data();
                
                // Convert RGBA to the format expected by X11 based on depth
                let mut x11_data: Vec<u8>;
                
                if depth == 32 {
                    // 32-bit depth: use BGRA format (little-endian ARGB)
                    x11_data = Vec::with_capacity(rgba_data.len());
                    for chunk in rgba_data.chunks_exact(4) {
                        let r = chunk[0];
                        let g = chunk[1];
                        let b = chunk[2];
                        let a = chunk[3];
                        x11_data.push(b);
                        x11_data.push(g);
                        x11_data.push(r);
                        x11_data.push(a);
                    }
                } else {
                    // 24-bit depth: use BGR format (3 bytes per pixel)
                    let pixel_count = (width * height) as usize;
                    x11_data = Vec::with_capacity(pixel_count * 3);
                    for chunk in rgba_data.chunks_exact(4) {
                        let r = chunk[0];
                        let g = chunk[1];
                        let b = chunk[2];
                        // Ignore alpha for 24-bit depth
                        x11_data.push(b);
                        x11_data.push(g);
                        x11_data.push(r);
                    }
                }
                
                let data = x11_data.as_ptr() as *mut c_void;
                let bytes_per_line = if depth == 32 { 0 } else { width * 3 };
                let bitmap_pad = if depth == 32 { 32 } else { 24 };
                
                let ximage = xlib::XCreateImage(
                    display,
                    visual,
                    depth,
                    xlib::ZPixmap,
                    0,
                    data as *mut i8,
                    width as c_uint,
                    height as c_uint,
                    bitmap_pad as c_int,
                    bytes_per_line as c_int,
                );

                if ximage.is_null() {
                    eprintln!("Failed to create XImage");
                    xlib::XCloseDisplay(display);
                    return;
                }

                // Create a graphics context for the window
                let gc = xlib::XCreateGC(display, x_window, 0, std::ptr::null_mut());

                // Put the image onto the window
                xlib::XPutImage(
                    display,
                    x_window,
                    gc,
                    ximage,
                    0,  // src_x
                    0,  // src_y
                    0,  // dest_x
                    0,  // dest_y
                    width as c_uint,
                    height as c_uint,
                );

                // Flush to ensure drawing appears on screen
                XFlush(display);
                XSync(display, 0);
                
                // Clean up
                xlib::XFreeGC(display, gc);
                
                // Set data to null before destroying to prevent X11 from freeing our data
                (*ximage).data = std::ptr::null_mut();
                xlib::XDestroyImage(ximage);
                xlib::XCloseDisplay(display);
            }
        }
    }
    
    // For non-Linux platforms, we would need platform-specific code
    #[cfg(not(target_os = "linux"))]
    {
        let _ = window;
        let _ = pixmap;
        let _ = is_transparent;
        // TODO: Implement for Windows and macOS
    }
}
