extern crate gl;

use freetype::Library;
use gl::types::*;
use std::collections::HashMap;
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

pub fn find_sdl_gl_driver() -> Option<u32> {
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

pub async fn init_freetype() -> HashMap<char, Character> {
    let lib = Library::init().unwrap();
    let face = lib.new_face("./a.ttf", 0).unwrap();
    face.set_pixel_sizes(0, 48).unwrap();

    let mut characters = HashMap::new();
    let c = 'A';
    face.load_char(c as usize, freetype::face::LoadFlag::RENDER)
        .unwrap();
    let glyph = face.glyph();
    let bitmap = glyph.bitmap();

    let mut texture = 0;
    unsafe {
        gl::GenTextures(1, &mut texture);
        gl::BindTexture(gl::TEXTURE_2D, texture);
        gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);

        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RED as i32,
            bitmap.width(),
            bitmap.rows(),
            0,
            gl::RED,
            gl::UNSIGNED_BYTE,
            bitmap.buffer().as_ptr() as *const _,
        );

        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

        characters.insert(
            c,
            Character {
                texture_id: texture,
                size: (bitmap.width(), bitmap.rows()),
                bearing: (glyph.bitmap_left(), glyph.bitmap_top()),
                advance: glyph.advance().x as i32,
            },
        );
    }

    characters
}

pub fn render_text(shader_program: GLuint, characters: &HashMap<char, Character>) {
    unsafe {
        gl::ClearColor(0.1, 0.1, 0.1, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);

        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);

        let character = characters.get(&'A').unwrap();

        let mut vao = 0;
        let mut vbo = 0;

        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);
        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

        let scale = 3.0f32;
        let x = 400.0f32;
        let y = 300.0f32;
        let w = character.size.0 as f32 * scale;
        let h = character.size.1 as f32 * scale;

        let x_pos = x + character.bearing.0 as f32 * scale;
        let y_pos = y - (character.size.1 - character.bearing.1) as f32 * scale;

        let vertices: [f32; 24] = [
            x_pos,
            y_pos + h,
            0.0,
            1.0,
            x_pos,
            y_pos,
            0.0,
            0.0,
            x_pos + w,
            y_pos,
            1.0,
            0.0,
            x_pos,
            y_pos + h,
            0.0,
            1.0,
            x_pos + w,
            y_pos,
            1.0,
            0.0,
            x_pos + w,
            y_pos + h,
            1.0,
            1.0,
        ];

        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<f32>()) as GLsizeiptr,
            vertices.as_ptr() as *const _,
            gl::STATIC_DRAW,
        );

        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(
            0,
            4,
            gl::FLOAT,
            gl::FALSE,
            4 * std::mem::size_of::<f32>() as GLsizei,
            std::ptr::null(),
        );

        gl::UseProgram(shader_program);

        let projection = [
            2.0 / 800.0,
            0.0,
            0.0,
            0.0,
            0.0,
            -2.0 / 600.0,
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
            0.0,
            -1.0,
            1.0,
            0.0,
            1.0f32,
        ];

        let proj_name = CString::new("projection").unwrap();
        let projection_loc = gl::GetUniformLocation(shader_program, proj_name.as_ptr());
        gl::UniformMatrix4fv(projection_loc, 1, gl::FALSE, projection.as_ptr());

        gl::ActiveTexture(gl::TEXTURE0);
        gl::BindTexture(gl::TEXTURE_2D, character.texture_id);
        let tex_name = CString::new("textTexture").unwrap();
        let texture_loc = gl::GetUniformLocation(shader_program, tex_name.as_ptr());
        gl::Uniform1i(texture_loc, 0);

        gl::DrawArrays(gl::TRIANGLES, 0, 6);

        gl::DeleteBuffers(1, &vbo);
        gl::DeleteVertexArrays(1, &vao);
    }
}
