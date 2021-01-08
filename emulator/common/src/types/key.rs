#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Key {
    T,
    Space,
}

impl Key {

    pub fn from_code(code: usize) -> Option<Self> {
        match code {
            32 => return Some(Key::Space),
            84 => return Some(Key::T),
            _other => return None,
        }
    }
}
