use ggez::{Context, graphics};

pub struct Flea {
    pub image: graphics::Image,
    pub batch: graphics::spritebatch::SpriteBatch,
    pub position: (f32, f32)
}

impl Flea {
    pub fn new(ctx: &mut Context, position: (f32, f32)) -> Flea {
        let image = graphics::Image::new(ctx, "/dog/flea.png").unwrap();
        let batch = graphics::spritebatch::SpriteBatch::new(image.clone());
        Flea { image, batch, position }
    }

    pub fn render(&mut self) {
        self.batch.add(
            graphics::DrawParam {
                dest: graphics::Point2::new(self.position.0, self.position.1),
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