use std::time::Instant;

use glow::*;
use glutin::event::{Event, WindowEvent};
use glutin::event_loop::ControlFlow;

use imgui::im_str;
use imgui_glow_renderer::Renderer;
use imgui_winit_support::{HiDpiMode, WinitPlatform};

pub(crate) fn run<F: 'static>(mut main_loop: F)
where
    F: FnMut(&imgui::Ui) -> (),
{
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new().with_title("A fantastic window!");

    let windowed_context = glutin::ContextBuilder::new()
        .build_windowed(wb, &event_loop)
        .unwrap();
    let windowed_context = unsafe { windowed_context.make_current().unwrap() };

    // Create glow Context
    let gl = unsafe {
        let context = glow::Context::from_loader_function(|s| {
            windowed_context.get_proc_address(s) as *const _
        });
        context
    };

    // Used to update imgui's timer in main loop
    let mut last_frame = Instant::now();

    // Create and configure imgui context
    let mut imgui = imgui::Context::create();

    // Disable imgui's automatic saving of settings
    imgui.set_ini_filename(None);

    // Create platform handler
    let mut platform = WinitPlatform::init(&mut imgui);
    platform.attach_window(
        imgui.io_mut(),
        windowed_context.window(),
        HiDpiMode::Default,
    );

    // Configure font (using built-in fixed size bitmap font)
    let scale_factor = windowed_context.window().scale_factor() as f32;

    let font_size = 13.0 * scale_factor;
    imgui
        .fonts()
        .add_font(&[imgui::FontSource::DefaultFontData {
            config: Some(imgui::FontConfig {
                size_pixels: font_size,
                ..imgui::FontConfig::default()
            }),
        }]);

    imgui.io_mut().font_global_scale = (1.0 / scale_factor) as f32;

    // Tweak imgui style
    let mut style = imgui.style_mut().clone();
    style.window_rounding = 0.0;
    style.window_border_size = 0.0;
    style.colors[imgui::StyleColor::TitleBg as usize] = [1.0, 1.0, 1.0, 1.0];

    // Create glow renderer
    let imgui_renderer = Renderer::new(&gl, &mut imgui);

    // Main event loop
    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                // Clean up on window close
                imgui_renderer.cleanup(&gl);

                *control_flow = ControlFlow::Exit;
            }

            Event::NewEvents(_) => {}

            Event::MainEventsCleared => {
                platform
                    .prepare_frame(imgui.io_mut(), &windowed_context.window())
                    .expect("Failed to prepare frame");
                windowed_context.window().request_redraw();
            }

            Event::RedrawRequested(_) => {
                // Draw application window

                // Blank BG
                unsafe {
                    gl.clear(glow::COLOR_BUFFER_BIT);
                }

                // Update imgui's timer
                let delta = Instant::now().duration_since(last_frame);
                imgui.io_mut().update_delta_time(delta);
                last_frame = Instant::now();

                // Pass events to imgui platform handler
                platform.handle_event(imgui.io_mut(), &windowed_context.window(), &event);

                // Create the imgui::Ui context
                let ui = imgui.frame();
                main_loop(&ui);
                // End demo app contents

                // Get draw list for rendering
                platform.prepare_render(&ui, &windowed_context.window());
                let draw_data = ui.render();

                // Render it
                imgui_renderer.render(&gl, &draw_data);
                windowed_context.swap_buffers().unwrap();
            }

            event => {
                // Pass to imgui platform by default
                platform.handle_event(imgui.io_mut(), &windowed_context.window(), &event);
            }
        }
    });
}
