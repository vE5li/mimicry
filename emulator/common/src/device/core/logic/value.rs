#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ValueState {
    Stable(u64),
    Metastable,
    Floating,
}
