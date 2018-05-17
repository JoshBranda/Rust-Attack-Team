/*
Copyright (c) 2018 Matt Carnovale, Julie Rutherford-Fields, Joshua Sander
This work is available under the "MIT License”.
Please see the file LICENSE in this distribution
for license terms.
*/

extern crate ggez;
extern crate rand;

use ggez::{GameResult, Context};
use ggez::graphics::{self, DrawMode, Point2};
use ggez::conf;
use ggez::event;

mod traffic;

// Screen dimensions. Currently portrait mode.
const WIN_W: u32 = 400;
const WIN_H: u32 = 700;

// Contains properties to track during gameplay
// In this example it is only tracking the x coord of the orb
struct MainState {
    cars: Vec<traffic::Car>
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let cars = vec![];
        let s = MainState { 
            cars: cars
        };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        // Create new cars
        if (self.cars.len() as u32) < 4 {
            let mut delay = 0.0;
            if (self.cars.len() as u32) >= 1{
                delay += traffic::SQUARE_SIZE * 6.3 * self.cars.len() as f32;
                self.cars.push(traffic::Car::construct(WIN_H, delay));
            } else {
                self.cars.push(traffic::Car::construct(WIN_H, delay));
            }
        }

        //Update cars
        for car in &mut self.cars {
            car.update();
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        //Draw our cars
        for car in &mut self.cars {
            car.draw(ctx)?;
        }

        graphics::present(ctx);
        Ok(())
    }
}

pub fn main() {
    let mut c = conf::Conf::new();
    c.window_setup.title 	= "C R A B B E R".to_string();
	c.window_mode.width 	= WIN_W;
    c.window_mode.height 	= WIN_H;
    let ctx 				= &mut Context::load_from_conf("super_simple", "ggez", c).unwrap();
    let state 				= &mut MainState::new(ctx).unwrap();
    event::run(ctx, state).unwrap();
}
  