
use ggez::{Context, graphics};

use super::minigame;
use super::timebar;
use super::flea;

use rand::Rng;

pub struct DogMinigame {
    pub dog_image: graphics::Image,
    pub dog_happy_image: graphics::Image,
    pub dog_sad_image: graphics::Image,
    pub dog_batch: graphics::spritebatch::SpriteBatch,
    pub fleas: Vec<flea::Flea>,
    pub time_bar: timebar::TimeBar,
    pub ended: bool,
    pub end_timer_base: f32,
    pub end_timer: f32,
    pub rng: rand::ThreadRng,
    pub dog_position: (f32, f32),
    pub time_left_base: f32,
    pub time_left: f32,
    pub fleas_left_base: i32,
    pub fleas_left: i32,
    pub key_to_press: i32, // 0 is A, 1 is D
    pub shakes_per_flea_base: i32,
    pub shakes_per_flea: i32,
    pub action_text: graphics::Text,
    pub font: graphics::Font,
}

impl DogMinigame {
    pub fn new(ctx: &mut Context, window_size: (f32, f32)) -> DogMinigame {
        let dog_image = graphics::Image::new(ctx, "/dog/dog_normal.png").unwrap();
        let dog_happy_image = graphics::Image::new(ctx, "/dog/dog_happy.png").unwrap();
        let dog_sad_image = graphics::Image::new(ctx, "/dog/dog_sad.png").unwrap();
        let mut fleas: Vec<flea::Flea> = vec![];
        let dog_batch = graphics::spritebatch::SpriteBatch::new(dog_image.clone());
        let dog_position = (window_size.0/2.0 - dog_image.width() as f32/2.0,
        window_size.1/2.0 - dog_image.height() as f32/2.0);
       
        let time_bar = timebar::TimeBar::new((0.0, 0.0), (window_size.0, 30.0), window_size.0);

        let mut rng = rand::thread_rng();

        let end_timer_base = 1.5;
        let end_timer = end_timer_base;

        let ended = false;

        let fleas_left_base = 5;
        let fleas_left = fleas_left_base;

        let key_to_press = 0;

        let shakes_per_flea_base = 5;
        let shakes_per_flea = shakes_per_flea_base;

        for i in 0..fleas_left_base {
            let rp = (rng.gen_range(dog_position.0, dog_position.0 + dog_image.width() as f32),
            rng.gen_range(dog_position.1, dog_position.1 + dog_image.height() as f32));
            fleas.push(flea::Flea::new(ctx, rp));
        }

        let time_left_base = 3.0;
        let time_left = time_left_base;

        let font = graphics::Font::new(ctx, "/fonts/satumt.TTF", 36).unwrap();

        let action_text = graphics::Text::new(ctx, "Press A and D to shake!", &font).unwrap();

        DogMinigame{ dog_image, dog_happy_image, dog_sad_image, fleas, dog_batch,
        time_bar, end_timer_base, end_timer, rng, ended, dog_position, time_left_base,
        time_left, fleas_left_base, fleas_left, key_to_press, shakes_per_flea_base,
        shakes_per_flea, font, action_text }
    }

    pub fn update(&mut self, dt: f64) {
        if !self.ended {
            self.time_bar.update(self.time_left, self.time_left_base);
        }
    }

    pub fn update_always(&mut self, ctx: &mut Context, dt: f64, is_a_pressed: bool,
    is_d_pressed: bool, window_size: (f32, f32), in_event: &mut bool, 
    current_minigame: &mut minigame::Minigame, energy: &mut f32) -> bool {
        if self.time_left > 0.0 {
            self.time_left -= 1.0 * dt as f32;
        }

        if self.shakes_per_flea > 0 {
            if self.key_to_press == 0 && is_a_pressed {
                self.key_to_press = 1;
                self.shakes_per_flea-=1;
            } else if self.key_to_press == 1 && is_d_pressed {
                self.key_to_press = 0;
                self.shakes_per_flea-=1;
            }
        } else {
            self.fleas_left-=1;
            self.shakes_per_flea = self.shakes_per_flea_base;
        }

        if self.fleas_left <= 0 {
            // won
        } else {
            if self.time_left <= 0.0 {
                // lost
            }
        }
        false
    }

    pub fn draw(&mut self) {
        self.dog_batch.add(
            graphics::DrawParam {
                dest: graphics::Point2::new(self.dog_position.0, self.dog_position.1),
                ..Default::default()
            }
        );

        for i in &mut self.fleas {
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