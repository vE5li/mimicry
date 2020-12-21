#[derive(Clone, Debug)]
pub enum InspectorItem {
    Group(Vec<InspectorItem>, Option<usize>, &'static str, bool),
    Label(usize, &'static str),
}
