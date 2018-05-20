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
    speed: f32,
    delay: f32,
    direction: bool
}

impl Car {

    pub fn construct(y: f32, win_w: u32, delay: f32, ltr_direction: bool) -> Car {
        let speed = 1.0;
        let w = SQUARE_SIZE * 2 as f32;
        let h = SQUARE_SIZE * 1.5 as f32;
        let x = Car::assign_starting_x(ltr_direction, w, win_w, delay);
        Car {
            form: Vehicle::construct(
                x,
                y,
                w,
                h,
                Car::assign_color()
            ),
            speed,
            delay,
            direction: ltr_direction
        }
    }

    fn assign_starting_x(ltr_direction: bool, w: f32, win_w: u32, delay: f32) -> f32 {      
       match ltr_direction {
           true => 0.0 - w - 10.0 - delay,
           false => win_w as f32 - 10.0 - delay
       }
    }

    fn assign_color() -> graphics::Color {
        let mut rng = thread_rng();
        let color: u32 = rng.gen_range(0,99999) % 7;

        match color {
            0 => graphics::Color::new(0.0, 0.0, 1.0, 1.0), 
            1 => graphics::Color::new(1.0, 0.0, 0.0, 1.0), 
            2 => graphics::Color::new(0.0, 1.0, 0.0, 1.0),  
            3 => graphics::Color::new(1.0, 1.0, 0.0, 1.0),  
            4 => graphics::Color::new(0.0, 1.0, 1.0, 1.0),  
            5 => graphics::Color::new(1.0, 0.0, 1.0, 1.0), 
            _ => graphics::Color::new(1.0, 0.5, 1.0, 0.5) 
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.form.draw(ctx)?;
        Ok(())
    }

    fn update(&mut self) {
        if self.direction {
            if self.form.x == 410.0 { self.form.x = 0.0 - self.form.w - 10.0}
            self.form.x = self.form.x + self.speed;
        } else{
            if self.form.x == -self.form.w { self.form.x = 400.0 - 10.0}
            self.form.x = self.form.x - self.speed;
        }
    }
}

pub struct Lane {
    v_type: u32,   //This will allow us to choose different sprites for Cars or Trucks
    speed: f32,
    num_of_cars: u32,
    cars: Vec<Car>
}

impl Lane {

    pub fn construct(win_h: u32, win_w: u32, y_modifier: f32) -> Lane {
        let y = win_h as f32 - y_modifier * SQUARE_SIZE;  //Will change based on lane #
        let num_of_cars= 4; //Should change based on speed / size
        let ltr_direction= Lane::generate_direction();
        Lane {
            v_type: Lane::generate_vehicle_type(),
            speed: Lane::generate_speed(),
            num_of_cars,                       
            cars: Lane::create_cars(y, win_w, num_of_cars, ltr_direction)
        }
    }

    fn create_cars(y: f32, win_w: u32, num_of_cars: u32, ltr_direction: bool) -> Vec<Car>{
        let mut cars = vec![];
        let mut delay = 0.0;
        while (cars.len() as u32) < num_of_cars {
            cars.push(Car::construct(y, win_w, delay, ltr_direction));
            delay += SQUARE_SIZE * 4.5;     
        }      
        cars
    }

    fn generate_vehicle_type() -> u32 {
        let mut rng = thread_rng();
        
        rng.gen_range(1,4)
    }

    fn generate_speed() -> f32 {
        let mut rng = thread_rng();
        
        rng.gen_range(1.0_f32, 4.0_f32)
    }

    fn generate_direction() -> bool {
        let mut rng = thread_rng();
        let six_sided_die: u32 = rng.gen_range(0,99999) % 6;
        match six_sided_die {
            0 => false,
            1 => true,
            2 => false,
            3 => false,
            4 => true,
            _ => true
        }
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

  