extern crate ggez;

use ggez::conf;
use ggez::event;
use ggez::graphics;
use ggez::{Context, GameResult};
use std::env;
use std::path;
use ggez::ContextBuilder;

struct MainState {
    text: graphics::Text,
    frames: usize,
}

const WINDOW_SIZE: (f32, f32) = (1024.0, 768.0);

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let font = graphics::Font::new(ctx, "/DejaVuSerif.ttf", 12)?;
        let text = graphics::Text::new(ctx, "INVIGORATION STATION", &font)?;

        let s = MainState { text, frames: 0 };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        // Drawables are drawn from their top-left corner.
        let dest_point = graphics::Point2::new(10.0, 10.0);
        graphics::draw(ctx, &self.text, dest_point, 0.0)?;
        graphics::present(ctx);

        self.frames += 1;
        if (self.frames % 100) == 0 {
            println!("FPS: {}", ggez::timer::get_fps(ctx));
        }

        Ok(())
    }
}

pub fn main() {
    let c = conf::Conf::new();
    let ctx = &mut ContextBuilder::new("invigorationstation", "hunterkepley")
        .window_setup(conf::WindowSetup{
            title: "Invigoration Station".to_owned(),
            icon: "".to_owned(),
            resizable: false,
            allow_highdpi: true,
            samples: conf::NumSamples::One
        })
        .window_mode(conf::WindowMode{
            width: WINDOW_SIZE.0 as u32,
            height: WINDOW_SIZE.1 as u32,
            borderless: false,
            fullscreen_type: conf::FullscreenType::Off,
            vsync: true,
            min_width: 0,
            min_height: 0,
            max_width: 0,
            max_height: 0
        })
        .build().unwrap();
    
    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        ctx.filesystem.mount(&path, true);
    }

    let state = &mut MainState::new(ctx).unwrap();
    if let Err(e) = event::run(ctx, state) {
        println!("Error encountered: {}", e);
    } else {
        println!("Game exited cleanly.");
    }
}