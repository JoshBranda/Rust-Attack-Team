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
pub mod river;

use background::{Road, River, Cubbies, Menu};

use constants::{ 
    WIN_W, 
    WIN_H, 
    SQUARE_SIZE, 
    NUM_LANE, 
    NUM_LOG,
    LANE_MODIFIER,
    RIVER_LANE_MODIFIER,
    START,
    GRASS,
    END};

use characters::Crab;
use ggez::event::{Keycode, Mod};
use ggez::{GameResult, Context};
use ggez::graphics::{self};
use ggez::graphics::set_background_color;
use ggez::{conf, event, timer};
use std::time::Duration;


struct MainState {
    road: Road,
    river: River,
    cubbies: Cubbies,
    player: Crab,
    lanes: Vec<traffic::Lane>,
    lane_modifier: f32,
    river_lanes: Vec<river::RiverLane>,
    river_lane_modifier: f32,
    game_over_man: graphics::Text,
    main_menu: bool,
    selection: u32
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let font = graphics::Font::new(_ctx, "/game_over.ttf", 48).unwrap();
        let text = graphics::Text::new(_ctx, "Game Over Man!", &font)?;
        let lanes = vec![];
        let river_lanes = vec![];
        let s = MainState { 
            road: Road::new(WIN_W, WIN_H),
            river: River::new(WIN_W, WIN_H),
            cubbies: Cubbies::construct(),
            player: Crab::new(WIN_W, START as u32),
            lanes: lanes,
            lane_modifier: LANE_MODIFIER,
            river_lanes: river_lanes,
            river_lane_modifier: RIVER_LANE_MODIFIER,
            game_over_man: text,
            main_menu: true,
            selection: 0
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

        // Check for collisions with vehicles
        'outer: for i in 0..self.lanes.len() {
            for j in 0..self.lanes[i].vehicles.len() {
                if self.player.get_left_edge() >= self.lanes[i].vehicles[j].get_right_edge() {
                    continue;
                }

                if self.player.get_right_edge() <= self.lanes[i].vehicles[j].get_left_edge() {
                    continue;
                }

                if self.player.get_bottom_edge() <= self.lanes[i].vehicles[j].get_top_edge() {
                    continue;
                }

                if self.player.get_top_edge() >= self.lanes[i].vehicles[j].get_bottom_edge() {
                    continue;
                }

                self.player.lose_life();
                break 'outer;
            }
        }

        // Check for collisions with cubbies
        if self.player.get_bottom_edge() < END && self.player.get_left_edge() % (SQUARE_SIZE * 4.0) < SQUARE_SIZE * 2.0 {
            self.player.lose_life();
        }


        // Check for occupied cubbie
        if self.player.get_bottom_edge() < END && self.player.get_left_edge() % (SQUARE_SIZE * 4.0) >= SQUARE_SIZE * 2.0 {
            let i = (self.player.get_left_edge() / (SQUARE_SIZE * 4.0)) as usize;
            if self.cubbies.get_is_occupied(i) == false {
                // Get points for it
                self.player.add_to_score(500);
                // Set occupied flag to true
                self.cubbies.set_is_occupied(i);
                // Reset board
                timer::sleep(Duration::from_secs(1));
                self.player.restart_x();
                self.player.restart_y();
            }
        }

        //Update lanes
        for lane in &mut self.lanes {
            lane.update_vehicles_in_lane();
        }

        //Create new river lanes
        if (self.river_lanes.len() as u32) < NUM_LOG {
            self.river_lanes.push(river::RiverLane::construct(self.river_lane_modifier));    
            self.river_lane_modifier += 1.0;  
        }

        //Update river lanes
        for river_lane in &mut self.river_lanes {
            river_lane.update_river_obstacles_in_river_lane();
        }

        //Check for game over
        if self.player.get_lives() <= 0 {
            self.player.set_lives();

            //Clear screen, optional
            graphics::clear(_ctx);

            //Game over has a scalable center, text should always be in center regardless of dimensions
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

        if self.main_menu {
            let mut draw_main = Menu{};
            draw_main.draw(ctx, self.selection)?;

        } else {

            //Draw background
            self.road.draw(ctx)?;
            self.river.draw(ctx)?;
            self.cubbies.draw(ctx)?;
            // self.cubbies.draw_cubbie_crab(ctx);


            //Draw our lanes
            for lane in &mut self.lanes {
                lane.draw_vehicles_in_lane(ctx)?;
            }

            //Draw our river lanes
            for river_lane in &mut self.river_lanes {
                river_lane.draw_river_obstacles_in_river_lane(ctx)?;
            }

            self.player.draw(ctx)?;

            //Draw the lives in the bottom left
            let lives = format! {"Lives: {}", self.player.get_lives()};
            let font_smaller = graphics::Font::new(ctx, "/game_over.ttf", 16).unwrap();
            let lives_text = graphics::Text::new(ctx, &lives, &font_smaller)?;
            let dest_point = graphics::Point2::new(0 as f32, WIN_H as f32 - SQUARE_SIZE);
            graphics::draw(ctx, &lives_text, dest_point, 0.0)?;

            //Draw the lives in the bottom left
            let score = format! {"Score: {}", self.player.get_score()};
            let score_text = graphics::Text::new(ctx, &score, &font_smaller)?;
            let score_width = score_text.width() as f32;
            let dest_point = graphics::Point2::new(WIN_W as f32 - score_width, WIN_H as f32 - SQUARE_SIZE);
            graphics::draw(ctx, &score_text, dest_point, 0.0)?;
        }

        graphics::present(ctx);
        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut ggez::Context, keycode: Keycode, _: Mod, _: bool) {
        if self.main_menu {
            match keycode {
                Keycode::Down => (self.selection = 1),
                Keycode::Up => (self.selection = 0),
                Keycode::Return => {if self.selection == 0 {self.main_menu = false}},
                _ => {}
            }
        }
        else {
            match keycode {
                Keycode::Up => self.player.move_up(),
                Keycode::Down => self.player.move_down(),
                Keycode::Right => self.player.move_right(),
                Keycode::Left => self.player.move_left(),

                _ => {}
            }
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
  