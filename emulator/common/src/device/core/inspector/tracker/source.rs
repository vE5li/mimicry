use crate::device::core::{ LogicState, Register, Gate };

#[derive(Copy, Clone, Debug)]
pub enum LabelSource {
    Register(usize),
    Gate(usize, bool),
}

impl LabelSource {

    pub fn get_state(&self, registers: &Vec<Register>, gates: &Vec<Gate>) -> LogicState {
        match self {

            LabelSource::Register(index) => return registers[*index].input.state,

            LabelSource::Gate(index, right) => {
                match right {
                    true => return gates[*index].right_input.state,
                    false => return gates[*index].left_input.state,
                }

            },
        }
    }
}
