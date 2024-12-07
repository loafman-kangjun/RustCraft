extern crate gl;

use gl::types::*;
use std::collections::HashMap;
use std::ffi::CString;

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