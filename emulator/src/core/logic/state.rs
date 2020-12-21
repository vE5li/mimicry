use super::Operator;

#[derive(Copy, Clone, Debug)]
pub enum LogicState {
    High,
    Low,
    Floating,
    Metastable,
}

impl LogicState {

    pub fn from_boolean(state: bool) -> Self {
        match state {
            true => return LogicState::High,
            false => return LogicState::Low,
        }
    }

    pub fn to_boolean(&self) -> bool {
        match self {
            LogicState::High => return true,
            LogicState::Low => return false,
            _other => panic!("attempt to convert a floating or metastable state to a boolean"),
        }
    }

    pub fn is_floating(&self) -> bool {
        match self {
            LogicState::Floating => return true,
            _other => return false,
        }
    }

    pub fn is_metastable(&self) -> bool {
        match self {
            LogicState::Metastable => return true,
            _other => return false,
        }
    }

    pub fn operator(&self, operant: Self, operator: Operator) -> Self {

        if self.is_floating() || operant.is_floating() {
            return LogicState::Floating;
        }

        if self.is_metastable() || operant.is_metastable() {
            return LogicState::Metastable;
        }

        match operator {
            Operator::And => return LogicState::from_boolean(self.to_boolean() & operant.to_boolean()),
            Operator::Or => return LogicState::from_boolean(self.to_boolean() | operant.to_boolean()),
            Operator::Xor => return LogicState::from_boolean(self.to_boolean() ^ operant.to_boolean()),
        }
    }
}
