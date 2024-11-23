extern crate sdl2;
extern crate gl;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
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

/// 显示开始页面
fn show_start_screen(canvas: &mut Canvas<Window>, event_pump: &mut sdl2::EventPump) -> bool {
    let mut running = true;
    let mut start_game = false;

    while running {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => {
                    running = false;
                }
                sdl2::event::Event::MouseButtonDown { x, y, .. } => {
                    // 检查是否点击了按钮
                    if Rect::new(300, 250, 200, 50).contains_point((x, y)) {
                        start_game = true;
                        running = false;
                    }
                }
                _ => {}
            }
        }

        // 绘制开始界面
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        // 绘制按钮
        canvas.set_draw_color(Color::RGB(0, 128, 255));
        canvas.fill_rect(Rect::new(300, 250, 200, 50)).unwrap();

        // TODO: 可以添加字体绘制以显示“开始游戏”文字

        canvas.present();
    }

    start_game
}

/// 初始化 OpenGL 和 SDL2
fn init_opengl(video_subsystem: &sdl2::VideoSubsystem) -> GLuint {
    // 加载 OpenGL 函数
    gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);

    // 定义着色器
    let vertex_shader_src = r#"
        #version 330 core
        layout (location = 0) in vec3 aPos;
        layout (location = 1) in vec3 aColor;

        out vec3 vertexColor;

        void main() {
            gl_Position = vec4(aPos, 1.0);
            vertexColor = aColor;
        }
    "#;

    let fragment_shader_src = r#"
        #version 330 core
        in vec3 vertexColor;
        out vec4 FragColor;

        void main() {
            FragColor = vec4(vertexColor, 1.0);
        }
    "#;

    // 编译着色器
    let vertex_shader = compile_shader(vertex_shader_src, gl::VERTEX_SHADER);
    let fragment_shader = compile_shader(fragment_shader_src, gl::FRAGMENT_SHADER);
    let shader_program = link_program(vertex_shader, fragment_shader);

    unsafe {
        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);
    }

    shader_program
}

/// 编译 OpenGL 着色器
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

/// 链接 OpenGL 程序
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

/// 渲染 OpenGL 场景
fn render_opengl_scene(shader_program: GLuint) {
    // 顶点数据
    let vertices: [f32; 18] = [
        -0.5, -0.5, 0.0, 1.0, 0.0, 0.0, // 左下角，红色
        0.5, -0.5, 0.0, 0.0, 1.0, 0.0, // 右下角，绿色
        0.0, 0.5, 0.0, 0.0, 0.0, 1.0, // 顶点，蓝色
    ];

    // 创建 VAO 和 VBO
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
    }

    unsafe {
        gl::ClearColor(0.1, 0.1, 0.1, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);
        gl::UseProgram(shader_program);
        gl::BindVertexArray(vao);
        gl::DrawArrays(gl::TRIANGLES, 0, 3);
    }
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("SDL2 + OpenGL", 800, 600)
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().index(find_sdl_gl_driver().unwrap()).build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    if show_start_screen(&mut canvas, &mut event_pump) {
        let shader_program = init_opengl(&video_subsystem);

        'opengl_loop: loop {
            for event in event_pump.poll_iter() {
                if let sdl2::event::Event::Quit { .. } = event {
                    break 'opengl_loop;
                }
            }

            render_opengl_scene(shader_program);
            canvas.present();
        }
    }
}
