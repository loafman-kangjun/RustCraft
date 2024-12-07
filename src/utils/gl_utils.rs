extern crate gl;

use gl::types::*;
use std::ffi::CString;

pub fn init_opengl(video_subsystem: &sdl2::VideoSubsystem) -> GLuint {
    gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);

    let vertex_shader_source = r#"
        #version 330 core
        layout (location = 0) in vec4 aPos;
        out vec2 TexCoords;
        uniform mat4 projection;
        void main() {
            gl_Position = projection * vec4(aPos.xy, 0.0, 1.0);
            TexCoords = aPos.zw;
        }
    "#;

    let fragment_shader_source = r#"
        #version 330 core
        in vec2 TexCoords;
        out vec4 FragColor;
        uniform sampler2D textTexture;
        void main() {
            float alpha = texture(textTexture, TexCoords).r;
            vec4 debugColor;
            if (alpha > 0.5) {
                debugColor = vec4(1.0, 0.0, 0.0, 1.0);
            } else if (alpha > 0.0) {
                debugColor = vec4(0.0, 1.0, 0.0, 1.0);
            } else {
                debugColor = vec4(0.0, 0.0, 1.0, 0.2);
            }
            FragColor = debugColor;
        }
    "#;

    let vertex_shader = compile_shader(vertex_shader_source, gl::VERTEX_SHADER);
    let fragment_shader = compile_shader(fragment_shader_source, gl::FRAGMENT_SHADER);
    let shader_program = link_program(vertex_shader, fragment_shader);

    unsafe {
        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);
    }

    shader_program
}

pub fn find_gl() -> Option<u32> {
    for (index, item) in sdl2::render::drivers().enumerate() {
        if item.name == "opengl" {
            return Some(index as u32);
        }
    }
    None
}

fn compile_shader(source: &str, shader_type: GLenum) -> GLuint {
    let shader = unsafe { gl::CreateShader(shader_type) };
    let c_str = CString::new(source.as_bytes()).unwrap();
    unsafe {
        gl::ShaderSource(shader, 1, &c_str.as_ptr(), std::ptr::null());
        gl::CompileShader(shader);
        shader
    }
}

fn link_program(vertex_shader: GLuint, fragment_shader: GLuint) -> GLuint {
    let program = unsafe { gl::CreateProgram() };
    unsafe {
        gl::AttachShader(program, vertex_shader);
        gl::AttachShader(program, fragment_shader);
        gl::LinkProgram(program);
        program
    }
}

pub struct Character {
    pub texture_id: GLuint,
    pub size: (i32, i32),
    pub bearing: (i32, i32),
    pub advance: i32,
}
