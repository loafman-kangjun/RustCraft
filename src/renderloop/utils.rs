extern crate gl;

use gl::types::*;
use std::ffi::CString;
use image::{ImageBuffer, Rgba};

pub fn clean_screen() {
    unsafe {
        gl::ClearColor(0.1, 0.1, 0.1, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);
    }
}

pub fn find_gl() -> Option<u32> {
    for (index, item) in sdl2::render::drivers().enumerate() {
        if item.name == "opengl" {
            return Some(index as u32);
        }
    }
    None
}

pub fn compile_shader(source: &str, shader_type: GLenum) -> GLuint {
    let shader = unsafe { gl::CreateShader(shader_type) };
    let c_str = CString::new(source.as_bytes()).unwrap();
    unsafe {
        gl::ShaderSource(shader, 1, &c_str.as_ptr(), std::ptr::null());
        gl::CompileShader(shader);
        shader
    }
}

pub fn link_program(vertex_shader: GLuint, fragment_shader: GLuint) -> GLuint {
    let program = unsafe { gl::CreateProgram() };
    unsafe {
        gl::AttachShader(program, vertex_shader);
        gl::AttachShader(program, fragment_shader);
        gl::LinkProgram(program);
        program
    }
}

pub fn save_fbo_to_file(fbo_texture: GLuint, width: u32, height: u32, filename: &str) {
    unsafe {
        // 分配内存来存储像素数据
        let mut pixels = vec![0u8; (width * height * 4) as usize];
        
        // 绑定FBO纹理
        gl::BindTexture(gl::TEXTURE_2D, fbo_texture);
        
        // 读取像素数据
        gl::GetTexImage(
            gl::TEXTURE_2D,
            0,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            pixels.as_mut_ptr() as *mut std::ffi::c_void
        );

        // 创建图像缓冲
        let mut img_buffer = ImageBuffer::<Rgba<u8>, Vec<u8>>::new(width, height);

        // 复制像素数据到图像缓冲
        for y in 0..height {
            for x in 0..width {
                let idx = ((height - 1 - y) * width + x) * 4;
                let pixel = Rgba([
                    pixels[idx as usize],
                    pixels[(idx + 1) as usize],
                    pixels[(idx + 2) as usize],
                    pixels[(idx + 3) as usize],
                ]);
                img_buffer.put_pixel(x, y, pixel);
            }
        }

        // 保存图像
        img_buffer.save(filename).expect("Failed to save image");
    }
}
