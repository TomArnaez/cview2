use editor::application::Editor;

use std::sync::Mutex;

use tauri::{Manager, RunEvent, WindowEvent, Window};
use vello::kurbo::{Affine, BezPath, Cap, Ellipse, Join, PathEl, Point, Rect, Stroke};
use vello::peniko::*;
use vello::util::*;
use vello::*;

use specta::{ts, Type};
use serde::{Deserialize, Serialize};

mod commands;

pub static EDITOR: Mutex<Option<Editor>> = Mutex::new(None);

use vello::*;
use PathEl::*;


struct RenderState {
    surface: RenderSurface,
    renderer: Renderer,
    window: Window,
}

trait Drawable {
    fn draw(&self, sb: &mut SceneBuilder);
}

struct Rectangle {
    x: f64,
    y: f64,
    width: f64,
    height: f64
}

impl Drawable for Rectangle {
    fn draw(&self, sb: &mut SceneBuilder) {
        println!("draw rectangle");
        let rect = Rect::from_origin_size(Point::new(self.x, self.y), (self.width, self.height));


        sb.fill(
            Fill::NonZero,
            Affine::translate((100.0, 100.0)),
            Color::rgb8(255, 0, 0),
            None,
            &rect,
        );
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    println!("Starting frontend...");

    *(EDITOR.lock().unwrap()) = Some(Editor::new());

    let specta_builder = {
        use commands::*;

        let specta_builder = tauri_specta::ts::builder()
            .commands(tauri_specta::collect_commands![
                init_after_frontend_ready, 
                on_mouse_move, 
                on_wheel_scroll,
                activate_tool]);


        #[cfg(debug_assertions)] // <- Only export on non-release builds
        let specta_builder = specta_builder.path("../src/bindings.ts");

        specta_builder.into_plugin()
    };
    tauri::Builder::default()
    .plugin(specta_builder)
    .plugin(tauri_plugin_shell::init())
    .setup(|app| {
            let window = app.get_window("main").unwrap();
            let mut render_cx = RenderContext::new().unwrap();

            let size = window.inner_size()?;

            let surface_future = render_cx.create_surface(&window, size.width, size.height);
            let surface: RenderSurface = pollster::block_on(surface_future).expect("Error creating surface");

            let renderer = Renderer::new(&render_cx.devices[0].device,
            RendererOptions {
                surface_format: Some(surface.format),
                use_cpu: false,
                antialiasing_support: vello::AaSupport::all(),
            }).unwrap();


            let render_state = RenderState {
                window,
                renderer,
                surface
            };

            app.manage(Mutex::new(render_state));
            app.manage(Mutex::new(render_cx));

            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|app_handle, event| {
            match event {
            RunEvent::WindowEvent {
                label: _,
                event: WindowEvent::Resized(size),
                ..
            } => {
                let surface_mutex = app_handle.state::<Mutex<RenderSurface>>();
                let mut surface = surface_mutex.lock().unwrap();
                surface.config.width = size.width;
                surface.config.height = size.height;


            }
            RunEvent::MainEventsCleared => {
                /*
                let render_state_mutex = app_handle.state::<Mutex<RenderState>>();
                let mut render_state = render_state_mutex.lock().unwrap();

                let render_cx_mutex = app_handle.state::<Mutex<RenderContext>>();
                let render_cx = render_cx_mutex.lock().unwrap();

                let device_handle = &render_cx.devices[render_state.surface.dev_id];

                let mut scene = Scene::new();
                let mut builder = SceneBuilder::for_scene(&mut scene);

                let rectangle = Rectangle { x: 0., y: 0., width: 100., height:100. };

                rectangle.draw(&mut builder);

                let surface_texture = render_state.surface.surface.get_current_texture().expect("Failed to get surface texture");

                let render_params = RenderParams {
                    antialiasing_method: AaConfig::Area,
                    width: render_state.surface.config.width,
                    height: render_state.surface.config.height,
                    base_color: Color::BLUE_VIOLET
                };

                let scene_complexity = vello::block_on_wgpu(
                    &device_handle.device,
                    render_state.renderer
                        .render_to_surface_async(
                            &device_handle.device,
                            &device_handle.queue,
                            &scene,
                            &surface_texture,
                            &render_params,
                        ),
                )
                .expect("failed to render to surface");

                println!("Rendered scene!");

                surface_texture.present();
                device_handle.device.poll(wgpu::Maintain::Poll);
                */
            }
            _ => {}
        }}
        )
}
