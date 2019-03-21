use sdl2::{self, pixels::Color};
use sdl2::pixels;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::chip8::{C8_HEIGHT, C8_WIDTH};

const SCALE: u32 = 8;

const SCREEN_WIDTH: u32 = C8_WIDTH as u32 * SCALE;
const SCREEN_HEIGHT: u32 = C8_HEIGHT as u32 * SCALE;

pub struct Display {
    canvas: Canvas<Window>,
}

impl Display {
    pub fn new(context: &sdl2::Sdl) -> Self {
        let video_subsystem = context.video().unwrap();
        let window = video_subsystem
            .window("chitta", SCREEN_WIDTH, SCREEN_HEIGHT)
            .position_centered()
            .vulkan()
            .build()
            .unwrap();

        let mut canvas = window
            .into_canvas()
            .build()
            .unwrap();

        canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));

        canvas.clear();
        canvas.present();

        Display{canvas}
    }

    pub fn draw(&mut self, pixels: &[[bool; C8_WIDTH]; C8_HEIGHT]) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();

        self.canvas.set_draw_color(Color::RGB(255, 255, 255));

        for (y, row) in pixels.iter().enumerate() {
            for (x, &pixel) in row.iter().enumerate() {
                if pixel {
                    let x = (x as u32 * SCALE) as i32;
                    let y = (y as u32 * SCALE) as i32;
                    let rect = Rect::new(x, y, SCALE, SCALE);
                    self.canvas.fill_rect(rect).unwrap();
                }
            }
        }
        self.canvas.present();
    }
}