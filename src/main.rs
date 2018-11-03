extern crate ggez;

use ggez::conf;
use ggez::event;
use ggez::graphics;
use ggez::graphics::DrawMode;
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
mod animation;
mod energy_bar;

struct MainState {
    text: graphics::Text,
    frames: usize,
    background_image: graphics::Image,
    pl: player::Player,
    energy_bar: energy_bar::EnergyBar,
    current_time: f64,
    current_duration: Instant,
    accumulator: f64,
    is_a_pressed: bool,
    is_d_pressed: bool,
}

const WINDOW_SIZE: (f32, f32) = (1024.0, 768.0);

fn get_dt(ctx: &mut Context) -> f32{
    timer::duration_to_f64(timer::get_delta(ctx)) as f32
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let font = graphics::Font::new(ctx, "/DejaVuSerif.ttf", 12)?;
        let text = graphics::Text::new(ctx, "INVIGORATION STATION", &font)?;
        
        // Stuff drawn in background / objects / background itself
        let background_image = graphics::Image::new(ctx, "/background.png").unwrap();
        
        // Player
        let pl = player::Player::new(ctx, "/player_stand.png", (0.0, 0.0), 200.0);
        let pl_walk_animation = animation::Animation::new(2, 50.0, vec![graphics::Image::new(ctx, "/player_move_1.png").unwrap(),
            graphics::Image::new(ctx, "/player_move_2.png").unwrap()]);
        
        // GUI elements
        let energy_bar_size: (f32, f32) = (300.0, 35.0);
        let energy_bar = energy_bar::EnergyBar::new((WINDOW_SIZE.0/2.0 - (energy_bar_size.0/2.0), WINDOW_SIZE.1-energy_bar_size.1), 
            energy_bar_size, energy_bar_size.0); // (position: (f32, f32), size: (f32, f32), maxWidth: f32)

        // Random variables for phsyics and such 
        let current_duration = Instant::now();
        let current_time = current_duration.elapsed().as_secs() as f64;
        let accumulator = 0.0;
        let is_a_pressed = false;
        let is_d_pressed = false;
        let s = MainState { text, frames: 0, background_image, pl, energy_bar, current_duration, current_time, 
            accumulator, is_a_pressed, is_d_pressed };
        Ok(s)
    }
}

fn handle_input(pl: &mut player::Player, ctx: &mut Context,
                is_a_pressed: bool, is_d_pressed: bool) {

    if is_a_pressed {
        pl.position.0 -= pl.move_speed * get_dt(ctx);
    }

    if is_d_pressed {
        pl.position.0 += pl.move_speed * get_dt(ctx);
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
        // Update GUI, make dirty update later?
        self.energy_bar.update(self.pl.energy);

        // Update player based on user input
        handle_input(&mut self.pl, ctx, self.is_a_pressed, self.is_d_pressed);

        self.pl.update(ctx, WINDOW_SIZE, self.energy_bar.size.1);
        
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

        // Background objects / Background itself
        let bg_dst = graphics::Point2::new(0.0, -5.0);
        graphics::draw(ctx, &self.background_image, bg_dst, 0.0)?;

        // Player drawing
        self.pl.draw();
        let pl_param = self.pl.return_param(dpiscale);
        graphics::draw_ex(ctx, &self.pl.batch, pl_param)?;
        self.pl.batch.clear();
        // End of player drawing
        
        // GUI drawing
        self.energy_bar.draw(ctx);

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

    fn key_down_event(&mut self, ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        if keycode == keyboard::Keycode::Escape {
            ctx.quit().expect("Should never fail");
        }
        if keycode == keyboard::Keycode::A {
            self.is_a_pressed = true;
        }
        if keycode == keyboard::Keycode::D {
            self.is_d_pressed = true;
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: Keycode, _keymod: Mod, _release: bool) {
        if keycode == keyboard::Keycode::A {
            self.is_a_pressed = false;
        }
        if keycode == keyboard::Keycode::D {
            self.is_d_pressed = false;
        }
    }

    fn focus_event(&mut self, _ctx: &mut Context, _gained: bool) {

    }

    fn quit_event(&mut self, _ctx: &mut Context) -> bool {
        println!("quit_event() callback called, quitting...");
        false
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