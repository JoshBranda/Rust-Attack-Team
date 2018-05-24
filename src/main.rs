/*
Copyright (c) 2018 Matt Carnovale, Julie Rutherford-Fields, Joshua Sander
This work is available under the "MIT License”.
Please see the file LICENSE in this distribution
for license terms.
*/

extern crate ggez;
extern crate rand;

pub mod background;
pub mod characters;
pub mod constants;
pub mod sprites;
pub mod traffic;

use background::Road;
use background::River;
// use background::Cubbie;

use constants::WIN_W;
use constants::WIN_H;
use constants::NUM_LANE;
use constants::START;
use constants::GRASS;

use constants::LANE_MODIFIER;
use characters::Crab;
use ggez::event::{Keycode, Mod};
use ggez::{GameResult, Context};
use ggez::graphics::{self};
use ggez::graphics::set_background_color;
use ggez::conf;
use ggez::event;


struct MainState {
    road: Road,
    river: River,
    // cubbie: Cubbie,
    player: Crab,
    lanes: Vec<traffic::Lane>,
    lane_modifier: f32
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let lanes = vec![];
        let s = MainState { 
            road: Road::new(WIN_W, WIN_H),
            river: River::new(WIN_W, WIN_H),
            // cubbie: Cubbie::new(WIN_W, WIN_H),
            player: Crab::new(WIN_W, START),
            lanes: lanes,
            lane_modifier: LANE_MODIFIER
        };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {

        // Create new lanes
        if (self.lanes.len() as u32) < NUM_LANE {
            self.lanes.push(traffic::Lane::construct(self.lane_modifier));    
            self.lane_modifier += 1.0;  
        }

        //Update lanes
        for lane in &mut self.lanes {
            lane.update_vehicles_in_lane();
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        //Draw background
        self.road.draw(ctx)?;
        self.river.draw(ctx)?;
        // self.cubbie.draw(ctx)?;


        //Draw our lanes
        for lane in &mut self.lanes {
            lane.draw_vehicles_in_lane(ctx)?;
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
    let ctx                 = &mut Context::load_from_conf("crabber", "ggez", c).unwrap();
    let state 				= &mut MainState::new(ctx).unwrap();
    set_background_color(ctx, GRASS);
    event::run(ctx, state).unwrap();
}
  