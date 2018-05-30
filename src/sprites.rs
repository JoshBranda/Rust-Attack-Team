/*
Copyright (c) 2018 Matt Carnovale, Julie Rutherford-Fields, Joshua Sander
This work is available under the "MIT License”.
Please see the file LICENSE in this distribution
for license terms.
*/

use ggez::{GameResult, Context};
use ggez::graphics::{self, set_color, Color, DrawMode};

pub struct CrabSprite {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32
}

impl CrabSprite {
    pub fn construct(x: f32, y: f32, w: f32, h: f32) -> CrabSprite {
        CrabSprite {
            x: x,
            y: y,
            w: w,
            h: h
        }
    }

    pub fn draw(&mut self, ctx: &mut Context, ) -> GameResult<()> {
        let image_small_crab = graphics::Image::new(ctx, "/tiny_crab.png")?;
        let dest_point = graphics::Point2::new(self.x, self.y);
        graphics::draw(ctx, &image_small_crab, dest_point, 0.0)?;

        Ok(())
    }
}

pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub colour: graphics::Color    //ggez Color struct: r: f32  g: f32  b: f32  a: f32
}

impl Rectangle {
    /* Context: an object provided by ggez that holds global resources (aka state: screen, 
     * audio hardware, timers, etc.) Typically, if a function is going to interact with 
     * hardware it will need access to Context
     */
    pub fn construct(x: f32, y: f32, w: f32, h: f32, colour: Color) -> Rectangle {
        Rectangle {
            x: x,
            y: y,
            w: w,
            h: h,
            colour: colour
        }
    }

    //Draws the given object to the screen
    pub fn draw(&mut self, ctx: &mut Context, ) -> GameResult<()> {


        //Sets the color for the object
        set_color(ctx, self.colour)?;
        /* A simple 2D rectangle: the origin of the rectangle is at the top-left,
         * with x increasing to the right and y increasing down.
         */
        let rectangle = graphics::Rect::new(self.x, self.y, self.w, self.h);
        //Draws a rectangle. DrawMode specifies whether a shape should be drawn filled or as an outline.

        graphics::rectangle(ctx, DrawMode::Fill, rectangle)?;

        //This is the Gameresult type returned if there was not an error
        Ok(())
    }

}