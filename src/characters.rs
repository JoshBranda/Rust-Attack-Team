extern crate ggez;

use sprites::Square;
use ggez::{GameResult, Context};
use ggez::graphics::{self, set_color, Color, DrawMode, Point2};

//Square size of graphic can be adjusted here
const SQUARE_SIZE: f32 = 20.0;

pub struct Crab {
    form: Square,
    win_w: f32,
    win_h: f32,
}

impl Crab {
    pub fn new(ctx: &mut Context, WIN_W: u32, WIN_H: u32) -> Crab {
        Crab {
            form: Square::new(
                ctx,
                5.0 * SQUARE_SIZE,
                WIN_H as f32 - 1.0 * SQUARE_SIZE,
                SQUARE_SIZE,
                SQUARE_SIZE,
                Color::new(0.0, 1.0, 0.0, 1.0),
            ),
            win_w: WIN_W as f32,
            win_h: WIN_H as f32
        }
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.form.draw(ctx)?;
        Ok(())
    }

    pub fn move_up(&mut self) {
        if self.form.y - SQUARE_SIZE + 1.0 > 0.0 {
            self.form.y -= SQUARE_SIZE;
        }
    }
    pub fn move_down(&mut self) {
        if self.form.y + SQUARE_SIZE < self.win_h {
            self.form.y += SQUARE_SIZE;
        }
    }
    pub fn move_right(&mut self) {
        if self.form.x + SQUARE_SIZE - 1.0 < self.win_w - SQUARE_SIZE {
            self.form.x += SQUARE_SIZE;
        }
    }
    pub fn move_left(&mut self) {
        if self.form.x - SQUARE_SIZE + 1.0 > 0.0 {
            self.form.x -= SQUARE_SIZE;
        }
    }
}
