/*
Copyright (c) 2018 Matt Carnovale, Julie Rutherford-Fields, Joshua Sander
This work is available under the "MIT License”.
Please see the file LICENSE in this distribution
for license terms.
*/

extern crate ggez;

use ggez::{GameResult, Context};
use ggez::graphics::{self, set_color, Color, DrawMode, Point2};

pub struct Square {
    pub x: f32,
    pub y: f32,
    width: f32,
    height: f32,
    colour: Color,
}

impl Square {
    pub fn new(_ctx: &mut Context, x: f32, y: f32, width: f32, height: f32, colour: Color) -> Square {
        Square {
            x: x,
            y: y,
            width: width,
            height: height,
            colour: colour,
        }
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        set_color(ctx, self.colour)?;
        let square = graphics::Rect::new(self.x, self.y, self.width, self.height);
        graphics::rectangle(ctx, DrawMode::Fill, square)?;
        Ok(())
    }
}
