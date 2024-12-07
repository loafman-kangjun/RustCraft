use crate::utils::gl_utils::{init_freetype, init_opengl, render_text};
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::{EventPump, VideoSubsystem};

pub async fn run_opengl_page(
    video_subsystem: &VideoSubsystem,
    canvas: &mut Canvas<Window>,
    event_pump: &mut EventPump,
) {
    let shader_program = init_opengl(video_subsystem);

    // 初始化FreeType并加载字符
    let characters = init_freetype().await;

    'opengl_loop: loop {
        for event in event_pump.poll_iter() {
            if let sdl2::event::Event::Quit { .. } = event {
                break 'opengl_loop;
            }
        }

        render_text(shader_program, &characters);

        canvas.present();
    }
}
