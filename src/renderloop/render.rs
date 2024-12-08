use crate::renderloop::freetype::init_freetype;
use crate::renderloop::init::*;
use crate::renderloop::text::render_text;
use crate::renderloop::utils::*;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::{EventPump, VideoSubsystem};

pub(crate) async fn render(
    video_subsystem: &VideoSubsystem,
    canvas: &mut Canvas<Window>,
    event_pump: &mut EventPump,
) {
    init_opengl(video_subsystem);

    let (shader_program, shader_program_fbo) = prepare_shader();
    let characters = init_freetype().await;

    'opengl_loop: loop {
        for event in event_pump.poll_iter() {
            if let sdl2::event::Event::Quit { .. } = event {
                break 'opengl_loop;
            }
        }

        clean_screen();
        render_text(shader_program, shader_program_fbo, &characters);
        canvas.present();
    }
}
