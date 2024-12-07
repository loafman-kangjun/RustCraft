use gl::types::GLuint;
use crate::renderloop::utils::*;

pub(crate) static VERTEX_SHADER_SOURCE: &str = include_str!("./shaders/text_vertex.glsl");
pub(crate) static FRAGMENT_SHADER_SOURCE: &str = include_str!("./shaders/text_fragment.glsl");

fn init_opengl(video_subsystem: &sdl2::VideoSubsystem) {
    gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);
}

fn compile_text_shader() -> GLuint {
    let vertex_shader = compile_shader(VERTEX_SHADER_SOURCE, gl::VERTEX_SHADER);
    let fragment_shader = compile_shader(FRAGMENT_SHADER_SOURCE, gl::FRAGMENT_SHADER);
    let shader_program = link_program(vertex_shader, fragment_shader);

    unsafe {
        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);
    }

    shader_program
}
