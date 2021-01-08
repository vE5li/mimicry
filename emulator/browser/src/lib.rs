extern crate webgl_font;
extern crate wasm_bindgen;
extern crate web_sys;
extern crate common;

mod renderer;

use common::*;
use renderer::WebGLRenderer;
use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(js_name = Device)]
pub struct DeviceWrapper {
    device: Device,
    renderer: WebGLRenderer,
}

#[wasm_bindgen(js_class = Device)]
impl DeviceWrapper {

    pub fn new() -> Result<DeviceWrapper, JsValue> {

        #[cfg(feature = "console_error_panic_hook")]
        console_error_panic_hook::set_once();

        let width = web_sys::window().unwrap().inner_width().unwrap().as_f64().unwrap() as f32;
        let height = web_sys::window().unwrap().inner_height().unwrap().as_f64().unwrap() as f32;
        let interface_size = FloatVector::from(width, height);

        let renderer = WebGLRenderer::new(interface_size)?;
        let device = Device::new(interface_size - FloatVector::with(20.0)); // why is this - 20.0 needed?

        return Ok(DeviceWrapper{
            device: device,
            renderer: renderer,
        });
    }

    pub fn render(&mut self) {
        self.device.render(&mut self.renderer);
    }

    pub fn resize(&mut self, width: i32, height: i32) {
        let interface_size = FloatVector::from(width as f32, height as f32);
        self.device.resize(interface_size);
        self.renderer.resize(interface_size);
        self.device.render(&mut self.renderer);
    }

    pub fn handle_key_input(&mut self, code: i32) {
        if let Some(key) = Key::from_code(code as usize) {
            self.device.handle_key_input(key);
            self.device.render(&mut self.renderer);
        }
    }
}
