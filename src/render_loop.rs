use glium::index::PrimitiveType;
use glium::{index::NoIndices, texture::Texture2d, Display, Program, Surface, VertexBuffer};

pub fn run_event_loop(
    event_loop: glium::winit::event_loop::EventLoop<()>,
    display: Display,
    program: Program,
    vertex_buffer: VertexBuffer<impl glium::vertex::Vertex>,
    indices: NoIndices,
    texture: Texture2d,
) {
    #[allow(deprecated)]
    event_loop.run(move |ev, window_target| {
        match ev {
            glium::winit::event::Event::WindowEvent { event, .. } => match event {
                glium::winit::event::WindowEvent::CloseRequested => {
                    window_target.exit();
                }
                glium::winit::event::WindowEvent::RedrawRequested => {
                    let mut target = display.draw();
                    target.clear_color(0.0, 0.0, 1.0, 1.0);

                    let uniforms = uniform! {
                        tex: &texture,
                    };

                    target.draw(&vertex_buffer, &indices, &program, &uniforms,
                                &Default::default()).unwrap();
                    target.finish().unwrap();
                }
                glium::winit::event::WindowEvent::Resized(window_size) => {
                    display.resize(window_size.into());
                }
                _ => (),
            },
            glium::winit::event::Event::AboutToWait => {
                window.request_redraw();
            }
            _ => (),
        }
    })
        .unwrap();
}
