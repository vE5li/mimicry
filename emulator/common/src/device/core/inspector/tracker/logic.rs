use super::LabelSource;
use crate::device::core::LogicState;

pub struct LogicTracker {
    pub source: LabelSource,
    pub states: Vec<LogicState>,
}

impl LogicTracker {

    pub fn new(source: LabelSource) -> Self {
        return Self {
            source: source,
            states: Vec::new(),
        };
    }
}
