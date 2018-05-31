/*
Copyright (c) 2018 Matt Carnovale, Julie Rutherford-Fields, Joshua Sander
This work is available under the "MIT License”.
Please see the file LICENSE in this distribution
for license terms.
*/

use constants::{
    CAR_W,
    MAX_NUM_OF_CARS,
    MAX_NUM_OF_TRUCKS,
    MAX_SPEED_OF_OBSTACLES, 
    MIN_DELAY,
    MAX_DELAY,
    SQUARE_SIZE,
    TRUCK_W, 
    WIN_H,
    WIN_W };

use ggez::{GameResult, Context};
use ggez::graphics::{Color};
use rand::{Rng, thread_rng};
use sprites::Rectangle;

//We can also make trucks & bikes/motorcycles
pub struct Vehicle {
    form: Rectangle,
    speed: f32,
    direction: bool
}

impl Vehicle {

    pub fn construct(w: f32, y: f32, speed: f32, delay: f32, ltr_direction: bool) -> Vehicle {
        // let w = w;
        let h = SQUARE_SIZE as f32;
        let x = Vehicle::assign_starting_x(ltr_direction, w, delay);
        Vehicle {
            form: Rectangle::construct(
                x,
                y,
                w,
                h,
                Vehicle::assign_color()
            ),
            speed,
            direction: ltr_direction
        }
    }

    fn assign_starting_x(ltr_direction: bool, w: f32, delay: f32) -> f32 {      
       match ltr_direction {
           true => 0.0 - w - 10.0 - delay,
           false => WIN_W as f32 - 10.0 - delay
       }
    }

    fn assign_color() -> Color {
        let mut rng = thread_rng();
        let color: u32 = rng.gen_range(0,99999) % 7;

        match color {
            0 => Color::new(0.0, 0.0, 1.0, 1.0), 
            1 => Color::new(1.0, 0.0, 0.0, 1.0), 
            2 => Color::new(0.0, 1.0, 0.0, 1.0),  
            3 => Color::new(1.0, 1.0, 0.0, 1.0),  
            4 => Color::new(0.0, 1.0, 1.0, 1.0),  
            5 => Color::new(1.0, 0.0, 1.0, 1.0), 
            _ => Color::new(1.0, 0.5, 1.0, 0.5) 
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.form.draw(ctx)?;
        Ok(())
    }

    fn update(&mut self) {
        if self.direction {
            if self.form.x >= WIN_W as f32 + 10.0 { self.form.x = 0.0 - self.form.w - 10.0}
            self.form.x = self.form.x + self.speed;
        } else{
            if self.form.x <= -self.form.w { self.form.x = WIN_W as f32 - 10.0}
            self.form.x = self.form.x - self.speed;
        }
    }

    pub fn get_left_edge(&mut self) -> f32 {
        return self.form.x;
    }

    pub fn get_right_edge(&mut self) -> f32 {
        return self.form.x + self.form.w;
    }

    pub fn get_bottom_edge(&mut self) -> f32 {
        return self.form.y;
    }

    pub fn get_top_edge(&mut self) -> f32 {
        return self.form.y - self.form.h;
    }
}

pub struct Lane {
    pub vehicles: Vec<Vehicle>
}

impl Lane {

    pub fn construct(y_modifier: f32) -> Lane {
        let y = WIN_H as f32 - y_modifier * SQUARE_SIZE;  //Will change based on lane #
        let vehicle_type= Lane::generate_vehicle_type();                     
        let num_of_vehicles= Lane::generate_number_of_vehicles(vehicle_type); //Should change based on speed / size
        let ltr_direction= Lane::generate_direction();
        let speed= Lane::generate_speed();
        Lane {
            vehicles: Lane::create_vehicle(vehicle_type, y, num_of_vehicles, speed, ltr_direction)
        }
    }

    fn create_vehicle(vehicle_type: u32, y: f32, num_of_vehicles: u32, 
                      speed: f32, ltr_direction: bool) -> Vec<Vehicle>{
        match vehicle_type {
            0 => Lane::create_trucks(y, num_of_vehicles, speed, ltr_direction),
            _ => Lane::create_cars(y, num_of_vehicles, speed, ltr_direction)
        }
    }

    fn create_trucks(y: f32, num_of_trucks: u32, speed: f32, ltr_direction: bool) -> Vec<Vehicle>{
        let mut trucks = vec![];
        let mut delay = 0.0;
        while (trucks.len() as u32) < num_of_trucks {
            trucks.push(Vehicle::construct(TRUCK_W, y, speed, delay, ltr_direction));
  
            delay += Lane::generate_truck_delay(num_of_trucks)
        }      
        trucks
    }

    fn create_cars(y: f32, num_of_cars: u32, speed: f32, ltr_direction: bool) -> Vec<Vehicle>{
        let mut cars = vec![];
        let mut delay = 0.0;
        while (cars.len() as u32) < num_of_cars {
            cars.push(Vehicle::construct(CAR_W, y, speed, delay, ltr_direction));
  
            delay += Lane::generate_car_delay(num_of_cars)
        }      
        cars
    }

    fn generate_vehicle_type() -> u32 {
        let mut rng = thread_rng();
        
        //0 = Trucks & 1..3 = Cars (we generally want more cars)
        rng.gen_range(0_u32,4_u32)
    }

    fn generate_number_of_vehicles(vehicle_type: u32) -> u32 {
        let mut rng = thread_rng();
        
        match vehicle_type {
            0 => rng.gen_range(1_u32, MAX_NUM_OF_TRUCKS),
            _ => rng.gen_range(1_u32, MAX_NUM_OF_CARS)
        }
    }

    fn generate_speed() -> f32 {
        let mut rng = thread_rng();

        rng.gen_range(0.5_f32, MAX_SPEED_OF_OBSTACLES)
    }

    fn generate_car_delay(num_of_vehicles: u32) -> f32 {
        let mut rng = thread_rng();

        match num_of_vehicles{
            MAX_NUM_OF_CARS => MIN_DELAY,
            _ => rng.gen_range(MIN_DELAY,MAX_DELAY)
        }
    }

    fn generate_truck_delay(num_of_vehicles: u32) -> f32 {
        let mut rng = thread_rng();

        match num_of_vehicles{
            MAX_NUM_OF_TRUCKS => MIN_DELAY + (SQUARE_SIZE * 2.0),
            _ => rng.gen_range(MIN_DELAY * 2.0 , MAX_DELAY * 2.0)
        }
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
        for vehicle in &mut self.vehicles {
            vehicle.draw(ctx)?;
        }

        Ok(())
    }

    pub fn update_vehicles_in_lane(&mut self) {
        for vehicle in &mut self.vehicles {
            vehicle.update();
        }
    }
}
