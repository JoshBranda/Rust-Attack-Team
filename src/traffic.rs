/*
Copyright (c) 2018 Matt Carnovale, Julie Rutherford-Fields, Joshua Sander
This work is available under the "3-clause ('new') BSD License”.
Please see the file LICENSE in this distribution
for license terms.
*/

use ggez::{GameResult, Context};
use ggez::graphics;


//Square size of graphic can be adjusted here
const SQUARE_SIZE: f32 = 20.0;

//The vehicles will be a variety of rectangles until we find some sprites
struct Vehicle {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    color: graphics::Color    //ggez Color struct: r: f32  g: f32  b: f32  a: f32
}

impl Vehicle {
    /* Context: an object provided by ggez that holds global resources (aka state: screen, 
     * audio hardware, timers, etc.) Typically, if a function is going to interact with 
     * hardware it will need access to Context
     */
    fn construct(x: f32, y: f32, w: f32, h: f32, color: graphics::Color) -> Vehicle {
        Vehicle {
            x: x,
            y: y,
            w: w,
            h: h,
            color: color
        }
    }

    //Draws the given object to the screen
    fn draw(&mut self, ctx: &mut Context, ) -> GameResult<()> {

        /* A simple 2D rectangle: the origin of the rectangle is at the top-left, 
         * with x increasing to the right and y increasing down.
         */    
        let rectangle = graphics::Rect::new(self.x, self.y, self.w, self.h);
        //Draws a rectangle. DrawMode specifies whether a shape should be drawn filled or as an outline.
        //TODO: add try/catch block
        let _is_ok = graphics::rectangle(ctx, graphics::DrawMode::Fill, rectangle);
        //Sets the foreground color
        let is_ok = graphics::set_color(ctx, self.color);

        //This is the Gameresult type returned if there was not an error
        is_ok
    }

}

//We can also make trucks & bikes/motorcycles
pub struct Car {
    form: Vehicle,
    speed: f32,
    lane: u32
    //For Speed & Lane, I'm imagining a "Traffic" struct that assigns the speed and lane?       
}

impl Car {

    pub fn construct(win_h: u32) -> Car {
        let lane = 1;
        let speed = 1.2;
        let w = SQUARE_SIZE * 2 as f32;
        let h = SQUARE_SIZE * 1.5 as f32;
        let x = 0.0 - w - 10.0;
        let y = win_h as f32 - (lane as f32 + 4.0) * SQUARE_SIZE;
        Car {
            form: Vehicle::construct(
                x,
                y,
                w,
                h,
                //blue
                graphics::Color::new(0.0, 0.0, 1.0, 1.0),
            ),
            lane: lane,
            speed: speed,
        }
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.form.draw(ctx)?;
        Ok(())
    }

    pub fn update(&mut self) {
        self.form.x = self.form.x % 400.0 + self.speed;
    }
}

  