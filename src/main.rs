#[macro_use]
extern crate glium;
use cgmath;
use glium::Surface;
use shader::shader_pro;
use shapevec::CUBE;

mod pngconver;
mod shader;
mod shapevec;

fn main() {
    let mut rotation_angle = 0.0;
    let event_loop = glium::winit::event_loop::EventLoop::builder()
        .build()
        .expect("event loop building");
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .with_title("Glium tutorial #6")
        .build(&event_loop);

    let image = pngconver::image();
    let texture = glium::Texture2d::new(&display, image).unwrap();

    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
    let vertex_buffer = glium::VertexBuffer::new(&display, &CUBE).unwrap();

    let perspective = cgmath::perspective(cgmath::Deg(45.0), 1.0, 0.1, 100.0);
    let view = cgmath::Matrix4::look_at_rh(
        cgmath::Point3::new(1.5, 1.5, 1.5),  // 摄像机位置
        cgmath::Point3::new(0.0, 0.0, 0.0),  // 目标
        cgmath::Vector3::new(0.0, 1.0, 0.0), // 上方向
    );

    // 加载着色器文件
    let program = shader_pro(&display);

    #[allow(deprecated)]
    event_loop
        .run(move |ev, window_target| match ev {
            glium::winit::event::Event::WindowEvent { event, .. } => match event {
                glium::winit::event::WindowEvent::CloseRequested => {
                    window_target.exit();
                }
                glium::winit::event::WindowEvent::RedrawRequested => {
                    let mut target = display.draw();
                    target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);
                    let model = cgmath::Matrix4::from_angle_y(cgmath::Deg(rotation_angle));
                    let uniforms = uniform! {
                        tex: &texture,
                        perspective: Into::<[[f32; 4]; 4]>::into(perspective),
                        view: Into::<[[f32; 4]; 4]>::into(view),
                        model: Into::<[[f32; 4]; 4]>::into(model),
                    };

                    target
                        .draw(
                            &vertex_buffer,
                            &indices,
                            &program,
                            &uniforms,
                            &Default::default(),
                        )
                        .unwrap();
                    target.finish().unwrap();
                }
                glium::winit::event::WindowEvent::Resized(window_size) => {
                    display.resize(window_size.into());
                }
                _ => (),
            },
            glium::winit::event::Event::AboutToWait => {
                rotation_angle += 0.01;
                window.request_redraw();
            }
            _ => (),
        })
        .unwrap();
}
