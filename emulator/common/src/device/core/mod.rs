mod logic;
mod inspector;

use crate::types::*;
use self::logic::*;
use self::inspector::*;

const INTERFACE_BORDER: f32 = 10.0;

pub struct Core {
    cycle_count: usize,
    gates: Vec<Gate>,
    registers: Vec<Register>,
    constants: Vec<Constant>,
    inspector: Option<Inspector>,
}

impl Core {

    pub fn load(_filename: String, interface_size: FloatVector, debugging: bool) -> Self {

        let mut gates = Vec::new();
        let mut registers = Vec::new();
        let mut constants = Vec::new();
        let mut core_group = Vec::new();

        let mut logic_trackers = Vec::new();
        let mut value_trackers = Vec::new();

        // TEMP

        logic_trackers.push(LogicTracker::new(LabelSource::Gate(0, false)));
        logic_trackers.push(LogicTracker::new(LabelSource::Gate(1, false)));
        logic_trackers.push(LogicTracker::new(LabelSource::Gate(2, false)));
        logic_trackers.push(LogicTracker::new(LabelSource::Gate(0, true)));
        logic_trackers.push(LogicTracker::new(LabelSource::Gate(1, true)));
        logic_trackers.push(LogicTracker::new(LabelSource::Gate(2, true)));

        logic_trackers.push(LogicTracker::new(LabelSource::Register(6)));
        logic_trackers.push(LogicTracker::new(LabelSource::Register(7)));
        logic_trackers.push(LogicTracker::new(LabelSource::Register(8)));

        logic_trackers.push(LogicTracker::new(LabelSource::Register(0)));
        logic_trackers.push(LogicTracker::new(LabelSource::Register(1)));
        logic_trackers.push(LogicTracker::new(LabelSource::Register(2)));

        logic_trackers.push(LogicTracker::new(LabelSource::Register(3)));
        logic_trackers.push(LogicTracker::new(LabelSource::Register(4)));
        logic_trackers.push(LogicTracker::new(LabelSource::Register(5)));

        value_trackers.push(ValueTracker::new(vec![2, 1, 0]));
        value_trackers.push(ValueTracker::new(vec![5, 4, 3]));

        let mut input0_group = Vec::new();
        input0_group.push(InspectorItem::Label(Label::new("register 0", 0)));
        input0_group.push(InspectorItem::Label(Label::new("register 1", 1)));
        input0_group.push(InspectorItem::Label(Label::new("register 2", 2)));
        core_group.push(InspectorItem::Group(Group::new("input 0", Some((0, Formatting::Binary)), input0_group)));

        let mut input1_group = Vec::new();
        input1_group.push(InspectorItem::Label(Label::new("register 0", 3)));
        input1_group.push(InspectorItem::Label(Label::new("register 1", 4)));
        input1_group.push(InspectorItem::Label(Label::new("register 2", 5)));
        core_group.push(InspectorItem::Group(Group::new("input 1", Some((1, Formatting::Binary)), input1_group)));

        let mut gates_group = Vec::new();
        gates_group.push(InspectorItem::Label(Label::new("AND gate", 6)));
        gates_group.push(InspectorItem::Label(Label::new("XOR gate", 7)));
        gates_group.push(InspectorItem::Label(Label::new("XOR gate", 8)));
        core_group.push(InspectorItem::Group(Group::new("gates", None, gates_group)));

        let mut buffer_group = Vec::new();
        buffer_group.push(InspectorItem::Label(Label::new("register 0", 9)));
        buffer_group.push(InspectorItem::Label(Label::new( "register 1", 1)));
        buffer_group.push(InspectorItem::Label(Label::new( "register 2", 1)));
        core_group.push(InspectorItem::Group(Group::new("buffer", None, buffer_group)));

        let mut gates_group = Vec::new();
        gates_group.push(InspectorItem::Label(Label::new( "output 0", 1)));
        gates_group.push(InspectorItem::Label(Label::new( "output 1", 1)));
        gates_group.push(InspectorItem::Label(Label::new( "output 2", 1)));
        core_group.push(InspectorItem::Group(Group::new("inverter", None, gates_group)));

        let root_item = InspectorItem::Group(Group::new("core", None, core_group));

        registers.push(Register::new(Input::new(LogicState::High), Output::Gate(0, false), true));
        registers.push(Register::new(Input::new(LogicState::Low), Output::Gate(1, false), true));
        registers.push(Register::new(Input::new(LogicState::High), Output::Gate(2, false), true));

        registers.push(Register::new(Input::new(LogicState::High), Output::Gate(0, true), true));
        registers.push(Register::new(Input::new(LogicState::High), Output::Gate(1, true), true));
        registers.push(Register::new(Input::new(LogicState::High), Output::Gate(2, true), true));

        gates.push(Gate::new(Operator::And, Output::Register(6)));
        gates.push(Gate::new(Operator::Xor, Output::Register(7)));
        gates.push(Gate::new(Operator::Xor, Output::Register(8)));

        let splitter0 = Output::Splitter(Box::new(Output::Register(0)), Box::new(Output::Gate(3, false)));
        let splitter1 = Output::Splitter(Box::new(Output::Register(1)), Box::new(Output::Gate(4, false)));
        let splitter2 = Output::Splitter(Box::new(Output::Register(2)), Box::new(Output::Gate(5, false)));

        registers.push(Register::new(Input::new(LogicState::Floating), splitter0, false));
        registers.push(Register::new(Input::new(LogicState::Floating), splitter1, false));
        registers.push(Register::new(Input::new(LogicState::Floating), splitter2, false));

        constants.push(Constant::new(LogicState::High, Output::Gate(3, true)));
        constants.push(Constant::new(LogicState::High, Output::Gate(4, true)));
        constants.push(Constant::new(LogicState::High, Output::Gate(5, true)));

        gates.push(Gate::new(Operator::Or, Output::Register(3)));
        gates.push(Gate::new(Operator::Or, Output::Register(4)));
        gates.push(Gate::new(Operator::Or, Output::Register(5)));

        // TEMP

        let inspector = debugging.then(|| Inspector::new(interface_size, logic_trackers, value_trackers, root_item));

        return Self {
            cycle_count: 0,
            gates: gates,
            registers: registers,
            constants: constants,
            inspector: inspector,
        };
    }

    pub fn handle_key_input(&mut self, key: Key) {
        if let Some(inspector) = &mut self.inspector {
            inspector.handle_key_input(key);
        }
    }

    pub fn resize(&mut self, interface_size: FloatVector) {
        if let Some(inspector) = &mut self.inspector {
            inspector.resize(interface_size - FloatVector::with(INTERFACE_BORDER * 2.0));
        }
    }

    pub fn tick(&mut self, rising: bool) {

        self.registers.iter_mut().for_each(|register| register.reset());
        self.gates.iter_mut().for_each(|gate| gate.reset());

        for index in 0..self.constants.len() {
            self.constants[index].update(&mut self.registers, &mut self.gates, rising);
        }

        for index in 0..self.registers.len() {
            if self.registers[index].rising == rising {
                let register = self.registers[index].clone();
                register.update(&mut self.registers, &mut self.gates, rising);
            }
        }

        if let Some(inspector) = &mut self.inspector {
            inspector.update(&self.registers, &self.gates);
        }

        if rising {
            self.cycle_count += 1;
        }
    }

    pub fn draw<T: Renderer>(&self, renderer: &mut T, position: FloatVector) {
        if let Some(inspector) = &self.inspector {
            inspector.draw(renderer, position + FloatVector::with(INTERFACE_BORDER));
        }
    }
}
