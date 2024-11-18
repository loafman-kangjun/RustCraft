#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 3],
    tex_coords: [f32; 2],
}
implement_vertex!(Vertex, position, tex_coords);

// pub const SHAPE: [Vertex; 6] = [
//     Vertex {
//         position: [-0.5, -0.5],
//         tex_coords: [0.0, 0.0],
//     },
//     Vertex {
//         position: [0.5, -0.5],
//         tex_coords: [1.0, 0.0],
//     },
//     Vertex {
//         position: [0.5, 0.5],
//         tex_coords: [1.0, 1.0],
//     },
//     Vertex {
//         position: [0.5, 0.5],
//         tex_coords: [1.0, 1.0],
//     },
//     Vertex {
//         position: [-0.5, 0.5],
//         tex_coords: [0.0, 1.0],
//     },
//     Vertex {
//         position: [-0.5, -0.5],
//         tex_coords: [0.0, 0.0],
//     },
// ];

pub const CUBE: [Vertex; 36] = [
    // Front face
    Vertex { position: [-0.5, -0.5,  0.5], tex_coords: [0.0, 0.0] },
    Vertex { position: [ 0.5, -0.5,  0.5], tex_coords: [1.0, 0.0] },
    Vertex { position: [ 0.5,  0.5,  0.5], tex_coords: [1.0, 1.0] },
    Vertex { position: [-0.5, -0.5,  0.5], tex_coords: [0.0, 0.0] },
    Vertex { position: [ 0.5,  0.5,  0.5], tex_coords: [1.0, 1.0] },
    Vertex { position: [-0.5,  0.5,  0.5], tex_coords: [0.0, 1.0] },

    // Back face
    Vertex { position: [-0.5, -0.5, -0.5], tex_coords: [0.0, 0.0] },
    Vertex { position: [-0.5,  0.5, -0.5], tex_coords: [0.0, 1.0] },
    Vertex { position: [ 0.5,  0.5, -0.5], tex_coords: [1.0, 1.0] },
    Vertex { position: [-0.5, -0.5, -0.5], tex_coords: [0.0, 0.0] },
    Vertex { position: [ 0.5,  0.5, -0.5], tex_coords: [1.0, 1.0] },
    Vertex { position: [ 0.5, -0.5, -0.5], tex_coords: [1.0, 0.0] },

    // Left face
    Vertex { position: [-0.5, -0.5, -0.5], tex_coords: [0.0, 0.0] },
    Vertex { position: [-0.5, -0.5,  0.5], tex_coords: [1.0, 0.0] },
    Vertex { position: [-0.5,  0.5,  0.5], tex_coords: [1.0, 1.0] },
    Vertex { position: [-0.5, -0.5, -0.5], tex_coords: [0.0, 0.0] },
    Vertex { position: [-0.5,  0.5,  0.5], tex_coords: [1.0, 1.0] },
    Vertex { position: [-0.5,  0.5, -0.5], tex_coords: [0.0, 1.0] },

    // Right face
    Vertex { position: [ 0.5, -0.5, -0.5], tex_coords: [0.0, 0.0] },
    Vertex { position: [ 0.5,  0.5, -0.5], tex_coords: [0.0, 1.0] },
    Vertex { position: [ 0.5,  0.5,  0.5], tex_coords: [1.0, 1.0] },
    Vertex { position: [ 0.5, -0.5, -0.5], tex_coords: [0.0, 0.0] },
    Vertex { position: [ 0.5,  0.5,  0.5], tex_coords: [1.0, 1.0] },
    Vertex { position: [ 0.5, -0.5,  0.5], tex_coords: [1.0, 0.0] },

    // Top face
    Vertex { position: [-0.5,  0.5, -0.5], tex_coords: [0.0, 0.0] },
    Vertex { position: [-0.5,  0.5,  0.5], tex_coords: [0.0, 1.0] },
    Vertex { position: [ 0.5,  0.5,  0.5], tex_coords: [1.0, 1.0] },
    Vertex { position: [-0.5,  0.5, -0.5], tex_coords: [0.0, 0.0] },
    Vertex { position: [ 0.5,  0.5,  0.5], tex_coords: [1.0, 1.0] },
    Vertex { position: [ 0.5,  0.5, -0.5], tex_coords: [1.0, 0.0] },

    // Bottom face
    Vertex { position: [-0.5, -0.5, -0.5], tex_coords: [0.0, 0.0] },
    Vertex { position: [ 0.5, -0.5, -0.5], tex_coords: [1.0, 0.0] },
    Vertex { position: [ 0.5, -0.5,  0.5], tex_coords: [1.0, 1.0] },
    Vertex { position: [-0.5, -0.5, -0.5], tex_coords: [0.0, 0.0] },
    Vertex { position: [ 0.5, -0.5,  0.5], tex_coords: [1.0, 1.0] },
    Vertex { position: [-0.5, -0.5,  0.5], tex_coords: [0.0, 1.0] },
];
