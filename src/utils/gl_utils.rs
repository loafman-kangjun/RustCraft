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
            
            // 显示纹理坐标的颜色调试
            vec4 debugColor;
            if (alpha > 0.5) {
                debugColor = vec4(1.0, 0.0, 0.0, 1.0);  // 红色表示字形
            } else if (alpha > 0.0) {
                debugColor = vec4(0.0, 1.0, 0.0, 1.0);  // 绿色表示边缘
            } else {
                debugColor = vec4(0.0, 0.0, 1.0, 0.2);  // 蓝色表示背景
            }
            
            FragColor = debugColor;
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
    face.load_char(c as usize, freetype::face::LoadFlag::RENDER).unwrap();
    let glyph = face.glyph();
    let bitmap = glyph.bitmap();

    println!("位图信息:");
    println!("宽度: {}, 高度: {}", bitmap.width(), bitmap.rows());
    println!("pitch: {}", bitmap.pitch());

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

        // 设置纹理参数
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
            buffer.as_ptr() as *const _,
        );

        // 检查OpenGL错误
        let error = gl::GetError();
        if error != gl::NO_ERROR {
            println!("纹理创建错误: 0x{:X}", error);
        }

        // 设置纹理过滤
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

        // 验证纹理数据
        let mut width = 0;
        let mut height = 0;
        gl::GetTexLevelParameteriv(gl::TEXTURE_2D, 0, gl::TEXTURE_WIDTH, &mut width);
        gl::GetTexLevelParameteriv(gl::TEXTURE_2D, 0, gl::TEXTURE_HEIGHT, &mut height);
        println!("OpenGL纹理尺寸: {}x{}", width, height);

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
        // 设置更暗的背景色以便于观察
        gl::ClearColor(0.1, 0.1, 0.1, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);
        
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        
        let character = characters.get(&'A').unwrap();
        
        // 打印字符信息
        println!("渲染字符 'A':");
        println!("位置: ({}, {})", 400.0f32, 300.0f32);
        println!("尺寸: {}x{}", character.size.0, character.size.1);
        println!("Bearing: ({}, {})", character.bearing.0, character.bearing.1);
        
        let mut vao = 0;
        let mut vbo = 0;
        
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);
        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        
        // 使用更大的缩放因子
        let scale = 3.0f32;
        let x = 400.0f32;
        let y = 300.0f32;
        let w = character.size.0 as f32 * scale;
        let h = character.size.1 as f32 * scale;
        
        let x_pos = x + character.bearing.0 as f32 * scale;
        let y_pos = y - (character.size.1 - character.bearing.1) as f32 * scale;
        
        // 打印实际渲染位置
        println!("实际渲染位置: ({}, {})", x_pos, y_pos);
        println!("渲染尺寸: {}x{}", w, h);
        
        let vertices: [f32; 24] = [
            x_pos,       y_pos + h,   0.0, 0.0,
            x_pos,       y_pos,       0.0, 1.0,
            x_pos + w,   y_pos,       1.0, 1.0,
            
            x_pos,       y_pos + h,   0.0, 0.0,
            x_pos + w,   y_pos,       1.0, 1.0,
            x_pos + w,   y_pos + h,   1.0, 0.0
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
            std::ptr::null()
        );

        gl::UseProgram(shader_program);
        
        // 使用简化的正交投影矩阵
        let projection = [
            2.0 / 800.0, 0.0, 0.0, 0.0,
            0.0, -2.0 / 600.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            -1.0, 1.0, 0.0, 1.0f32
        ];

        let projection_loc = gl::GetUniformLocation(shader_program, 
            CString::new("projection").unwrap().as_ptr());
        gl::UniformMatrix4fv(projection_loc, 1, gl::FALSE, projection.as_ptr());

        gl::ActiveTexture(gl::TEXTURE0);
        gl::BindTexture(gl::TEXTURE_2D, character.texture_id);
        let texture_loc = gl::GetUniformLocation(shader_program, 
            CString::new("textTexture").unwrap().as_ptr());
        gl::Uniform1i(texture_loc, 0);

        gl::DrawArrays(gl::TRIANGLES, 0, 6);

        // 检查渲染错误
        let error = gl::GetError();
        if error != gl::NO_ERROR {
            println!("渲染错误: 0x{:X}", error);
        }

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
