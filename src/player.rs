extern crate ggez;

use ggez::{Context, graphics};

pub struct Player {
    pub player_image: graphics::Image,
    pub batch: graphics::spritebatch::SpriteBatch,
    pub position: (f32, f32),
    pub move_speed: f32,
    pub max_speed: f32,
    pub size: (u32, u32),
    pub energy: f32,
}

impl Player {
    pub fn new(ctx: &mut Context, image_location: &str, position: (f32, f32), move_speed: f32) -> Player {
        let player_image = graphics::Image::new(ctx, image_location).unwrap();
        let batch = graphics::spritebatch::SpriteBatch::new(player_image.clone());
        let max_speed: f32 = move_speed;
        let size: (u32, u32) = (player_image.width(), player_image.height());
        let energy = 100.0;
        Player{ player_image, batch, position, move_speed, max_speed, size, energy }
    }

    pub fn draw(&mut self) {    
        self.batch.add(
            graphics::DrawParam {
                dest: graphics::Point2::new(self.position.0, self.position.1),
                ..Default::default()
            }
        );
    }

    pub fn update(&mut self, ctx: &mut Context, window_size: (f32, f32), bottom_offset: f32) {
        // bottom offset for the bar at the bottom where the GUI is being rendered.
        self.position.1 = window_size.1 - self.size.1 as f32 - bottom_offset;

        if self.position.0 <= 0.0 {
            self.position.0 = 0.0;
        } else if self.position.0 + self.size.0 as f32 >= window_size.0 {
            self.position.0 = window_size.0 - self.size.0 as f32;
        }
    }

    pub fn return_param(&mut self, dpi_scale: graphics::Point2) -> graphics::DrawParam {
        // Parameters for the player
        graphics::DrawParam {
            dest: graphics::Point2::new(0.0, 0.0),
            scale: dpi_scale,
            ..Default::default()
        }
    }
}