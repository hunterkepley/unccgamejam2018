extern crate ggez;

use ggez::{graphics, Context};

pub struct EnergyBar {
    pub position: (f32, f32),
    pub size: (f32, f32),
    pub current_width: f32,
    pub max_width: f32,
    pub rect: graphics::Rect,
    pub energy_rect: graphics::Rect,
    pub offset: f32,
}

impl EnergyBar {
    pub fn new(position: (f32, f32), size: (f32, f32), max_width: f32) -> EnergyBar {
        let current_width = size.0;
        // Background for energy bar
        let rect = graphics::Rect::new(position.0, position.1, size.0, size.1);
        // Actual energy bar
        let offset = 5.0;
        let energy_rect = graphics::Rect::new(position.0 + offset, position.1 + offset, size.0 - 2.0*offset, size.1 - 2.0*offset);
        EnergyBar{ position, size, current_width, max_width, rect, energy_rect, offset }
    }

    pub fn update(&mut self, energy: f32) {
        // Resize bar to fit current energy level yet keep ratio of max_width
        self.current_width = self.max_width * (energy / 100.0);
        self.energy_rect = graphics::Rect::new(self.position.0 + self.offset, self.position.1 + self.offset, 
            self.current_width - 2.0*self.offset, self.size.1 - 2.0*self.offset);
    }

    pub fn draw(&mut self, ctx: &mut Context) {
        graphics::set_color(ctx, (50, 50, 50).into()).unwrap(); // Gray
        graphics::rectangle(ctx, graphics::DrawMode::Fill, self.rect).unwrap();
        graphics::set_color(ctx, (255, 255, 70).into()).unwrap(); // Yellow
        graphics::rectangle(ctx, graphics::DrawMode::Fill, self.energy_rect).unwrap();
        graphics::set_color(ctx, (255, 255, 255).into()).unwrap(); // Reset to white
    }
}