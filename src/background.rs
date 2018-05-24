/*
Copyright (c) 2018 Matt Carnovale, Julie Rutherford-Fields, Joshua Sander
This work is available under the "MIT License”.
Please see the file LICENSE in this distribution
for license terms.
*/
extern crate ggez;

use constants::{SQUARE_SIZE, MID_ROW, NUM_ROW, NUM_LANE, NUM_LOG, ROAD, RIVER};
use sprites::Rectangle;
use ggez::{GameResult, Context};


pub struct Road {
    form: Rectangle
}

pub struct River {
    form: Rectangle
}

// pub struct Cubbie {
//     form: Rectangle
// }


impl Road {
    pub fn new(w: u32, h: u32) -> Road {
        Road {
            form: Rectangle::construct(
                0.0,
                h as f32 - (MID_ROW - 1) as f32 * SQUARE_SIZE,
                w as f32,
                NUM_LANE as f32 * SQUARE_SIZE,
                ROAD,
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
                h as f32 - (NUM_ROW as f32 - 3.0) * SQUARE_SIZE,
                w as f32,
                NUM_LOG as f32 * SQUARE_SIZE + 1.0,
                RIVER,
            ),
        }
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.form.draw(ctx)?;
        Ok(())
    }
}

