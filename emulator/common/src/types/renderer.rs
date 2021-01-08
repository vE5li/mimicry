use super::*;

pub trait Renderer {

    fn clear(&mut self, color: Color);

    fn display(&mut self);

    fn draw_rectangle(&mut self, position: FloatVector, size: FloatVector, color: Color);

    fn draw_text(&mut self, text: &str, position: FloatVector, color: Color, size: u32);

    fn draw_text_right(&mut self, text: &str, position: FloatVector, color: Color, size: u32);

    fn draw_line_segment(&mut self, position: FloatVector, vertices: &Vec<Vertex>);
}
