/*
Copyright (c) 2018 Matt Carnovale, Julie Rutherford-Fields, Joshua Sander
This work is available under the "MIT License”.
Please see the file LICENSE in this distribution
for license terms.
*/

use constants::{CAR_W, MAX_DELAY, MAX_NUM_OF_CARS, MAX_NUM_OF_TRUCKS, MAX_SPEED_OF_OBSTACLES,
                MIN_DELAY, SQUARE_SIZE, TRUCK_W, WIN_H, WIN_W};

use ggez::graphics::Color;
use ggez::{Context, GameResult};
use rand::{thread_rng, Rng};
use sprites::Rectangle;

/// A Vehicle is what we use to represent the trucks & cars
/// one would expect in the original game.
/// Each transport has a form (a rectanglar shape), a speed, and a direction.
/// If the direction is true it's moving from left to right.
pub struct Vehicle {
    form: Rectangle,
    speed: f32,
    direction: bool,
}

impl Vehicle {
    /// Returns a Vehicle with the width, starting y coordinate
    /// speed, delay (space between other objects in row), and direction.
    pub fn construct(w: f32, y: f32, speed: f32, delay: f32, ltr_direction: bool) -> Vehicle {
        // let w = w;
        let h = SQUARE_SIZE as f32;
        let x = Vehicle::assign_starting_x(ltr_direction, w, delay);
        Vehicle {
            form: Rectangle::construct(x, y, w, h, Vehicle::assign_color()),
            speed,
            direction: ltr_direction,
        }
    }

    /// Assigns the starting x-coordinate of the vehicle based on
    /// the ltr_direction flag (left to right), the width of the object, and
    /// the delay (how far offscreen it should start to allow space between
    /// other objects in the same row).
    fn assign_starting_x(ltr_direction: bool, w: f32, delay: f32) -> f32 {
        match ltr_direction {
            true => 0.0 - w - 10.0 - delay,
            false => WIN_W as f32 - 10.0 - delay,
        }
    }

    ///Assigns a random color from the set of 7 available colors
    fn assign_color() -> Color {
        let mut rng = thread_rng();
        let color: u32 = rng.gen_range(0, 99999) % 7;

        match color {
            0 => Color::new(0.0, 0.0, 1.0, 1.0),
            1 => Color::new(1.0, 0.0, 0.0, 1.0),
            2 => Color::new(0.0, 1.0, 0.0, 1.0),
            3 => Color::new(1.0, 1.0, 0.0, 1.0),
            4 => Color::new(0.0, 1.0, 1.0, 1.0),
            5 => Color::new(1.0, 0.0, 1.0, 1.0),
            _ => Color::new(1.0, 0.5, 1.0, 0.5),
        }
    }

    /// The draw function is provided by the ggez crate and is used
    /// to create and display the rectangular graphic that is assigned
    /// to the form attribute of the vehicle
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.form.draw(ctx)?;
        Ok(())
    }

    /// Updates the x-coordinate of the vehicle so it
    /// appears to move across the screen. The way the
    /// x-coordinate is updated depends on the direction
    /// attribute of the vehicle struct. Once the coordinate
    /// reaches a value beyond the width of the view, it gets
    /// reset to its starting value. This allows us to minimize
    /// the number of vehicle tranports needed in a vector of them.
    fn update(&mut self) {
        if self.direction {
            if self.form.x >= WIN_W as f32 + 10.0 {
                self.form.x = 0.0 - self.form.w - 10.0
            }
            self.form.x = self.form.x + self.speed;
        } else {
            if self.form.x <= -self.form.w {
                self.form.x = WIN_W as f32 - 10.0
            }
            self.form.x = self.form.x - self.speed;
        }
    }

    /// Getter to acquire the left x-coordinate of the
    /// vehicle which can be used to inform
    /// when a player collides with it.
    pub fn get_left_edge(&mut self) -> f32 {
        return self.form.x;
    }

    /// Getter to acquire the right x-coordinate of the
    /// vehicle which can be used to inform
    /// when a player collides with it.
    pub fn get_right_edge(&mut self) -> f32 {
        return self.form.x + self.form.w;
    }

    /// Getter to acquire the bottom y-coordinate of the
    /// vehicle which can be used to inform
    /// when a player collides with it.
    pub fn get_bottom_edge(&mut self) -> f32 {
        return self.form.y;
    }

    /// Getter to acquire the top y-coordinate of the
    /// vehicle which can be used to inform
    /// when a player collides with it.
    pub fn get_top_edge(&mut self) -> f32 {
        return self.form.y - self.form.h;
    }
}

/// The Lane manages and represents a row of vehicles (traffic).
/// It requires a vector of Vehicle structs.
pub struct Lane {
    pub vehicles: Vec<Vehicle>,
}

impl Lane {
    /// Returns a Lane struct that manages a row of either trucks
    /// or cars (not mixed). This vector is assigned a direction,
    /// transportation type, speed, and the number of vehicles
    /// available.  The values of these attributes are generated and
    /// used to create the vehicles structs.
    pub fn construct(y_modifier: f32) -> Lane {
        let y = WIN_H as f32 - y_modifier * SQUARE_SIZE;
        let ltr_direction = Lane::generate_direction();
        let vehicle_type = Lane::generate_vehicle_type();
        let num_of_vehicles = Lane::generate_number_of_vehicles(vehicle_type);
        let speed = Lane::generate_speed();
        Lane {
            vehicles: Lane::create_vehicle(vehicle_type, y, num_of_vehicles, speed, ltr_direction),
        }
    }

    /// Returns a vector of either trucks or cars depending on the randomly
    /// generated vehicle type.
    fn create_vehicle(
        vehicle_type: u32,
        y: f32,
        num_of_vehicles: u32,
        speed: f32,
        ltr_direction: bool,
    ) -> Vec<Vehicle> {
        match vehicle_type {
            0 => Lane::create_trucks(y, num_of_vehicles, speed, ltr_direction),
            _ => Lane::create_cars(y, num_of_vehicles, speed, ltr_direction),
        }
    }

    /// Returns a vector of trucks, using the assigned starting y-coordinate,
    /// a randomly generated number of cars, speed, and direction.
    fn create_trucks(y: f32, num_of_trucks: u32, speed: f32, ltr_direction: bool) -> Vec<Vehicle> {
        let mut trucks = vec![];
        let mut delay = 0.0;
        while (trucks.len() as u32) < num_of_trucks {
            trucks.push(Vehicle::construct(TRUCK_W, y, speed, delay, ltr_direction));

            delay += Lane::generate_truck_delay(num_of_trucks)
        }
        trucks
    }

    /// Returns a vector of cars, using the assigned starting y-coordinate,
    /// a randomly generated number of cars, speed, and direction.
    fn create_cars(y: f32, num_of_cars: u32, speed: f32, ltr_direction: bool) -> Vec<Vehicle> {
        let mut cars = vec![];
        let mut delay = 0.0;
        while (cars.len() as u32) < num_of_cars {
            cars.push(Vehicle::construct(CAR_W, y, speed, delay, ltr_direction));

            delay += Lane::generate_car_delay(num_of_cars)
        }
        cars
    }

    /// Generates a vehicle type based on a randomly generated number
    /// between 0-3. The trucks are assinged if this returns a 0.
    /// Otherwise, a car is assinged if it returns 1-3. This allows
    /// flexiblity for changes and ensures we will typically have more
    /// cars than trucks.
    fn generate_vehicle_type() -> u32 {
        let mut rng = thread_rng();

        //0 = Trucks & 1..3 = Cars (we generally want more cars)
        rng.gen_range(0_u32, 4_u32)
    }

    /// Uses the max number of trucks and cars constant to generate a bounded
    /// random number of vehicles in a row. These max numbers are scalable
    /// depending on the width of the window.
    fn generate_number_of_vehicles(vehicle_type: u32) -> u32 {
        let mut rng = thread_rng();

        match vehicle_type {
            0 => rng.gen_range(1_u32, MAX_NUM_OF_TRUCKS),
            _ => rng.gen_range(1_u32, MAX_NUM_OF_CARS),
        }
    }

    /// Uses the obstacle max speed constant to generate a bounded random
    /// number that will be used to assign the speed of the vehicles
    /// in a row.
    fn generate_speed() -> f32 {
        let mut rng = thread_rng();

        rng.gen_range(0.5_f32, MAX_SPEED_OF_OBSTACLES)
    }

    /// Used to generate the delay--or space--between cars. This distance
    /// is based on the number of cars in a row (the more items there are
    /// the less space there is to leave between them)
    fn generate_car_delay(num_of_vehicles: u32) -> f32 {
        let mut rng = thread_rng();

        match num_of_vehicles {
            MAX_NUM_OF_CARS => MIN_DELAY,
            _ => rng.gen_range(MIN_DELAY, MAX_DELAY),
        }
    }

    /// Used to generate the delay--or space--between trucks. This distance
    /// is based on the number of trucks in a row (the more items there are
    /// the less space there is to leave between them)
    fn generate_truck_delay(num_of_vehicles: u32) -> f32 {
        let mut rng = thread_rng();

        match num_of_vehicles {
            MAX_NUM_OF_TRUCKS => MIN_DELAY + (SQUARE_SIZE * 2.0),
            _ => rng.gen_range(MIN_DELAY * 2.0, MAX_DELAY * 2.0),
        }
    }

    /// Used to randomly generated the direction the vehicles will
    /// move across the screen (left to right if true and right to left if
    /// false). The six sided die match statement was really just to give
    /// myself a laugh. This is important and let's all be glad it's not
    /// the much respected 20 sided die.
    fn generate_direction() -> bool {
        let mut rng = thread_rng();
        let six_sided_die: u32 = rng.gen_range(0, 99999) % 6;
        match six_sided_die {
            0 => false,
            1 => true,
            2 => false,
            3 => false,
            4 => true,
            _ => true,
        }
    }

    /// Calls upon the draw routine for each vehicle in the vector
    pub fn draw_vehicles_in_lane(&mut self, ctx: &mut Context) -> GameResult<()> {
        for vehicle in &mut self.vehicles {
            vehicle.draw(ctx)?;
        }

        Ok(())
    }

    /// Calls upon the update routine for each vehicle in the vector
    pub fn update_vehicles_in_lane(&mut self) {
        for vehicle in &mut self.vehicles {
            vehicle.update();
        }
    }
}
