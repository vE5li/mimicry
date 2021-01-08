use super::{ LogicState, Operator, Input, Output };

#[derive(Clone, Debug)]
pub struct Gate {
    pub operator: Operator,
    pub left_input: Input,
    pub right_input: Input,
    pub state: LogicState,
    pub output: Output,
}

impl Gate {

    pub fn new(operator: Operator, output: Output) -> Self {
        return Self {
            operator: operator,
            left_input: Input::new(LogicState::Floating),
            right_input: Input::new(LogicState::Floating),
            state: LogicState::Floating,
            output: output,
        };
    }

    pub fn reset(&mut self) {
        self.left_input.reset();
        self.right_input.reset();
    }
}
