use std::fmt;

#[derive(Clone, Debug)]
pub enum Action {
    Move(u32, u32),
    Throw(u32, u32),
    Shoot(u32), // target id
    HunkerDown,
    Message(String),
}
impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Action::Move(x, y) => write!(f, "MOVE {} {}", x, y),
            Action::Throw(x, y) => write!(f, "THROW {} {}", x, y),
            Action::Shoot(id) => write!(f, "SHOOT {}", id),
            Action::HunkerDown => write!(f, "HUNKER_DOWN"),
            Action::Message(text) => write!(f, "MESSAGE {}", text),
        }
    }
}