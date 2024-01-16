use vello::{util::RenderSurface, Renderer};
use raw_window_handle::HasWindowHandle;

struct RenderState {
    surface: RenderSurface,
}

fn setup(window: &dyn HasWindowHandle ) {
    let mut render_state = None::<RenderState>;
    let mut renderers: Vec<Option<Renderer>> = vec![];
}