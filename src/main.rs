#[macro_use]
extern crate glium;
use glium::Surface;

fn main() {
    let event_loop = glium::winit::event_loop::EventLoop::builder()
        .build()
        .expect("event loop building");
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .with_title("Glium tutorial #6")
        .build(&event_loop);

    let image = image::load(std::io::Cursor::new(&include_bytes!("./opengl.png")),
                            image::ImageFormat::Png).unwrap().to_rgba8();
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    let texture = glium::Texture2d::new(&display, image).unwrap();

    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 2],
        tex_coords: [f32; 2],
    }
    implement_vertex!(Vertex, position, tex_coords);

    let shape = vec![
        Vertex { position: [-0.5, -0.5], tex_coords: [0.0, 0.0] },
        Vertex { position: [ 0.5, -0.5], tex_coords: [1.0, 0.0] },
        Vertex { position: [ 0.5,  0.5], tex_coords: [1.0, 1.0] },
        Vertex { position: [ 0.5,  0.5], tex_coords: [1.0, 1.0] },
        Vertex { position: [-0.5,  0.5], tex_coords: [0.0, 1.0] },
        Vertex { position: [-0.5, -0.5], tex_coords: [0.0, 0.0] },
    ];
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();

    // 加载着色器文件
    let vertex_shader_src = std::str::from_utf8(include_bytes!("vertex_shader.glsl"))
        .expect("Failed to read vertex shader");
    let fragment_shader_src = std::str::from_utf8(include_bytes!("fragment_shader.glsl"))
        .expect("Failed to read fragment shader");

    let program = glium::Program::from_source(&display, &vertex_shader_src, &fragment_shader_src, None)
        .expect("Failed to create program");

    #[allow(deprecated)]
    event_loop.run(move |ev, window_target| {
        match ev {
            glium::winit::event::Event::WindowEvent { event, .. } => match event {
                glium::winit::event::WindowEvent::CloseRequested => {
                    window_target.exit();
                },
                glium::winit::event::WindowEvent::RedrawRequested => {
                    let mut target = display.draw();
                    target.clear_color(0.0, 0.0, 1.0, 1.0);

                    let uniforms = uniform! {
                        tex: &texture,
                    };

                    target.draw(&vertex_buffer, &indices, &program, &uniforms,
                                &Default::default()).unwrap();
                    target.finish().unwrap();
                },
                glium::winit::event::WindowEvent::Resized(window_size) => {
                    display.resize(window_size.into());
                },
                _ => (),
            },
            glium::winit::event::Event::AboutToWait => {
                window.request_redraw();
            },
            _ => (),
        }
    })
    .unwrap();
}
