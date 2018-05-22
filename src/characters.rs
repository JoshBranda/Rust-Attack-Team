/*
Copyright (c) 2018 Matt Carnovale, Julie Rutherford-Fields, Joshua Sander
This work is available under the "MIT License”.
Please see the file LICENSE in this distribution
for license terms.
*/
extern crate ggez;

use constants::SQUARE_SIZE;
use sprites::Rectangle;
use ggez::{GameResult, Context};
use ggez::graphics::{Color};

pub struct Crab {
    form: Rectangle,
    win_w: f32,
    win_h: f32,
}

impl Crab {
    pub fn new(w: u32, h: u32) -> Crab {
        Crab {
            form: Rectangle::construct(
                5.0 * SQUARE_SIZE,
                h as f32 - 1.0 * SQUARE_SIZE,
                SQUARE_SIZE,
                SQUARE_SIZE,
                Color::new(0.0, 1.0, 0.0, 1.0),
            ),
            win_w: w as f32,
            win_h: h as f32
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

//Unit tests for Crab functions.  All paths are tested except for draw
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn move_left_succeeds_with_space() {
        let multiplier: u32 = 2;
        let mut crab = Crab::new((SQUARE_SIZE as u32) * multiplier, (SQUARE_SIZE as u32) * multiplier);

        let starting_x = crab.form.x;
        crab.move_left();

        assert_ne!(crab.form.x, starting_x);
        assert_eq!(crab.form.x, starting_x - SQUARE_SIZE);
    }

    #[test]
    fn move_left_fails_without_space() {
        let multiplier: u32 = 2;
        let mut crab = Crab::new((SQUARE_SIZE as u32) * multiplier, (SQUARE_SIZE as u32) * multiplier);

        crab.form.x = SQUARE_SIZE - 1.0;
        let starting_x = crab.form.x;
        crab.move_left();

        assert_eq!(crab.form.x, starting_x);
    }

    #[test]
    fn move_right_succeeds_with_space() {
        let multiplier: u32 = 7;
        let mut crab = Crab::new((SQUARE_SIZE as u32) * multiplier, (SQUARE_SIZE as u32) * multiplier);

        let starting_x = crab.form.x;
        crab.move_right();

        assert_ne!(crab.form.x, starting_x);
        assert_eq!(crab.form.x, starting_x + SQUARE_SIZE);
    }

    #[test]
    fn move_right_fails_without_space() {
        let multiplier: u32 = 6;
        let mut crab = Crab::new((SQUARE_SIZE as u32) * multiplier, (SQUARE_SIZE as u32) * multiplier);

        let starting_x = crab.form.x;
        crab.move_right();

        assert_eq!(crab.form.x, starting_x);
    }

    #[test]
    fn move_down_succeeds_with_space() {
        let multiplier: u32 = 7;
        let mut crab = Crab::new((SQUARE_SIZE as u32) * multiplier, (SQUARE_SIZE as u32) * multiplier);

        crab.form.y = crab.form.y - SQUARE_SIZE;
        let starting_y = crab.form.y;
        crab.move_down();

        assert_ne!(crab.form.y, starting_y);
        assert_eq!(crab.form.y, starting_y + SQUARE_SIZE);
    }

    #[test]
    fn move_down_fails_without_space() {
        let multiplier: u32 = 2;
        let mut crab = Crab::new((SQUARE_SIZE as u32) * multiplier, (SQUARE_SIZE as u32) * multiplier);


        let starting_y = crab.form.y;
        crab.move_down();

        assert_eq!(crab.form.y, starting_y);
    }

    #[test]
    fn move_up_succeeds_with_space() {
        let multiplier: u32 = 2;
        let mut crab = Crab::new((SQUARE_SIZE as u32) * multiplier, (SQUARE_SIZE as u32) * multiplier);

        let starting_y = crab.form.y;
        crab.move_up();

        assert_ne!(crab.form.y, starting_y);
        assert_eq!(crab.form.y, starting_y - SQUARE_SIZE);
    }

    #[test]
    fn move_up_fails_without_space() {
        let multiplier: u32 = 2;
        let mut crab = Crab::new((SQUARE_SIZE as u32) * multiplier, (SQUARE_SIZE as u32) * multiplier);

        crab.form.y = 0 as f32;
        let starting_y = crab.form.y;
        crab.move_up();

        assert_eq!(crab.form.y, starting_y);
    }

}