use ggez::{Context, graphics};

pub struct Animation {
    pub frames: u32,
    pub interval: f32,
    pub current_interval: f32,
    pub frame_number: u32,
    pub images: Vec<graphics::Image>,
}

impl Animation {
    pub fn new(frames: u32, interval: f32, images: Vec<graphics::Image>) -> Animation {
        let current_interval = 0.0;
        let frame_number = 0;
        Animation{ frames, interval, current_interval, frame_number, images }
    }

    pub fn run_animation(&mut self, dt: f32) {
        if self.current_interval >= self.interval {
            self.current_interval = 0.0;
            self.frame_number+=1;
            if self.frame_number > self.frames {
                self.frame_number = 1;
            }
        } else {
            self.current_interval += 1.0 * dt;
        }

    }
}