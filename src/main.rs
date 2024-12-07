mod pages;
mod renderloop;

use pages::declaration_page::show_declaration_page;
use pages::opengl_page::run_opengl_page;
use pages::start_screen::show_start_screen;
use renderloop::utils::find_gl;

#[tokio::main]
async fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("RustCraft", 800, 600)
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window
        .into_canvas()
        .index(find_gl().unwrap())
        .build()
        .unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    // 显示初始页面并根据用户选择进入对应页面
    match show_start_screen(&mut canvas, &mut event_pump) {
        Some("opengl") => run_opengl_page(&video_subsystem, &mut canvas, &mut event_pump).await,
        Some("declaration") => show_declaration_page(&mut canvas, &mut event_pump),
        _ => {
            println!("Exiting...");
        }
    }
}
