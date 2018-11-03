use ggez::{Context, graphics};

pub struct RobberMinigame {
    robber_image: graphics::Image,
    robber_dead_image: graphics::Image,
    gun_image: graphics::Image,
    gun_fired_image: graphics::Image,
    robber_batch: graphics::spritebatch::SpriteBatch,
    gun_batch: graphics::spritebatch::SpriteBatch
}

impl RobberMinigame {
    pub fn new(ctx: &mut Context, robber_image_location: &str, robber_dead_image_location: &str,
    gun_image_location: &str, gun_fired_image_location: &str) -> RobberMinigame {
        let robber_image = graphics::Image::new(ctx, robber_image_location).unwrap();
        let robber_dead_image = graphics::Image::new(ctx, robber_dead_image_location).unwrap();
        let gun_image = graphics::Image::new(ctx, gun_image_location).unwrap();
        let gun_fired_image = graphics::Image::new(ctx, gun_fired_image_location).unwrap();
        let robber_batch = graphics::spritebatch::SpriteBatch::new(robber_image.clone());
        let gun_batch = graphics::spritebatch::SpriteBatch::new(gun_image.clone());
        RobberMinigame{ robber_image, robber_dead_image, gun_image, gun_fired_image, robber_batch, gun_batch }
    }

    pub fn update(&mut self) {
        
    }

    pub fn draw(&mut self) {

    }
}