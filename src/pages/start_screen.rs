use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;

pub fn show_start_screen(
    canvas: &mut Canvas<Window>,
    event_pump: &mut EventPump,
) -> Option<&'static str> {
    loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        // 绘制按钮1：OpenGL 页面
        canvas.set_draw_color(Color::RGB(255, 0, 0));
        canvas
            .fill_rect(sdl2::rect::Rect::new(200, 200, 200, 100))
            .unwrap();

        // 绘制按钮2：声明页面
        canvas.set_draw_color(Color::RGB(0, 255, 0));
        canvas
            .fill_rect(sdl2::rect::Rect::new(400, 200, 200, 100))
            .unwrap();

        canvas.present();

        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => return None,
                sdl2::event::Event::MouseButtonDown { x, y, .. } => {
                    // 检测点击按钮1
                    if x >= 200 && x <= 400 && y >= 200 && y <= 300 {
                        return Some("opengl");
                    }
                    // 检测点击按钮2
                    if x >= 400 && x <= 600 && y >= 200 && y <= 300 {
                        return Some("declaration");
                    }
                }
                _ => {}
            }
        }
    }
}
