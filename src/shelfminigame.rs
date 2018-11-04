use ggez::{Context, graphics};

use super::minigame;
use super::timebar;

pub struct ShelfMinigame {
    pub shelf_image: graphics::Image,
    pub trophy_clean_image: graphics::Image,
    pub trophy_dirty_image: graphics::Image,
    pub trophy_batch: graphics::spritebatch::SpriteBatch,
    
}

impl ShelfMinigame {
    pub fn new() -> ShelfMinigame {
        
    }
}