use crate::utils::gl_utils::{init_opengl, render_opengl_scene};
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;
use sdl2::VideoSubsystem;
use std::time::{Duration, Instant};

pub fn run_opengl_page(
    video_subsystem: &VideoSubsystem,
    canvas: &mut Canvas<Window>,
    event_pump: &mut EventPump,
) {
    let shader_program = init_opengl(video_subsystem);

    let mut last_time = Instant::now();
    let mut frame_count = 0;

    'opengl_loop: loop {
        let current_time = Instant::now();
        let elapsed = current_time - last_time;
        frame_count += 1;

        if elapsed >= Duration::from_secs(1) {
            let fps = frame_count as f32 / elapsed.as_secs_f32();
            println!("FPS: {:.2}", fps);
            frame_count = 0;
            last_time = current_time;
        }

        for event in event_pump.poll_iter() {
            if let sdl2::event::Event::Quit { .. } = event {
                break 'opengl_loop;
            }
        }

        render_opengl_scene(shader_program);
        canvas.present();
    }
}
