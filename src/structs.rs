use napi_derive::napi;
use napi::bindgen_prelude::*;
use gpui::{Application, Pixels as GPixels, Point as GPoint, Size as GSize, Bounds as GBounds, Render, IntoElement, ParentElement, div, Window, Context, App, AppContext};
use std::cell::RefCell;
use crate::renderer::render_div;
use napi::threadsafe_function::ThreadsafeFunctionCallMode;

thread_local! {
    static APP_CX: RefCell<Option<*mut App>> = RefCell::new(None);
}

pub struct NapiRootView {
    pub(crate) root: Option<DivElement>,
}

impl Render for NapiRootView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        if let Some(ref root) = self.root {
            render_div(root)
        } else {
            div().child("No root set")
        }
    }
}

#[napi(object)]
#[derive(Clone, Copy)]
pub struct Pixels {
    pub value: f64,
}

impl From<Pixels> for GPixels {
    fn from(p: Pixels) -> Self {
        gpui::px(p.value as f32)
    }
}

#[napi(object)]
#[derive(Clone, Copy)]
pub struct Size {
    pub width: f64,
    pub height: f64,
}

impl From<Size> for GSize<GPixels> {
    fn from(s: Size) -> Self {
        GSize {
            width: gpui::px(s.width as f32),
            height: gpui::px(s.height as f32),
        }
    }
}

#[napi]
#[derive(Clone)]
pub struct Hsla {
    pub h: f64,
    pub s: f64,
    pub l: f64,
    pub a: f64,
}

impl From<Hsla> for gpui::Hsla {
    fn from(color: Hsla) -> Self {
        gpui::Hsla {
            h: color.h as f32,
            s: color.s as f32,
            l: color.l as f32,
            a: color.a as f32,
        }
    }
}
#[napi]
impl Hsla {
    #[napi(constructor)]
    pub fn new(h: f64, s: f64, l: f64, a: f64) -> Self {
        Self { h, s, l, a }
    }

    #[napi]
    pub fn to_rgb(&self) -> Rgba {
        let gpui_hsla: gpui::Hsla = self.clone().into();
        let gpui_rgba: gpui::Rgba = gpui_hsla.into();
        Rgba {
            r: gpui_rgba.r as f64,
            g: gpui_rgba.g as f64,
            b: gpui_rgba.b as f64,
            a: gpui_rgba.a as f64,
        }
    }
}

#[napi]
#[derive(Clone)]
pub struct Rgba {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
}

impl From<Rgba> for gpui::Rgba {
    fn from(color: Rgba) -> Self {
        gpui::Rgba {
            r: color.r as f32,
            g: color.g as f32,
            b: color.b as f32,
            a: color.a as f32,
        }
    }
}

#[napi]
impl Rgba {
    #[napi(constructor)]
    pub fn new(r: f64, g: f64, b: f64, a: f64) -> Self {
        Self { r, g, b, a }
    }

    #[napi]
    pub fn to_hsla(&self) -> Hsla {
        let gpui_rgba: gpui::Rgba = self.clone().into();
        let gpui_hsla: gpui::Hsla = gpui_rgba.into();
        Hsla {
            h: gpui_hsla.h as f64,
            s: gpui_hsla.s as f64,
            l: gpui_hsla.l as f64,
            a: gpui_hsla.a as f64,
        }
    }
}

#[napi(object)]
#[derive(Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl From<Point> for GPoint<GPixels> {
    fn from(p: Point) -> Self {
        GPoint {
            x: gpui::px(p.x as f32),
            y: gpui::px(p.y as f32),
        }
    }
}

#[napi(object)]
#[derive(Clone, Copy)]
pub struct Bounds {
    pub origin: Point,
    pub size: Size,
}

impl From<Bounds> for GBounds<GPixels> {
    fn from(b: Bounds) -> Self {
        GBounds {
            origin: b.origin.into(),
            size: b.size.into(),
        }
    }
}

#[napi(object)]
pub struct WindowOptions {
    pub bounds: Option<Bounds>,
    pub titlebar: Option<bool>,
    pub center: Option<bool>,
    pub focus: Option<bool>,
    pub show: Option<bool>,
    pub kind: Option<crate::enums::WindowKind>,
    pub is_movable: Option<bool>,
    pub display_id: Option<u32>,
}

#[napi]
pub struct AppHandle {
    // In a real implementation, we might store a reference-counted handle or use a global.
}

#[napi]
impl AppHandle {
    #[napi(constructor)]
    pub fn new() -> Self {
        Self {}
    }

    #[napi]
    pub fn run(&self, callback: Function<(), ()>) -> Result<()> {
        let app = Application::new();
        let tsfn = callback.build_threadsafe_function()
            .build()?;

        app.run(move |cx| {
            println!("GPUI App started. Bridging context to JS...");
            
            APP_CX.with(|rcx: &RefCell<Option<*mut App>>| {
                *rcx.borrow_mut() = Some(cx as *mut App);
            });

            // Call the JS callback - this will set up windows and UI
            let _ = tsfn.call((), ThreadsafeFunctionCallMode::Blocking);
            
            println!("JS callback completed, entering main loop...");
            
            // Keep the app running - don't exit here
            // The app will continue running until all windows are closed
        });
        Ok(())
    }

    #[napi]
    pub fn open_window(&self, options: WindowOptions) -> WindowHandle {
        // Set default window bounds if not provided
        let window_bounds = options.bounds.map(|b| gpui::WindowBounds::Windowed(b.into()))
            .or_else(|| {
                // Default window size: 800x600
                let default_bounds = GBounds {
                    origin: GPoint { x: gpui::px(100.0), y: gpui::px(100.0) },
                    size: GSize { width: gpui::px(800.0), height: gpui::px(600.0) },
                };
                Some(gpui::WindowBounds::Windowed(default_bounds))
            });

        let gpui_options = gpui::WindowOptions {
            window_bounds,
            titlebar: options.titlebar.map(|_| gpui::TitlebarOptions {
                title: None,
                appears_transparent: false,
                traffic_light_position: None,
            }),
            focus: options.focus.unwrap_or(true),
            show: options.show.unwrap_or(true),
            kind: gpui::WindowKind::Normal,
            is_movable: options.is_movable.unwrap_or(true),
            display_id: None, // DisplayId constructor is private, can't convert from u32
            ..Default::default()
        };

        println!("Opening window with GPUI options...");

        let handle = APP_CX.with(|rcx: &RefCell<Option<*mut App>>| {
            let ptr = rcx.borrow().expect("AppContext not found. Are you calling open_window outside of app.run?");
            let cx = unsafe { &mut *ptr };

            cx.open_window(gpui_options, |_window, cx| {
                cx.new(|_cx| NapiRootView { root: None })
            })
        });

        WindowHandle { id: 1, handle: Some(handle.expect("Failed to open window")) }
    }
}

#[napi]
pub struct WindowHandle {
    pub id: u32,
    pub(crate) handle: Option<gpui::WindowHandle<NapiRootView>>,
}

#[napi]
impl WindowHandle {
    #[napi]
    pub fn close(&self) { 
        println!("Closing window {}", self.id);
    }

    #[napi]
    pub fn focus(&self) { 
        println!("Focusing window {}", self.id);
    }

    #[napi]
    pub fn set_root(&self, root: &DivElement) {
        println!("Setting root element for window {}", self.id);
        if let Some(ref handle) = self.handle {
            let root = root.clone();
            APP_CX.with(|rcx: &RefCell<Option<*mut App>>| {
                if let Some(ptr) = *rcx.borrow() {
                    let cx = unsafe { &mut *ptr };
                    let _ = handle.update(cx, move |view, _window, _cx| {
                        view.root = Some(root);
                    });
                } else {
                    println!("Error: AppContext not available in set_root");
                }
            });
        }
    }
}

#[napi]
#[derive(Clone)]
pub struct DivElement {
    pub(crate) width: Option<f64>,
    pub(crate) height: Option<f64>,
    pub(crate) background: Option<String>,
    pub(crate) padding: Option<f64>,
    pub(crate) margin: Option<f64>,
    pub(crate) border_width: Option<f64>,
    pub(crate) border_color: Option<String>,
    pub(crate) corner_radius: Option<f64>,
    pub(crate) flex: bool,
    pub(crate) flex_direction: Option<crate::enums::FlexDirection>,
    pub(crate) children: Vec<Child>,
}

#[derive(Clone)]
pub enum Child {
    Text(String),
    Element(DivElement),
}

impl Default for DivElement {
    fn default() -> Self {
        Self {
            width: None,
            height: None,
            background: None,
            padding: None,
            margin: None,
            border_width: None,
            border_color: None,
            corner_radius: None,
            flex: false,
            flex_direction: None,
            children: Vec::new(),
        }
    }
}

#[napi]
impl DivElement {
    #[napi(constructor)]
    pub fn new() -> Self {
        Self::default()
    }

    #[napi]
    pub fn child(&mut self, child: Either<String, &DivElement>) -> Self {
        match child {
            Either::A(text) => self.children.push(Child::Text(text)),
            Either::B(element) => self.children.push(Child::Element(element.clone())),
        }
        self.clone()
    }

    #[napi]
    pub fn flex(&mut self) -> Self { 
        self.flex = true;
        self.clone() 
    }
    
    #[napi]
    pub fn flex_col(&mut self) -> Self { 
        self.flex = true;
        self.flex_direction = Some(crate::enums::FlexDirection::Column);
        self.clone() 
    }

    #[napi]
    pub fn flex_row(&mut self) -> Self { 
        self.flex = true;
        self.flex_direction = Some(crate::enums::FlexDirection::Row);
        self.clone() 
    }

    #[napi]
    pub fn w_full(&mut self) -> Self { 
        self.width = Some(100.0); 
        self.clone() 
    }

    #[napi]
    pub fn h_full(&mut self) -> Self { 
        self.height = Some(100.0);
        self.clone() 
    }

    #[napi]
    pub fn bg(&mut self, color: String) -> Self { 
        self.background = Some(color);
        self.clone() 
    }

    #[napi]
    pub fn p(&mut self, value: f64) -> Self { 
        self.padding = Some(value);
        self.clone() 
    }

    #[napi]
    pub fn m(&mut self, value: f64) -> Self { 
        self.margin = Some(value);
        self.clone() 
    }

    #[napi]
    pub fn border(&mut self, value: f64) -> Self { 
        self.border_width = Some(value);
        self.clone() 
    }

    #[napi]
    pub fn border_color(&mut self, color: String) -> Self { 
        self.border_color = Some(color);
        self.clone() 
    }

    #[napi]
    pub fn rounded(&mut self, value: f64) -> Self { 
        self.corner_radius = Some(value);
        self.clone() 
    }

    #[napi]
    pub fn shadow(&mut self) -> Self { self.clone() }

    #[napi]
    pub fn gap(&mut self, _value: f64) -> Self { self.clone() }

    #[napi]
    pub fn on_click(&mut self, _callback: Function) -> Self { self.clone() }
}

#[napi]
#[derive(Clone)]
pub struct ImageElement {}

#[napi]
impl ImageElement {
    #[napi(constructor)]
    pub fn new() -> Self { Self {} }
    #[napi]
    pub fn source(&self, _src: String) -> Self { self.clone() }
}

#[napi]
#[derive(Clone)]
pub struct SvgElement {}

#[napi]
impl SvgElement {
    #[napi(constructor)]
    pub fn new() -> Self { Self {} }
    #[napi]
    pub fn path(&self, _path: String) -> Self { self.clone() }
}

#[napi]
#[derive(Clone)]
pub struct TextElement {}

#[napi]
impl TextElement {
    #[napi(constructor)]
    pub fn new() -> Self { Self {} }
    #[napi]
    pub fn text(&self, _content: String) -> Self { self.clone() }
}

#[napi]
#[derive(Clone)]
pub struct ListElement {}

#[napi]
impl ListElement {
    #[napi(constructor)]
    pub fn new() -> Self { Self {} }
}

#[napi]
#[derive(Clone)]
pub struct UniformListElement {}

#[napi]
impl UniformListElement {
    #[napi(constructor)]
    pub fn new() -> Self { Self {} }

    #[napi]
    pub fn items(&self, _count: u32) -> Self { self.clone() }
}

#[napi]
pub struct FocusHandle {
    pub id: u32,
}

#[napi]
pub struct Model {
    pub id: u32,
}

#[napi]
impl Model {
    #[napi]
    pub fn subscribe(&self, _callback: Function) -> Subscription {
        Subscription {}
    }

    #[napi]
    pub fn update(&self, _callback: Function) {
        // Update model state
    }
}

#[napi]
pub struct View {
    pub id: u32,
}

#[napi]
pub struct Subscription {}

#[napi]
impl Subscription {
    #[napi]
    pub fn unsubscribe(&self) {
        println!("Unsubscribed");
    }
}

#[napi]
pub struct Task {}

#[napi]
impl Task {
    #[napi]
    pub fn detach(&self) {
        println!("Task detached");
    }
}

#[napi(object)]
pub struct MouseEvent {
    pub position: Point,
    pub button: crate::enums::MouseButton,
    pub modifiers: KeyModifiers,
}

#[napi(object)]
pub struct KeyboardEvent {
    pub key: String,
    pub modifiers: KeyModifiers,
}

#[napi(object)]
pub struct KeyModifiers {
    pub shift: bool,
    pub control: bool,
    pub alt: bool,
    pub meta: bool,
}

#[napi]
pub struct NapiAppContext {}

#[napi]
pub struct NapiVisualContext {}

#[napi(object)]
pub struct EntityId {
    pub id: i64,
}

#[napi]
pub struct Entity {
    pub id: i64,
}
