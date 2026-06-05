#![windows_subsystem = "console"]

use gl::types::*;
use glutin::{
    config::ConfigTemplateBuilder,
    context::{ContextApi, ContextAttributesBuilder, NotCurrentGlContext},
    display::GetGlDisplay,
    prelude::*,
    surface::{SurfaceAttributesBuilder, WindowSurface},
};
use glutin_winit::DisplayBuilder;
use std::num::NonZeroU32;
use winit::raw_window_handle::HasWindowHandle;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowId},
};

struct App {
    window: Option<Window>,
    gl_surface: Option<glutin::surface::Surface<WindowSurface>>,
    gl_context: Option<glutin::context::PossiblyCurrentContext>,
}

impl App {
    fn new() -> Self {
        Self { window: None, gl_surface: None, gl_context: None }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attrs = Window::default_attributes()
            .with_title("Game")
            .with_inner_size(winit::dpi::LogicalSize::new(1280u32, 720u32));

        let template = ConfigTemplateBuilder::new();
        let display_builder = DisplayBuilder::new().with_window_attributes(Some(window_attrs));

        let (window, gl_config) = display_builder
            .build(event_loop, template, |configs| {
                configs
                    .reduce(|a, b| if a.num_samples() > b.num_samples() { a } else { b })
                    .unwrap()
            })
            .unwrap();

        let window = window.unwrap();
        let raw_window_handle = window.window_handle().unwrap().as_raw();
        let gl_display = gl_config.display();

        let context_attrs = ContextAttributesBuilder::new()
            .with_context_api(ContextApi::OpenGl(Some(glutin::context::Version::new(4, 6))))
            .build(Some(raw_window_handle));

        let not_current_ctx = unsafe {
            gl_display.create_context(&gl_config, &context_attrs).unwrap()
        };

        let (width, height): (u32, u32) = window.inner_size().into();
        let surface_attrs = SurfaceAttributesBuilder::<WindowSurface>::new().build(
            raw_window_handle,
            NonZeroU32::new(width).unwrap(),
            NonZeroU32::new(height).unwrap(),
        );

        let surface = unsafe {
            gl_display.create_window_surface(&gl_config, &surface_attrs).unwrap()
        };

        let context = not_current_ctx.make_current(&surface).unwrap();

        gl::load_with(|s| {
            let s = std::ffi::CString::new(s).unwrap();
            gl_display.get_proc_address(&s)
        });

        self.window = Some(window);
        self.gl_surface = Some(surface);
        self.gl_context = Some(context);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => {
                if size.width > 0 && size.height > 0 {
                    if let (Some(surface), Some(context)) =
                        (&self.gl_surface, &self.gl_context)
                    {
                        surface.resize(
                            context,
                            NonZeroU32::new(size.width).unwrap(),
                            NonZeroU32::new(size.height).unwrap(),
                        );
                        unsafe {
                            gl::Viewport(0, 0, size.width as GLsizei, size.height as GLsizei);
                        }
                    }
                }
            }
            WindowEvent::RedrawRequested => {
                // -------------------------------------------------------
                // Your render code goes here
                // -------------------------------------------------------
                unsafe {
                    gl::ClearColor(0.1, 0.1, 0.15, 1.0);
                    gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
                }
                // -------------------------------------------------------

                if let (Some(surface), Some(context)) = (&self.gl_surface, &self.gl_context) {
                    surface.swap_buffers(context).unwrap();
                }
                self.window.as_ref().unwrap().request_redraw();
            }
            _ => {}
        }
    }
}

fn main() {
    println!("hello");

    let event_loop = EventLoop::new().unwrap();
    let mut app = App::new();
    event_loop.run_app(&mut app).unwrap();
}