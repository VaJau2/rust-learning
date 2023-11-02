use raylib::prelude::*;
use crate::game_objects::GameObjectsFactory;
use crate::game_objects::GameObject;
use crate::{SCREEN_WIDTH, SCREEN_HEIGHT};
use crate::game::pause_handler::PauseHandler;

mod pause_handler;

/// Main game class
/// contains game loop and handles array of game objects
/// 
pub struct Game {
    pub pause_handler: PauseHandler,
    pub frame_count: i32,
    pub raylib_handle: RaylibHandle,
    pub thread: RaylibThread,
    objects: Vec<Box<dyn GameObject>>,
}

impl Game {
    pub fn init() -> Game {
        let (mut raylib_handle, thread) = raylib::init()
            .size(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32)
            .title("Hello world")
            .vsync()
            .build();

        raylib_handle.set_target_fps(60);
        let objects = GameObjectsFactory::create_objects();
        let pause_handler = PauseHandler::default();
        
        Game {
            pause_handler,
            frame_count: 0,
            raylib_handle,
            thread,
            objects,
        }
    }

    pub fn is_running(&self) -> bool {
        !self.raylib_handle.window_should_close()
    }

    pub fn update(&mut self) {
        self.pause_handler.update(&self.raylib_handle);

        if !self.pause_handler.pause {
            for object in self.objects.iter_mut() {
                object.update_input(&self.raylib_handle);
            }
        }
        
        self.render();
    }

    fn render(&mut self) {
        let mut draw_handle = self.raylib_handle.begin_drawing(&self.thread);
    
        draw_handle.clear_background(Color::BLACK);
    
        for object in self.objects.iter_mut() {
            object.draw(&mut draw_handle);
        }
    }
}
