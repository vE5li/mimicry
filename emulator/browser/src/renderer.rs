use common::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{ WebGlRenderingContext, WebGlProgram, WebGlShader, HtmlCanvasElement };
use std::convert::TryInto;
use webgl_font::*;

const CHARACTER_SIZE_SCALAR: f32 = 0.9;
const CHARACTER_PADDING: f32 = 0.25;
const CHARACTER_STEP: f32 = 0.82;

pub const DEFAULT_VERTEX_SHADER: &'static str = r#"
attribute vec2 vertex_position;
uniform vec2 screen_position;
uniform vec2 screen_size;
uniform vec2 screen_divident;
void main() {
    vec2 offset = vertex_position * screen_size;
    vec4 final_position = vec4((offset + screen_position) / screen_divident, 0.0, 1.0);
    final_position.x = final_position.x - 1.0;
    final_position.y = 1.0 - final_position.y;
    gl_Position = final_position;
}
"#;

pub const DEFAULT_FRAGMENT_SHADER: &'static str = r#"
precision mediump float;
uniform vec3 color;
void main() {
    gl_FragColor = vec4(color, 1.0);
}
"#;

pub const LINE_VERTEX_SHADER: &'static str = r#"
attribute vec2 vertex_position;
attribute vec3 vertex_color;
varying vec3 fragment_color;
uniform vec2 screen_position;
uniform vec2 screen_divident;
void main() {
    vec4 final_position = vec4((vertex_position + screen_position) / screen_divident, 0.0, 1.0);
    final_position.x = final_position.x - 1.0;
    final_position.y = 1.0 - final_position.y;
    gl_Position = final_position;
    fragment_color = vertex_color;
}
"#;

pub const LINE_FRAGMENT_SHADER: &'static str = r#"
precision mediump float;
varying vec3 fragment_color;
void main() {
    gl_FragColor = vec4(fragment_color, 1.0);
}
"#;

pub fn compile_shader(context: &WebGlRenderingContext, shader_type: u32, source: &str) -> Result<WebGlShader, String> {
    let shader = context.create_shader(shader_type).ok_or_else(|| String::from("Unable to create shader object"))?;
    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    match context.get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS).as_bool().unwrap_or(false) {
        true => return Ok(shader),
        false => return Err(context.get_shader_info_log(&shader).unwrap_or_else(|| String::from("Unknown error creating shader"))),
    }
}

pub fn link_program(context: &WebGlRenderingContext, vert_shader: &WebGlShader, frag_shader: &WebGlShader) -> Result<WebGlProgram, String> {
    let program = context.create_program().ok_or_else(|| String::from("Unable to create shader object"))?;
    context.attach_shader(&program, vert_shader);
    context.attach_shader(&program, frag_shader);
    context.link_program(&program);

    match context.get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS).as_bool().unwrap_or(false) {
        true => return Ok(program),
        false => return Err(context.get_program_info_log(&program).unwrap_or_else(|| String::from("Unknown error creating program object"))),
    }
}

fn color_as_floats(color: Color) -> (f32, f32, f32) {
    return (color.red as f32 / 255.0, color.green as f32 / 255.0, color.blue as f32 / 255.0);
}

macro_rules! buffer_data_with_array_buffer_view {
    ($context:expr, $array_type:ident, $buffer_type:ident, $source_array:expr) => (
        unsafe {
            let array = js_sys::$array_type::view($source_array);
            $context.buffer_data_with_array_buffer_view(WebGlRenderingContext::$buffer_type, &array, WebGlRenderingContext::STATIC_DRAW);
        }
    );
}

pub struct WebGLRenderer {
    context: WebGlRenderingContext,
    default_program: WebGlProgram,
    line_program: WebGlProgram,
    interface_size: FloatVector,
    canvas: HtmlCanvasElement,
}

impl WebGLRenderer {

    pub fn new(interface_size: FloatVector) -> Result<Self, JsValue> {

        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id("canvas").unwrap();
        let canvas = canvas.dyn_into::<HtmlCanvasElement>()?;

        canvas.set_width(interface_size.x as u32);
        canvas.set_height(interface_size.y as u32);

        let context = canvas.get_context("webgl")?.unwrap().dyn_into::<WebGlRenderingContext>()?;

        let default_vertex_shader = compile_shader(&context, WebGlRenderingContext::VERTEX_SHADER, DEFAULT_VERTEX_SHADER)?;
        let default_fragment_shader = compile_shader(&context, WebGlRenderingContext::FRAGMENT_SHADER, DEFAULT_FRAGMENT_SHADER)?;
        let default_program = link_program(&context, &default_vertex_shader, &default_fragment_shader)?;

        let line_vertex_shader = compile_shader(&context, WebGlRenderingContext::VERTEX_SHADER, LINE_VERTEX_SHADER)?;
        let line_fragment_shader = compile_shader(&context, WebGlRenderingContext::FRAGMENT_SHADER, LINE_FRAGMENT_SHADER)?;
        let line_program = link_program(&context, &line_vertex_shader, &line_fragment_shader)?;

        return Ok(Self {
            context: context,
            default_program: default_program,
            line_program: line_program,
            interface_size: interface_size,
            canvas: canvas,
        });
    }

    pub fn resize(&mut self, interface_size: FloatVector) {
        self.canvas.set_width(interface_size.x as u32);
        self.canvas.set_height(interface_size.y as u32);
        self.interface_size = interface_size;
        self.context.viewport(0, 0, interface_size.x as i32, interface_size.y as i32);
    }

    fn draw_default(&mut self, position: FloatVector, size: FloatVector, color: Color, vertices: &[f32], indices: &[u16]) {
        self.context.use_program(Some(&self.default_program));

        let screen_divident = self.interface_size / FloatVector::with(2.0);
        let (red, green, blue) = color_as_floats(color);

        let location = self.context.get_uniform_location(&self.default_program, "screen_divident").unwrap();
        self.context.uniform2f(Some(&location), screen_divident.x, screen_divident.y);
        let location = self.context.get_uniform_location(&self.default_program, "screen_position").unwrap();
        self.context.uniform2f(Some(&location), position.x, position.y);
        let location = self.context.get_uniform_location(&self.default_program, "screen_size").unwrap();
        self.context.uniform2f(Some(&location), size.x, size.y);
        let location = self.context.get_uniform_location(&self.default_program, "color").unwrap();
        self.context.uniform3f(Some(&location), red, green, blue);

        let vertex_buffer = self.context.create_buffer().ok_or("failed to create buffer").unwrap();
        self.context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&vertex_buffer));
        buffer_data_with_array_buffer_view!(&self.context, Float32Array, ARRAY_BUFFER, &vertices);

        let index_buffer = self.context.create_buffer().ok_or("failed to create buffer").unwrap();
        self.context.bind_buffer(WebGlRenderingContext::ELEMENT_ARRAY_BUFFER, Some(&index_buffer));
        buffer_data_with_array_buffer_view!(&self.context, Uint16Array, ELEMENT_ARRAY_BUFFER, &indices);

        let position_location = self.context.get_attrib_location(&self.default_program, "vertex_position").try_into().unwrap();
        self.context.vertex_attrib_pointer_with_i32(position_location, 2, WebGlRenderingContext::FLOAT, false, 0, 0);
        self.context.enable_vertex_attrib_array(position_location);

        self.context.draw_elements_with_i32(WebGlRenderingContext::TRIANGLES, indices.len().try_into().unwrap(), WebGlRenderingContext::UNSIGNED_SHORT, 0);
    }

    fn draw_array(&mut self, position: FloatVector, vertex_data: &[f32], color_data: &[f32], vertex_count: usize) {
        self.context.use_program(Some(&self.line_program));

        let screen_divident = self.interface_size / FloatVector::with(2.0);

        let location = self.context.get_uniform_location(&self.line_program, "screen_divident").unwrap();
        self.context.uniform2f(Some(&location), screen_divident.x, screen_divident.y);
        let location = self.context.get_uniform_location(&self.line_program, "screen_position").unwrap();
        self.context.uniform2f(Some(&location), position.x, position.y);

        let vertex_buffer = self.context.create_buffer().ok_or("failed to create buffer").unwrap();
        self.context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&vertex_buffer));
        buffer_data_with_array_buffer_view!(&self.context, Float32Array, ARRAY_BUFFER, &vertex_data);

        let position_location = self.context.get_attrib_location(&self.line_program, "vertex_position").try_into().unwrap();
        self.context.vertex_attrib_pointer_with_i32(position_location, 2, WebGlRenderingContext::FLOAT, false, 0, 0);
        self.context.enable_vertex_attrib_array(position_location);

        let color_buffer = self.context.create_buffer().ok_or("failed to create buffer").unwrap();
        self.context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&color_buffer));
        buffer_data_with_array_buffer_view!(&self.context, Float32Array, ARRAY_BUFFER, &color_data);

        let color_location = self.context.get_attrib_location(&self.line_program, "vertex_color").try_into().unwrap();
        self.context.vertex_attrib_pointer_with_i32(color_location, 3, WebGlRenderingContext::FLOAT, false, 0, 0);
        self.context.enable_vertex_attrib_array(color_location);

        self.context.draw_arrays(WebGlRenderingContext::LINE_STRIP, 0, vertex_count as i32);
    }
}

impl Renderer for WebGLRenderer {

    fn clear(&mut self, color: Color) {
        let (red, green, blue) = color_as_floats(color);
        self.context.clear_color(red, green, blue, 1.0);
        self.context.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);
    }

    fn display(&mut self) {
        self.context.flush();
        self.context.finish();
    }

    fn draw_rectangle(&mut self, position: FloatVector, size: FloatVector, color: Color) {
        let vertices = [0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 1.0, 1.0];
        let indices = [0, 1, 2, 0, 2, 3];
        self.draw_default(position, size, color, &vertices[..], &indices[..]);
    }

    fn draw_text(&mut self, text: &str, position: FloatVector, color: Color, size: u32) {
        let mut offset = 0.0;
        let size = size as f32 * CHARACTER_SIZE_SCALAR;
        let padding = CHARACTER_PADDING * size;
        let character_size = FloatVector::with(size);

        for code in text.chars() {
            if let Some((vertex_slice, index_slice)) = get_character_geometry(code as usize) {
                let character_position = position + FloatVector::from(offset, padding);
                self.draw_default(character_position, character_size, color, vertex_slice, index_slice);
            }
            offset += size * CHARACTER_STEP;
        }
    }

    fn draw_text_right(&mut self, text: &str, position: FloatVector, color: Color, size: u32) {
        let offset_position = position - FloatVector::with_x(((text.len() * size as usize) as f32) * CHARACTER_STEP);
        self.draw_text(text, offset_position, color, size);
    }

    fn draw_line_segment(&mut self, position: FloatVector, vertices: &Vec<Vertex>) {
        let mut vertex_data = Vec::new();
        let mut color_data = Vec::new();

        for vertex in vertices {
            vertex_data.push(vertex.position.x);
            vertex_data.push(vertex.position.y);

            let (red, green, blue) = color_as_floats(vertex.color);
            color_data.push(red);
            color_data.push(green);
            color_data.push(blue);
        }

        self.draw_array(position, &vertex_data[..], &color_data[..], vertices.len());
    }
}
