/*
Copyright (c) 2018 Matt Carnovale, Julie Rutherford-Fields, Joshua Sander
This work is available under the "MIT License”.
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
    v_type: u32,   //This will allow us to choose different sprites for Cars or Trucks
    speed: f32,
    num_of_cars: u32,
    cars: Vec<Car>
}

impl Lane {

    pub fn construct(win_h: u32) -> Lane {
        let y = win_h as f32 - 5.0 * SQUARE_SIZE;  //Will change based on lane #
        let num_of_cars= 4; //Should change based on speed / size
        Lane {
            v_type: Lane::generate_vehicle_type(),
            speed: Lane::generate_speed(),
            num_of_cars,                       
            cars: Lane::create_cars(y, num_of_cars)
        }
    }

    fn create_cars(y: f32, num_of_cars: u32) -> Vec<Car>{
        let mut cars = vec![];
        let mut delay = 0.0;
        while (cars.len() as u32) < num_of_cars {
            cars.push(Car::construct(y, delay));
            delay += SQUARE_SIZE * 6.3;     
        }      
        cars
    }

    fn generate_vehicle_type() -> u32 {
        let mut rng = thread_rng();
        
        rng.gen_range(1,4)
    }

    fn generate_speed() -> f32 {
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

  