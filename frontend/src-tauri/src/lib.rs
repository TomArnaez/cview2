use std::sync::Mutex;

use tauri::{Manager, RunEvent, WindowEvent, Window};
use vello::kurbo::{Affine, BezPath, Cap, Ellipse, Join, PathEl, Point, Rect, Stroke};
use vello::peniko::*;
use vello::util::*;
use vello::*;

use specta::{ts, Type};
use serde::{Deserialize, Serialize};

use vello::*;struct RenderState {
    surface: RenderSurface,
    window: Window,
}

#[derive(Serialize, Type)]
enum ToolMessage {
    Line,
    Brush
}
#[derive(Serialize, Type)]
enum DebugMessage {
    Info,
    Error
}

#[derive(Serialize, Type)]
enum Message {
    ToolMessage(ToolMessage),
    DebugMessage(DebugMessage),
}

fn funky_paths(sb: &mut SceneBuilder) {
    use PathEl::*;
    let missing_movetos = [
        MoveTo((0., 0.).into()),
        LineTo((100.0, 100.0).into()),
        LineTo((100.0, 200.0).into()),
        ClosePath,
        LineTo((0.0, 400.0).into()),
        LineTo((100.0, 400.0).into()),
    ];
    let only_movetos = [MoveTo((0.0, 0.0).into()), MoveTo((100.0, 100.0).into())];
    let empty: [PathEl; 0] = [];
    sb.fill(
        Fill::NonZero,
        Affine::translate((100.0, 100.0)),
        Color::rgb8(0, 0, 255),
        None,
        &missing_movetos,
    );
    sb.fill(
        Fill::NonZero,
        Affine::IDENTITY,
        Color::rgb8(0, 0, 255),
        None,
        &empty,
    );
    sb.fill(
        Fill::NonZero,
        Affine::IDENTITY,
        Color::rgb8(0, 0, 255),
        None,
        &only_movetos,
    );
    sb.stroke(
        &Stroke::new(8.0),
        Affine::translate((100.0, 100.0)),
        Color::rgb8(0, 255, 255),
        None,
        &missing_movetos,
    );
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    println!("{:?}", ts::export::<Message>(&Default::default()).unwrap());

    tauri::Builder::default()
    .plugin(tauri_plugin_shell::init())
    .setup(|app| {
            let window = app.get_window("main").unwrap();
            let mut render_cx = RenderContext::new().unwrap();

            let size = window.inner_size()?;

            let surface_future = render_cx.create_surface(&window, size.width, size.height);
            let surface: RenderSurface = pollster::block_on(surface_future).expect("Error creating surface");

            let render_state = RenderState {
                window,
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
                let render_state_mutex = app_handle.state::<Mutex<RenderState>>();
                let render_state = render_state_mutex.lock().unwrap();

                let render_cx_mutex = app_handle.state::<Mutex<RenderContext>>();
                let render_cx = render_cx_mutex.lock().unwrap();

                let device_handle = &render_cx.devices[render_state.surface.dev_id];

                let mut scene = Scene::new();
                let mut builder = SceneBuilder::for_scene(&mut scene);

                let mut renderer = Renderer::new(&render_cx.devices[0].device,
                RendererOptions {
                    surface_format: Some(render_state.surface.format),
                    use_cpu: false,
                    antialiasing_support: vello::AaSupport::all(),
                }).unwrap();

                funky_paths(&mut builder);

                let surface_texture = render_state.surface.surface.get_current_texture().expect("Failed to get surface texture");

                let render_params = RenderParams {
                    antialiasing_method: AaConfig::Area,
                    width: render_state.surface.config.width,
                    height: render_state.surface.config.height,
                    base_color: Color::BLUE_VIOLET
                };

                let scene_complexity = vello::block_on_wgpu(
                    &device_handle.device,
                    renderer
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
            }
            _ => {}
        }}
        )
}
