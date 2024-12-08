use crate::renderloop::utils::*;
use gl::types::GLuint;

pub(crate) static FBO_VERTEX_SHADER_SOURCE: &str = include_str!("./shaders/fbo_vertex.glsl");
pub(crate) static FBO_FRAGMENT_SHADER_SOURCE: &str = include_str!("./shaders/fbo_fragment.glsl");
pub(crate) static TEXT_VERTEX_SHADER_SOURCE: &str = include_str!("./shaders/text_vertex.glsl");
pub(crate) static TEXT_FRAGMENT_SHADER_SOURCE: &str = include_str!("./shaders/text_fragment.glsl");
pub(crate) static TR_VERTEX_SHADER_SOURCE: &str = include_str!("./shaders/shader_vertex.glsl");
pub(crate) static TR_FRAGMENT_SHADER_SOURCE: &str = include_str!("./shaders/shader_fragment.glsl");

pub fn init_opengl(video_subsystem: &sdl2::VideoSubsystem) {
    gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);
}

pub fn prepare_shader() -> (GLuint, GLuint, GLuint) {
    let vertex_shader = compile_shader(TEXT_VERTEX_SHADER_SOURCE, gl::VERTEX_SHADER);
    let fragment_shader = compile_shader(TEXT_FRAGMENT_SHADER_SOURCE, gl::FRAGMENT_SHADER);
    let text_shader_program = link_program(vertex_shader, fragment_shader);

    let vertex_tr = compile_shader(TR_VERTEX_SHADER_SOURCE, gl::VERTEX_SHADER);
    let fragment_tr = compile_shader(TR_FRAGMENT_SHADER_SOURCE, gl::FRAGMENT_SHADER);
    let tr_shader_program = link_program(vertex_tr, fragment_tr);

    let vertex_fbo = compile_shader(FBO_VERTEX_SHADER_SOURCE, gl::VERTEX_SHADER);
    let fragment_fbo = compile_shader(FBO_FRAGMENT_SHADER_SOURCE, gl::FRAGMENT_SHADER);
    let fbo_shader_program = link_program(vertex_fbo, fragment_fbo);

    // unsafe {
    //     gl::DeleteShader(vertex_shader);
    //     gl::DeleteShader(fragment_shader);
    //     gl::DeleteShader(vertex_fbo);
    //     gl::DeleteShader(fragment_fbo);
    // }

    (text_shader_program, fbo_shader_program, tr_shader_program)
}
