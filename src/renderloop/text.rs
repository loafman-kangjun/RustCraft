extern crate gl;

use crate::renderloop::structs::{Character, QuadGeometry};
use cgmath::{ortho, Matrix, Point2, Vector2};
use gl::types::*;
use std::collections::HashMap;
use std::ffi::CString;

impl QuadGeometry {
    fn new(character: &Character, base_pos: Point2<f32>, scale: Vector2<f32>) -> Self {
        let size = Vector2::new(
            character.size.0 as f32 * scale.x,
            character.size.1 as f32 * scale.y,
        );

        let bearing = Vector2::new(
            character.bearing.0 as f32,
            character.bearing.1 as f32
        );

        let pos = Point2::new(
            base_pos.x + bearing.x * scale.x,
            base_pos.y - (character.size.1 as f32 - bearing.y) * scale.y,
        );

        let vertices = [
            pos.x,         pos.y + size.y,  0.0, 1.0,
            pos.x,         pos.y,           0.0, 0.0,
            pos.x + size.x, pos.y,          1.0, 0.0,
            pos.x,         pos.y + size.y,  0.0, 1.0,
            pos.x + size.x, pos.y,          1.0, 0.0,
            pos.x + size.x, pos.y + size.y, 1.0, 1.0,
        ];

        Self { vertices }
    }
}

pub fn render_text(shader_program: GLuint, characters: &HashMap<char, Character>) {
    unsafe {
        gl::ClearColor(0.1, 0.1, 0.1, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);

        let character = characters.get(&'A').unwrap();

        let mut vao = 0;
        let mut vbo = 0;

        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);
        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

        let quad = QuadGeometry::new(
            character,
            Point2::new(400.0f32, 300.0f32),
            Vector2::new(3.0f32, 3.0f32)
        );

        gl::BufferData(
            gl::ARRAY_BUFFER,
            (quad.vertices.len() * std::mem::size_of::<f32>()) as GLsizeiptr,
            quad.vertices.as_ptr() as *const _,
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

        let projection = ortho(
            0.0,   // left
            800.0, // right
            0.0,   // bottom
            600.0, // top
            -1.0,  // near
            1.0,   // far
        );

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
