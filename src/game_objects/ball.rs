use raylib::prelude::*; 
use raylib::consts::MouseButton::*; 
use raylib::consts::KeyboardKey::KEY_SPACE;
use crate::GameObject;

const POSITION_AMOUNT: f32 = 0.025;

pub struct Ball {
    pub position: Vector2,
    pub speed: f32,
    pub radius: f32,
    pub color: Color,
}

impl GameObject for Ball {
    fn update_input(self: &mut Ball, raylib_handle: &RaylibHandle) {
        if raylib_handle.is_mouse_button_down(MOUSE_LEFT_BUTTON) {
            self.position = self.position.lerp(
                raylib_handle.get_mouse_position(), 
                POSITION_AMOUNT
            );
        }

        if raylib_handle.is_mouse_button_pressed(MOUSE_RIGHT_BUTTON) {
            self.position = raylib_handle.get_mouse_position();
        }
    
        if raylib_handle.is_key_pressed(KEY_SPACE) {
            self.color = if self.color == Color::GREEN { Color::YELLOW } else { Color::GREEN };
        }
    }

    fn draw(self: &Ball, draw_handle: &mut RaylibDrawHandle) {
        draw_handle.draw_circle_v(self.position, self.radius, self.color);
    }
}
