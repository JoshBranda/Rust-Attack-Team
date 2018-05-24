/*
Copyright (c) 2018 Matt Carnovale, Julie Rutherford-Fields, Joshua Sander
This work is available under the "MIT License”.
Please see the file LICENSE in this distribution
for license terms.
*/

use ggez::graphics::Color;

// Square pixel dimensions as basis for all graphics
pub const SQUARE_SIZE: f32 = 25.0;

// Screen dimensions. Currently portrait mode.
pub const WIN_W: u32 	= 650;
pub const WIN_H: u32 	= 400;

// Game background dimensions
pub const NUM_ROW: u32 	= WIN_H / SQUARE_SIZE as u32;		// Number of rows starting at index 0
pub const MID_ROW: u32 	= NUM_ROW / 2;						// Middle row index
pub const START: u32 	= WIN_H - SQUARE_SIZE as u32;		// Starting position as y coordinate in pixels
pub const END  : u32 	= WIN_H - SQUARE_SIZE as u32 * 3;	// Ending position as y coordinate in pixels
pub const NUM_COL: u32 	= WIN_W / SQUARE_SIZE as u32;		// Number of columns starting at index 0
pub const MID_COL: u32 	= NUM_COL / 2;						// Middle row index
pub const NUM_LANE: u32 = MID_ROW - 3;						// Number of lanes of traffic
pub const NUM_LOG: u32 	= NUM_ROW - MID_ROW - 3;			// Number of log lanes

// Colors
pub const GRASS: Color = Color{r: 0.0, g: 0.7, b: 0.23, a: 1.0};
pub const ROAD: Color = Color{r: 0.1, g: 0.2, b: 0.3, a: 1.0};
pub const RIVER: Color = Color{r: 0.3, g: 0.3, b: 1.0, a: 1.0};

//Crab lives
pub const LIVES: i32 = 3;

//Traffic nums
pub const LANE_MODIFIER: f32 = WIN_H as f32/ (WIN_H as f32 / 2.0) + 1.0;
pub const MAX_NUM_OF_CARS: u32 = WIN_W / 100;
pub const MAX_SPEED_OF_CARS: f32 = WIN_W as f32 / (WIN_W as f32 / 2.0);
pub const MIN_DELAY: f32 = WIN_W as f32/ (WIN_W as f32/ 100.0);
pub const MAX_DELAY: f32 = MIN_DELAY + (SQUARE_SIZE * 3.0);
pub const NUM_OF_LANES: u32 = 5; //This can change based on difficulty/level

