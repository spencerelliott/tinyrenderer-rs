extern crate pixels;

use pixels::{wgpu::Surface, Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode, WindowEvent};
use winit::event_loop::{EventLoop, ControlFlow};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

use std::time::Instant;

const SURFACE_WIDTH: u32 = 320;
const SURFACE_HEIGHT: u32 = 240;

fn clear(screen: &mut [u8]) {
    for (i, byte) in screen.iter_mut().enumerate() {
        *byte = if i % 4 == 3 { 255 } else { 0 };
    }
}

fn set_pixel(frame: &mut [u8], x: u32, y: u32, rgba: [u8; 4]) {
    let index = (((y * SURFACE_WIDTH) + x) * 4) as usize;
    let pixel_slice = &mut frame[index..index+4];

    pixel_slice[0] = rgba[0];
    pixel_slice[1] = rgba[1];
    pixel_slice[2] = rgba[2];
    pixel_slice[3] = rgba[3];
}

fn main() -> Result<(), Error> {
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();

    let window = {
        let size = LogicalSize::new(1024 as f64, 768 as f64);
        WindowBuilder::new()
            .with_title("Hello tinyrenderer")
            .with_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut hidpi_factor = window.hidpi_factor();

    let mut pixels = {
        let surface = Surface::create(&window);
        let surface_texture = SurfaceTexture::new(SURFACE_WIDTH, SURFACE_HEIGHT, surface);
        Pixels::new(SURFACE_WIDTH, SURFACE_HEIGHT, surface_texture)?
    };

    let mut last_frame = Instant::now();

    event_loop.run(move |event, _, control_flow| {
        if let Event::WindowEvent {
            event: WindowEvent::RedrawRequested,
            ..
        } = event
        {
            let previous_frame_time = last_frame;

            let frame = pixels.get_frame();
            clear(frame);
            set_pixel(frame, 10, 10, [255, 0, 0, 0]);
            set_pixel(frame, 11, 10, [255, 0, 0, 0]);
            set_pixel(frame, 12, 10, [255, 0, 0, 0]);
            pixels.render();

            last_frame = Instant::now();

            let delta = last_frame - previous_frame_time;

            let fps = (1.0 / ((delta.as_millis() as f64) / 1000.0)).round();

            window.set_title(&format!("tinyrenderer ({} fps)", fps));
        }

        if input.update(event) {
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            if let Some(factor) = input.hidpi_changed() {
                hidpi_factor = factor;
            }

            window.request_redraw();
        }
    });
}
