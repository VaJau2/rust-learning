use raylib::prelude::*;
use crate::game_objects::*;

mod game_objects;

const SCREEN_WIDTH: f32 = 640.0;
const SCREEN_HEIGHT: f32 = 480.0;

fn main() {
    let (mut raylib_handle, thread) = raylib::init()
        .size(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32)
        .title("Hello world")
        .vsync()
        .build();

    let mut objects: Vec<Box<dyn GameObject>> = game_objects::create_objects();

    while !raylib_handle.window_should_close() {
        for object in objects.iter_mut() {
            object.update_input(&raylib_handle);
        }
        
        render(&mut raylib_handle, &thread, &objects);
    }
}


fn render(raylib_handle: &mut RaylibHandle,  thread: &RaylibThread, objects: &Vec<Box<dyn GameObject>>) {
    let mut draw_handle = raylib_handle.begin_drawing(&thread);

    draw_handle.clear_background(Color::BLACK);

    for object in objects {
        object.draw(&mut draw_handle);
    }
}
