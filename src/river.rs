/*
Copyright (c) 2018 Matt Carnovale, Julie Rutherford-Fields, Joshua Sander
This work is available under the "MIT License”.
Please see the file LICENSE in this distribution
for license terms.
*/

use ggez::{GameResult, Context};
use ggez::graphics::{Color};
use rand::{Rng, thread_rng};
use sprites::Rectangle;
use constants::{
    WIN_W, 
    WIN_H, 
    SQUARE_SIZE, 
    MAX_NUM_OF_TURTLES, 
    MAX_NUM_OF_LOGS, 
    MAX_SPEED_OF_OBSTACLES, 
    MIN_DELAY, 
    MAX_DELAY,
    LOG_W,
    TURTLE_W,
    LOG,
    TURTLE };

pub struct RiverObstacle {
    form: Rectangle,
    speed: f32,
    direction: bool
}

impl RiverObstacle {

    pub fn construct(w: f32, y: f32, speed: f32, delay: f32, 
                    ltr_direction: bool, river_obstacle_type: u32) -> RiverObstacle {
        let h = SQUARE_SIZE as f32;
        let x = RiverObstacle::assign_starting_x(ltr_direction, w, delay);
        RiverObstacle {
            form: Rectangle::construct(
                x,
                y,
                w,
                h,
                RiverObstacle::assign_color(river_obstacle_type)
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

    fn assign_color(river_obstacle_type: u32) -> Color {
        match river_obstacle_type {
            0 => LOG, 
            _ => TURTLE 
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
}

pub struct RiverLane {
    river_obstacles: Vec<RiverObstacle>
}

impl RiverLane {

    pub fn construct(y_modifier: f32) -> RiverLane {
        let y = WIN_H as f32 - y_modifier * SQUARE_SIZE; 
        let ltr_direction= RiverLane::generate_direction();
        let river_obstacle_type= RiverLane::generate_river_obstacle_type(ltr_direction);                     
        let num_of_river_obstacles= RiverLane::generate_number_of_river_obstacles(river_obstacle_type);
        let speed= RiverLane::generate_speed(river_obstacle_type);
        RiverLane {
            river_obstacles: RiverLane::create_river_obstacle(river_obstacle_type, y, num_of_river_obstacles, speed, 
                                            ltr_direction)
        }
    }

    fn create_river_obstacle(river_obstacle_type: u32, y: f32, num_of_river_obstacles: u32, 
                      speed: f32, ltr_direction: bool) -> Vec<RiverObstacle>{
        match river_obstacle_type {
            0 => RiverLane::create_logs(y, num_of_river_obstacles, speed, 
                                    ltr_direction, river_obstacle_type),
            _ => RiverLane::create_turtles(y, num_of_river_obstacles, speed, 
                                    ltr_direction, river_obstacle_type)
        }
    }

    fn create_logs(y: f32, num_of_trucks: u32, speed: f32,
                     ltr_direction: bool, river_obstacle_type: u32) -> Vec<RiverObstacle>{
        let mut trucks = vec![];
        let mut delay = 0.0;
        while (trucks.len() as u32) < num_of_trucks {
            trucks.push(RiverObstacle::construct(LOG_W, y, speed, delay, 
                                                    ltr_direction, river_obstacle_type));
  
            delay += RiverLane::generate_log_delay(num_of_trucks)
        }      
        trucks
    }

    fn create_turtles(y: f32, num_of_turtles: u32, speed: f32, 
                        ltr_direction: bool, river_obstacle_type: u32) -> Vec<RiverObstacle>{
        let mut turtles = vec![];
        let mut delay = 0.0;
        while (turtles.len() as u32) < num_of_turtles {
            turtles.push(RiverObstacle::construct(TURTLE_W, y, speed, delay, 
                                                    ltr_direction, river_obstacle_type));
  
            delay += RiverLane::generate_turtle_delay(num_of_turtles)
        }      
        turtles
    }

    fn generate_river_obstacle_type(ltr_direction: bool) -> u32 {
        match ltr_direction{
            true => 0,
            false => 1
        }
    }

    fn generate_number_of_river_obstacles(river_obstacle_type: u32) -> u32 {
        let mut rng = thread_rng();
        
        match river_obstacle_type {
            0 => rng.gen_range(2_u32, MAX_NUM_OF_LOGS),
            _ => rng.gen_range(2_u32, MAX_NUM_OF_TURTLES)
        }
    }

    fn generate_speed(river_obstacle_type: u32) -> f32 {
        let mut rng = thread_rng();

        match river_obstacle_type{
            0 => rng.gen_range(0.5_f32, MAX_SPEED_OF_OBSTACLES),
            _ => rng.gen_range(0.5_f32, MAX_SPEED_OF_OBSTACLES - 0.5)
        }
        
    }

    fn generate_turtle_delay(num_of_river_obstacles: u32) -> f32 {
        let mut rng = thread_rng();

        match num_of_river_obstacles{
            MAX_NUM_OF_TURTLES => MIN_DELAY,
            _ => rng.gen_range(MIN_DELAY, MAX_DELAY)
        }
    }

    fn generate_log_delay(num_of_river_obstacles: u32) -> f32 {
        let mut rng = thread_rng();

        match num_of_river_obstacles{
            MAX_NUM_OF_LOGS => MIN_DELAY + (SQUARE_SIZE * 2.0),
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

    pub fn draw_river_obstacles_in_river_lane(&mut self, ctx: &mut Context) -> GameResult<()> {
        for river_obstacle in &mut self.river_obstacles {
            river_obstacle.draw(ctx)?;
        }

        Ok(())
    }

    pub fn update_river_obstacles_in_river_lane(&mut self) {
        for river_obstacle in &mut self.river_obstacles {
            river_obstacle.update();
        }
    }

}
  