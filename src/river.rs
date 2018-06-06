/*
Copyright (c) 2018 Matt Carnovale, Julie Rutherford-Fields, Joshua Sander
This work is available under the "MIT License”.
Please see the file LICENSE in this distribution
for license terms.
*/

use constants::{LOG, LOG_W, MAX_DELAY, MAX_NUM_OF_LOGS, MAX_NUM_OF_TURTLES,
                MAX_SPEED_OF_OBSTACLES, MIN_DELAY, SQUARE_SIZE, TURTLE, TURTLE_W, WIN_H, WIN_W};
use ggez::graphics::Color;
use ggez::{Context, GameResult};
use rand::{thread_rng, Rng};
use sprites::Rectangle;

/// A River Transport is what we use to represent the logs & sea turtles
/// one would expect in the original game.
/// Each transport has a form (a rectanglar shape), a speed, and a direction.
/// If the direction is true it's moving from left to right.
pub struct RiverTransport {
    form: Rectangle,
    speed: f32,
    direction: bool,
}

impl RiverTransport {
    /// Returns a River Transport with the width, starting y coordinate
    /// speed, delay (space between other objects in row), direction,
    /// and type (log or turtle).
    pub fn construct(
        w: f32,
        y: f32,
        speed: f32,
        delay: f32,
        ltr_direction: bool,
        river_transport_type: u32,
    ) -> RiverTransport {
        let h = SQUARE_SIZE as f32;
        let x = RiverTransport::assign_starting_x(ltr_direction, w, delay);
        RiverTransport {
            form: Rectangle::construct(
                x,
                y,
                w,
                h,
                RiverTransport::assign_color(river_transport_type),
            ),
            speed,
            direction: ltr_direction,
        }
    }

    /// Assigns the starting x-coordinate of the river transport based on
    /// the ltr_direction flag (left to right), the width of the object, and
    /// the delay (how far offscreen it should start to allow space between
    /// other objects in the same row).
    fn assign_starting_x(ltr_direction: bool, w: f32, delay: f32) -> f32 {
        match ltr_direction {
            true => 0.0 - w - 10.0 - delay,
            false => WIN_W as f32 - 10.0 - delay,
        }
    }

    /// Assigns the appropriate color to the river transport based on the
    /// type that has been randomly generated for the row.
    fn assign_color(river_transport_type: u32) -> Color {
        match river_transport_type {
            0 => LOG,
            _ => TURTLE,
        }
    }

    /// The draw function is provided by the ggez crate and is used
    /// to create and display the rectangular graphic that is assigned
    /// to the form attribute of the river transport
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.form.draw(ctx)?;
        Ok(())
    }

    /// Updates the x-coordinate of the river transport so it
    /// appears to move across the screen. The way the
    /// x-coordinate is updated depends on the direction
    /// attribute of the river struct. Once the coordinate
    /// reaches a value beyond the width of the view, it gets
    /// reset to its starting value. This allows us to minimize
    /// the number of river transports needed in a vector of them.
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
    /// river transport which can be used to inform
    /// when a player is attempting to ride it.
    pub fn get_left_edge(&mut self) -> f32 {
        return self.form.x;
    }

    /// Getter to acquire the right x-coordinate of the
    /// river transport which can be used to inform
    /// when a player is attempting to ride it.
    pub fn get_right_edge(&mut self) -> f32 {
        return self.form.x + self.form.w;
    }

    /// Getter to acquire the bottom y-coordinate of the
    /// river transport which can be used to inform
    /// when a player is attempting to ride it.
    pub fn get_bottom_edge(&mut self) -> f32 {
        return self.form.y;
    }

    /// Getter to acquire the top y-coordinate of the
    /// river transport which can be used to inform
    /// when a player is attempting to ride it.
    pub fn get_top_edge(&mut self) -> f32 {
        return self.form.y - self.form.h;
    }

    /// Getter to acquire the speed of the
    /// river transport which can be used to inform
    /// when a player is riding a river transport
    pub fn get_speed(&mut self) -> f32 {
        return self.speed;
    }

    /// Getter to acquire the direction of the
    /// river transport which can be used to inform
    /// when a player is riding a river transport
    pub fn get_direction(&mut self) -> bool {
        return self.direction;
    }
}

/// The River Lane manages and represents a row river transports
/// It requires a vector of River Transport structs.
pub struct RiverLane {
    pub river_transports: Vec<RiverTransport>,
}

impl RiverLane {
    /// Returns a River Lane struct that manages a row of either logs
    /// or turtles (not mixed). This vector is assigned a direction,
    /// transportation type, speed, and the number of river transports
    /// available.  The values of these attributes are generated and
    /// used to create the river transport structs.
    pub fn construct(y_modifier: f32) -> RiverLane {
        let y = WIN_H as f32 - y_modifier * SQUARE_SIZE;
        let ltr_direction = RiverLane::generate_direction();
        let river_transport_type = RiverLane::generate_river_transport_type(ltr_direction);
        let num_of_river_transports =
            RiverLane::generate_number_of_river_transports(river_transport_type);
        let speed = RiverLane::generate_speed(river_transport_type);
        RiverLane {
            river_transports: RiverLane::create_river_transport(
                river_transport_type,
                y,
                num_of_river_transports,
                speed,
                ltr_direction,
            ),
        }
    }

    /// Returns a vector of either logs or turtles depending on the randomly
    /// generated river transport type.
    fn create_river_transport(
        river_transport_type: u32,
        y: f32,
        num_of_river_transports: u32,
        speed: f32,
        ltr_direction: bool,
    ) -> Vec<RiverTransport> {
        match river_transport_type {
            0 => RiverLane::create_logs(
                y,
                num_of_river_transports,
                speed,
                ltr_direction,
                river_transport_type,
            ),
            _ => RiverLane::create_turtles(
                y,
                num_of_river_transports,
                speed,
                ltr_direction,
                river_transport_type,
            ),
        }
    }

    /// Returns a vector of logs, using the assigned starting y-coordinate,
    /// a randomly generated number of cars, speed, direction, and type.
    fn create_logs(
        y: f32,
        num_of_logs: u32,
        speed: f32,
        ltr_direction: bool,
        river_transport_type: u32,
    ) -> Vec<RiverTransport> {
        let mut logs = vec![];
        let mut delay = 0.0;
        while (logs.len() as u32) < num_of_logs {
            logs.push(RiverTransport::construct(
                LOG_W,
                y,
                speed,
                delay,
                ltr_direction,
                river_transport_type,
            ));

            delay += RiverLane::generate_log_delay(num_of_logs)
        }
        logs
    }

    /// Returns a vector of turtles, using the assigned starting y-coordinate,
    /// a randomly generated number of cars, speed, direction, and type.
    fn create_turtles(
        y: f32,
        num_of_turtles: u32,
        speed: f32,
        ltr_direction: bool,
        river_transport_type: u32,
    ) -> Vec<RiverTransport> {
        let mut turtles = vec![];
        let mut delay = 0.0;
        while (turtles.len() as u32) < num_of_turtles {
            turtles.push(RiverTransport::construct(
                TURTLE_W,
                y,
                speed,
                delay,
                ltr_direction,
                river_transport_type,
            ));

            delay += RiverLane::generate_turtle_delay(num_of_turtles)
        }
        turtles
    }

    /// Generates a river transport type based on the direction assigned
    /// to the row. In the original game, logs go in one direction and
    /// turtles go the other. If the direction is true (left to right),
    /// the row is assigned logs (0) as the river transport. Otherwise,
    /// it is assigned to contain turtles (1)
    fn generate_river_transport_type(ltr_direction: bool) -> u32 {
        match ltr_direction {
            true => 0,
            false => 1,
        }
    }

    /// Uses the max number of logs and turtles constant to generate a bounded
    /// random number of river transports in a row. These max numbers are scalable
    /// depending on the width of the window.
    fn generate_number_of_river_transports(river_transport_type: u32) -> u32 {
        let mut rng = thread_rng();

        match river_transport_type {
            0 => rng.gen_range(2_u32, MAX_NUM_OF_LOGS),
            _ => rng.gen_range(2_u32, MAX_NUM_OF_TURTLES),
        }
    }

    /// Uses the obstacle max speed constant to generate a bounded random
    /// number that will be used to assign the speed of the river transports
    /// in a row. The turtles (_) have a slightly lower max speed to make the
    /// game easier and because they are swimming against the current--David
    /// Attenborough would be proud of these noble creatures.
    fn generate_speed(river_transport_type: u32) -> f32 {
        let mut rng = thread_rng();

        match river_transport_type {
            0 => rng.gen_range(0.5_f32, MAX_SPEED_OF_OBSTACLES),
            _ => rng.gen_range(0.5_f32, MAX_SPEED_OF_OBSTACLES - 0.5),
        }
    }

    /// Used to generate the delay--or space--between turtles. This distance
    /// is based on the number of turtles in a row (the more items there are
    /// the less space there is to leave between them)
    fn generate_turtle_delay(num_of_river_transports: u32) -> f32 {
        let mut rng = thread_rng();

        match num_of_river_transports {
            MAX_NUM_OF_TURTLES => MIN_DELAY,
            _ => rng.gen_range(MIN_DELAY, MAX_DELAY),
        }
    }

    /// Used to generate the delay--or space--between logs. This distance
    /// is based on the number of logs in a row (the more items there are
    /// the less space there is to leave between them)
    fn generate_log_delay(num_of_river_transports: u32) -> f32 {
        let mut rng = thread_rng();

        match num_of_river_transports {
            MAX_NUM_OF_LOGS => MIN_DELAY + (SQUARE_SIZE * 2.0),
            _ => rng.gen_range(MIN_DELAY * 2.0, MAX_DELAY * 2.0),
        }
    }

    /// Used to randomly generated the direction the river transports will
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

    /// Calls upon the draw routine for each river transport in the vector
    pub fn draw_river_transports_in_river_lane(&mut self, ctx: &mut Context) -> GameResult<()> {
        for river_transport in &mut self.river_transports {
            river_transport.draw(ctx)?;
        }

        Ok(())
    }

    /// Calls upon the update routine for each river transport in the vector
    pub fn update_river_transports_in_river_lane(&mut self) {
        for river_transport in &mut self.river_transports {
            river_transport.update();
        }
    }
}
