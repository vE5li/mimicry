mod formatting;
mod label;
mod group;

pub use self::formatting::Formatting;
pub use self::label::Label;
pub use self::group::Group;

#[derive(Clone, Debug)]
pub enum InspectorItem {
    Group(Group),
    Label(Label),
}
