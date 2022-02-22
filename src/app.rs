use pixels::{Pixels, SurfaceTexture};
use std::process;
use std::sync::{Arc, Mutex};
use std::time::SystemTime;
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};

use crate::canvas::Canvas;

pub struct App<C>
where
    C: 'static,
{
    window: Window,
    event_loop: EventLoop<()>,
    canvas: Option<Arc<Mutex<Canvas>>>,
    context: Option<Arc<Mutex<C>>>,
}

impl<C> App<C> {
    pub fn new(name: String) -> Self {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_title(&name)
            .with_inner_size(LogicalSize::new(200u16, 200u16))
            .with_resizable(true)
            .build(&event_loop)
            .unwrap();

        Self {
            window,
            event_loop,
            canvas: None,
            context: None,
        }
    }

    pub fn set_canvas(&mut self, canvas: Canvas) {
        self.window
            .set_inner_size(LogicalSize::new(canvas.width, canvas.height));
        self.window.request_redraw();

        self.canvas = Some(Arc::new(Mutex::new(canvas)));
    }

    pub fn set_context(&mut self, context: C)
    where
        C: Sized,
    {
        self.context = Some(Arc::new(Mutex::new(context)));
    }

    pub fn draw<F>(self, draw_fn: F)
    where
        F: Fn(&mut Canvas, &mut C) + 'static,
    {
        let canvas = self.canvas.unwrap();
        let context = self.context.unwrap();

        let window_size = self.window.inner_size();

        let surface_texture =
            SurfaceTexture::new(window_size.width, window_size.height, &self.window);

        let mut pixels =
            Pixels::new(window_size.width, window_size.height, surface_texture).unwrap();

        let frame_rate = 60.;
        let frame_delay = 1_000_000_000f64 / frame_rate;

        let mut last_time = SystemTime::now();

        self.event_loop.run(move |event, _, control_flow| {
            let elapsed = last_time.elapsed().unwrap().as_nanos() as f64;

            if elapsed > frame_delay {
                last_time = SystemTime::now();
                self.window.request_redraw();
            }

            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    window_id,
                } if window_id == self.window.id() => {
                    *control_flow = ControlFlow::Exit;
                    process::exit(0);
                }
                Event::WindowEvent {
                    event: WindowEvent::Resized(size),
                    window_id,
                } if window_id == self.window.id() => {
                    pixels.resize_surface(size.width, size.height)
                }
                Event::RedrawRequested(_window_id) => {
                    let mut canvas = canvas.lock().unwrap();
                    let mut context = context.lock().unwrap();

                    draw_fn(&mut canvas, &mut context);

                    let frame = canvas.get_frame();

                    for (i, pixel) in pixels.get_frame().chunks_exact_mut(4).enumerate() {
                        let p = frame.get(i).unwrap();
                        pixel.clone_from_slice(p);
                    }

                    if pixels.render().is_err() {
                        *control_flow = ControlFlow::Exit;
                        return;
                    }
                }
                _ => {}
            }
        });
    }
}
