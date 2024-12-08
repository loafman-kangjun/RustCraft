use crate::renderloop::render::render;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::{EventPump, VideoSubsystem};

pub async fn run_opengl_page(
    video_subsystem: &VideoSubsystem,
    canvas: &mut Canvas<Window>,
    event_pump: &mut EventPump,
) {
    render(video_subsystem, canvas, event_pump).await;
}
