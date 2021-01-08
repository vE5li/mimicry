use super::{ FloatVector, Color };

#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub position: FloatVector,
    pub color: Color,
}

impl Vertex {

    pub fn new(position: FloatVector, color: Color) -> Self {
        return Self {
            position: position,
            color: color,
        };
    }
}
