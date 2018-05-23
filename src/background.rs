/*
Copyright (c) 2018 Matt Carnovale, Julie Rutherford-Fields, Joshua Sander
This work is available under the "MIT License”.
Please see the file LICENSE in this distribution
for license terms.
*/
extern crate ggez;

use constants::SQUARE_SIZE;
use constants::MID_R;
use constants::NUM_R;
use constants::LOGS;

use sprites::Rectangle;
use ggez::{GameResult, Context};
use ggez::graphics::{Color};

pub struct Start {
    form: Rectangle
}

pub struct Middle {
    form: Rectangle
}

pub struct River {
    form: Rectangle
}

pub struct End {
    form: Rectangle
}

impl Start {
    pub fn new(w: u32, h: u32) -> Start {
        Start {
            form: Rectangle::construct(
                0.0,
                h as f32 - 2.0 * SQUARE_SIZE,
                w as f32,
                2.0 * SQUARE_SIZE,
                Color::new(0.0, 1.0, 0.5, 0.5),
            ),
        }
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.form.draw(ctx)?;
        Ok(())
    }
}

impl Middle {
    pub fn new(w: u32, h: u32) -> Middle {
        Middle {
            form: Rectangle::construct(
                0.0,
                h as f32 - (MID_R as f32 * SQUARE_SIZE) + 1.0,
                w as f32,
                1.0 * SQUARE_SIZE,
                Color::new(0.0, 1.0, 0.5, 0.5),
            ),
        }
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.form.draw(ctx)?;
        Ok(())
    }
}

impl River {
    pub fn new(w: u32, h: u32) -> River {
        River {
            form: Rectangle::construct(
                0.0,
                h as f32 - (NUM_R as f32 - 3.0) * SQUARE_SIZE,
                w as f32,
                LOGS as f32 * SQUARE_SIZE + 1.0,
                Color::new(0.0, 0.5, 1.0, 0.5)
            ),
        }
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.form.draw(ctx)?;
        Ok(())
    }
}

impl End {
    pub fn new(w: u32, h: u32) -> End {
        End {
            form: Rectangle::construct(
                0.0,
                h as f32 - (NUM_R as f32) * SQUARE_SIZE,
                w as f32,
                SQUARE_SIZE * 3.0,
                Color::new(0.0, 1.0, 0.5, 0.5)
            ),
        }
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.form.draw(ctx)?;
        Ok(())
    }
}