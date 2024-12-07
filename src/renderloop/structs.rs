use gl::types::GLuint;

pub struct Character {
    pub texture_id: GLuint,
    pub size: (i32, i32),
    pub bearing: (i32, i32),
    pub advance: i32,
}