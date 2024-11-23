mod utils;

use utils::sdl_utils::show_start_screen;
use utils::gl_utils::{init_opengl, render_opengl_scene, find_sdl_gl_driver};
use sdl2::event::Event;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("SDL2 + OpenGL", 800, 600)
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().index(find_sdl_gl_driver().unwrap()).build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    if show_start_screen(&mut canvas, &mut event_pump) {
        let shader_program = init_opengl(&video_subsystem);

        'opengl_loop: loop {
            for event in event_pump.poll_iter() {
                if let Event::Quit { .. } = event {
                    break 'opengl_loop;
                }
            }

            render_opengl_scene(shader_program);
            canvas.present();
        }
    }
}
