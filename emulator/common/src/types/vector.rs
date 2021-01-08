use std::ops::{ Add, Sub, Mul, Div };

#[derive(Copy, Clone, Debug)]
pub struct FloatVector {
    pub x: f32,
    pub y: f32,
}

impl FloatVector {

    pub const fn new() -> Self {
        return Self {
            x: 0.0,
            y: 0.0,
        };
    }

    pub const fn from(x: f32, y: f32) -> Self {
        return Self {
            x: x,
            y: y,
        };
    }

    pub const fn with(value: f32) -> Self {
        return Self {
            x: value,
            y: value,
        };
    }

    pub const fn with_x(x: f32) -> Self {
        return Self {
            x: x,
            y: 0.0,
        };
    }

    pub const fn with_y(y: f32) -> Self {
        return Self {
            x: 0.0,
            y: y,
        };
    }
}

impl Add for FloatVector {

    type Output = FloatVector;

    fn add(self, other: Self) -> Self {
        return Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl Sub for FloatVector {

    type Output = FloatVector;

    fn sub(self, other: Self) -> Self {
        return Self {
            x: self.x - other.x,
            y: self.y - other.y,
        };
    }
}

impl Mul for FloatVector {

    type Output = FloatVector;

    fn mul(self, other: Self) -> Self {
        return Self {
            x: self.x * other.x,
            y: self.y * other.y,
        };
    }
}

impl Div for FloatVector {

    type Output = FloatVector;

    fn div(self, other: Self) -> Self {
        return Self {
            x: self.x / other.x,
            y: self.y / other.y,
        };
    }
}
