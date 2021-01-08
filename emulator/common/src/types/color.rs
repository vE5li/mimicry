#[derive(Copy, Clone, Debug)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Color {

    pub const fn from(red: u8, green: u8, blue: u8) -> Self {
        return Self {
            red: red,
            green: green,
            blue: blue,
        };
    }

    pub const fn monochrome(brightness: u8) -> Self {
        return Self {
            red: brightness,
            green: brightness,
            blue: brightness,
        };
    }
}
