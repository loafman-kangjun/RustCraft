extern crate sdl2;

use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::time::Duration;
use tokio::task;

// 按钮状态
struct Button {
    rect: Rect,
    color: Color,
    hover_color: Color,
    label: &'static str,
    is_hovered: bool,
}

impl Button {
    fn new(rect: Rect, color: Color, hover_color: Color, label: &'static str) -> Self {
        Button {
            rect,
            color,
            hover_color,
            label,
            is_hovered: false,
        }
    }

    fn draw(&self, canvas: &mut Canvas<Window>, font_color: Color) {
        let color = if self.is_hovered {
            self.hover_color
        } else {
            self.color
        };
        canvas.set_draw_color(color);
        canvas.fill_rect(self.rect).unwrap();
        // TODO: 添加文本绘制（需要额外的字体支持库）
    }

    fn check_hover(&mut self, x: i32, y: i32) {
        self.is_hovered = self.rect.contains_point((x, y));
    }
}

#[tokio::main]
async fn main() {
    // 初始化 SDL2
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Game Start Screen", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    // 定义按钮
    let mut start_button = Button::new(
        Rect::new(300, 250, 200, 50),
        Color::RGB(0, 128, 255),
        Color::RGB(0, 200, 255),
        "Start",
    );

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut running = true;

    // 异步加载资源任务（模拟）
    let loading_task = task::spawn(async {
        println!("Loading resources...");
        tokio::time::sleep(Duration::from_secs(3)).await;
        println!("Resources loaded!");
    });

    while running {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(sdl2::keyboard::Keycode::Escape),
                    ..
                } => {
                    running = false;
                }
                Event::MouseMotion { x, y, .. } => {
                    start_button.check_hover(x, y);
                }
                Event::MouseButtonDown { x, y, .. } => {
                    if start_button.rect.contains_point((x, y)) {
                        println!("Start button clicked!");
                        running = false;
                    }
                }
                _ => {}
            }
        }

        // 清屏
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        // 绘制按钮
        start_button.draw(&mut canvas, Color::RGB(255, 255, 255));

        // 显示更新
        canvas.present();

        // 模拟帧率控制
        std::thread::sleep(Duration::from_millis(16));
    }

    // 等待异步任务完成
    loading_task.await.unwrap();
}
