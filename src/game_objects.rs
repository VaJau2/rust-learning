use raylib::prelude::*; 
use crate::game_objects::ball::Ball;
use crate::{SCREEN_WIDTH, SCREEN_HEIGHT};

mod ball;

pub trait GameObject {
    fn update_input(&mut self, raylib_handle: &RaylibHandle);
    fn draw(&self, draw_handle: &mut RaylibDrawHandle);
}

pub struct GameObjectsFactory {}

impl GameObjectsFactory {
    pub fn create_objects() -> Vec<Box<dyn GameObject>> {
        let ball = Ball {
            position: Vector2::new(SCREEN_WIDTH / 2.0, SCREEN_HEIGHT / 2.0),
            speed: 3.0,
            radius: 40.0,
            color: Color::GREEN
        };
    
        let mut objects: Vec<Box<dyn GameObject>> = Vec::new();
        objects.push(Box::new(ball));
    
        objects
    }
   
}
