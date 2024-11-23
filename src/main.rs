extern crate sdl2;
extern crate gl;

use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::Sdl;
use gl::types::*;
use std::ffi::CString;

/// 查找 OpenGL 驱动的索引
fn find_sdl_gl_driver() -> Option<u32> {
    for (index, item) in sdl2::render::drivers().enumerate() {
        if item.name == "opengl" {
            return Some(index as u32);
        }
    }
    None
}

/// 初始化 OpenGL 和 SDL2
fn init_sdl_and_opengl() -> (Sdl, Canvas<Window>) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("OpenGL with SDL2 Canvas", 800, 600)
        .opengl() // 允许 OpenGL 使用
        .build()
        .unwrap();

    let canvas = window
        .into_canvas()
        .index(find_sdl_gl_driver().unwrap())
        .build()
        .unwrap();

    (sdl_context, canvas)
}

/// 加载 OpenGL 着色器
fn compile_shader(source: &str, shader_type: GLenum) -> GLuint {
    let shader = unsafe { gl::CreateShader(shader_type) };
    let c_str = CString::new(source).unwrap();
    unsafe {
        gl::ShaderSource(shader, 1, &c_str.as_ptr(), std::ptr::null());
        gl::CompileShader(shader);

        // 检查编译错误
        let mut success = gl::FALSE as GLint;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
        if success == gl::FALSE as GLint {
            let mut info_log = vec![0; 512];
            gl::GetShaderInfoLog(
                shader,
                512,
                std::ptr::null_mut(),
                info_log.as_mut_ptr() as *mut _,
            );
            panic!(
                "Shader compilation failed: {}",
                String::from_utf8_lossy(&info_log)
            );
        }
    }
    shader
}

/// 链接 OpenGL 程序
fn link_program(vertex_shader: GLuint, fragment_shader: GLuint) -> GLuint {
    let program = unsafe { gl::CreateProgram() };
    unsafe {
        gl::AttachShader(program, vertex_shader);
        gl::AttachShader(program, fragment_shader);
        gl::LinkProgram(program);

        // 检查链接错误
        let mut success = gl::FALSE as GLint;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);
        if success == gl::FALSE as GLint {
            let mut info_log = vec![0; 512];
            gl::GetProgramInfoLog(
                program,
                512,
                std::ptr::null_mut(),
                info_log.as_mut_ptr() as *mut _,
            );
            panic!(
                "Program linking failed: {}",
                String::from_utf8_lossy(&info_log)
            );
        }
    }
    program
}

fn main() {
    // 初始化 SDL2 和 OpenGL
    let (sdl_context, mut canvas) = init_sdl_and_opengl();

    // 获取窗口上下文并加载 OpenGL 函数
    let video_subsystem = sdl_context.video().unwrap();
    gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);

    // 使用 Canvas 的 OpenGL 上下文
    canvas.window().gl_set_context_to_current();

    // 定义着色器
    let vertex_shader_src = r#"
        #version 330 core
        layout (location = 0) in vec3 aPos; // 顶点位置
        layout (location = 1) in vec3 aColor; // 顶点颜色
        out vec3 vertexColor; // 传递颜色到片段着色器
        void main() {
            gl_Position = vec4(aPos, 1.0);
            vertexColor = aColor;
        }
    "#;

    let fragment_shader_src = r#"
        #version 330 core
        in vec3 vertexColor; // 从顶点着色器接收颜色
        out vec4 FragColor;
        void main() {
            FragColor = vec4(vertexColor, 1.0);
        }
    "#;

    // 编译和链接着色器
    let vertex_shader = compile_shader(vertex_shader_src, gl::VERTEX_SHADER);
    let fragment_shader = compile_shader(fragment_shader_src, gl::FRAGMENT_SHADER);
    let shader_program = link_program(vertex_shader, fragment_shader);

    // 删除着色器
    unsafe {
        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);
    }

    // 顶点数据
    let vertices: [f32; 18] = [
        // 位置         // 颜色
        -0.5, -0.5, 0.0, 1.0, 0.0, 0.0, // 左下角，红色
        0.5, -0.5, 0.0, 0.0, 1.0, 0.0, // 右下角，绿色
        0.0, 0.5, 0.0, 0.0, 0.0, 1.0, // 顶点，蓝色
    ];

    // 创建 VAO 和 VBO
    let (mut vao, mut vbo) = (0, 0);
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);

        // 绑定 VAO
        gl::BindVertexArray(vao);

        // 绑定 VBO
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<GLfloat>()) as GLsizeiptr,
            vertices.as_ptr() as *const _,
            gl::STATIC_DRAW,
        );

        // 设置顶点属性
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            6 * std::mem::size_of::<GLfloat>() as GLsizei,
            std::ptr::null(),
        );
        gl::EnableVertexAttribArray(0);

        gl::VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            6 * std::mem::size_of::<GLfloat>() as GLsizei,
            (3 * std::mem::size_of::<GLfloat>()) as *const _,
        );
        gl::EnableVertexAttribArray(1);
    }

    // 渲染循环
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        // 处理事件
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'running,
                _ => {}
            }
        }

        // 清屏
        unsafe {
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            // 使用着色器程序
            gl::UseProgram(shader_program);

            // 绑定 VAO
            gl::BindVertexArray(vao);

            // 绘制三角形
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }

        // 显示结果
        canvas.present();
    }

    // 清理资源
    unsafe {
        gl::DeleteVertexArrays(1, &vao);
        gl::DeleteBuffers(1, &vbo);
        gl::DeleteProgram(shader_program);
    }
}
