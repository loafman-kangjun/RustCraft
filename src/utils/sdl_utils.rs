use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub fn show_start_screen(canvas: &mut Canvas<Window>, event_pump: &mut sdl2::EventPump) -> bool {
    let mut running = true;
    let mut start_game = false;

    while running {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => running = false,
                sdl2::event::Event::MouseButtonDown { x, y, .. } => {
                    if Rect::new(300, 250, 200, 50).contains_point((x, y)) {
                        start_game = true;
                        running = false;
                    }
                }
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        canvas.set_draw_color(Color::RGB(0, 128, 255));
        canvas.fill_rect(Rect::new(300, 250, 200, 50)).unwrap();

        canvas.present();
    }

    start_game
}
