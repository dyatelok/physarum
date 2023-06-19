use egui::{ClippedPrimitive, Context, TexturesDelta};
use egui_wgpu::renderer::{Renderer, ScreenDescriptor};
use pixels::{wgpu, PixelsContext};
use winit::event_loop::EventLoopWindowTarget;
use winit::window::Window;

use std::f32::consts::PI;

use crate::sim::params::Params;

/// Manages all state required for rendering egui over `Pixels`.
pub(crate) struct Framework {
    // State for egui.
    egui_ctx: Context,
    egui_state: egui_winit::State,
    screen_descriptor: ScreenDescriptor,
    renderer: Renderer,
    paint_jobs: Vec<ClippedPrimitive>,
    textures: TexturesDelta,

    // State for the GUI
    gui: Gui,
}

impl Framework {
    /// Create egui.
    pub(crate) fn new<T>(
        event_loop: &EventLoopWindowTarget<T>,
        width: u32,
        height: u32,
        scale_factor: f32,
        pixels: &pixels::Pixels,
    ) -> Self {
        let max_texture_size = pixels.device().limits().max_texture_dimension_2d as usize;

        let egui_ctx = Context::default();
        let mut egui_state = egui_winit::State::new(event_loop);
        egui_state.set_max_texture_side(max_texture_size);
        egui_state.set_pixels_per_point(scale_factor);
        let screen_descriptor = ScreenDescriptor {
            size_in_pixels: [width, height],
            pixels_per_point: scale_factor,
        };
        let renderer = Renderer::new(pixels.device(), pixels.render_texture_format(), None, 1);
        let textures = TexturesDelta::default();
        let gui = Gui::new();

        Self {
            egui_ctx,
            egui_state,
            screen_descriptor,
            renderer,
            paint_jobs: Vec::new(),
            textures,
            gui,
        }
    }

    /// Handle input events from the window manager.
    pub(crate) fn handle_event(&mut self, event: &winit::event::WindowEvent) {
        let _ = self.egui_state.on_event(&self.egui_ctx, event);
    }

    /// Resize egui.
    pub(crate) fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.screen_descriptor.size_in_pixels = [width, height];
        }
    }

    /// Update scaling factor.
    pub(crate) fn scale_factor(&mut self, scale_factor: f64) {
        self.screen_descriptor.pixels_per_point = scale_factor as f32;
    }

    /// Prepare egui.
    pub(crate) fn prepare(&mut self, window: &Window, params: &mut Params) {
        // Run the egui frame and create all paint jobs to prepare for rendering.
        let raw_input = self.egui_state.take_egui_input(window);
        let output = self.egui_ctx.run(raw_input, |egui_ctx| {
            // Draw the demo application.
            self.gui.ui(egui_ctx, params);
        });

        self.textures.append(output.textures_delta);
        self.egui_state
            .handle_platform_output(window, &self.egui_ctx, output.platform_output);
        self.paint_jobs = self.egui_ctx.tessellate(output.shapes);
    }

    /// Render egui.
    pub(crate) fn render(
        &mut self,
        encoder: &mut wgpu::CommandEncoder,
        render_target: &wgpu::TextureView,
        context: &PixelsContext,
    ) {
        // Upload all resources to the GPU.
        for (id, image_delta) in &self.textures.set {
            self.renderer
                .update_texture(&context.device, &context.queue, *id, image_delta);
        }
        self.renderer.update_buffers(
            &context.device,
            &context.queue,
            encoder,
            &self.paint_jobs,
            &self.screen_descriptor,
        );

        // Render egui with WGPU
        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("egui"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: render_target,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });

            self.renderer
                .render(&mut rpass, &self.paint_jobs, &self.screen_descriptor);
        }

        // Cleanup
        let textures = std::mem::take(&mut self.textures);
        for id in &textures.free {
            self.renderer.free_texture(id);
        }
    }
}

/// Example application state. A real application will need a lot more state than this.
struct Gui {
    /// Only show the egui window when true.
    about_window: bool,
    parameters_window: bool,
    saveload_window: bool,
}

impl Gui {
    /// Create a `Gui`.
    fn new() -> Self {
        Self {
            about_window: false,
            parameters_window: false,
            saveload_window: false,
        }
    }

    /// Create the UI using egui.
    fn ui(&mut self, ctx: &Context, params: &mut Params) {
        egui::TopBottomPanel::top("menubar_container").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("About", |_| {
                    self.about_window = true;
                });
                ui.menu_button("Parameters", |_| {
                    self.parameters_window = true;
                });
                ui.menu_button("Save / Load", |_| {
                    self.saveload_window = true;
                });
                ui.horizontal(|ui| {
                    ui.toggle_value(&mut params.do_render_pheromone, "pheromone");
                    ui.toggle_value(&mut params.do_render_agents, "agents");
                });
            });
        });

        egui::Window::new("About")
            .open(&mut self.about_window)
            .show(ctx, |ui| {
                ui.label("Learn more about this program at");
                ui.hyperlink("https://github.com/dyatelok/physarum");
            });

        egui::Window::new("Paramerers")
            .open(&mut self.parameters_window)
            .show(ctx, |ui| {
                ui.label("Those are simulation parameters.");

                ui.separator();

                ui.label("Random wobble per step:");
                ui.add(egui::Slider::new(&mut params.wobble, 0.0..=PI / 6.0).text("radians"));

                ui.separator();

                ui.label("Forced rotation per step:");
                ui.add(egui::Slider::new(&mut params.forced_rot, 0.0..=PI / 12.0).text("radians"));

                ui.separator();

                if ui.button("Randomize parameters").clicked() {
                    params.randomize();
                }
            });

        egui::Window::new("Save / Load")
            .open(&mut self.saveload_window)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    if ui.button("Save parameters").clicked() {
                        params.save();
                    }
                    ui.label("Save to file:");
                });
                ui.horizontal(|ui| {
                    ui.add_sized(
                        egui::Vec2::from((100.0, 15.0)),
                        egui::TextEdit::singleline(&mut params.save_to),
                    );
                    ui.label(".json");
                });

                ui.separator();

                ui.horizontal(|ui| {
                    if ui.button("Load parameters").clicked() {
                        params.load();
                    }
                    ui.label("Load from file:");
                });
                ui.horizontal(|ui| {
                    ui.add_sized(
                        egui::Vec2::from((100.0, 15.0)),
                        egui::TextEdit::singleline(&mut params.load_from),
                    );
                    ui.label(".json");
                });
                /*ui.label("Learn more about this program at");
                ui.hyperlink("https://github.com/dyatelok/physarum");*/
            });
    }
}
