use ggez::{Context, graphics};

pub struct Object {
    pub image: graphics::Image,
    pub batch: graphics::spritebatch::SpriteBatch,
    pub position: (f32, f32),
    pub size: (u32, u32),
}

impl Object {
    pub fn new(ctx: &mut Context, image_location: &str, position: (f32, f32)) -> Object {
        let image = graphics::Image::new(ctx, image_location).unwrap();
        let batch = graphics::spritebatch::SpriteBatch::new(image.clone());
        let size = (image.width(), image.height());
        Object { image, batch, position, size }
    }

    pub fn update(&mut self) {
        
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