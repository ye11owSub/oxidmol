use crate::renderer::{State, SurfaceSource};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Viewer {
    state: State<'static>,
}

#[wasm_bindgen]
impl Viewer {
    pub async fn create(canvas: web_sys::HtmlCanvasElement, width: u32, height: u32) -> Viewer {
        console_error_panic_hook::set_once();
        let state = State::new(SurfaceSource::Canvas(canvas), width, height).await;
        Viewer { state }
    }

    pub fn load_atoms(&mut self, bytes: &[u8]) {
        self.state.upload_atom_bytes(bytes).unwrap();
    }

    pub fn render(&self) {
        self.state.render().unwrap();
    }

    pub fn resize(&mut self, w: u32, h: u32) {
        self.state.resize(w, h);
    }

    pub fn orbit(&mut self, dx: f32, dy: f32) {
        self.state.camera.orbit(dx, dy);
    }

    pub fn zoom(&mut self, delta: f32) {
        self.state.camera.zoom(delta);
    }
}
