#[derive(Debug)]
pub struct InputManager {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub mouse_pos: (f32, f32),
}

impl InputManager {
    pub fn new() -> Self {
        InputManager {
            up: false,
            down: false,
            left: false,
            right: false,
            mouse_pos: (0.0, 0.0),
        }
    }
}
