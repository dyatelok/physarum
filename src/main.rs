#![deny(clippy::all)]
#![forbid(unsafe_code)]

use crate::gui::Framework;
use error_iter::ErrorIter as _;
use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use std::time::Instant;
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

mod gui;

use crate::sim::consts::HEIGHT;
use crate::sim::consts::WIDTH;
use crate::sim::world::World;

use crate::sim::consts::TARGET_FPS;
use crate::sim::consts::TPF;

use crate::sim::params::Params;

pub mod sim;

fn main() -> Result<(), Error> {

    env_logger::init();
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64 * 2.0, HEIGHT as f64 * 2.0);
        WindowBuilder::new()
            .with_title("Physarum + Pixels + egui")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };    

    let (mut pixels, mut framework) = {
        let window_size = window.inner_size();
        let scale_factor = window.scale_factor() as f32;
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        let pixels = Pixels::new(WIDTH, HEIGHT, surface_texture)?;
        let framework = Framework::new(
            &event_loop,
            window_size.width,
            window_size.height,
            scale_factor,
            &pixels,
        );

        (pixels, framework)
    };

    let mut params = Params::new();

    let mut world = World::new(&params);

    event_loop.run(move |event, _, control_flow| {
        let start_time = Instant::now();
        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            if input.key_pressed(VirtualKeyCode::Space) {
                world.update(&params);
            }

            // Update the scale factor
            if let Some(scale_factor) = input.scale_factor() {
                framework.scale_factor(scale_factor);
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                if let Err(err) = pixels.resize_surface(size.width, size.height) {
                    log_error("pixels.resize_surface", err);
                    *control_flow = ControlFlow::Exit;
                    return;
                }
                framework.resize(size.width, size.height);
            }

            // Update internal state and request a redraw

            if params.do_update_world {
                world = World::new(&params);
                params.do_update_world = false;
            }
            
            let start_compute = Instant::now();
            for _ in 0..TPF {
                world.update(&params);
            }
            let compute_time = Instant::now().duration_since(start_compute).as_secs_f32();

            let start_draw = Instant::now();
            window.request_redraw();
            let draw_time = Instant::now().duration_since(start_draw).as_secs_f32();


            let elapsed_time_f32 = Instant::now().duration_since(start_time).as_secs_f32();


            let fps = 1.0 / elapsed_time_f32;

            println!("tpf: {} , fps: {:.1} , loop time: {:.2} ms , total compute time: {:.2} ms , compute time per tick: {:.2} ms , draw time: {:.2} ms", 
                TPF, 
                fps, 
                elapsed_time_f32 * 1000.0,
                compute_time * 1000.0,
                compute_time * 1000.0 / TPF as f32,
                draw_time * 1000.0
            );

            let elapsed_time = (elapsed_time_f32 * 1000.0) as u64;

                        let wait_millis = match 1000 / TARGET_FPS >= elapsed_time {
                true => 1000 / TARGET_FPS - elapsed_time,
                false => 0,
            };
            let new_inst = start_time + std::time::Duration::from_millis(wait_millis);

            *control_flow = ControlFlow::WaitUntil(new_inst);
        }

        match event {
            Event::WindowEvent { event, .. } => {
                // Update egui inputs
                framework.handle_event(&event);
            }
            // Draw the current frame
            Event::RedrawRequested(_) => {
                // Draw the world
                world.draw(pixels.frame_mut(),&params);

                // Prepare egui
                framework.prepare(&window,&mut params);

                // Render everything together
                let render_result = pixels.render_with(|encoder, render_target, context| {
                    // Render the world texture
                    context.scaling_renderer.render(encoder, render_target);

                    // Render egui
                    framework.render(encoder, render_target, context);

                    Ok(())
                });

                // Basic error handling
                if let Err(err) = render_result {
                    log_error("pixels.render", err);
                    *control_flow = ControlFlow::Exit;
                }
            }
            _ => (),
        }
    });
}

fn log_error<E: std::error::Error + 'static>(method_name: &str, err: E) {
    error!("{method_name}() failed: {err}");
    for source in err.sources().skip(1) {
        error!("  Caused by: {source}");
    }
}








