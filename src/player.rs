use ggez::{Context, graphics};

use super::animation;

pub struct Player {
    pub player_image: graphics::Image,
    pub batch: graphics::spritebatch::SpriteBatch,
    pub position: (f32, f32),
    pub base_height: f32,
    pub move_speed: f32,
    pub max_speed: f32,
    pub size: (u32, u32),
    pub energy: f32,
    pub energy_base: f32,
    pub walk_animation_right: animation::Animation,
    pub walk_animation_left: animation::Animation,
    pub is_standing: bool,
    pub direction: u32, // 0 is right, 1 is left
    pub jump_wobble_height: f32, // for when the player is walking and hops up and down
    pub base_wobble_height: f32,
    pub wobbling_up: bool, // true is up, false is down
}

impl Player {
    pub fn new(ctx: &mut Context, image_location: &str, _position: (f32, f32), move_speed: f32, window_size: (f32, f32), bottom_offset: f32) -> Player {
        let player_image = graphics::Image::new(ctx, image_location).unwrap();
        let batch = graphics::spritebatch::SpriteBatch::new(player_image.clone());
        let max_speed: f32 = move_speed;
        let size: (u32, u32) = (player_image.width(), player_image.height());
        let energy_base = 100.0;
        let energy = energy_base;
        let is_standing = true;
        let direction = 0;
        // Animation
        let walk_animation_right = animation::Animation::new(2, 0.25, vec![graphics::Image::new(ctx, "/player/player_move_2_r.png").unwrap(),
            graphics::Image::new(ctx, "/player/player_move_1_r.png").unwrap()]);
        let walk_animation_left = animation::Animation::new(2, 0.25, vec![graphics::Image::new(ctx, "/player/player_move_2_l.png").unwrap(),
            graphics::Image::new(ctx, "/player/player_move_1_l.png").unwrap()]);
        let base_wobble_height = 20.0;
        let jump_wobble_height = 0.0;
        let wobbling_up = true;
        let position: (f32, f32) = (_position.0, window_size.1 - size.1 as f32 - bottom_offset);
        let base_height = position.1;
        Player{ player_image, batch, position, base_height, move_speed, max_speed, size, energy, walk_animation_right, 
            walk_animation_left, is_standing, direction, jump_wobble_height, base_wobble_height, wobbling_up, energy_base }
    }

    pub fn draw(&mut self) {    
        self.batch.add(
            graphics::DrawParam {
                dest: graphics::Point2::new(self.position.0, self.position.1),
                ..Default::default()
            }
        );
    }

    pub fn update(&mut self, _ctx: &mut Context, bg_size: (f32, f32)) {
        // bottom offset for the bar at the bottom where the GUI is being rendered.

        /*if self.position.0 <= 0.0 {
            self.position.0 = 0.0;
        } else if self.position.0 + self.size.0 as f32 >= bg_size.0 {
            self.position.0 = bg_size.0 - self.size.0 as f32;
        }*/
    }

    pub fn update_fixed(&mut self, ctx: &mut Context, dt: f64, is_a_pressed: bool, is_d_pressed: bool, win: bool, lose: bool) {
        if !win && !lose {
            let jump_wobble_interval = 175.0;

            self.energy -= 4.0 * dt as f32;
            if self.energy >= 100.0 {
                self.energy = 100.0;
            }

            // Player bouncing whilst walking is in here
            if is_a_pressed || is_d_pressed {
                if self.wobbling_up {
                    self.jump_wobble_height += jump_wobble_interval * dt as f32;
                    if self.jump_wobble_height >= self.base_wobble_height {
                        self.wobbling_up = false;
                    }
                } else {
                    self.jump_wobble_height -= jump_wobble_interval * dt as f32;
                    if self.jump_wobble_height <= 0.0 {
                        self.wobbling_up = true;
                    }
                }
                self.position.1 = self.base_height - self.jump_wobble_height;
            }

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
            } else {
                if self.jump_wobble_height > 0.0 {
                    self.jump_wobble_height -= jump_wobble_interval * dt as f32;
                } else if self.jump_wobble_height != 0.0 { // just incase it goes into negative numbers.
                    self.jump_wobble_height = 0.0;
                }
                self.position.1 = self.base_height - self.jump_wobble_height;
            }
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