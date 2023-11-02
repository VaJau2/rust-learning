use raylib::prelude::*; 
use crate::game_objects::GameObject;
use crate::{SCREEN_WIDTH, SCREEN_HEIGHT};

pub struct Ball {
    pub position: Vector2,
    pub speed: Vector2,
    pub radius: f32,
}

impl Ball {
    pub fn create() -> Ball {
        Ball {
            position: Vector2::new(SCREEN_WIDTH / 2.0, SCREEN_HEIGHT / 2.0),
            speed: Vector2::new(4.0, 3.0),
            radius: 40.0,
        }
    }
}

impl GameObject for Ball {
    fn update_input(self: &mut Ball, _raylib_handle: &RaylibHandle) {
        self.position += self.speed;

        if self.position.x >= SCREEN_WIDTH - self.radius || self.position.x <= self.radius {
            self.speed.x *= -1.0;
        }

        if self.position.y >= SCREEN_HEIGHT - self.radius || self.position.y <= self.radius {
            self.speed.y *= -1.0;
        }
    }

    fn draw(self: &Ball, draw_handle: &mut RaylibDrawHandle) {
        draw_handle.draw_circle_v(self.position, self.radius, Color::GREEN);
    }
}
