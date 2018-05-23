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
use constants::WIN_H;
use characters::Crab;
use ggez::event::{Keycode, Mod};
use ggez::{GameResult, Context};
use ggez::graphics::{self};
use ggez::conf;
use ggez::event;
use ggez::timer;
use std::time::Duration;

const NUM_OF_LANES: u32 = 5; //This can change based on difficulty/level

// Contains properties to track during gameplay
// In this example it is only tracking the x coord of the orb
struct MainState {
    player: Crab,
    lanes: Vec<traffic::Lane>,
    lane_modifier: f32,
    game_over_man: graphics::Text
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let font = graphics::Font::new(_ctx, "/game_over.ttf", 48).unwrap();
        let text = graphics::Text::new(_ctx, "Game Over Man!", &font)?;
        let lanes = vec![];
        let s = MainState { 
            player: Crab::new(WIN_W, WIN_H),
            lanes: lanes,
            lane_modifier: 6.0,
            game_over_man: text
        };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {

        // Create new lanes
        if (self.lanes.len() as u32) < NUM_OF_LANES {
            self.lanes.push(traffic::Lane::construct(self.lane_modifier));    
            self.lane_modifier += 1.0;  
        }

        //Update lanes
        for lane in &mut self.lanes {
            lane.update_vehicles_in_lane();
        }

        //Check for game over
        if self.player.get_lives() <= 0 {
            self.player.set_lives();

            //Clear screen, optional
            graphics::clear(_ctx);

            //Scalable center, text should always be in center regardless of dimensions
            let center:f32 = WIN_W as f32 / 2.0 - *&self.game_over_man.width() as f32 / 2.0;

            let dest_point = graphics::Point2::new(center, WIN_H as f32 / 2.0);
            graphics::draw(_ctx, &self.game_over_man, dest_point, 0.0)?;
            graphics::present(_ctx);
            timer::sleep(Duration::from_secs(2));
        }

        //Take a life
        if self.player.get_life_lost() == true {
            self.player.set_life_lost();
            timer::sleep(Duration::from_secs(1));
            self.player.restart_x();
            self.player.restart_y();
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

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
    let ctx 				= &mut Context::load_from_conf("super_simple", "ggez", c).unwrap();
    let state 				= &mut MainState::new(ctx).unwrap();
    event::run(ctx, state).unwrap();
}
  