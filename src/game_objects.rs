use raylib::prelude::*; 
use crate::game_objects::ball::Ball;

mod ball;

pub trait GameObject {
    fn update_input(&mut self, raylib_handle: &RaylibHandle);
    fn draw(&self, draw_handle: &mut RaylibDrawHandle);
}

pub struct GameObjectsFactory {}

impl GameObjectsFactory {
    pub fn create_objects() -> Vec<Box<dyn GameObject>> {
        let ball = Ball::create();
    
        let mut objects: Vec<Box<dyn GameObject>> = Vec::new();
        objects.push(Box::new(ball));
    
        objects
    }
}
