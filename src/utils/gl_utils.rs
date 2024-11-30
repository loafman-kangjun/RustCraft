extern crate gl;

use freetype::Library;
use gl::types::*;
use tokio::fs::File;
use std::collections::HashMap;
use std::ffi::CString;
use tokio::io::AsyncWriteExt;

// const VER_SHADER_SOURCE: &str = include_str!("../shaders/vertex_shader.glsl");
// const FRA_SHADER_SOURCE: &str = include_str!("../shaders/fragment_shader.glsl");
/// 初始化 OpenGL
pub fn init_opengl(video_subsystem: &sdl2::VideoSubsystem) -> GLuint {
    gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);

    // 文字渲染着色器源码
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
            vec4 sampled = vec4(1.0, 1.0, 1.0, texture(textTexture, TexCoords).r);
            FragColor = sampled;
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

/// 渲染 OpenGL 场景
// pub fn render_opengl_scene(shader_program: GLuint) {
//     let vertices: [f32; 18] = [
//         -0.5, -0.5, 0.0, 1.0, 0.0, 0.0, 0.5, -0.5, 0.0, 0.0, 1.0, 0.0, 0.0, 0.5, 0.0, 0.0, 0.0, 1.0,
//     ];

//     let (mut vao, mut vbo) = (0, 0);
//     unsafe {
//         gl::GenVertexArrays(1, &mut vao);
//         gl::GenBuffers(1, &mut vbo);

//         gl::BindVertexArray(vao);
//         gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
//         gl::BufferData(
//             gl::ARRAY_BUFFER,
//             (vertices.len() * std::mem::size_of::<f32>()) as GLsizeiptr,
//             vertices.as_ptr() as *const _,
//             gl::STATIC_DRAW,
//         );

//         gl::VertexAttribPointer(
//             0,
//             3,
//             gl::FLOAT,
//             gl::FALSE,
//             6 * std::mem::size_of::<f32>() as GLsizei,
//             std::ptr::null(),
//         );
//         gl::EnableVertexAttribArray(0);

//         gl::VertexAttribPointer(
//             1,
//             3,
//             gl::FLOAT,
//             gl::FALSE,
//             6 * std::mem::size_of::<f32>() as GLsizei,
//             (3 * std::mem::size_of::<f32>()) as *const _,
//         );
//         gl::EnableVertexAttribArray(1);

//         gl::ClearColor(0.1, 0.1, 0.1, 1.0);
//         gl::Clear(gl::COLOR_BUFFER_BIT);

//         gl::UseProgram(shader_program);
//         gl::BindVertexArray(vao);
//         gl::DrawArrays(gl::TRIANGLES, 0, 3);
//     }
// }

/// 编译着色器
fn compile_shader(source: &str, shader_type: GLenum) -> GLuint {
    let shader = unsafe { gl::CreateShader(shader_type) };
    let c_str = CString::new(source.as_bytes()).unwrap();
    unsafe {
        gl::ShaderSource(shader, 1, &c_str.as_ptr(), std::ptr::null());
        gl::CompileShader(shader);

        let mut success = gl::FALSE as GLint;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);

        if success == gl::FALSE as GLint {
            let mut info_log = vec![0u8; 512];
            let mut length = 0;
            gl::GetShaderInfoLog(
                shader,
                512,
                &mut length,
                info_log.as_mut_ptr() as *mut GLchar,
            );
            panic!(
                "着色器编译失败: {}",
                String::from_utf8_lossy(&info_log[..length as usize])
            );
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
        if success == gl::FALSE as GLint {
            let mut info_log = vec![0u8; 512];
            let mut length = 0;
            gl::GetProgramInfoLog(
                program,
                512,
                &mut length,
                info_log.as_mut_ptr() as *mut GLchar,
            );
            panic!(
                "程序链接失败: {}",
                String::from_utf8_lossy(&info_log[..length as usize])
            );
        }
    }
    program
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
    face.load_char(c as usize, freetype::face::LoadFlag::RENDER).unwrap();
    let glyph = face.glyph();
    let bitmap = glyph.bitmap();

    // 打印位图信息
    println!("位图信息:");
    println!("宽度: {}, 高度: {}", bitmap.width(), bitmap.rows());
    println!("pitch: {}", bitmap.pitch());
    
    // 保存位图数据到文件
    let buffer = bitmap.buffer();
    let mut file = File::create("bitmap_raw.txt").await.unwrap();
    for y in 0..bitmap.rows() {
        for x in 0..bitmap.width() {
            let idx = y as usize * bitmap.pitch() as usize + x as usize;
            file.write_all(format!("{:3} ", buffer[idx]).as_bytes()).await.unwrap();
        }
        file.write_all(b"\n").await.unwrap();
    }

    // 保存可视化的ASCII艺术
    let mut file = File::create("bitmap_visual.txt").await.unwrap();
    for y in 0..bitmap.rows() {
        for x in 0..bitmap.width() {
            let idx = y as i32 * bitmap.pitch() + x as i32;
            let value = buffer[idx as usize];
            let char = if value > 128 { '#' } else { '.' };
            file.write_all(char.to_string().as_bytes()).await.unwrap();
        }
        file.write_all(b"\n").await.unwrap();
    }

    let mut texture = 0;
    unsafe {
        gl::GenTextures(1, &mut texture);
        gl::BindTexture(gl::TEXTURE_2D, texture);
        
        // 打印纹理数据
        println!("纹理信息:");
        println!("纹理ID: {}", texture);
        println!("纹理大小: {}x{}", bitmap.width(), bitmap.rows());
        
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

        // 检查是否有OpenGL错误
        let error = gl::GetError();
        if error != gl::NO_ERROR {
            println!("创建纹理时的OpenGL错误: 0x{:X}", error);
        }

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
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);

        let character = characters.get(&'A').unwrap();

        // 创建正交投影矩阵
        let window_width = 800.0f32;
        let window_height = 600.0f32;
        let projection = [
            2.0 / window_width,
            0.0,
            0.0,
            0.0,
            0.0,
            -2.0 / window_height,
            0.0,
            0.0,
            0.0,
            0.0,
            -1.0,
            0.0,
            -1.0,
            1.0,
            0.0,
            1.0f32,
        ];

        let mut vao = 0;
        let mut vbo = 0;

        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);
        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

        let x = 100.0f32;
        let y = 100.0f32;
        let w = character.size.0 as f32;
        let h = character.size.1 as f32;

        let vertices: [f32; 24] = [
            x,
            y + h,
            0.0,
            0.0,
            x,
            y,
            0.0,
            1.0,
            x + w,
            y,
            1.0,
            1.0,
            x,
            y + h,
            0.0,
            0.0,
            x + w,
            y,
            1.0,
            1.0,
            x + w,
            y + h,
            1.0,
            0.0,
        ];

        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<f32>()) as GLsizeiptr,
            vertices.as_ptr() as *const _,
            gl::STATIC_DRAW,
        );

        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(0, 4, gl::FLOAT, gl::FALSE, 0, std::ptr::null());

        gl::UseProgram(shader_program);

        let projection_loc =
            gl::GetUniformLocation(shader_program, CString::new("projection").unwrap().as_ptr());
        gl::UniformMatrix4fv(projection_loc, 1, gl::FALSE, projection.as_ptr());

        gl::ActiveTexture(gl::TEXTURE0);
        gl::BindTexture(gl::TEXTURE_2D, character.texture_id);
        let texture_loc = gl::GetUniformLocation(
            shader_program,
            CString::new("textTexture").unwrap().as_ptr(),
        );
        gl::Uniform1i(texture_loc, 0);

        gl::DrawArrays(gl::TRIANGLES, 0, 6);

        gl::DeleteBuffers(1, &vbo);
        gl::DeleteVertexArrays(1, &vao);
    }
}
