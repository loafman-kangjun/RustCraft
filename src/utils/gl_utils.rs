extern crate gl;

use freetype::Library;
use gl::types::*;
use std::cmp::min;
use std::collections::HashMap;
use std::ffi::CString;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

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
            float alpha = texture(textTexture, TexCoords).r;
            FragColor = vec4(1.0, 1.0, 1.0, alpha);
        }
    "#;

    let vertex_shader = compile_shader(vertex_shader_source, gl::VERTEX_SHADER);
    validate_shader(vertex_shader, "顶点");
    
    let fragment_shader = compile_shader(fragment_shader_source, gl::FRAGMENT_SHADER);
    validate_shader(fragment_shader, "片段");

    let shader_program = link_program(vertex_shader, fragment_shader);
    
    // 验证着色器程序
    unsafe {
        gl::ValidateProgram(shader_program);
        let mut validate_status = gl::FALSE as GLint;
        gl::GetProgramiv(shader_program, gl::VALIDATE_STATUS, &mut validate_status);
        println!("着色器程序验证状态: {}", validate_status);

        let mut log_length = 0;
        gl::GetProgramiv(shader_program, gl::INFO_LOG_LENGTH, &mut log_length);
        if log_length > 0 {
            let mut info_log = vec![0u8; log_length as usize];
            gl::GetProgramInfoLog(
                shader_program,
                log_length,
                std::ptr::null_mut(),
                info_log.as_mut_ptr() as *mut GLchar
            );
            println!("程序验证日志: {}", 
                String::from_utf8_lossy(&info_log));
        }
    }

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
    face.load_char(c as usize, freetype::face::LoadFlag::RENDER)
        .unwrap();
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
            file.write_all(format!("{:3} ", buffer[idx]).as_bytes())
                .await
                .unwrap();
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
        println!("生成的纹理ID: {}", texture);
        
        gl::BindTexture(gl::TEXTURE_2D, texture);
        
        // 在上传纹理数据前检查bitmap数据
        let buffer = bitmap.buffer();
        println!("位图数据大小: {}", buffer.len());
        println!("位图维度: {}x{}", bitmap.width(), bitmap.rows());
        println!("位图pitch: {}", bitmap.pitch());
        
        // 检查buffer中的一些值
        println!("位图数据样本:");
        for i in 0..min(10, buffer.len()) {
            print!("{} ", buffer[i]);
        }
        println!();

        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RED as i32,
            bitmap.width(),
            bitmap.rows(),
            0,
            gl::RED,
            gl::UNSIGNED_BYTE,
            buffer.as_ptr() as *const _,
        );

        // 检查OpenGL错误
        let error = gl::GetError();
        if error != gl::NO_ERROR {
            println!("纹理创建错误: 0x{:X}", error);
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
        
        // 创建正交投影矩阵 (修改为更合适的范围)
        let window_width = 800.0f32;
        let window_height = 600.0f32;
        let projection = [
            1.0 / (window_width * 0.5), 0.0, 0.0, 0.0,
            0.0, 1.0 / (window_height * 0.5), 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            -1.0, -1.0, 0.0, 1.0f32
        ];

        let mut vao = 0;
        let mut vbo = 0;
        
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);
        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        
        // 修改顶点位置计算
        let x = 200.0f32;  // 更改位置使其更容易看到
        let y = 300.0f32;
        let w = character.size.0 as f32;
        let h = character.size.1 as f32;
        
        // 修改顶点数据，确保正确的纹理坐标
        let vertices: [f32; 24] = [
            x,     y - h,   0.0, 0.0,  // 左下
            x + w, y - h,   1.0, 0.0,  // 右下
            x + w, y,       1.0, 1.0,  // 右上

            x,     y - h,   0.0, 0.0,  // 左下
            x + w, y,       1.0, 1.0,  // 右上
            x,     y,       0.0, 1.0   // 左上
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
            0,
            std::ptr::null()
        );

        gl::UseProgram(shader_program);
        
        // 设置投影矩阵
        let projection_loc = gl::GetUniformLocation(shader_program, 
            CString::new("projection").unwrap().as_ptr());
        gl::UniformMatrix4fv(projection_loc, 1, gl::FALSE, projection.as_ptr());

        // 设置纹理
        gl::ActiveTexture(gl::TEXTURE0);
        gl::BindTexture(gl::TEXTURE_2D, character.texture_id);
        let texture_loc = gl::GetUniformLocation(shader_program, 
            CString::new("textTexture").unwrap().as_ptr());
        gl::Uniform1i(texture_loc, 0);

        gl::DrawArrays(gl::TRIANGLES, 0, 6);

        // 清理
        gl::DeleteBuffers(1, &vbo);
        gl::DeleteVertexArrays(1, &vao);
    }
}

// 添加一个辅助函数来验证着色器编译状态
fn validate_shader(shader: GLuint, shader_type: &str) {
    unsafe {
        let mut success = gl::FALSE as GLint;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
        println!("{} 着色器编译状态: {}", shader_type, success);

        let mut log_length = 0;
        gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut log_length);
        if log_length > 0 {
            let mut info_log = vec![0u8; log_length as usize];
            gl::GetShaderInfoLog(
                shader,
                log_length,
                std::ptr::null_mut(),
                info_log.as_mut_ptr() as *mut GLchar
            );
            println!("{} 着色器日志: {}", 
                shader_type, 
                String::from_utf8_lossy(&info_log));
        }
    }
}
