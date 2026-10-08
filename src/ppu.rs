use rand::RngExt;
use sdl2::{
    Sdl, event::Event, keyboard::Keycode, pixels::Color, rect::Point, render::WindowCanvas,
};
use std::time::Duration;

// TODO - maybe change this from usize to u8?
const WIDTH: usize = 160;
const HEIGHT: usize = 144;
const SCALE: usize = 6;

pub(crate) struct Screen {
    pixels: [[u8; WIDTH]; HEIGHT],
    sdl_context: Sdl,
    canvas: WindowCanvas,
}

impl Screen {
    pub fn new() -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let real_width = (WIDTH * SCALE) as u32;
        let real_height = (HEIGHT * SCALE) as u32;

        let window = video_subsystem
            .window("Lobsters", real_width, real_height)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();
        canvas.set_scale(SCALE as f32, SCALE as f32).unwrap();

        Screen {
            pixels: [[0; WIDTH as usize]; HEIGHT as usize],
            sdl_context,
            canvas,
        }
    }

    pub fn randomize(&mut self) {
        let mut rng = rand::rng();
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                self.pixels[y][x] = rng.random_range(0..4);
            }
        }
    }

    pub fn draw(&mut self) {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let color = Self::shade_to_color(self.pixels[y][x]);
                self.canvas.set_draw_color(color);
                self.canvas
                    .draw_point(Point::new(x as i32, y as i32))
                    .unwrap();
            }
        }
        self.canvas.present();
    }

    pub fn render_loop(&mut self) {
        let mut event_pump = self.sdl_context.event_pump().unwrap();
        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    _ => {}
                }
            }
            self.randomize();
            self.draw();
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 10));
        }
    }

    fn shade_to_color(value: u8) -> Color {
        match value {
            0 => Color::RGB(255, 255, 255),
            1 => Color::RGB(192, 192, 192),
            2 => Color::RGB(96, 96, 96),
            _ => Color::RGB(0, 0, 0),
        }
    }
}
