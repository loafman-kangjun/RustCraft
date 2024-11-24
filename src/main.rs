mod utils;

use sdl2::event::Event;
use std::time::{Duration, Instant};
use utils::gl_utils::{find_sdl_gl_driver, init_opengl, render_opengl_scene};
use utils::sdl_utils::{show_start_screen, show_declaration_page};

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("RustCraft", 800, 600)
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window
        .into_canvas()
        .index(find_sdl_gl_driver().unwrap())
        .build()
        .unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    // 在初始页面选择
    match show_start_screen(&mut canvas, &mut event_pump) {
        Some("opengl") => {
            let shader_program = init_opengl(&video_subsystem);

            let mut last_time = Instant::now();
            let mut frame_count = 0;

            'opengl_loop: loop {
                let current_time = Instant::now();
                let elapsed = current_time - last_time;
                frame_count += 1;

                if elapsed >= Duration::from_secs(1) {
                    let fps = frame_count as f32 / elapsed.as_secs_f32();
                    println!("FPS: {:.2}", fps);
                    frame_count = 0;
                    last_time = current_time;
                }

                for event in event_pump.poll_iter() {
                    if let Event::Quit { .. } = event {
                        break 'opengl_loop;
                    }
                }

                render_opengl_scene(shader_program);
                canvas.present();
            }
        }
        Some("declaration") => {
            show_declaration_page(&mut canvas, &mut event_pump);
        }
        _ => {
            println!("Exiting...");
        }
    }
}
