use super::Formatting;

#[derive(Clone, Debug)]
pub enum InspectorItem {
    Group(Vec<InspectorItem>, Option<(usize, Formatting)>, &'static str, bool),
    Label(usize, &'static str),
}
