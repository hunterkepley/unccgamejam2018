use ggez::{Context, graphics};

use super::minigame;

pub struct Object {
    pub image: graphics::Image,
    pub event_image: graphics::Image,
    pub batch: graphics::spritebatch::SpriteBatch,
    pub position: (f32, f32),
    pub size: (u32, u32),
    pub has_event: bool,
    pub this_minigame: minigame::Minigame,
    pub neutral_position: (f32, f32),
    pub event_position: (f32, f32)
}

impl Object {
    pub fn new(ctx: &mut Context, image_location: &str, event_image_location: &str, position: (f32, f32),
    event_position: (f32, f32), this_minigame: minigame::Minigame) -> Object {
        let image = graphics::Image::new(ctx, image_location).unwrap();
        let event_image = graphics::Image::new(ctx, event_image_location).unwrap();
        let batch = graphics::spritebatch::SpriteBatch::new(image.clone());
        let size = (image.width(), image.height());
        let has_event = false;
        let neutral_position = position;
        Object { image, event_image, batch, position, size, has_event, this_minigame, neutral_position, event_position }
    }

    pub fn start_event(&mut self, background_image: graphics::Image, window_size: (f32, f32)) {
        self.has_event = true;
        self.size = (self.event_image.width(), self.event_image.height());
        self.batch.set_image(self.event_image.clone());
        // Move it to new image location
        self.position = self.event_position;
    }

    pub fn end_event(&mut self, background_image: graphics::Image, window_size: (f32, f32)) { // USE THIS TO END THE EVENT AFTER LOST/WON
        self.has_event = false;
        self.size = (self.image.width(), self.image.height());
        self.batch.set_image(self.image.clone());
        // Move it back
        self.position = self.neutral_position;
    }

    pub fn update(&mut self, gc_center: (f32, f32), pl_size: (u32, u32), is_x_pressed: bool, in_event: &mut bool, 
    current_minigame: &mut minigame::Minigame) {
        if self.has_event { // If the event is active, check if player is colliding
            if gc_center.0 + pl_size.0 as f32/2.0 as f32 >= self.position.0 &&
            gc_center.0 - pl_size.0 as f32/2.0 <= self.position.0 + self.size.0 as f32 {
                if is_x_pressed {
                    *in_event = true;
                    *current_minigame = self.this_minigame.clone();
                }
            }
        }
    }

    pub fn draw(&mut self, camera_offset: (f32, f32)) {
        self.batch.add(
            graphics::DrawParam {
                dest: graphics::Point2::new(self.position.0+camera_offset.0, self.position.1+camera_offset.1),
                ..Default::default()
            }
        );
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