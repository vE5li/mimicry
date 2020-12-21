mod logic;
mod inspector;

use self::logic::*;
use self::inspector::*;
use sfml::graphics::{ RenderTarget, RenderWindow, Color, Font, Transformable, Shape };
use sfml::system::Vector2f;

const INTERFACE_BORDER: f32 = 5.0;

pub struct Core<'a> {
    cycle_count: usize,
    gates: Vec<Gate>,
    registers: Vec<Register>,
    constants: Vec<Constant>,
    inspector: Option<Inspector<'a>>,
}

impl<'a> Core<'a> {

    pub fn load(_filename: String, font: &'a Font, interface_size: Vector2f, debugging: bool) -> Self {

        let mut gates = Vec::new();
        let mut registers = Vec::new();
        let mut constants = Vec::new();
        let mut core_group = Vec::new();

        let mut trackers = Vec::new();
        let mut value_trackers = Vec::new();

        // TEMP

        trackers.push(Tracker::new(LabelSource::Gate(0, false)));
        trackers.push(Tracker::new(LabelSource::Gate(1, false)));
        trackers.push(Tracker::new(LabelSource::Gate(2, false)));
        trackers.push(Tracker::new(LabelSource::Gate(0, true)));
        trackers.push(Tracker::new(LabelSource::Gate(1, true)));
        trackers.push(Tracker::new(LabelSource::Gate(2, true)));

        trackers.push(Tracker::new(LabelSource::Register(6)));
        trackers.push(Tracker::new(LabelSource::Register(7)));
        trackers.push(Tracker::new(LabelSource::Register(8)));

        trackers.push(Tracker::new(LabelSource::Register(0)));
        trackers.push(Tracker::new(LabelSource::Register(1)));
        trackers.push(Tracker::new(LabelSource::Register(2)));

        trackers.push(Tracker::new(LabelSource::Register(3)));
        trackers.push(Tracker::new(LabelSource::Register(4)));
        trackers.push(Tracker::new(LabelSource::Register(5)));

        value_trackers.push(ValueTracker::new(vec![2, 1, 0]));
        value_trackers.push(ValueTracker::new(vec![5, 4, 3]));

        let mut input0_group = Vec::new();
        input0_group.push(InspectorItem::Label(0, "register 0"));
        input0_group.push(InspectorItem::Label(1, "register 1"));
        input0_group.push(InspectorItem::Label(2, "register 2"));
        core_group.push(InspectorItem::Group(input0_group, Some(0), "input 0", true));

        let mut input1_group = Vec::new();
        input1_group.push(InspectorItem::Label(3, "register 0"));
        input1_group.push(InspectorItem::Label(4, "register 1"));
        input1_group.push(InspectorItem::Label(5, "register 2"));
        core_group.push(InspectorItem::Group(input1_group, Some(1), "input 1", true));

        let mut gates_group = Vec::new();
        gates_group.push(InspectorItem::Label(6, "AND gate"));
        gates_group.push(InspectorItem::Label(7, "XOR gate"));
        gates_group.push(InspectorItem::Label(8, "XOR gate"));
        core_group.push(InspectorItem::Group(gates_group, None, "gates", true));

        let mut buffer_group = Vec::new();
        buffer_group.push(InspectorItem::Label(9, "register 0"));
        buffer_group.push(InspectorItem::Label(10, "register 1"));
        buffer_group.push(InspectorItem::Label(11, "register 2"));
        core_group.push(InspectorItem::Group(buffer_group, None, "buffer", true));

        let mut gates_group = Vec::new();
        gates_group.push(InspectorItem::Label(12, "output 0"));
        gates_group.push(InspectorItem::Label(13, "output 1"));
        gates_group.push(InspectorItem::Label(14, "output 2"));
        core_group.push(InspectorItem::Group(gates_group, None, "inverter", true));

        let root_item = InspectorItem::Group(core_group, None, "core", true);

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

        let inspector = match debugging {
            true => Some(Inspector::new(font, interface_size, trackers, value_trackers, root_item)),
            false => None,
        };

        return Self {
            cycle_count: 0,
            gates: gates,
            registers: registers,
            constants: constants,
            inspector: inspector,
        };
    }

    pub fn update_size(&mut self, interface_size: Vector2f) {
        if let Some(inspector) = &mut self.inspector {
            inspector.update_size(interface_size - (INTERFACE_BORDER * 2.0));
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

    pub fn draw(&mut self, window: &mut RenderWindow) {
        if let Some(inspector) = &mut self.inspector {
            let position = Vector2f::new(INTERFACE_BORDER, INTERFACE_BORDER);
            inspector.draw(window, position);
        }
    }
}
