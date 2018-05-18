/*
Copyright (c) 2018 Matt Carnovale, Julie Rutherford-Fields, Joshua Sander
This work is available under the "3-clause ('new') BSD License”.
Please see the file LICENSE in this distribution
for license terms.
*/

use ggez::{GameResult, Context};
use ggez::graphics;
use rand::{Rng, thread_rng};


//Square size of graphic can be adjusted here
pub const SQUARE_SIZE: f32 = 20.0;

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

        //Sets the color for the object
        graphics::set_color(ctx, self.color)?;
        /* A simple 2D rectangle: the origin of the rectangle is at the top-left, 
         * with x increasing to the right and y increasing down.
         */    
        let rectangle = graphics::Rect::new(self.x, self.y, self.w, self.h);
        //Draws a rectangle. DrawMode specifies whether a shape should be drawn filled or as an outline.
       
        graphics::rectangle(ctx, graphics::DrawMode::Fill, rectangle)?;

        //This is the Gameresult type returned if there was not an error
        Ok(())
    }

}

//We can also make trucks & bikes/motorcycles
pub struct Car {
    form: Vehicle,
    speed: f32
    // lane: u32
    //For Speed & Lane, I'm imagining a "Traffic" struct that assigns the speed and lane?       
}

impl Car {

    pub fn construct(y: f32, delay: f32) -> Car {
        // let lane = 1;
        let speed = 1.0;
        let w = SQUARE_SIZE * 2 as f32;
        let h = SQUARE_SIZE * 1.5 as f32;
        let x = 0.0 - w - 10.0 - delay;
        Car {
            form: Vehicle::construct(
                x,
                y,
                w,
                h,
                //blue
                graphics::Color::new(0.0, 0.0, 1.0, 1.0),
            ),
            // lane: lane,
            speed: speed,
        }
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.form.draw(ctx)?;
        Ok(())
    }

    pub fn update(&mut self) {
        self.form.x = self.form.x % 500.0 + self.speed;
    }
}

pub struct Lane {
    y: f32,
    v_type: u32,
    speed: f32,
    num_of_cars: u32,
    cars: Vec<Car>
}

impl Lane {

    pub fn construct(&mut self, win_h: u32) -> Lane {
        Lane {
            y: win_h as f32 - 5.0 * SQUARE_SIZE,  //Will change based on lane #
            v_type: self.generate_vehicle_type(),
            speed: self.generate_speed(),
            num_of_cars: 4,                       //Should change based on speed / size
            cars: self.create_cars()
        }
    }

    fn create_cars(&mut self) -> Vec<Car>{
        let mut cars = vec![];
        let mut delay = 0.0;
        while (cars.len() as u32) < self.num_of_cars {
            cars.push(Car::construct(self.y, delay));
            delay += SQUARE_SIZE * 6.3;     
            // if (cars.len() as u32) >= 1{
            //     delay += SQUARE_SIZE * 6.3;
            //     cars.push(Car::construct(win_h, delay));
            // } else {
            //     cars.push(Car::construct(win_h, delay));
            // }
        }      
        cars
    }

    fn generate_vehicle_type(&mut self,) -> u32 {
        let mut rng = thread_rng();
        
        rng.gen_range(1,4)
    }

    fn generate_speed(&mut self,) -> f32 {
        let mut rng = thread_rng();
        
        rng.gen_range(1.0f32, 4.0f32)
    }

    pub fn draw_vehicles_in_lane(&mut self, ctx: &mut Context) -> GameResult<()> {
        for car in &mut self.cars {
            car.draw(ctx)?;
        }

        Ok(())
    }

    pub fn update_vehicles_in_lane(&mut self) {
        for car in &mut self.cars {
            car.update();
        }
    }

}

  