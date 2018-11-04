use ggez::{Context, graphics};

pub struct DustCloud {
    pub image: graphics::Image,
    pub batch: graphics::spritebatch::SpriteBatch,
    pub position: (f32, f32),
    pub move_speed: (f32, f32),
}

impl DustCloud {
    pub fn new(ctx: &mut Context, position: (f32, f32), move_speed: (f32, f32)) -> DustCloud {
        let image = graphics::Image::new(ctx, "/shelf/dustcloud.png").unwrap();
        let batch = graphics::spritebatch::SpriteBatch::new(image.clone());
        DustCloud { image, position, move_speed, batch }
    }

    pub fn update(&mut self, dt: f64) {
        self.position.0 += self.move_speed.0 * dt as f32;
        self.position.1 += self.move_speed.1 * dt as f32;
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