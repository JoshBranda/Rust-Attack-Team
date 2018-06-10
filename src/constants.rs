/*
Copyright (c) 2018 Matt Carnovale, Julie Rutherford-Fields, Joshua Sander
This work is available under the "MIT License”.
Please see the file LICENSE in this distribution
for license terms.
*/

use ggez::graphics::Color;

/// Square pixel dimensions as basis for all graphics
pub const SQUARE_SIZE: f32 = 25.0;

/// Window width in pixels - best if WIN_W % 100 == 50
pub const WIN_W: u32 = 650;
/// Window height - must be divisible by SQUARE_SIZE
pub const WIN_H: u32 = 400;

// Game background dimensions
/// Number of rows in window starting at index 0
pub const NUM_ROW: u32 = WIN_H / SQUARE_SIZE as u32;
/// Index of middle row
pub const MID_ROW: u32 = NUM_ROW / 2;
/// Starting position of crab as y coordinate in pixels
pub const START: f32 = WIN_H as f32 - SQUARE_SIZE;
/// Ending position of crab as y coordinate in pixels
pub const END: f32 = SQUARE_SIZE * 3.0;
/// Number of columns in window starting at index 0
pub const NUM_COL: u32 = WIN_W / SQUARE_SIZE as u32;
/// Index of middle row
pub const MID_COL: u32 = NUM_COL / 2;
/// Number of lanes of road traffic
pub const NUM_LANE: u32 = MID_ROW - 3;
/// Number of lanes of river traffic
pub const NUM_LOG: u32 = NUM_ROW - MID_ROW - 3;
/// Number of cubbies to be drawn to fit the window
pub const CUB_NUM: u32 = (NUM_COL / 2 - 1) / 2;

// Colors
/// Grass RGBA value
pub const GRASS: Color = Color {
    r: 0.0,
    g: 0.7,
    b: 0.23,
    a: 1.0,
};
/// Road RGBA value
pub const ROAD: Color = Color {
    r: 0.1,
    g: 0.2,
    b: 0.3,
    a: 1.0,
};
/// River RGBA value
pub const RIVER: Color = Color {
    r: 0.3,
    g: 0.3,
    b: 1.0,
    a: 1.0,
};
/// Log RGBA value
pub const LOG: Color = Color {
    r: 0.6,
    g: 0.3,
    b: 0.1,
    a: 1.0,
};
/// Turtle RGBA value
pub const TURTLE: Color = Color {
    r: 0.5,
    g: 0.5,
    b: 0.5,
    a: 1.0,
};

// Crab lives
/// Default number of crab lives
pub const LIVES: i32 = 3;
/// Pixel allowance for crab to land on log
pub const LOG_EDGE_BUFFER: f32 = 10.0;

// Traffic nums
/// Defines the y coordinate of the first lane of road traffic
pub const LANE_MODIFIER: f32 = WIN_H as f32 / (WIN_H as f32 / 2.0) + 1.0;
/// Maximum number of cars per lane per window width
pub const MAX_NUM_OF_CARS: u32 = WIN_W / 100;
/// Maximum number of trucks per lane per window width
pub const MAX_NUM_OF_TRUCKS: u32 = (MAX_NUM_OF_CARS / 2) + 1;
/// Maximum speed of cars and trucks
pub const MAX_SPEED_OF_OBSTACLES: f32 = WIN_W as f32 / (WIN_W as f32 / 2.0);
/// Minimum delay between vehicles
pub const MIN_DELAY: f32 = WIN_W as f32 / (WIN_W as f32 / 100.0);
/// Maximum delay between vehicles
pub const MAX_DELAY: f32 = MIN_DELAY + (SQUARE_SIZE * 3.0);
/// Width of car object
pub const CAR_W: f32 = SQUARE_SIZE * 2.0;
/// Width of truck object
pub const TRUCK_W: f32 = SQUARE_SIZE * 4.0;

// River nums
/// Defines the y coordinate of the first lane of river traffic
pub const RIVER_LANE_MODIFIER: f32 = LANE_MODIFIER + NUM_LANE as f32 + 1.0;
/// Maximum number of logs per lane per window width
pub const MAX_NUM_OF_LOGS: u32 = (MAX_NUM_OF_CARS / 2) + 1;
/// Maximum number of turtles per lane per window width
pub const MAX_NUM_OF_TURTLES: u32 = MAX_NUM_OF_CARS - 1;
/// Width of log, to be kept consistent with that of a truck
pub const LOG_W: f32 = TRUCK_W;
/// Width of log, to be kept consistent with that of a car
pub const TURTLE_W: f32 = CAR_W;

// Dev toggles
/// Toggles collisions with game obstacles to support
/// dev and testing of new features
pub const COLLISIONS_ON: bool = true;
/// Sets the number of cubbies needed to be occupied in order to win
pub const WINNING_CUBBIES: u32 = 0;
