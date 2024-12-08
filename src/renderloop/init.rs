use crate::renderloop::utils::*;
use gl::types::GLuint;

pub(crate) static VERTEX_SHADER_SOURCE: &str = include_str!("./shaders/text_vertex.glsl");
pub(crate) static FRAGMENT_SHADER_SOURCE: &str = include_str!("./shaders/text_fragment.glsl");
pub(crate) static VERTEX_FBO_SHADER_SOURCE: &str = include_str!("./shaders/vertex_fbo.glsl");
pub(crate) static FRAGMENT_FBO_SHADER_SOURCE: &str = include_str!("./shaders/fragment_fbo.glsl");

pub fn init_opengl(video_subsystem: &sdl2::VideoSubsystem) {
    gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);
}

pub fn prepare_shader() -> (GLuint, GLuint) {
    let vertex_shader = compile_shader(VERTEX_SHADER_SOURCE, gl::VERTEX_SHADER);
    let fragment_shader = compile_shader(FRAGMENT_SHADER_SOURCE, gl::FRAGMENT_SHADER);
    let shader_program = link_program(vertex_shader, fragment_shader);

    let vertex_fbo = compile_shader(VERTEX_FBO_SHADER_SOURCE, gl::VERTEX_SHADER);
    let fragment_fbo = compile_shader(FRAGMENT_FBO_SHADER_SOURCE, gl::FRAGMENT_SHADER);
    let shader_program_fbo = link_program(vertex_fbo, fragment_fbo);

    unsafe {
        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);
        gl::DeleteShader(vertex_fbo);
        gl::DeleteShader(fragment_fbo);
    }

    (shader_program, shader_program_fbo)
}
