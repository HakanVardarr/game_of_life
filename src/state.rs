#[derive(Copy, Clone, PartialEq)]
pub enum State {
    Live,
    Dead,
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            State::Live => write!(f, "O"),
            State::Dead => write!(f, " ",),
        }
    }
}
