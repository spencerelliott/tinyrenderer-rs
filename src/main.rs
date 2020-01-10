#[macro_use]
extern crate lazy_static;
extern crate pixels;

mod model;

use std::fs::File;
use std::io::Write;
use std::path::Path;

use pixels::{wgpu::Surface, Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

use std::time::Instant;

use model::Model;

const SURFACE_WIDTH: u32 = 800;
const SURFACE_HEIGHT: u32 = 600;

const ASPECT_RATIO: f32 = SURFACE_WIDTH as f32 / SURFACE_HEIGHT as f32;

const BUFFER_SIZE: usize = (SURFACE_HEIGHT * SURFACE_WIDTH * 4) as usize;

const CLEAR_BUFFER: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];

fn clear(mut screen: &mut [u8]) {
    screen.write(&CLEAR_BUFFER).unwrap();
}

fn set_pixel(frame: &mut [u8], x: u32, y: u32, rgba: [u8; 4]) {
    if x >= SURFACE_WIDTH || y >= SURFACE_HEIGHT {
        return;
    }

    let index = ((((SURFACE_HEIGHT - 1 - y) * SURFACE_WIDTH) + x) * 4) as usize;
    let pixel_slice = &mut frame[index..index + 4];

    pixel_slice[0] = rgba[0];
    pixel_slice[1] = rgba[1];
    pixel_slice[2] = rgba[2];
    pixel_slice[3] = rgba[3];
}

fn line(frame: &mut [u8], x0: u32, y0: u32, x1: u32, y1: u32, rgba: [u8; 4]) {
    let steep = i32::abs(x0 as i32 - x1 as i32) < i32::abs(y0 as i32 - y1 as i32);

    let mut m_x0 = x0;
    let mut m_y0 = y0;
    let mut m_x1 = x1;
    let mut m_y1 = y1;

    if steep {
        std::mem::swap(&mut m_x0, &mut m_y0);
        std::mem::swap(&mut m_x1, &mut m_y1);
    }

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
    let mut frame_delta: u128 = 0;
    let mut frame_count = 0;

    let white = [255, 255, 255, 0];
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

            for face in parsed_model.iter_faces() {
                for vert_idx in 0..3 {
                    if let Some(v0) = parsed_model.get_vertex(face.point[vert_idx] as usize) {
                        if let Some(v1) =
                            parsed_model.get_vertex(face.point[(vert_idx + 1) % 3] as usize)
                        {
                            let x0 = ((v0.x + ASPECT_RATIO) * SURFACE_WIDTH as f32 / 2.0 / ASPECT_RATIO).round();
                            let y0 = ((v0.y + 1.0) * SURFACE_HEIGHT as f32 / 2.0).round();
                            let x1 = ((v1.x + ASPECT_RATIO) * SURFACE_WIDTH as f32 / 2.0 / ASPECT_RATIO).round();
                            let y1 = ((v1.y + 1.0) * SURFACE_HEIGHT as f32 / 2.0).round();

                            line(frame, x0 as u32, y0 as u32, x1 as u32, y1 as u32, white);
                        } else {
                            println!("Missing index {:?} for v1", vert_idx);
                        }
                    } else {
                        println!("Missing index {:?} for v0", vert_idx);
                    }
                }
            }
            pixels.render();

            last_frame = Instant::now();

            let delta = last_frame - previous_frame_time;
            frame_delta = frame_delta + delta.as_millis();

            if frame_delta >= 1000 {
                window.set_title(&format!("tinyrenderer ({} fps)", frame_count));

                frame_delta = frame_delta - 1000;
                frame_count = 0;

            }

            frame_count = frame_count + 1;
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
