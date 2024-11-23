extern crate gl;

use gl::types::*;
use std::ffi::CString;
use std::fs;
use std::path::Path;

/// 初始化 OpenGL
pub fn init_opengl(video_subsystem: &sdl2::VideoSubsystem) -> GLuint {
    gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);

    let vertex_shader_src = load_shader_code("src/shaders/vertex_shader.glsl");
    let fragment_shader_src = load_shader_code("src/shaders/fragment_shader.glsl");

    let vertex_shader = compile_shader(&vertex_shader_src, gl::VERTEX_SHADER);
    let fragment_shader = compile_shader(&fragment_shader_src, gl::FRAGMENT_SHADER);

    let shader_program = link_program(vertex_shader, fragment_shader);

    unsafe {
        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);
    }

    shader_program
}

pub fn find_sdl_gl_driver() -> Option<u32> {
    for (index, item) in sdl2::render::drivers().enumerate() {
        if item.name == "opengl" {
            return Some(index as u32);
        }
    }
    None
}

/// 渲染 OpenGL 场景
pub fn render_opengl_scene(shader_program: GLuint) {
    let vertices: [f32; 18] = [
        -0.5, -0.5, 0.0, 1.0, 0.0, 0.0,
        0.5, -0.5, 0.0, 0.0, 1.0, 0.0,
        0.0, 0.5, 0.0, 0.0, 0.0, 1.0,
    ];

    let (mut vao, mut vbo) = (0, 0);
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);

        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<f32>()) as GLsizeiptr,
            vertices.as_ptr() as *const _,
            gl::STATIC_DRAW,
        );

        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 6 * std::mem::size_of::<f32>() as GLsizei, std::ptr::null());
        gl::EnableVertexAttribArray(0);

        gl::VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            6 * std::mem::size_of::<f32>() as GLsizei,
            (3 * std::mem::size_of::<f32>()) as *const _,
        );
        gl::EnableVertexAttribArray(1);

        gl::ClearColor(0.1, 0.1, 0.1, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);

        gl::UseProgram(shader_program);
        gl::BindVertexArray(vao);
        gl::DrawArrays(gl::TRIANGLES, 0, 3);
    }
}

/// 加载 GLSL 着色器代码
fn load_shader_code<P: AsRef<Path>>(path: P) -> String {
    fs::read_to_string(path).expect("Failed to load shader file")
}

/// 编译着色器
fn compile_shader(source: &str, shader_type: GLenum) -> GLuint {
    let shader = unsafe { gl::CreateShader(shader_type) };
    let c_str = CString::new(source).unwrap();
    unsafe {
        gl::ShaderSource(shader, 1, &c_str.as_ptr(), std::ptr::null());
        gl::CompileShader(shader);

        let mut success = gl::FALSE as GLint;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
        if success == gl::FALSE.into() {
            let mut info_log = vec![0; 512];
            gl::GetShaderInfoLog(
                shader,
                512,
                std::ptr::null_mut(),
                info_log.as_mut_ptr() as *mut _,
            );
            panic!("Shader compilation failed: {}", String::from_utf8_lossy(&info_log));
        }
    }
    shader
}

/// 链接着色器程序
fn link_program(vertex_shader: GLuint, fragment_shader: GLuint) -> GLuint {
    let program = unsafe { gl::CreateProgram() };
    unsafe {
        gl::AttachShader(program, vertex_shader);
        gl::AttachShader(program, fragment_shader);
        gl::LinkProgram(program);

        let mut success = gl::FALSE as GLint;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);
        if success == gl::FALSE.into() {
            let mut info_log = vec![0; 512];
            gl::GetProgramInfoLog(
                program,
                512,
                std::ptr::null_mut(),
                info_log.as_mut_ptr() as *mut _,
            );
            panic!("Program linking failed: {}", String::from_utf8_lossy(&info_log));
        }
    }
    program
}
