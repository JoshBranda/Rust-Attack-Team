/*
Copyright (c) 2018 Matt Carnovale, Julie Rutherford-Fields, Joshua Sander
This work is available under the "MIT License”.
Please see the file LICENSE in this distribution
for license terms.
*/

use constants::{
    END,
    LIVES,
    SQUARE_SIZE, 
    WIN_H, 
    WIN_W
    };

use ggez::{GameResult, Context};    
use sprites::{CrabSprite};

pub struct Crab {
    form: CrabSprite,
    win_w: f32,
    win_h: f32,
    lives: i32,
    life_lost: bool,
    score: isize,
    speed: f32,
    direction: bool,
    progress: f32
}

impl Crab {
    pub fn new(w: u32, h: u32) -> Crab {
        Crab {
            form: CrabSprite::construct(
                w as f32 / 2.0,
                h as f32 - 1.0 * SQUARE_SIZE,
                SQUARE_SIZE,
                SQUARE_SIZE
            ),
            win_w: WIN_W as f32,
            win_h: WIN_H as f32,
            lives: LIVES,
            life_lost: false,
            score: 0,
            speed: 0 as f32,
            direction: false,
            progress: 0.0
        }
    }

    pub fn update(&mut self) {
        if self.direction {
            if self.get_right_edge() >= WIN_W as f32 + self.speed {
                self.lose_life();
            }
            else {
                self.form.x = self.form.x + self.speed;
            }
        } else{
            if self.form.x <= 0 as f32 {
                self.lose_life();
            }
            {
                self.form.x = self.form.x - self.speed;
            }
        }
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.form.draw(ctx)?;
        Ok(())
    }

    pub fn get_left_edge(&mut self) -> f32 {
        return self.form.x;
    }

    pub fn get_right_edge(&mut self) -> f32 {
        return self.form.x + self.form.w;
    }

    pub fn get_bottom_edge(&mut self) -> f32 {
        return self.form.y;
    }

    pub fn get_top_edge(&mut self) -> f32 {
        return self.form.y - self.form.h;
    }

    pub fn move_up(&mut self) {
        if self.form.y - SQUARE_SIZE + 1.0 > 0.0 {
            self.form.y -= SQUARE_SIZE;
            self.update_progress();
        }
    }

    pub fn move_down(&mut self) {
        if self.form.y + SQUARE_SIZE < self.win_h {
            self.form.y += SQUARE_SIZE;
        }
    }

    pub fn move_right(&mut self) {
        if self.form.x + SQUARE_SIZE - 1.0 < self.win_w - SQUARE_SIZE {
            self.form.x += SQUARE_SIZE;
        }
    }

    pub fn move_left(&mut self) {
        if self.form.x - SQUARE_SIZE + 1.0 > 0.0 {
            self.form.x -= SQUARE_SIZE;
        }
    }

    pub fn occupied_cubbie_override(&mut self) {
        self.form.y = END;
    }

    pub fn lose_life(&mut self) {
        self.lives = self.lives - 1;
        self.life_lost = true;
        // self.reset_score();
    }

    pub fn get_life_lost(&mut self) -> bool {
        return self.life_lost;
    }

    pub fn set_life_lost(&mut self) {
        self.life_lost = false;
    }

    pub fn get_lives(&mut self) -> i32 {
        return self.lives
    }

    pub fn set_lives(&mut self) {
        self.lives = LIVES;
    }

    pub fn restart_x(&mut self) {
        self.form.x = self.win_w / 2.0;
    }

    pub fn restart_y(&mut self) {
        self.form.y = self.win_h - 1.0 * SQUARE_SIZE;
    }

    pub fn get_score(&mut self) -> isize {
        return self.score;
    }

    pub fn add_to_score(&mut self, to_add: isize) {
        self.score += to_add;
    }

    // pub fn reset_score(&mut self) {
    //     self.score = 0;
    // }

    pub fn set_speed(&mut self, new_speed: f32) {self.speed = new_speed; }

    pub fn set_direction(&mut self, new_direction: bool) {self.direction = new_direction; }

    pub fn update_progress(&mut self) {
        if self.progress < self.form.y {
            self.add_to_score(10);
            self.progress += SQUARE_SIZE;
        }
    }
}

//Unit tests for Crab functions.  All paths are tested except for draw
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn move_left_succeeds_with_space() {
        let mut crab = Crab::new(WIN_W, WIN_H);

        let starting_x = crab.form.x;
        crab.move_left();

        assert_ne!(crab.form.x, starting_x);
        assert_eq!(crab.form.x, starting_x - SQUARE_SIZE);
    }

    #[test]
    fn move_left_fails_without_space() {
        let mut crab = Crab::new(0, WIN_H);

        let starting_x = crab.form.x;
        crab.move_left();

        assert_eq!(crab.form.x, starting_x);
    }

    #[test]
    fn move_right_succeeds_with_space() {
        let mut crab = Crab::new(WIN_W, WIN_H);

        let starting_x = crab.form.x;
        crab.move_right();

        assert_ne!(crab.form.x, starting_x);
        assert_eq!(crab.form.x, starting_x + SQUARE_SIZE);
    }

    #[test]
    fn move_right_fails_without_space() {
        let mut crab = Crab::new(WIN_W * 2, WIN_H);
        crab.form.x = crab.form.x * 2.0;

        let starting_x = crab.form.x;
        crab.move_right();

        assert_eq!(crab.form.x, starting_x);
    }

    #[test]
    fn move_down_succeeds_with_space() {
        let mut crab = Crab::new(WIN_W, WIN_H - SQUARE_SIZE as u32);
        let starting_y = crab.form.y;

        crab.move_down();

        assert_ne!(crab.form.y, starting_y);
        assert_eq!(crab.form.y, starting_y + SQUARE_SIZE);
    }

    #[test]
    fn move_down_fails_without_space() {
        let mut crab = Crab::new(WIN_W, WIN_H);

        let starting_y = crab.form.y;
        crab.move_down();

        assert_eq!(crab.form.y, starting_y);
    }

    #[test]
    fn move_up_succeeds_with_space() {
        let mut crab = Crab::new(WIN_W, WIN_H);

        let starting_y = crab.form.y;
        crab.move_up();

        assert_ne!(crab.form.y, starting_y);
        assert_eq!(crab.form.y, starting_y - SQUARE_SIZE);
    }

    #[test]
    fn move_up_fails_without_space() {
        let mut crab = Crab::new(WIN_W, 0);

        let starting_y = crab.form.y;
        crab.move_up();

        assert_eq!(crab.form.y, starting_y);
    }

    #[test]
    fn test_lose_life() {
        let mut crab = Crab::new(WIN_W, WIN_H);
        let updated_life = LIVES - 1;
        let life_lost = true;

        crab.lose_life();

        assert_eq!(crab.get_lives(), updated_life);
        assert_eq!(crab.get_life_lost(), life_lost);
    }

    #[test]
    fn test_get_life_lost() {
        let mut crab = Crab::new(WIN_W, WIN_H);
        let life_lost_before = false;
        let life_lost_after = true;

        assert_eq!(crab.get_life_lost(), life_lost_before);
        crab.lose_life();
        assert_eq!(crab.get_life_lost(), life_lost_after);

    }

    #[test]
    fn test_set_life_lost() {
        let mut crab = Crab::new(WIN_W, WIN_H);
        let life_lost = false;

        assert_eq!(crab.get_life_lost(), life_lost);
        crab.set_life_lost();
        assert_eq!(crab.get_life_lost(), life_lost);
    }

    #[test]
    fn test_get_lives() {
        let mut crab = Crab::new(WIN_W, WIN_H);
        let total_lives = LIVES;

        assert_eq!(crab.get_lives(), total_lives);
    }

    #[test]
    fn test_restart_x() {
        let mut crab = Crab::new(WIN_W, WIN_H);
        let starting_x = crab.form.x;

        assert_eq!(crab.form.x, starting_x);
        crab.move_left();
        assert_ne!(crab.form.x, starting_x);
        crab.restart_x();
        assert_eq!(crab.form.x, starting_x);
    }

    #[test]
    fn test_restart_y() {
        let mut crab = Crab::new(WIN_W, WIN_H);
        let starting_y = crab.form.y;

        assert_eq!(crab.form.y, starting_y);
        crab.move_up();
        assert_ne!(crab.form.y, starting_y);
        crab.restart_y();
        assert_eq!(crab.form.y, starting_y);
    }
}