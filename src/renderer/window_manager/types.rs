use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use winit::window::WindowId;

/// Shared state between the window manager and the event loop
pub type SharedState = Arc<Mutex<WindowManagerState>>;

/// Window manager state that can be accessed from both JS and the event loop
pub struct WindowManagerState {
    pub windows: HashMap<u64, WindowState>,
    pub pending_commands: Vec<WindowCommand>,
    pub should_exit: bool,
}

/// State for each managed window
pub struct WindowState {
    pub width: u32,
    pub height: u32,
    pub pixel_buffer: Vec<u32>,
    pub needs_redraw: bool,
    pub title: String,
    pub x: Option<i32>,
    pub y: Option<i32>,
    pub always_on_top: bool,
    pub transparent: bool,
    pub decorations: bool,
    pub winit_id: Option<WindowId>,
}

/// Commands that can be sent to the window manager
pub enum WindowCommand {
    CreateWindow {
        id: u64,
        width: u32,
        height: u32,
        title: String,
        x: Option<i32>,
        y: Option<i32>,
        always_on_top: bool,
        transparent: bool,
        decorations: bool,
    },
    SetPixel {
        window_id: u64,
        x: u32,
        y: u32,
        color: u32,
    },
    Clear {
        window_id: u64,
        color: u32,
    },
    Present {
        window_id: u64,
    },
    SetPosition {
        window_id: u64,
        x: i32,
        y: i32,
    },
    SetAlwaysOnTop {
        window_id: u64,
        always_on_top: bool,
    },
    SetTitle {
        window_id: u64,
        title: String,
    },
    SetIgnoreInput {
        window_id: u64,
        ignore: bool,
    },
    CloseWindow {
        window_id: u64,
    },
}

/// Internal window data managed by the event loop
pub struct ManagedWindow {
    pub window: Arc<winit::window::Window>,
    pub surface: softbuffer::Surface<Arc<winit::window::Window>, Arc<winit::window::Window>>,
    pub _context: softbuffer::Context<Arc<winit::window::Window>>,
    pub state_id: u64,
}
