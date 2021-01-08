use super::{ LogicState, Output, Register, Gate };

pub struct Constant {
    output: Output,
    state: LogicState,
}

impl Constant {

    pub fn new(state: LogicState, output: Output) -> Self {
        return Self {
            state: state,
            output: output,
        };
    }

    pub fn update(&self, registers: &mut Vec<Register>, gates: &mut Vec<Gate>, rising: bool) {
        self.output.update(registers, gates, rising, self.state);
    }
}
