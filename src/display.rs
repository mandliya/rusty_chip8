use sdl2::pixels;
use sdl2::rect::Rect;
use sdl2;
use sdl2::render::Canvas;
use sdl2::video::Window;
use consts::SCALE;
use consts::SCREEN_WIDTH;
use consts::SCREEN_HEIGHT;
use consts::TITLE;

pub struct Display {
    gfx: [[u8; SCREEN_WIDTH as usize]; SCREEN_HEIGHT as usize],
    draw_flag: bool,
    screen: Canvas<Window>
}

impl Display {
    pub fn new(sdl_context: &sdl2::Sdl) -> Self {

        let video_subsystem = sdl_context.video()
            .expect("Fatal error: Failed to get SDL2 video subsytem.");

        let window = video_subsystem.window(
            TITLE,
            (SCREEN_WIDTH * SCALE) as u32,
            (SCREEN_HEIGHT * SCALE) as u32)
            .position_centered()
            .opengl()
            .build()
            .expect("Fatal error: Failed to get SDL2 window");

        let mut canvas = window.into_canvas()
            .build()
            .expect("Fatal error: Failed to convert window to canvas");

        canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();

        Display{
            gfx: [[0; SCREEN_WIDTH as usize]; SCREEN_HEIGHT as usize],
            draw_flag: true,
            screen: canvas,
        }
    }

    pub fn clear(&mut self) {
        self.gfx = [[0; 64]; 32];
        self.draw_flag = true;
        self.screen.clear();
    }

    pub fn draw(&mut self, x: usize, y: usize, sprite: &[u8]) -> u8 {
        let mut collision = 0u8;
        let n = sprite.len() as usize;
        let mut yj: usize;
        let mut xi: usize;

        for j in 0..n {
            for i in 0..8 {
                yj = (y + j) % SCREEN_HEIGHT as usize;
                xi = (x + i) % SCREEN_WIDTH as usize;
                if (sprite[j] & (0x80 >> i)) != 0 {
                    if self.gfx[yj][xi] == 1 {
                         collision = 1
                    }
                    self.gfx[yj][xi] ^= 1;
                }
            }
        }
        self.draw_flag = true;
        collision
    }

    pub fn draw_screen(&mut self) {
        if !self.draw_flag {
            return
        }
        for (y, row) in self.gfx.iter().enumerate() {
            for (x, &col) in row.iter().enumerate() {
                let x = (x as u32) * SCALE;
                let y = (y as u32) * SCALE;

                self.screen.set_draw_color(color(col));
                let _ = self.screen.fill_rect(
                    Rect::new(x as i32,y as i32, SCALE, SCALE)
                );
            }
        }
        self.screen.present();
        self.draw_flag = false;
    }
}
 fn color(value: u8) -> pixels::Color {
    if value == 0 {
        pixels::Color::RGB(0, 0, 0)
    } else {
        pixels::Color::RGB(0, 250, 0)
    }
}