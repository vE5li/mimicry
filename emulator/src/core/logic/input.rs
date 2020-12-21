use super::LogicState;

#[derive(Copy, Clone, Debug)]
pub struct Input {
    pub state: LogicState,
    pub updated: bool,
}

impl Input {

    pub fn new(state: LogicState) -> Self {
        return Self {
            state: state,
            updated: false,
        };
    }

    pub fn update(&mut self, state: LogicState) {
        // ensure self.updated == false
        self.state = state;
        self.updated = true;
    }

    pub fn reset(&mut self) {
        self.updated = false;
    }
}
