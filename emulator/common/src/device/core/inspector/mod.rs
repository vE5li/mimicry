mod tracker;
mod item;

pub use self::tracker::*;
pub use self::item::*;

use super::{ LogicState, ValueState, Register, Gate };
use crate::types::*;

const LABEL_HEIGHT: f32 = 20.0;
const TRACKER_STEP: f32 = 20.0;
const LEVEL_STEP: f32 = 20.0;
const TRACKER_OFFSET: f32 = 7.0;
const TRACKER_MARGIN: f32 = 3.0;
const ITEM_GAP: f32 = 2.0;
const STATE_OFFSET: f32 = 10.0;
const TEXT_SIZE: u32 = 12;
const IDENTIFIER_OFFSET: f32 = 10.0;

const TEXT_PADDING: f32 = 1.0;

const LABEL_COLOR: Color = Color::from(55, 55, 55);
const TRACKER_COLOR: Color = Color::from(40, 40, 40);
const OVERLAY_COLOR: Color = Color::from(45, 45, 45);
const TEXT_COLOR: Color = Color::from(160, 160, 160);
const STABLE_COLOR: Color = Color::from(100, 150, 100);
const METASTABLE_COLOR: Color = Color::from(250, 100, 100);
const FLOATING_COLOR: Color = Color::from(150, 100, 100);
const VALUE_COLOR: Color = Color::from(100, 100, 150);

pub struct Inspector {
    interface_size: FloatVector,
    label_width: f32,
    show_trackers: bool,
    root_item: InspectorItem,
    logic_trackers: Vec<LogicTracker>,
    value_trackers: Vec<ValueTracker>,
    step_size: f32,
}

impl Inspector {

    pub fn new(interface_size: FloatVector, logic_trackers: Vec<LogicTracker>, value_trackers: Vec<ValueTracker>, root_item: InspectorItem) -> Self {

        // TODO: calculate from total space
        let label_width = 400.0;

        return Self {
            interface_size: interface_size,
            label_width: label_width,
            show_trackers: true,
            root_item: root_item,
            logic_trackers: logic_trackers,
            value_trackers: value_trackers,
            step_size: TRACKER_STEP,
        };
    }

    pub fn handle_key_input(&mut self, key: Key) {
        if key == Key::T {
            self.show_trackers = !self.show_trackers;
        }
    }

    pub fn resize(&mut self, interface_size: FloatVector) {
        self.interface_size = interface_size;
    }

    pub fn update(&mut self, registers: &Vec<Register>, gates: &Vec<Gate>) {

        for index in 0..self.logic_trackers.len() {
            let source = self.logic_trackers[index].source.clone();
            let state = source.get_state(registers, gates);
            self.logic_trackers[index].states.push(state);
        }

        for index in 0..self.value_trackers.len() {
            let state = self.get_value_state(index);
            self.value_trackers[index].states.push(state);
        }
    }

    fn get_value_state(&self, index: usize) -> ValueState {
        let mut value = 0;

        for index in &self.value_trackers[index].trackers {
            match self.logic_trackers[*index].states.last().unwrap() {

                LogicState::High => {
                    value = value << 1;
                    value = value | 1;
                },

                LogicState::Low => value = value << 1,

                LogicState::Metastable => return ValueState::Metastable,

                LogicState::Floating => return ValueState::Floating,
            }
        }

        return ValueState::Stable(value);
    }

    pub fn draw<T: Renderer>(&self, renderer: &mut T, mut position: FloatVector) {
        self.draw_item(renderer, &self.root_item, &mut position, 0.0);
    }

    fn draw_state_text<T: Renderer>(&self, renderer: &mut T, position: FloatVector, color: Color, source: &str) {
        let right_position = position + FloatVector::from(self.label_width - STATE_OFFSET, TEXT_PADDING);
        renderer.draw_text_right(source, right_position, color, TEXT_SIZE);
    }

    fn draw_common<T: Renderer>(&self, renderer: &mut T, text: &str, position: FloatVector, level: f32) {

        renderer.draw_rectangle(position, FloatVector::from(self.label_width, LABEL_HEIGHT), LABEL_COLOR);
        renderer.draw_text(text, position + FloatVector::from(level + IDENTIFIER_OFFSET, TEXT_PADDING), TEXT_COLOR, TEXT_SIZE);

        if self.show_trackers {

            let tracker_width = self.interface_size.x - self.label_width - TRACKER_OFFSET;
            renderer.draw_rectangle(position + FloatVector::with_x(self.label_width + TRACKER_OFFSET), FloatVector::from(tracker_width, LABEL_HEIGHT), TRACKER_COLOR);

            let mut offset = position.x + self.label_width + TRACKER_OFFSET;
            while offset < self.interface_size.x + position.x {
                let delta = self.interface_size.x + position.x - offset;

                if delta < self.step_size {
                    renderer.draw_rectangle(FloatVector::from(offset, position.y), FloatVector::from(delta, LABEL_HEIGHT), OVERLAY_COLOR);
                    break;
                } else {
                    renderer.draw_rectangle(FloatVector::from(offset, position.y), FloatVector::from(self.step_size, LABEL_HEIGHT), OVERLAY_COLOR);
                    offset += self.step_size * 2.0;
                }
            }
        }
    }

    fn draw_item<T: Renderer>(&self, renderer: &mut T, item: &InspectorItem, position: &mut FloatVector, level: f32) {
        match item {

            InspectorItem::Group(group) => { // items, tracker, identifier, expanded

                let text = match group.expanded {
                    true => format!("- {}", group.identifier),
                    false => format!("+ {}", group.identifier),
                };

                self.draw_common(renderer, &text, *position, level);

                if let Some((index, formatting)) = group.tracker {
                    if let Some(value) = self.value_trackers[index].states.last() {

                        let color = Self::get_value_color(value);
                        let length = self.value_trackers[index].trackers.len();
                        let text = Self::get_value_text(value, formatting, length);
                        self.draw_state_text(renderer, *position, color, &text);

                        if self.show_trackers {

                            let mut step_offset = 0.0;
                            let mut vertices = Vec::new();
                            let mut previous_state = ValueState::Stable(0);

                            for state in &self.value_trackers[index].states {

                                let color = Self::get_value_color(state);
                                let height = LABEL_HEIGHT / 2.0;

                                if previous_state != *state && step_offset != 0.0 {
                                    vertices.push(Vertex::new(FloatVector::from(step_offset, TRACKER_MARGIN), color));
                                    vertices.push(Vertex::new(FloatVector::from(step_offset, LABEL_HEIGHT - TRACKER_MARGIN), color));
                                }

                                vertices.push(Vertex::new(FloatVector::from(step_offset, height), color));
                                step_offset += self.step_size;

                                let max_width = self.interface_size.x - self.label_width - TRACKER_OFFSET;
                                if step_offset > max_width {
                                    let clamped_position = FloatVector::from(max_width, height);
                                    vertices.push(Vertex::new(clamped_position, color));
                                    break;
                                } else {
                                    let vertex_position = FloatVector::from(step_offset, height);
                                    vertices.push(Vertex::new(vertex_position, color));
                                }

                                previous_state = *state;
                            }

                            let graph_position = *position + FloatVector::with_x(self.label_width + TRACKER_OFFSET);
                            renderer.draw_line_segment(graph_position, &vertices);
                        }
                    }
                }

                position.y += LABEL_HEIGHT + ITEM_GAP;
                if group.expanded {
                    group.items.iter().for_each(|item| self.draw_item(renderer, item, position, level + LEVEL_STEP));
                }
            },

            InspectorItem::Label(label) => {

                self.draw_common(renderer, &format!("  {}", label.identifier), *position, level);

                if let Some(state) = self.logic_trackers[label.tracker].states.last() {

                    let color = Self::get_state_color(state);
                    let text = Self::get_state_text(state);
                    self.draw_state_text(renderer, *position, color, text);

                    if self.show_trackers {

                        let mut step_offset = 0.0;
                        let mut vertices = Vec::new();

                        for state in &self.logic_trackers[label.tracker].states {

                            let color = Self::get_state_color(state);
                            let height = Self::get_state_height(state);
                            vertices.push(Vertex::new(FloatVector::from(step_offset, height), color));
                            step_offset += self.step_size;

                            let max_width = self.interface_size.x - self.label_width - TRACKER_OFFSET;
                            if step_offset > max_width {
                                let clamped_position = FloatVector::from(max_width, height);
                                vertices.push(Vertex::new(clamped_position, color));
                                break;
                            } else {
                                let vertex_position = FloatVector::from(step_offset, height);
                                vertices.push(Vertex::new(vertex_position, color));
                            }
                        }

                        let graph_position = *position + FloatVector::with_x(self.label_width + TRACKER_OFFSET);
                        renderer.draw_line_segment(graph_position, &vertices);
                    }
                }

                position.y += LABEL_HEIGHT + ITEM_GAP;
            },
        }
    }

    fn get_value_color(value: &ValueState) -> Color {
        match value {
            ValueState::Stable(..) => return VALUE_COLOR,
            ValueState::Metastable => return METASTABLE_COLOR,
            ValueState::Floating => return FLOATING_COLOR,
        }
    }

    fn get_value_text(value: &ValueState, formatting: Formatting, length: usize) -> String {
        match value {

            ValueState::Stable(value) => {
                match formatting {
                    Formatting::Binary => format!("{:0length$b}", value, length = length),
                    Formatting::SignedDecimal => format!("{}", value),
                    Formatting::UnsignedDecimal => format!("{}", value),
                    Formatting::Hexadecimal => format!("{:x}", value),
                }
            },

            ValueState::Metastable => return String::from("metastable"),

            ValueState::Floating => return String::from("floating"),
        }
    }

    fn get_state_color(state: &LogicState) -> Color {
        match state {
            LogicState::High => return STABLE_COLOR,
            LogicState::Low => return STABLE_COLOR,
            LogicState::Metastable => return METASTABLE_COLOR,
            LogicState::Floating => return FLOATING_COLOR,
        }
    }

    fn get_state_text(state: &LogicState) -> &'static str {
        match state {
            LogicState::High => return "high",
            LogicState::Low => return "low",
            LogicState::Metastable => return "metastable",
            LogicState::Floating => return "floating",
        }
    }

    fn get_state_height(state: &LogicState) -> f32 {
        match state {
            LogicState::High => return TRACKER_MARGIN,
            LogicState::Low => return LABEL_HEIGHT - TRACKER_MARGIN,
            LogicState::Metastable => return LABEL_HEIGHT / 2.0,
            LogicState::Floating => return LABEL_HEIGHT / 2.0,
        }
    }
}
