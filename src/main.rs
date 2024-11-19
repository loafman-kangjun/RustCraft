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

    let draw_params = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::draw_parameters::DepthTest::IfLess, // 如果当前像素比已有深度更近，则绘制
            write: true,                                     // 写入深度缓冲区
            ..Default::default()
        },
        ..Default::default()
    };

    window.set_cursor_visible(false);

    // window
    //     .set_cursor_grab(glium::winit::window::CursorGrabMode::Confined)
    //     .unwrap();

    struct Camera {
        position: cgmath::Point3<f32>,
        yaw: f32,   // 水平方向
        pitch: f32, // 垂直方向
    }

    impl Camera {
        fn new(position: cgmath::Point3<f32>, yaw: f32, pitch: f32) -> Self {
            Camera {
                position,
                yaw,
                pitch,
            }
        }

        fn get_view_matrix(&self) -> cgmath::Matrix4<f32> {
            let forward = cgmath::Vector3 {
                x: self.yaw.cos() * self.pitch.cos(),
                y: self.pitch.sin(),
                z: self.yaw.sin() * self.pitch.cos(),
            };
            let target = cgmath::Point3::new(0.0, 0.0, 0.0) + forward;
            let up = cgmath::Vector3::new(0.0, 1.0, 0.0);
            println!("{:?}", target);
            cgmath::Matrix4::look_at_rh(self.position, target, up)
           
            // cgmath::Matrix4::look_at_rh(
            //     cgmath::Point3::new(1.5, 1.5, 1.5),  // 摄像机位置
            //     cgmath::Point3::new(0.0, 0.0, 0.0),  // 目标
            //     cgmath::Vector3::new(0.0, 1.0, 0.0), // 上方向
            // )
        }
    }

    // 加载着色器文件
    let program = shader_pro(&display);

    let mut camera = Camera::new(cgmath::Point3::new(1.5, 1.5, 1.5), 0.0, 0.0);
    let mut last_mouse_position = glium::winit::dpi::PhysicalPosition::new(400.0, 300.0);

    #[allow(deprecated)]
    event_loop
        .run(move |ev, window_target| match ev {
            glium::winit::event::Event::WindowEvent { event, .. } => match event {
                glium::winit::event::WindowEvent::CloseRequested => {
                    window_target.exit();
                }
                glium::winit::event::WindowEvent::CursorMoved { position, .. } => {
                    let delta_x = position.x as f32 - last_mouse_position.x as f32;
                    let delta_y = position.y as f32 - last_mouse_position.y as f32;

                    // 调整摄像头方向
                    let sensitivity = 0.002;
                    camera.yaw += delta_x * sensitivity;
                    camera.pitch -= delta_y * sensitivity;

                    // 限制 pitch 范围
                    camera.pitch = camera.pitch.clamp(
                        -std::f32::consts::FRAC_PI_2 + 0.1,
                        std::f32::consts::FRAC_PI_2 - 0.1,
                    );

                    last_mouse_position = position;
                }
                glium::winit::event::WindowEvent::RedrawRequested => {
                    let mut target = display.draw();
                    target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);
                    let window_size = window.inner_size();
                    let aspect_ratio = window_size.width as f32 / window_size.height as f32;
                    let perspective =
                        cgmath::perspective(cgmath::Deg(45.0), aspect_ratio, 0.1, 100.0);
                    let model = cgmath::Matrix4::from_angle_y(cgmath::Deg(rotation_angle));
                    let view = camera.get_view_matrix();
                    let uniforms = uniform! {
                        tex: &texture,
                        perspective: Into::<[[f32; 4]; 4]>::into(perspective),
                        view: Into::<[[f32; 4]; 4]>::into(view),
                        model: Into::<[[f32; 4]; 4]>::into(model),
                    };

                    target
                        .draw(&vertex_buffer, &indices, &program, &uniforms, &draw_params)
                        .unwrap();
                    target.finish().unwrap();
                }
                glium::winit::event::WindowEvent::Resized(window_size) => {
                    display.resize(window_size.into());
                    window.request_redraw();
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
