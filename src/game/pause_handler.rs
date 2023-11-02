use crate::game::RaylibHandle;
use crate::game::KeyboardKey::KEY_SPACE;

pub struct PauseHandler {
    pub pause: bool,
}

impl PauseHandler {
    pub fn default() -> PauseHandler {
        PauseHandler { pause: false }
    }

    pub fn update(&mut self, raylib_handle: &RaylibHandle) {
        if raylib_handle.is_key_pressed(KEY_SPACE) {
            self.pause = !self.pause;
        } 
    }
}
