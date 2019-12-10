#[derive(Debug)]
pub struct InputManager {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
}

impl InputManager {
    pub fn new() -> Self {
        InputManager {
            up: false,
            down: false,
            left: false,
            right: false,
        }
    }
}
