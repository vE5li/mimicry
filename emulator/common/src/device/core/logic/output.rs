use super::{ LogicState, Register, Gate };

#[derive(Clone, Debug)]
pub enum Output {
    Register(usize),
    Gate(usize, bool),
    Splitter(Box<Output>, Box<Output>),
}

impl Output {

    pub fn update(&self, registers: &mut Vec<Register>, gates: &mut Vec<Gate>, rising: bool, state: LogicState) {
        match self {

            Output::Register(index) => {
                match rising == registers[*index].rising {
                    true => registers[*index].input.update(LogicState::Metastable),
                    false => registers[*index].input.update(state),
                }
            },

            Output::Gate(index, right) => {

                match right {
                    true => gates[*index].right_input.update(state),
                    false => gates[*index].left_input.update(state),
                }

                let gate = gates[*index].clone();
                if gate.left_input.updated && gate.right_input.updated {
                    let state = gate.left_input.state.operator(gate.right_input.state, gate.operator);
                    gates[*index].state = state;
                    gate.output.update(registers, gates, rising, state);
                }
            },

            Output::Splitter(left_output, right_output) => {
                left_output.update(registers, gates, rising, state);
                right_output.update(registers, gates, rising, state);
            },
        }
    }
}
