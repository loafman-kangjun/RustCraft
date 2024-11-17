use glium::backend::glutin::Display;
use glium::glutin::surface::WindowSurface;

pub fn shader_pro(display: &Display<WindowSurface>) -> glium::program::Program {
    let vertex_shader_src = std::str::from_utf8(include_bytes!("gl/vertex_shader.glsl"))
        .expect("Failed to read vertex shader");
    let fragment_shader_src = std::str::from_utf8(include_bytes!("gl/fragment_shader.glsl"))
        .expect("Failed to read fragment shader");

    let program =
        glium::Program::from_source(display, &vertex_shader_src, &fragment_shader_src, None)
            .expect("Failed to create program");
    return program;
}
