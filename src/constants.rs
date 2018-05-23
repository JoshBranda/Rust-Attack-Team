/*
Copyright (c) 2018 Matt Carnovale, Julie Rutherford-Fields, Joshua Sander
This work is available under the "MIT License”.
Please see the file LICENSE in this distribution
for license terms.
*/

// Square pixel dimensions as basis for all graphics
pub const SQUARE_SIZE: f32 = 25.0;

// Screen dimensions. Currently portrait mode.
pub const WIN_W: u32 = 650;
pub const WIN_H: u32 = 400;

// Game background dimensions
pub const NUM_R: u32 = WIN_H / SQUARE_SIZE as u32;		// Number of rows starting at index 0
pub const MID_R: u32 = NUM_R / 2;						// Middle row index
pub const START: u32 = SQUARE_SIZE as u32;				// Starting position as y coordinate in pixels
pub const END  : u32 = WIN_H - SQUARE_SIZE as u32 * 3;	// Ending position as y coordinate in pixels

pub const NUM_C: u32 = WIN_W / SQUARE_SIZE as u32;		// Number of columns starting at index 0
pub const MID_C: u32 = NUM_C / 2;						// Middle row index
pub const LANES: u32 = MID_R - 3;						// Number of lanes of traffic
pub const LOGS : u32 = NUM_R - MID_R - 3;				// Number of log lanes 
