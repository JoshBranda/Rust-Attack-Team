/*
Copyright (c) 2018 Matt Carnovale, Julie Rutherford-Fields, Joshua Sander
This work is available under the "MIT License”.
Please see the file LICENSE in this distribution
for license terms.
*/

extern crate ggez;
extern crate rand;

pub mod characters;
pub mod constants;
pub mod sprites;
pub mod traffic;

use constants::WIN_W;
use constants::WIN_H
use constants::SQUARE_SIZE;
use characters::Crab;
use sprites::Square;
use ggez::event::{Keycode, Mod};
use ggez::{GameResult, Context};
use ggez::graphics::{self, set_color, Color, DrawMode, Point2};
use ggez::conf;
use ggez::event;


// Contains properties to track during gameplay
// In this example it is only tracking the x coord of the orb
struct MainState {
    player: Crab,
    cars: Vec<traffic::Car>
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let w = WIN_W;
        let h = WIN_H;
        let cars = vec![];
        let s = MainState { 
            cars: cars,
            player: Crab::new(_ctx, w, h)
        };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        // Create new cars
        if (self.cars.len() as u32) < 4 {
            let w = WIN_W;
            let h = WIN_H;
            let mut delay = 0.0;
            if (self.cars.len() as u32) >= 1{
                delay += SQUARE_SIZE * 6.3 * self.cars.len() as f32;
                self.cars.push(traffic::Car::construct(h, delay));
            } else {
                self.cars.push(traffic::Car::construct(w, delay));
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

        self.player.draw(ctx)?;

        graphics::present(ctx);
        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut ggez::Context, keycode: Keycode, _: Mod, _: bool) {
        match keycode {
            Keycode::Up => self.player.move_up(),
            Keycode::Down => self.player.move_down(),
            Keycode::Right => self.player.move_right(),
            Keycode::Left => self.player.move_left(),

            _ => {}
        }
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
  