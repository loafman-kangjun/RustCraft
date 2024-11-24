use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;

pub fn show_declaration_page(canvas: &mut Canvas<Window>, event_pump: &mut EventPump) {
    loop {
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();

        // 绘制声明内容
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas
            .draw_rect(sdl2::rect::Rect::new(100, 100, 600, 400))
            .unwrap();

        // 可以在这里加入文字渲染（需要额外的字体支持）

        canvas.present();

        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => return,
                sdl2::event::Event::KeyDown {
                    keycode: Some(sdl2::keyboard::Keycode::Escape),
                    ..
                } => {
                    return; // 按下 ESC 键退出声明页面
                }
                _ => {}
            }
        }
    }
}
