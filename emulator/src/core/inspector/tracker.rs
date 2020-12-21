use super::LabelSource;
use core::logic::{ LogicState };

pub struct Tracker {
    pub source: LabelSource,
    pub states: Vec<LogicState>,
}

impl Tracker {

    pub fn new(source: LabelSource) -> Self {
        return Self {
            source: source,
            states: Vec::new(),
        };
    }
}
