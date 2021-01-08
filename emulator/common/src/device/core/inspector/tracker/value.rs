use crate::device::core::ValueState;

pub struct ValueTracker {
    pub trackers: Vec<usize>,
    pub states: Vec<ValueState>,
}

impl ValueTracker {

    pub fn new(trackers: Vec<usize>) -> Self {
        return Self {
            trackers: trackers,
            states: Vec::new(),
        };
    }
}
