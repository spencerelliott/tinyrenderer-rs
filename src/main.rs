extern crate pixels;

use pixels::{wgpu::Surface, Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode, WindowEvent};
use winit::event_loop::{EventLoop, ControlFlow};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

fn main() -> Result<(), Error> {
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();

    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Hello tinyrenderer")
            .with_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut hidpi_factor = window.hidpi_factor();

    let mut pixels = {
        let surface = Surface::create(&window);
        let surface_texture = SurfaceTexture::new(WIDTH, HEIGHT, surface);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };

    event_loop.run(move |event, _, control_flow| {
        if let Event::WindowEvent {
            event: WindowEvent::RedrawRequested,
            ..
        } = event
        {
            pixels.render();
        }

        if input.update(event) {
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            if let Some(factor) = input.hidpi_changed() {
                hidpi_factor = factor;
            }

            if let Some(size) = input.window_resized() {
               let size = size.to_physical(hidpi_factor);
               let width = size.width.round() as u32;
               let height = size.height.round() as u32;

               pixels.resize(width, height);
            }

            window.request_redraw();
        }
    });
}
