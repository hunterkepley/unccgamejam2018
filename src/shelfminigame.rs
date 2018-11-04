use ggez::{Context, graphics};

use super::minigame;
use super::timebar;
use super::dustcloud;

use rand::Rng;

pub struct ShelfMinigame {
    pub shelf_image: graphics::Image,
    pub trophy_clean_image: graphics::Image,
    pub trophy_dirty_image: graphics::Image,
    pub trophy_really_dirty_image: graphics::Image,
    pub duster_image: graphics::Image,
    pub dust_cloud_image: graphics::Image,
    pub trophy_batch: graphics::spritebatch::SpriteBatch,
    pub shelf_batch: graphics::spritebatch::SpriteBatch,
    pub duster_batch: graphics::spritebatch::SpriteBatch,
    pub font: graphics::Font,
    pub action_text: graphics::Text,
    pub end_timer_base: f32,
    pub end_timer: f32,
    pub time_left_base: f32,
    pub time_left: f32,
    pub swipes_left_base: i32,
    pub swipes_left: i32,
    pub dust_clouds: Vec<dustcloud::DustCloud>,
    pub swipe_cooldown_base: f32,
    pub swipe_cooldown: f32,
    pub ended: bool,
    pub time_bar: timebar::TimeBar,
    pub rng: rand::ThreadRng,
    pub shelf_position: (f32, f32),
    pub duster_position: (f32, f32),
    pub duster_base_position: (f32, f32),
    pub trophy_position: (f32, f32),
    pub duster_move_timer_base: f32,
    pub duster_move_timer: f32,
    pub duster_step: i32,
}

impl ShelfMinigame {
    pub fn new(ctx: &mut Context, window_size: (f32, f32)) -> ShelfMinigame {
        let shelf_image = graphics::Image::new(ctx, "/shelf/shelf.png").unwrap();
        let trophy_clean_image = graphics::Image::new(ctx, "/shelf/trophy_clean.png").unwrap();
        let trophy_dirty_image = graphics::Image::new(ctx, "/shelf/trophy_dirty.png").unwrap();
        let duster_image = graphics::Image::new(ctx, "/shelf/feather_duster.png").unwrap();
        let dust_cloud_image = graphics::Image::new(ctx, "/shelf/dustcloud.png").unwrap();
        let trophy_really_dirty_image = graphics::Image::new(ctx, "/shelf/trophy_really_dirty.png").unwrap();
        let trophy_batch = graphics::spritebatch::SpriteBatch::new(trophy_dirty_image.clone());
        let duster_batch = graphics::spritebatch::SpriteBatch::new(duster_image.clone());
        let shelf_batch = graphics::spritebatch::SpriteBatch::new(shelf_image.clone());

        let font = graphics::Font::new(ctx, "/fonts/satumt.TTF", 36).unwrap();

        let mut rng = rand::thread_rng();

        let swipes_left_base = 5;
        let swipes_left = swipes_left_base;

        let swipe_cooldown_base = 0.3;
        let swipe_cooldown = swipe_cooldown_base;

        let end_timer_base = 1.5;
        let end_timer = end_timer_base;

        let time_left_base = 5.0;
        let time_left = time_left_base;

        let action_text = graphics::Text::new(ctx, "Press D to dust!", &font).unwrap();

        let dust_clouds: Vec<dustcloud::DustCloud> = vec![];

        let ended = false;

        let time_bar = timebar::TimeBar::new((0.0, 0.0), (window_size.0, 30.0), window_size.0);

        let shelf_position = (0.0, 0.0);
        let duster_base_position = (400.0, 300.0);
        let duster_position = duster_base_position;
        let trophy_position = (0.0, 0.0);

        let duster_move_timer_base = swipe_cooldown_base/2.0;
        let duster_move_timer = duster_move_timer_base;

        let duster_step = 0;

        ShelfMinigame{ shelf_image, trophy_clean_image, trophy_dirty_image, duster_image,
        trophy_batch, font, action_text, end_timer_base, end_timer,
        swipes_left_base, swipes_left, dust_clouds, swipe_cooldown_base, swipe_cooldown,
        ended, time_bar, time_left, time_left_base, dust_cloud_image, rng , shelf_position,
        duster_position, trophy_position, duster_batch, shelf_batch, trophy_really_dirty_image,
        duster_move_timer_base, duster_move_timer, duster_step, duster_base_position }
    }

    pub fn update(&mut self, dt: f64) {
        if !self.ended {
            self.time_bar.update(self.time_left, self.time_left_base);
        }

        for i in &mut self.dust_clouds {
            i.update(dt);
        }
    }

    pub fn update_always(&mut self, ctx: &mut Context, dt: f64, is_d_pressed: bool, window_size: (f32, f32),
    in_event: &mut bool, current_minigame: &mut minigame::Minigame, energy: &mut f32) -> bool {
        if self.time_left > 0.0 {
            self.time_left -= 1.0 * dt as f32;
        }

        if self.swipe_cooldown > 0.0 {
            self.swipe_cooldown -= 1.0*dt as f32;
        } else {
            if is_d_pressed {
                self.swipe_cooldown = self.swipe_cooldown_base;
                self.swipes_left -= 1;
                self.dust_clouds.push(dustcloud::DustCloud::new(ctx, (window_size.0/3.0 + self.rng.gen_range(-200.0, 200.0), 
                window_size.1/3.0 + self.rng.gen_range(-200.0, 200.0)), (self.rng.gen_range(-100.0, 100.0), self.rng.gen_range(-100.0, 100.0))));
                self.duster_step = 2;
            }
        }

        if self.duster_move_timer > 0.0 {
            self.duster_move_timer -= 1.0 * dt as f32;
        } else {
            self.duster_move_timer = self.duster_move_timer_base;
            self.duster_step -= 1;
        }

        if self.duster_step == 2 {
            self.duster_position.1 -= 350.0 * dt as f32;
        } else if self.duster_step == 1 {
            self.duster_position.1 += 350.0 * dt as f32;
        } else {
            self.duster_position.1 = self.duster_base_position.1;
        }
        
        if self.time_left < 0.0 { // lost
            if !self.ended {
                self.trophy_batch.set_image(self.trophy_really_dirty_image.clone());
                self.ended = true;
            }
            if self.end_timer > 0.0  {
                self.end_timer -= 1.0 * dt as f32;
            } else {
                *in_event = false;
                self.time_left = self.time_left_base;
                self.swipes_left = self.swipes_left_base;
                self.swipe_cooldown = 0.0;
                self.ended = false;
                self.trophy_batch.set_image(self.trophy_dirty_image.clone());
                self.dust_clouds.clear();
                self.end_timer = self.end_timer_base;
                self.duster_step = 0;
                self.duster_move_timer = self.duster_move_timer_base;
                self.duster_position.1 = self.duster_base_position.1;
                
                *energy -= 30.0;
                return false;
            }
        }

        if self.swipes_left <= 0 { // won
            if !self.ended {
                self.trophy_batch.set_image(self.trophy_clean_image.clone());
                self.ended = true;
            }
            if self.end_timer > 0.0 {
                self.end_timer -= 1.0 * dt as f32;
            } else {
                *in_event = false;
                self.time_left = self.time_left_base;
                self.swipes_left = self.swipes_left_base;
                self.swipe_cooldown = 0.0;
                self.ended = false;
                self.trophy_batch.set_image(self.trophy_dirty_image.clone());
                self.dust_clouds.clear();
                self.end_timer = self.end_timer_base;
                self.duster_step = 0;
                self.duster_move_timer = self.duster_move_timer_base;
                self.duster_position.1 = self.duster_base_position.1;
                
                *energy += 20.0;
                return false;
            }
        }
        true
    }

    pub fn draw(&mut self) { 
        self.trophy_batch.add(
            graphics::DrawParam {
                dest: graphics::Point2::new(self.trophy_position.0, self.trophy_position.1),
                ..Default::default()
            }
        );
        self.shelf_batch.add(
            graphics::DrawParam {
                dest: graphics::Point2::new(self.shelf_position.0, self.shelf_position.1),
                ..Default::default()
            }
        );
        self.duster_batch.add(
            graphics::DrawParam {
                dest: graphics::Point2::new(self.duster_position.0, self.duster_position.1),
                ..Default::default()
            }
        );
        for i in &mut self.dust_clouds {
            i.render();
        }
    }

    pub fn return_param(&mut self, dpi_scale: graphics::Point2) -> graphics::DrawParam {
        graphics::DrawParam {
            dest: graphics::Point2::new(0.0, 0.0),
            scale: dpi_scale,
            ..Default::default()
        }
    }
}