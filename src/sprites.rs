/*
Copyright (c) 2018 Matt Carnovale, Julie Rutherford-Fields, Joshua Sander
This work is available under the "MIT License”.
Please see the file LICENSE in this distribution
for license terms.
*/

use ggez::{GameResult, Context};
use ggez::graphics::{self, set_color, Color, DrawMode};


/// Represents the crab sprite graphic
pub struct CrabSprite {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32
}

/// Implements a new CrabSprite
impl CrabSprite {
    /// Constructs a new CrabSprite
    pub fn construct(x: f32, y: f32, w: f32, h: f32) -> CrabSprite {
        CrabSprite {
            x: x,
            y: y,
            w: w,
            h: h
        }
    }

    /// Draws CrabSprite graphic onscreen
    pub fn draw(&mut self, ctx: &mut Context, ) -> GameResult<()> {
        let image_small_crab = graphics::Image::new(ctx, "/tiny_crab.png")?;
        let dest_point = graphics::Point2::new(self.x, self.y);
        graphics::draw(ctx, &image_small_crab, dest_point, 0.0)?;

        Ok(())
    }
}


/// Represents a Rectangle object
pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub colour: graphics::Color    //ggez Color struct: r: f32  g: f32  b: f32  a: f32
}

/// Implements a Rectangle object
impl Rectangle {

    /// Constructs a Rectangle object 
    pub fn construct(x: f32, y: f32, w: f32, h: f32, colour: Color) -> Rectangle {
        Rectangle {
            x: x,
            y: y,
            w: w,
            h: h,
            colour: colour
        }
    }

    /// Draws the Rectangle object to the screen
    pub fn draw(&mut self, ctx: &mut Context, ) -> GameResult<()> {


        /// Sets the color for the object
        set_color(ctx, self.colour)?;

        let rectangle = graphics::Rect::new(self.x, self.y, self.w, self.h);
        
        /// Draws a rectangle. DrawMode specifies whether
        /// a shape should be drawn filled or as an outline.
        graphics::rectangle(ctx, DrawMode::Fill, rectangle)?;

        Ok(())
    }
}
