use super::{ LogicState, Input, Output, Gate };

#[derive(Clone, Debug)]
pub struct Register {
    pub input: Input,
    pub state: LogicState,
    pub output: Output,
    pub rising: bool,
}

impl Register {

    pub fn new(input: Input, output: Output, rising: bool) -> Self {
        return Self {
            input: input,
            state: input.state,
            output: output,
            rising: rising,
        };
    }

    pub fn update(&self, registers: &mut Vec<Register>, gates: &mut Vec<Gate>, rising: bool) {
        self.output.update(registers, gates, rising, self.state);
    }

    pub fn reset(&mut self) {
        self.state = self.input.state;
        self.input.reset();
    }
}
