use super::state::State;

#[derive(Copy, Clone)]
pub struct Cell {
    state: State,
}

impl Cell {
    pub fn dead() -> Self {
        Cell { state: State::Dead }
    }
    pub fn live() -> Self {
        Cell { state: State::Live }
    }
    pub fn is_alive(&self) -> bool {
        if self.state == State::Live {
            return true;
        }
        return false;
    }
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.state {
            State::Live => write!(f, "{}", State::Live),
            State::Dead => write!(f, "{}", State::Dead),
        }
    }
}
