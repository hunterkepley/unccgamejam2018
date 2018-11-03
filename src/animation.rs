extern crate ggez;

use ggez::{Context, graphics};

pub struct Animation {
    pub frames: u32,
    pub interval: f32,
    pub current_interval: f32,
    
}

impl Animation {
    pub fn new(frames: u32, interval: f32) -> Animation {
        let current_interval = 0.0;
        Animation{ frames, interval, current_interval }
    }
}