#[derive(Clone, Debug)]
pub struct State {}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

impl State {
    pub fn new() -> Self {
        State {}
    }
}
