extern crate pixels;

mod model;

use std::fs::File;
use std::path::Path;

use pixels::{wgpu::Surface, Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode, WindowEvent};
use winit::event_loop::{EventLoop, ControlFlow};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

use std::time::Instant;

use model::Model;

const SURFACE_WIDTH: u32 = 500;
const SURFACE_HEIGHT: u32 = 375;

const MAX_INDEX: usize = (SURFACE_WIDTH * SURFACE_HEIGHT) as usize;

fn clear(screen: &mut [u8]) {
    for bytes in screen.chunks_exact_mut(4) {
        bytes.copy_from_slice(&[0, 0, 0, 255]);
    }
}

fn set_pixel(frame: &mut [u8], x: u32, y: u32, rgba: [u8; 4]) {
    let index = ((((SURFACE_HEIGHT-y) * SURFACE_WIDTH) + x) * 4) as usize;

    if index > MAX_INDEX*4 {
        return;
    }

    let pixel_slice = &mut frame[index..index+4];

    pixel_slice[0] = rgba[0];
    pixel_slice[1] = rgba[1];
    pixel_slice[2] = rgba[2];
    pixel_slice[3] = rgba[3];
}

fn line(frame: &mut [u8], x0: u32, y0: u32, x1: u32, y1: u32, rgba: [u8; 4]) {
    let steep = i32::abs(x0 as i32 - x1 as i32) < i32::abs(y0 as i32 - y1 as i32);

    let mut m_x0 = if steep { x1 } else { x0 };
    let mut m_y0 = if steep { y1 } else { y0 };
    let mut m_x1 = if steep { x0 } else { x1 };
    let mut m_y1 = if steep { y0 } else { y1 };

    if m_x0 > m_x1 {
        std::mem::swap(&mut m_x0, &mut m_x1);
        std::mem::swap(&mut m_y0, &mut m_y1);
    }

    let dx = m_x1 as i32 - m_x0 as i32;
    let dy = m_y1 as i32 - m_y0 as i32;

    let derror = i32::abs(dy) * 2;
    let mut error = 0;

    let mut y = m_y0;

    for x in m_x0..m_x1 {
        if steep {
            set_pixel(frame, y, x, rgba);
        } else {
            set_pixel(frame, x, y, rgba);
        }

        error += derror;

        if error > dx {
            if m_y1 > m_y0 { 
                y = y + 1;
            } else { 
                y = y - 1; 
            }
            error -= dx * 2;
        }
    }
}

fn main() -> Result<(), Error> {
    let model_path = Path::new("./obj/african_head.obj");
    println!("{:?}", model_path);
    let model_file = File::open(model_path).unwrap();

    let parsed_model = Model::new(&model_file);

    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();

    let window = {
        let size = LogicalSize::new(1024 as f64, 768 as f64);
        WindowBuilder::new()
            .with_title("tinyrenderer")
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

    let red = [255, 0, 0, 0];
    let green = [0, 255, 0, 0];

    event_loop.run(move |event, _, control_flow| {
        if let Event::WindowEvent {
            event: WindowEvent::RedrawRequested,
            ..
        } = event
        {
            let previous_frame_time = last_frame;

            let frame = pixels.get_frame();
            clear(frame);
            line(frame, 13, 20, 80, 40, red);
            line(frame, 20, 13, 40, 80, red);
            line(frame, 40, 40, 80, 40, green);
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

    Ok(())
}
