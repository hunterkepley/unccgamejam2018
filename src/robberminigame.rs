use ggez::{Context, graphics};

pub struct RobberMinigame {
    pub robber_image: graphics::Image,
    pub robber_dead_image: graphics::Image,
    pub gun_image: graphics::Image,
    pub gun_fired_image: graphics::Image,
    pub robber_batch: graphics::spritebatch::SpriteBatch,
    pub gun_batch: graphics::spritebatch::SpriteBatch,
    pub robber_position: (f32, f32),
    pub gun_position: (f32, f32),
    pub move_interval: f32,
    pub move_interval_base: f32,
    pub shots_left: i32, // How many needed to kill
    pub misses_left: i32, // How many you can miss
    pub time_left: f32, //......time left
    pub shots_left_text: graphics::Text,
    pub misses_left_text: graphics::Text,
    pub time_left_text: graphics::Text,
    pub action_text: graphics::Text,
}

impl RobberMinigame {
    pub fn new(ctx: &mut Context, window_size: (f32, f32), robber_image_location: &str, 
    robber_dead_image_location: &str, gun_image_location: &str, gun_fired_image_location: &str) -> RobberMinigame {
        let robber_image = graphics::Image::new(ctx, robber_image_location).unwrap();
        let robber_dead_image = graphics::Image::new(ctx, robber_dead_image_location).unwrap();
        let gun_image = graphics::Image::new(ctx, gun_image_location).unwrap();
        let gun_fired_image = graphics::Image::new(ctx, gun_fired_image_location).unwrap();
        let robber_batch = graphics::spritebatch::SpriteBatch::new(robber_image.clone());
        let gun_batch = graphics::spritebatch::SpriteBatch::new(gun_image.clone());
        let robber_position = (0.0, 0.0);
        let gun_position = (window_size.0 - gun_image.width() as f32, window_size.1 - gun_image.height() as f32);

        let font = graphics::Font::new(ctx, "/fonts/DejaVuSerif.ttf", 24).unwrap();
        let big_font = graphics::Font::new(ctx, "/fonts/satumt.TTF", 36).unwrap();
        
        let shots_left_text = graphics::Text::new(ctx, "Shots left: ", &font).unwrap();
        let misses_left_text = graphics::Text::new(ctx, "Misses left: ", &font).unwrap();
        let time_left_text = graphics::Text::new(ctx, "Time left: ", &font).unwrap();
        let action_text = graphics::Text::new(ctx, "Press X to shoot!", &big_font).unwrap();

        let move_interval_base = 10.0;
        let move_interval = move_interval_base;

        let shots_left = 10;
        let misses_left = 2;

        let time_left = 50.0;

        RobberMinigame{ robber_image, robber_dead_image, gun_image, gun_fired_image, robber_batch, gun_batch,
        robber_position, gun_position, move_interval, move_interval_base, shots_left, misses_left, time_left,
        shots_left_text, misses_left_text, time_left_text, action_text }
    }

    pub fn update(&mut self) {
        
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