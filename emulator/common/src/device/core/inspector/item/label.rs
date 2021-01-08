#[derive(Clone, Debug)]
pub struct Label {
    pub identifier: &'static str,
    pub tracker: usize,
}

impl Label {

    pub fn new(identifier: &'static str, tracker: usize) -> Self {
        return Self {
            identifier: identifier,
            tracker: tracker,
        }
    }
}
