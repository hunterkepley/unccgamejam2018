use ggez::{Context, graphics};

use super::minigame;
use super::timebar;

pub struct RobberMinigame {
    pub robber_image: graphics::Image,
    pub robber_dead_image: graphics::Image,
    pub robber_win_image: graphics::Image,
    pub gun_image: graphics::Image,
    pub gun_fired_image: graphics::Image,
    pub robber_batch: graphics::spritebatch::SpriteBatch,
    pub gun_batch: graphics::spritebatch::SpriteBatch,
    pub robber_position: (f32, f32),
    pub gun_position: (f32, f32),
    pub move_interval: f32,
    pub move_interval_base: f32,
    pub misses_left_base: i32,
    pub shots_left_base: i32,
    pub time_left_base: f32,
    pub misses_left: i32, // How many you can miss
    pub shots_left: i32, // Shots to hit left
    pub time_left: f32, //......time left
    pub font: graphics::Font,
    pub big_font: graphics::Font,
    pub shots_left_text: graphics::Text,
    pub misses_left_text: graphics::Text,
    pub action_text: graphics::Text,
    pub move_speed: f32,
    pub shot_interval_base: f32,
    pub shot_interval: f32,
    pub end_timer_base: f32,
    pub end_timer: f32,
    pub robber_dead: bool,
    pub fire_timer_base: f32,
    pub fire_timer: f32,
    pub time_bar: timebar::TimeBar,
}

impl RobberMinigame {
    pub fn new(ctx: &mut Context, window_size: (f32, f32), robber_image_location: &str, robber_dead_image_location: &str,
    robber_win_image_location: &str, gun_image_location: &str, gun_fired_image_location: &str) -> RobberMinigame {
        let robber_image = graphics::Image::new(ctx, robber_image_location).unwrap();
        let robber_dead_image = graphics::Image::new(ctx, robber_dead_image_location).unwrap();
        let robber_win_image = graphics::Image::new(ctx, robber_win_image_location).unwrap();
        let gun_image = graphics::Image::new(ctx, gun_image_location).unwrap();
        let gun_fired_image = graphics::Image::new(ctx, gun_fired_image_location).unwrap();
        let robber_batch = graphics::spritebatch::SpriteBatch::new(robber_image.clone());
        let gun_batch = graphics::spritebatch::SpriteBatch::new(gun_image.clone());
        let robber_position = (0.0, 0.0);
        let gun_position = (window_size.0 - gun_image.width() as f32, window_size.1 - gun_image.height() as f32);

        let font = graphics::Font::new(ctx, "/fonts/DejaVuSerif.ttf", 24).unwrap();
        let big_font = graphics::Font::new(ctx, "/fonts/satumt.TTF", 36).unwrap();

        let shots_left = 4;
        let misses_left = 2;

        let time_left = 5.5;

        let shots_left_base = shots_left;
        let misses_left_base = misses_left;
        let time_left_base = time_left;

        let shots_left_text = graphics::Text::new(ctx, &format!("Shots to hit: {:?}", shots_left), &font).unwrap();
        let misses_left_text = graphics::Text::new(ctx, &format!("Misses left: {:?}", misses_left), &font).unwrap();
        let action_text = graphics::Text::new(ctx, "Press F to fire!", &big_font).unwrap();

        let move_interval_base = 1.0;
        let move_interval = move_interval_base;

        let shot_interval_base = 0.76;
        let shot_interval = 0.0;

        let move_speed = 350.0;

        let end_timer_base = 1.5;
        let end_timer = end_timer_base;

        let robber_dead = false;

        let fire_timer_base = 0.1;
        let fire_timer = 0.0;
        
        let time_bar = timebar::TimeBar::new((0.0, 0.0), (window_size.0, 30.0), window_size.0);

        RobberMinigame{ robber_image, robber_dead_image, robber_win_image, gun_image, gun_fired_image, robber_batch, gun_batch,
        robber_position, gun_position, move_interval, move_interval_base, shots_left_base, misses_left_base,
        time_left_base, misses_left, shots_left, time_left, font, big_font, shots_left_text, misses_left_text, 
        action_text, move_speed, shot_interval_base, shot_interval, end_timer_base, end_timer, robber_dead, fire_timer_base,
        fire_timer, time_bar }
    }

    pub fn update(&mut self, dt: f64) {
        if !self.robber_dead {
            self.time_bar.update(self.time_left, self.time_left_base);

            if self.move_interval > 0.0 {
                self.move_interval -= 1.0 * dt as f32;
            } else {
                self.move_interval = self.move_interval_base;
                self.move_speed *= -1.0;
            }
            self.robber_position.0 += self.move_speed * dt as f32;
        }
    }

    pub fn update_always(&mut self, ctx: &mut Context, dt: f64, is_f_pressed: bool, window_size: (f32, f32), 
    in_event: &mut bool, current_minigame: &mut minigame::Minigame, energy: &mut f32) -> bool {

        if self.time_left > 0.0 {
            self.time_left -= 1.0 * dt as f32;
        }

        if self.shot_interval > 0.0 {
            self.shot_interval-=1.0*dt as f32;
        } else {
            let mut missed = true;
            if is_f_pressed {
                self.fire_timer = self.fire_timer_base;
                self.gun_batch.set_image(self.gun_fired_image.clone());
                if self.robber_position.0 <= window_size.0*0.2 {
                    missed = false;
                }
                if missed {
                    self.misses_left -= 1;
                    self.misses_left_text = graphics::Text::new(ctx, &format!("Misses left: {:?}", self.misses_left), &self.font).unwrap();
                    self.shot_interval = self.shot_interval_base;
                } else {
                    self.shots_left -= 1;
                    self.shots_left_text = graphics::Text::new(ctx, &format!("Shots to hit: {:?}", self.shots_left), &self.font).unwrap();
                    self.shot_interval = self.shot_interval_base;
                }
            }
        }

        if self.fire_timer > 0.0 {
            self.fire_timer -= 1.0 * dt as f32;
        } else {
            self.gun_batch.set_image(self.gun_image.clone());
        }

        if self.misses_left <= 0 || self.time_left < 0.0 { // lost
            if !self.robber_dead {
                self.robber_batch.set_image(self.robber_win_image.clone());
                self.robber_dead = true;
            }
            if self.end_timer > 0.0 && self.shots_left > 0 {
                self.end_timer -= 1.0 * dt as f32;
            } else {
                *in_event = false;
                self.time_left = self.time_left_base;
                self.shots_left = self.shots_left_base;
                self.misses_left = self.misses_left_base;
                self.shot_interval = 0.0;
                *current_minigame = minigame::Minigame::Nothing;
                self.end_timer = self.end_timer_base;
                self.robber_batch.set_image(self.robber_image.clone());
                self.gun_batch.set_image(self.gun_image.clone());
                self.robber_dead = false;
                self.fire_timer = 0.0;
                self.shots_left_text = graphics::Text::new(ctx, &format!("Shots to hit: {:?}", self.shots_left), &self.font).unwrap();
                self.misses_left_text = graphics::Text::new(ctx, &format!("Misses left: {:?}", self.misses_left), &self.font).unwrap();

                *energy -= 15.0;
                return true;
            }
        }
        if self.shots_left <= 0 { // won
            if !self.robber_dead {
                self.robber_batch.set_image(self.robber_dead_image.clone());
                self.robber_dead = true;
            }
            if self.end_timer > 0.0 {
                self.end_timer -= 1.0 * dt as f32;
            } else {
                *in_event = false;
                self.time_left = self.time_left_base;
                self.shots_left = self.shots_left_base;
                self.misses_left = self.misses_left_base;
                self.shot_interval = 0.0;
                *current_minigame = minigame::Minigame::Nothing;
                self.end_timer = self.end_timer_base;
                self.robber_batch.set_image(self.robber_image.clone());
                self.gun_batch.set_image(self.gun_image.clone());
                self.robber_dead = false;
                self.fire_timer = 0.0;
                self.shots_left_text = graphics::Text::new(ctx, &format!("Shots to hit: {:?}", self.shots_left), &self.font).unwrap();
                self.misses_left_text = graphics::Text::new(ctx, &format!("Misses left: {:?}", self.misses_left), &self.font).unwrap();

                *energy += 25.0;
                return true;
            }
        }
        false
    }

    pub fn draw(&mut self) {
        self.robber_batch.add(
            graphics::DrawParam {
                dest: graphics::Point2::new(self.robber_position.0, self.robber_position.1),
                ..Default::default()
            }
        );

        self.gun_batch.add(
            graphics::DrawParam {
                dest: graphics::Point2::new(self.gun_position.0, self.gun_position.1),
                ..Default::default()
            }
        );
    }

    pub fn return_param(&mut self, dpi_scale: graphics::Point2) -> graphics::DrawParam {
        graphics::DrawParam {
            dest: graphics::Point2::new(0.0, 0.0),
            scale: dpi_scale,
            ..Default::default()
        }
    }
}