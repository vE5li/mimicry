use super::{ InspectorItem, Formatting };

#[derive(Clone, Debug)]
pub struct Group {
    pub identifier: &'static str,
    pub tracker: Option<(usize, Formatting)>,
    pub items: Vec<InspectorItem>,
    pub expanded: bool,
}

impl Group {

    pub fn new(identifier: &'static str, tracker: Option<(usize, Formatting)>, items: Vec<InspectorItem>) -> Self {
        return Self {
            identifier: identifier,
            tracker: tracker,
            items: items,
            expanded: true, // TODO: false
        }
    }
}
