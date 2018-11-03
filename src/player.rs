use ggez::{Context, graphics};

use super::animation;

pub struct Player {
    pub player_image: graphics::Image,
    pub batch: graphics::spritebatch::SpriteBatch,
    pub position: (f32, f32),
    pub move_speed: f32,
    pub max_speed: f32,
    pub size: (u32, u32),
    pub energy: f32,
    pub walk_animation_right: animation::Animation,
    pub walk_animation_left: animation::Animation,
    pub is_standing: bool,
    pub direction: u32, // 0 is right, 1 is left
}

impl Player {
    pub fn new(ctx: &mut Context, image_location: &str, position: (f32, f32), move_speed: f32) -> Player {
        let player_image = graphics::Image::new(ctx, image_location).unwrap();
        let batch = graphics::spritebatch::SpriteBatch::new(player_image.clone());
        let max_speed: f32 = move_speed;
        let size: (u32, u32) = (player_image.width(), player_image.height());
        let energy = 100.0;
        let is_standing = true;
        let direction = 0;
        // Animation
        let walk_animation_right = animation::Animation::new(2, 0.25, vec![graphics::Image::new(ctx, "/player/player_move_2_r.png").unwrap(),
            graphics::Image::new(ctx, "/player/player_move_1_r.png").unwrap()]);
        let walk_animation_left = animation::Animation::new(2, 0.25, vec![graphics::Image::new(ctx, "/player/player_move_2_l.png").unwrap(),
            graphics::Image::new(ctx, "/player/player_move_1_l.png").unwrap()]);
        Player{ player_image, batch, position, move_speed, max_speed, size, energy, walk_animation_right, 
            walk_animation_left, is_standing, direction }
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

    pub fn update_fixed(&mut self, ctx: &mut Context, dt: f64, is_a_pressed: bool, is_d_pressed: bool) {
        // Walk animation and resetting player picture to standing still.
        if is_d_pressed {
            if self.direction == 1 {
                self.walk_animation_right.current_interval = self.walk_animation_right.interval - 0.01;
            }
            self.direction = 0;
            self.batch = self.walk_animation_right.run_animation(dt, self.batch.clone());
            self.is_standing = false;
        } else if is_a_pressed {
            if self.direction == 0 {
                self.walk_animation_left.current_interval = self.walk_animation_left.interval - 0.01;
            }
            self.direction = 1;
            self.batch = self.walk_animation_left.run_animation(dt, self.batch.clone());
            self.is_standing = false;
        } else if !self.is_standing {
            if self.direction == 0 {
                self.batch.set_image(graphics::Image::new(ctx, "/player/player_stand_r.png").unwrap());
            } else {
                self.batch.set_image(graphics::Image::new(ctx, "/player/player_stand_l.png").unwrap());
            }
            self.is_standing = true;
            self.walk_animation_right.current_interval = self.walk_animation_right.interval - 0.01;
            self.walk_animation_left.current_interval = self.walk_animation_left.interval - 0.01;
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

/*
let param = graphics::DrawParam {
    dest: graphics::Point2::new(pos.0.x, pos.0.y),
    rotation: orientation.0 + asset.rotation,
    offset: na::Point2::new(0.5, 0.5),
    scale: na::Point2::new(render.scale * scale_y, render.scale) * asset.scale,
    ..Default::default()
};
*/