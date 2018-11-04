
use ggez::{Context, graphics};

use super::minigame;
use super::timebar;
use super::flea;

use rand::Rng;

pub struct DogMinigame {
    pub dog_image: graphics::Image,
    pub dog_happy_image: graphics::Image,
    pub dog_sad_image: graphics::Image,
    pub flea_picker: graphics::Image,
    pub dog_batch: graphics::spritebatch::SpriteBatch,
    pub fleas: Vec<flea>
}

impl DogMinigame {
    pub fn new() -> DogMinigame {
        
        DogMinigame{ }
    }

    pub fn update(&mut self) {

    }

    pub fn render(&mut self) {
        
    }
}