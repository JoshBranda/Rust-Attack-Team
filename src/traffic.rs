/*
Copyright (c) 2018 Matt Carnovale, Julie Rutherford-Fields, Joshua Sander
This work is available under the "3-clause ('new') BSD License”.
Please see the file LICENSE in this distribution
for license terms.
*/
use ggez::{GameResult, Context};
use ggez::graphics;

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
    fn construct(context: &mut Context, x: f32, y: f32, w: f32, h: f32, color: graphics::Color) -> Vehicle {
        Vehicle {
            x: x,
            y: y,
            w: w,
            h: h,
            color: color
        }
    }

    //Draws the given object to the screen
    fn draw(&mut self, context: &mut Context, ) -> GameResult<()> {

        /* A simple 2D rectangle: the origin of the rectangle is at the top-left, 
         * with x increasing to the right and y increasing down.
         */    
        let rectangle = graphics::Rect::new(self.x, self.y, self.w, self.h);
        //Draws a rectangle. DrawMode specifies whether a shape should be drawn filled or as an outline.
        //TODO: add try/catch block
        let _is_ok = graphics::rectangle(context, graphics::DrawMode::Fill, rectangle);
        //Sets the foreground color
        let is_ok = graphics::set_color(context, self.color);

        //This is the Gameresult type returned if there was not an error
        is_ok
    }

}
  