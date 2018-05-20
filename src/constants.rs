/*
Copyright (c) 2018 Matt Carnovale, Julie Rutherford-Fields, Joshua Sander
This work is available under the "MIT License”.
Please see the file LICENSE in this distribution
for license terms.
*/

// Square pixel dimensions as basis for all graphics
pub const SQUARE_SIZE: f32 = 50.0;

// Screen dimensions. Currently portrait mode.
pub const WIN_W: u32 = 800;
pub const WIN_H: u32 = 600;

// Game course dimensions
pub const NUM_R: u32 = WIN_W / SQUARE_SIZE as u32 - 1;	// Index of rows starting at position 0
pub const MID_R: u32 = NUM_R / 2;						// Middle row index
pub const START: u32 = SQUARE_SIZE as u32;				// Starting position as y coordinate in pixels
pub const END  : u32 = WIN_H - SQUARE_SIZE as u32 * 4;	// Ending position as y coordinate in pixels
