extern crate ggez;

use ggez::conf;
use ggez::event;
use ggez::graphics;
use ggez::ContextBuilder;
use ggez::timer;
use ggez::{Context, GameResult};
use std::env;
use std::path;
use sdl2::keyboard::Keycode;
use sdl2::keyboard::Mod;
use sdl2::keyboard;
use sdl2::mouse;

use std::time::Instant;

mod player;

struct MainState {
    text: graphics::Text,
    frames: usize,
    pl: player::Player,
    current_time: f64,
    current_duration: Instant,
    accumulator: f64,
    is_a_pressed: bool,
    is_d_pressed: bool,
}

const WINDOW_SIZE: (f32, f32) = (1024.0, 768.0);

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let font = graphics::Font::new(ctx, "/DejaVuSerif.ttf", 12)?;
        let text = graphics::Text::new(ctx, "INVIGORATION STATION", &font)?;
        let pl = player::Player::new(ctx, "/player.png", (0.0, 0.0), 200.0);
        let current_duration = Instant::now();
        let current_time = current_duration.elapsed().as_secs() as f64;
        let accumulator = 0.0;
        let is_a_pressed = false;
        let is_d_pressed = false;
        let s = MainState { text, frames: 0, pl, current_duration, current_time, 
            accumulator, is_a_pressed, is_d_pressed };
        Ok(s)
    }
}

fn handle_input(pl: &mut player::Player, ctx: &mut Context,
                is_a_pressed: bool, is_d_pressed: bool) {

    if is_a_pressed {
    }

    if is_d_pressed {
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        const DT: f64 = 0.01;

        let duration = Instant::now();
        let new_time = timer::duration_to_f64(duration.duration_since(self.current_duration));

        //println!("Acc: {:?} dt: {:?}; {:?}", self.accumulator, DT, new_time);
        self.current_time = new_time;
        self.current_duration = duration;

        self.accumulator += new_time;

        // Updates that are non-critical time based
        // Update player based on user input
        handle_input(&mut self.pl, ctx, self.is_a_pressed, self.is_d_pressed);

        // self.pl.update(ctx, WINDOW_SIZE);

        
        // Updates that involve physics/can be affected by time
        while self.accumulator >= DT {
            // Update physics
            // self.pl.update_physics(ctx, DT, WINDOW_SIZE, self.is_a_pressed, self.is_d_pressed);
            self.accumulator -= DT;
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        let size = graphics::get_size(ctx);
        let origin = graphics::DrawParam {
            dest: graphics::Point2::new(0.0, 0.0),
            ..Default::default()
        };
        // for re-rendering canvases, we need to take the DPI into account
        let dpiscale = {
            let dsize = graphics::get_drawable_size(ctx);
            graphics::Point2::new(
                size.0 as f32 / dsize.0 as f32,
                size.1 as f32 / dsize.1 as f32,
            )
        };
        let _canvas_origin = graphics::DrawParam {
            scale: dpiscale,
            ..origin
        };

        // Player drawing
        self.pl.draw();
        let pl_param = self.pl.return_param(dpiscale);
        graphics::draw_ex(ctx, &self.pl.batch, pl_param)?;
        self.pl.batch.clear();
        // End of player drawing

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