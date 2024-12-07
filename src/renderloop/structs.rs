use gl::types::GLuint;
#[derive(Debug)]
pub struct QuadGeometry {
    pub vertices: [f32; 24],
}

pub struct Character {
    pub texture_id: GLuint,
    pub size: (i32, i32),
    pub bearing: (i32, i32),
    pub advance: i32,
}