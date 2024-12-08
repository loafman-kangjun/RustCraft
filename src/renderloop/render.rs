use crate::renderloop::freetype::init_freetype;
use crate::renderloop::init::*;
use crate::renderloop::text::render_text;
use crate::renderloop::text::render_tr;
use crate::renderloop::utils::*;
use gl::types::GLuint;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::{EventPump, VideoSubsystem};
use sdl2::keyboard::Keycode;

pub(crate) async fn render(
    video_subsystem: &VideoSubsystem,
    canvas: &mut Canvas<Window>,
    event_pump: &mut EventPump,
) {
    init_opengl(video_subsystem);

    let (shader_program, shader_program_fbo, shader_program_tr) = prepare_shader();
    let characters = init_freetype().await;
    let mut text_fbo;

    'opengl_loop: loop {
        clean_screen();
        text_fbo = render_text(shader_program, &characters);
        let triangle_fbo = render_tr(shader_program_tr);
        
        reder_fbo(text_fbo, shader_program_fbo);
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'opengl_loop,
                sdl2::event::Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                    // 当按下S键时保存FBO
                    save_fbo_to_file(text_fbo, 800, 600, "text_fbo.png");
                    save_fbo_to_file(triangle_fbo, 800, 600, "triangle_fbo.png");
                    println!("FBOs saved to files!");
                }
                _ => {}
            }
        }
        canvas.present();
    }
}

fn reder_fbo(fbo_texture: GLuint, shader_program_fbo: GLuint) {
    unsafe {
        gl::FramebufferTexture2D(
            gl::FRAMEBUFFER,
            gl::COLOR_ATTACHMENT0,
            gl::TEXTURE_2D,
            fbo_texture,
            0,
        );

        // 切换回默认帧缓冲
        gl::BindFramebuffer(gl::FRAMEBUFFER, 0);

        // 这里需要使用另一个着色器程序来渲染FBO纹理到屏幕
        gl::UseProgram(shader_program_fbo);

        // 绑定FBO纹理
        gl::ActiveTexture(gl::TEXTURE0);
        gl::BindTexture(gl::TEXTURE_2D, fbo_texture);

        // 渲染全屏四边形
        gl::DrawArrays(gl::TRIANGLES, 0, 6);

        // 清理资源
        gl::DeleteTextures(1, &fbo_texture);
    }
}
