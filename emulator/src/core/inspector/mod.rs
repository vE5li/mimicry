mod source;
mod value;
mod tracker;
mod item;

pub use self::source::LabelSource;
pub use self::value::{ ValueState, ValueTracker };
pub use self::tracker::Tracker;
pub use self::item::InspectorItem;

use core::{ LogicState, Register, Gate };
use sfml::system::Vector2f;
use sfml::graphics::*;

const LABEL_SIZE: Vector2f = Vector2f::new(400.0, 20.0);
const TEXT_OFFSET: Vector2f = Vector2f::new(10.0, 1.0);
const LABEL_GAP: f32 = 3.0;
const TRACKER_STEP: f32 = 20.0;
const LEVEL_STEP: f32 = 20.0;
const TRACKER_OFFSET: f32 = 5.0;
const TRACKER_MARGIN: f32 = 3.0;
const ITEM_GAP: f32 = 3.0;
const STATE_OFFSET: f32 = 10.0;

const LABEL_COLOR: Color = Color::rgb(55, 55, 55);
const TRACKER_COLOR: Color = Color::rgb(40, 40, 40);
const TEXT_COLOR: Color = Color::rgb(150, 150, 150);
const STABLE_COLOR: Color = Color::rgb(100, 150, 100);
const METASTABLE_COLOR: Color = Color::rgb(250, 100, 100);
const FLOATING_COLOR: Color = Color::rgb(150, 100, 100);
const VALUE_COLOR: Color = Color::rgb(180, 80, 150);

pub struct Inspector<'a> {
    interface_size: Vector2f,
    label_width: f32,
    show_trackers: bool,
    root_item: InspectorItem,
    label_background: RectangleShape<'a>,
    tracker_background: RectangleShape<'a>,
    label_text: Text<'a>,
    state_text: Text<'a>,
    trackers: Vec<Tracker>,
    value_trackers: Vec<ValueTracker>,
    step_size: f32,
}

impl<'a> Inspector<'a> {

    pub fn new(font: &'a Font, interface_size: Vector2f, trackers: Vec<Tracker>, value_trackers: Vec<ValueTracker>, root_item: InspectorItem) -> Self {

        let label_width = LABEL_SIZE.x;
        let track_size = Vector2f::new(interface_size.x - LABEL_SIZE.x - TRACKER_OFFSET, LABEL_SIZE.y);

        let mut label_background = RectangleShape::with_size(LABEL_SIZE);
        label_background.set_fill_color(LABEL_COLOR);

        let mut tracker_background = RectangleShape::with_size(track_size);
        tracker_background.set_fill_color(TRACKER_COLOR);

        let mut label_text = Text::default();
        label_text.set_fill_color(TEXT_COLOR);
        label_text.set_character_size(14);
        label_text.set_font(font);

        let mut state_text = Text::default();
        state_text.set_character_size(14);
        state_text.set_font(font);

        return Self {
            interface_size: interface_size,
            label_width: label_width,
            show_trackers: true,
            root_item: root_item,
            label_background: label_background,
            tracker_background: tracker_background,
            label_text: label_text,
            state_text: state_text,
            trackers: trackers,
            value_trackers: value_trackers,
            step_size: TRACKER_STEP,
        };
    }

    pub fn update_size(&mut self, interface_size: Vector2f) {
        self.interface_size = interface_size;
        self.update_graphics();
    }

    fn update_graphics(&mut self) {
        let label_size = Vector2f::new(self.label_width, LABEL_SIZE.y);
        let tracker_size = Vector2f::new(self.interface_size.x - self.label_width - TRACKER_OFFSET, LABEL_SIZE.y);
        self.label_background.set_size(label_size);
        self.tracker_background.set_size(tracker_size);
    }

    pub fn update(&mut self, registers: &Vec<Register>, gates: &Vec<Gate>) {

        for index in 0..self.trackers.len() {
            let source = self.trackers[index].source.clone();
            let state = source.get_state(registers, gates);
            self.trackers[index].states.push(state);
        }

        for index in 0..self.value_trackers.len() {
            let state = self.get_value_state(index);
            self.value_trackers[index].states.push(state);
        }
    }

    fn get_value_state(&self, index: usize) -> ValueState {
        let mut value = 0;

        for index in &self.value_trackers[index].trackers {
            match self.trackers[*index].states.last().unwrap() {

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

    pub fn draw(&mut self, window: &mut RenderWindow, mut position: Vector2f) {
        let root_item = self.root_item.clone();
        self.draw_item(window, &root_item, &mut position, 0.0);

    }

    fn draw_state_text(&mut self, window: &mut RenderWindow, position: Vector2f, color: Color, source: &str) {

        self.state_text.set_string(source);
        self.state_text.set_fill_color(color);

        let text_width = self.state_text.local_bounds().width + STATE_OFFSET;
        let text_position = position + Vector2f::new(self.label_width - text_width, 0.0);
        self.state_text.set_position(text_position);
        window.draw(&self.state_text);
    }

    fn draw_common(&mut self, window: &mut RenderWindow, text: &str, position: Vector2f, level: f32) {

        self.label_background.set_position(position);
        self.label_text.set_position(position + TEXT_OFFSET + Vector2f::new(level, 0.0));
        self.label_text.set_string(text);

        window.draw(&self.label_background);
        window.draw(&self.label_text);

        if self.show_trackers {
            self.tracker_background.set_position(position + Vector2f::new(LABEL_SIZE.x + TRACKER_OFFSET, 0.0));
            window.draw(&self.tracker_background);
        }
    }

    fn draw_item(&mut self, window: &mut RenderWindow, item: &InspectorItem, position: &mut Vector2f, level: f32) {
        match item {

            InspectorItem::Group(items, tracker, identifier, expanded) => {

                let text = match expanded {
                    true => format!("- {}", identifier),
                    false => format!("+ {}", identifier),
                };

                self.draw_common(window, &text, *position, level);

                if let Some(index) = tracker {
                    if let Some(value) = self.value_trackers[*index].states.last() {

                        let color = Self::get_value_color(value);
                        let text = Self::get_value_text(value);

                        self.draw_state_text(window, *position, color, &text);
                    }
                }

                position.y += LABEL_SIZE.y + ITEM_GAP;
                if *expanded {
                    items.iter().for_each(|item| self.draw_item(window, item, position, level + LEVEL_STEP));
                }
            },

            InspectorItem::Label(index, identifier) => {

                self.draw_common(window, &format!("  {}", identifier), *position, level);

                if let Some(state) = self.trackers[*index].states.last() {
                    let color = Self::get_state_color(state);
                    let text = Self::get_state_text(state);

                    self.draw_state_text(window, *position, color, text);

                    if self.show_trackers {

                        let start_position = *position + Vector2f::new(LABEL_SIZE.x + TRACKER_OFFSET, 0.0);
                        let mut step_offset = 0.0;
                        let mut tracker_graph = VertexArray::default();
                        tracker_graph.set_primitive_type(PrimitiveType::LineStrip);

                        for state in &self.trackers[*index].states {
                            let height = Self::get_state_height(state);
                            tracker_graph.append(&Vertex::with_pos_color(start_position + Vector2f::new(step_offset, height), color));
                            step_offset += self.step_size;
                            tracker_graph.append(&Vertex::with_pos_color(start_position + Vector2f::new(step_offset, height), color));
                        }

                        window.draw(&tracker_graph);
                    }
                }

                position.y += LABEL_SIZE.y + ITEM_GAP;
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

    fn get_value_text(value: &ValueState) -> String {
        match value {
            ValueState::Stable(value) => return value.to_string(),
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
            LogicState::Low => return LABEL_SIZE.y - TRACKER_MARGIN,
            LogicState::Metastable => return LABEL_SIZE.y / 2.0,
            LogicState::Floating => return LABEL_SIZE.y / 2.0,
        }
    }
}
