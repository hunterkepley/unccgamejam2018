extern crate ggez;
extern crate rand;

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

use rand::Rng;

mod player;
mod animation;
mod energy_bar;
mod camera;
mod object;
mod minigame;
mod robberminigame;
mod shelfminigame;
mod timebar;
mod dustcloud;

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
    is_x_pressed: bool,
    is_f_pressed: bool,
    gc: camera::Camera,
    bg_position: (f32, f32),
    porch_object: object::Object,
    objects: Vec<object::Object>,
    event_timer: f32,
    event_timer_base: f32,
    in_event: bool,
    current_minigame: minigame::Minigame,
    robber_minigame: robberminigame::RobberMinigame,
    shelf_minigame: shelfminigame::ShelfMinigame,
    solid_background: graphics::Image,
    current_minigame_index: i32,
    game_time_bar: timebar::TimeBar,
    game_time_left_base: f32,
    game_time_left: f32,
    win: bool,
    lose: bool,
    notify_image: graphics::Image,
    notify_position: (f32, f32),
    show_notify: bool,
    notify_timer_base: f32,
    notify_timer: f32,
    notify_blink: bool,
    small_notify_image: graphics::Image,
    show_small_notify: bool,
    rng: rand::ThreadRng,
    in_menu: bool,
    end_game: bool,
    restart_game: bool,
    menu_image: graphics::Image,
    end_timer: f32,
    end_timer_base: f32,
    player_win_image: graphics::Image,
    player_lose_image: graphics::Image,
    lose_image: graphics::Image,
    win_image: graphics::Image,
}

const WINDOW_SIZE: (f32, f32) = (1024.0, 768.0);

fn get_dt(ctx: &mut Context) -> f32{
    timer::duration_to_f64(timer::get_delta(ctx)) as f32
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let font = graphics::Font::new(ctx, "/fonts/satumt.TTF", 16)?;
        let text = graphics::Text::new(ctx, "Energy", &font)?;
        
        // Stuff drawn in background / objects / background itself
        let background_image = graphics::Image::new(ctx, "/misc/background.png").unwrap();
        
        // GUI elements
        let energy_bar_size: (f32, f32) = (300.0, 35.0);
        let energy_bar = energy_bar::EnergyBar::new((WINDOW_SIZE.0/2.0 - (energy_bar_size.0/2.0), WINDOW_SIZE.1-energy_bar_size.1), 
            energy_bar_size, energy_bar_size.0); // (position: (f32, f32), size: (f32, f32), maxWidth: f32)

        // Player
        let _image_location = "/player/player_stand_r.png";
        let _pl_image = graphics::Image::new(ctx, _image_location).unwrap();
        let pl = player::Player::new(ctx, _image_location, (WINDOW_SIZE.0/2.0 - _pl_image.width() as f32/2.0, 0.0), 300.0, WINDOW_SIZE, energy_bar.size.1);

        
        let current_duration = Instant::now();
        let current_time = current_duration.elapsed().as_secs() as f64;
        let accumulator = 0.0;
        let is_a_pressed = false;
        let is_d_pressed = false;
        let is_x_pressed = false;
        let is_f_pressed = false;

        let gc = camera::Camera::new((0.0, 0.0), WINDOW_SIZE);

        let bg_position = (0.0, 0.0);

        // game objects
        let _door_closed_image_location = "/misc/door_closed.png";
        let _door_closed_image = graphics::Image::new(ctx, _door_closed_image_location).unwrap();
        let _door_open_image_location = "/misc/door_opened.png";
        let _door_open_image = graphics::Image::new(ctx, _door_open_image_location).unwrap();

        let porch_object = object::Object::new(ctx, "/misc/porch.png", "/misc/porch.png", (0.0, 0.0), (0.0, 0.0), minigame::Minigame::Nothing);

        let _trophy_clean_image_location = "/misc/shelf_ow.png";
        let _trophy_clean_image = graphics::Image::new(ctx, _trophy_clean_image_location).unwrap();

        let objects = vec![
            object::Object::new(ctx, _door_closed_image_location, "/misc/door_opened.png",
                (background_image.width() as f32 - _door_closed_image.width() as f32/2.0, WINDOW_SIZE.1 - _door_closed_image.height() as f32 - 45.0), 
                (background_image.width() as f32 - _door_open_image.width() as f32, WINDOW_SIZE.1 - _door_open_image.height() as f32 - 45.0),
                minigame::Minigame::Robber),

            object::Object::new(ctx, _trophy_clean_image_location, "/misc/shelf_event.png",
                (300.0, 0.0),
                (300.0, 0.0),
                minigame::Minigame::Shelf)
        ];

        let event_timer_base = 5.0;
        let event_timer = event_timer_base;

        let in_event = false;

        let solid_background = graphics::Image::new(ctx, "/misc/solid_background.png").unwrap();

        let current_minigame = minigame::Minigame::Nothing;

        let robber_minigame = robberminigame::RobberMinigame::new(ctx, WINDOW_SIZE, "/burglar/burglar_live.png", 
        "/burglar/burglar_dead.png", "/burglar/burglar_win.png", "/burglar/gun_loaded.png", "/burglar/gun_shot.png");

        let shelf_minigame = shelfminigame::ShelfMinigame::new(ctx, WINDOW_SIZE);

        let current_minigame_index = -1;

        let game_time_bar = timebar::TimeBar::new((30.0, 0.0), (WINDOW_SIZE.0 - 60.0, 30.0), WINDOW_SIZE.0 - 60.0);

        let game_time_left_base = 40.0;
        let game_time_left = game_time_left_base;

        let win = false;
        let lose = false;

        let notify_image = graphics::Image::new(ctx, "/misc/notifier.png").unwrap();
        let small_notify_image = graphics::Image::new(ctx, "/misc/notifier_small.png").unwrap();
        let notify_position = (0.0, 50.0);
        let show_notify = false;
        let show_small_notify = false;
        let notify_timer_base = 0.1;
        let notify_timer = notify_timer_base;
        let notify_blink = false;

        let mut rng = rand::thread_rng();

        let in_menu = true;

        let end_game = false;
        let restart_game = false;

        let menu_image = graphics::Image::new(ctx, "/misc/menu.png").unwrap();

        let end_timer_base = 1.5;
        let end_timer = end_timer_base;

        let player_win_image = graphics::Image::new(ctx, "/player/player_win.png").unwrap();
        let player_lose_image = graphics::Image::new(ctx, "/player/player_lose.png").unwrap();
        let lose_image = graphics::Image::new(ctx, "/misc/lose.png").unwrap();
        let win_image = graphics::Image::new(ctx, "/misc/win.png").unwrap();

        let s = MainState { text, frames: 0, background_image, pl, energy_bar, current_time, current_duration, 
            accumulator, is_a_pressed, is_d_pressed, is_x_pressed, gc, bg_position, porch_object, objects, event_timer, 
            event_timer_base, in_event, current_minigame, robber_minigame, solid_background, current_minigame_index, is_f_pressed,
            game_time_bar, game_time_left_base, game_time_left, win, lose, notify_image, notify_position, show_notify, notify_blink,
            notify_timer_base, notify_timer, small_notify_image, show_small_notify, shelf_minigame, rng, in_menu, end_game, restart_game,
            menu_image, end_timer, end_timer_base, player_win_image, player_lose_image, lose_image, win_image };

        Ok(s)
    }
}

fn handle_input(pl: &mut player::Player, gc: &mut camera::Camera, bg_position: (f32, f32), 
            bg_image: graphics::Image, ctx: &mut Context, is_a_pressed: bool, is_d_pressed: bool,
            is_x_pressed: bool, in_event: bool, in_menu: &mut bool, end_game: &mut bool, restart_game: &mut bool,
            win: bool, lose: bool) {

    if !in_event && !*in_menu && !*end_game && !win && !lose {
        if is_a_pressed {
            gc.center.0 -= pl.move_speed * get_dt(ctx);
        }

        if is_d_pressed {
            gc.center.0 += pl.move_speed * get_dt(ctx);
        }

        if gc.center.0 <= bg_position.0 + pl.size.0 as f32/2.0 {
            gc.center.0 = bg_position.0 + pl.size.0 as f32/2.0;
        } else if gc.center.0 >= bg_position.0 + bg_image.width() as f32 - pl.size.0 as f32/2.0 {
            gc.center.0 = bg_position.0 + bg_image.width() as f32 - pl.size.0 as f32/2.0;
        }
    }

    if is_x_pressed {
        if *end_game {
            *restart_game = true;
            *end_game = false;
            *in_menu = false;
        } else {
            *in_menu = false;
        }
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
        // Update player based on user input

        // Updates that are non-critical time based
        handle_input(&mut self.pl, &mut self.gc, self.bg_position, self.background_image.clone(), ctx, 
            self.is_a_pressed, self.is_d_pressed, self.is_x_pressed, self.in_event, &mut self.in_menu,
            &mut self.end_game, &mut self.restart_game, self.win, self.lose);

        // Update GUI, make dirty update later?
        if !self.in_event && !self.in_menu && !self.end_game {
            self.energy_bar.update(self.pl.energy);

            self.pl.update(ctx, self.in_event, DT);

            for i in &mut self.objects {
                i.update(self.gc.center, self.pl.size, self.is_x_pressed, &mut self.in_event, &mut self.current_minigame);
            }
        } else {
            if self.current_minigame == minigame::Minigame::Robber {
                let quit_robber = self.robber_minigame.update_always(ctx, DT, self.is_f_pressed, WINDOW_SIZE, &mut self.in_event, 
                &mut self.current_minigame, &mut self.pl.energy);
                if quit_robber {
                    self.objects[0].end_event(self.background_image.clone(), WINDOW_SIZE);
                    self.current_minigame = minigame::Minigame::Nothing;
                    self.current_minigame_index = -1;
                }
            } else if self.current_minigame == minigame::Minigame::Shelf {
                let quit_shelf = self.shelf_minigame.update_always(ctx, DT, self.is_d_pressed, WINDOW_SIZE, &mut self.in_event, 
                &mut self.current_minigame, &mut self.pl.energy);
                if quit_shelf {
                    self.objects[1].end_event(self.background_image.clone(), WINDOW_SIZE);
                    self.current_minigame = minigame::Minigame::Nothing;
                    self.current_minigame_index = -1;
                }
            }
        }

        if !self.restart_game {
            if !self.in_event {
                if self.game_time_left > 0.0 {
                    self.game_time_left -= 1.0 * DT as f32;
                } else {
                    self.game_time_left = 0.0;
                    self.win = true;
                    self.pl.batch.set_image(self.player_win_image.clone());
                    if self.end_timer > 0.0 {
                        self.end_timer -= 1.0 * DT as f32;
                    } else {
                        self.end_timer = self.end_timer_base;
                        self.end_game = true;
                    }
                }
                if self.pl.energy < 0.0 {
                    self.lose = true;
                    self.pl.batch.set_image(self.player_lose_image.clone());
                    if self.end_timer > 0.0 {
                        self.end_timer -= 1.0 * DT as f32;
                    } else {
                        self.end_timer = self.end_timer_base;
                        self.end_game = true;
                    }
                }
            }
        } else { // restart game
            self.game_time_left = self.game_time_left_base;
            self.pl.energy = self.pl.energy_base;
            self.current_minigame_index = -1;
            self.event_timer = self.event_timer_base;
            self.pl.position = (WINDOW_SIZE.0/2.0 - self.pl.player_image.width() as f32/2.0, 0.0);
            self.win = false;
            self.lose = false;
            self.restart_game = false;
            self.end_game = false;
            self.in_event = false;
            self.pl.batch.set_image(self.pl.player_image.clone());
        }

        self.game_time_bar.update(self.game_time_left, self.game_time_left_base);
        
        // Updates that involve physics/can be affected by time
        while self.accumulator >= DT {
            // Update fixed-interval updates
            // notify blinker
            if self.current_minigame_index != -1 {
                if self.objects[self.current_minigame_index as usize].position.0 > self.gc.position.0 + self.gc.size.0 {
                    self.show_notify = true;
                    self.show_small_notify = false;
                    self.notify_position.0 = WINDOW_SIZE.0 - self.notify_image.width() as f32;
                } else if self.objects[self.current_minigame_index as usize].position.0 + 
                (self.objects[self.current_minigame_index as usize].size.0 as f32) < self.gc.position.0 {
                    self.show_notify = true;
                    self.show_small_notify = false;
                    self.notify_position.0 = 0.0;
                } else {
                    self.show_notify = false;
                    self.show_small_notify = true;
                }
            } else {
                self.show_notify = false;
                self.show_small_notify = false;
            }
            if self.show_notify || self.show_small_notify {
                if self.notify_timer > 0.0 {
                    self.notify_timer -= 1.0 * DT as f32;
                } else {
                    if !self.notify_blink {
                        self.notify_blink = true;
                    } else {
                        self.notify_blink = false;
                    }
                    self.notify_timer = self.notify_timer_base;
                }
            }

            // Timer for events
            if !self.in_event && !self.in_menu && !self.end_game {
                if self.event_timer > 0.0 {
                    self.event_timer-=1.0 * DT as f32;
                } else {
                    let index: usize = self.rng.gen_range(0,2);
                    self.objects[index].start_event(self.background_image.clone(), WINDOW_SIZE);
                    self.current_minigame_index = index as i32;
                    self.event_timer = self.event_timer_base;
                }
                self.pl.update_fixed(ctx, DT, self.is_a_pressed, self.is_d_pressed, self.win, self.lose, self.in_event);
                // self.gc.center.0 = self.pl.position.0 + self.gc.size.0 / 2.0;
                // self.gc.center.1 = self.pl.position.1 + self.gc.size.1 / 2.0;
                self.gc.update();
            } else {
                if self.current_minigame == minigame::Minigame::Robber {
                    self.robber_minigame.update(DT);
                } else if self.current_minigame == minigame::Minigame::Shelf {
                    self.shelf_minigame.update(DT);
                }
            }

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

        if !self.in_event && !self.in_menu && !self.end_game {
            // Porch
            self.porch_object.draw(self.gc.offset);
            let porch_object_param = self.porch_object.return_param(dpiscale);
            graphics::draw_ex(ctx, &self.porch_object.batch, porch_object_param)?;
            self.porch_object.batch.clear();


            // Background objects / Background itself
            let bg_dst = graphics::Point2::new(self.bg_position.0+self.gc.offset.0, self.bg_position.1+self.gc.offset.1);
            graphics::draw(ctx, &self.background_image, bg_dst, 0.0)?;

            // Objects
            for i in &mut self.objects {
                i.draw(self.gc.offset);
                let object_param = i.return_param(dpiscale);
                graphics::draw_ex(ctx, &i.batch, object_param)?;
                i.batch.clear();
            }

            // Player drawing
            self.pl.draw();
            let pl_param = self.pl.return_param(dpiscale);
            graphics::draw_ex(ctx, &self.pl.batch, pl_param)?;
            self.pl.batch.clear();
            // End of player drawing
            
            // GUI drawing
            self.energy_bar.draw(ctx);
            self.game_time_bar.draw(ctx);
            if self.show_notify && !self.notify_blink {
                let notify_dst = graphics::Point2::new(self.notify_position.0, self.notify_position.1);
                graphics::draw(ctx, &self.notify_image, notify_dst, 0.0)?;
            }
            if self.show_small_notify && !self.notify_blink {
                let notify_dst = graphics::Point2::new(self.objects[self.current_minigame_index as usize].position.0 +
                self.objects[self.current_minigame_index as usize].size.0 as f32 / 2.0 + self.gc.offset.0 - self.small_notify_image.width() as f32 / 2.0,
                self.objects[self.current_minigame_index as usize].position.1 +
                self.objects[self.current_minigame_index as usize].size.1 as f32 / 2.0 + self.gc.offset.1 - self.small_notify_image.height() as f32 / 2.0);
                graphics::draw(ctx, &self.small_notify_image, notify_dst, 0.0)?;
            }

            // Drawables are drawn from their top-left corner.
            // Text drawing for energy
            let dest_point = graphics::Point2::new(self.energy_bar.position.0 - 75.0, self.energy_bar.position.1 + 2.0);
            graphics::draw(ctx, &self.text, dest_point, 0.0)?;
        } else {
            let bg_dst = graphics::Point2::new(0.0, 0.0);
            graphics::draw(ctx, &self.solid_background, bg_dst, 0.0)?;

            if self.in_menu {
                let menu_dst = graphics::Point2::new(-110.0, 0.0);
                graphics::draw(ctx, &self.menu_image, menu_dst, 0.0)?;
            }

            if self.end_game {
                if self.win {
                    let menu_dst = graphics::Point2::new(-110.0, 0.0);
                    graphics::draw(ctx, &self.win_image, menu_dst, 0.0)?;
                }
                if self.lose {
                    let menu_dst = graphics::Point2::new(-110.0, 0.0);
                    graphics::draw(ctx, &self.lose_image, menu_dst, 0.0)?;
                }
            }

            if self.current_minigame == minigame::Minigame::Robber {
                self.robber_minigame.draw();
                let robber_param = self.robber_minigame.return_param(dpiscale);
                graphics::draw_ex(ctx, &self.robber_minigame.robber_batch, robber_param)?;
                if !self.robber_minigame.robber_dead {
                    graphics::draw_ex(ctx, &self.robber_minigame.gun_batch, robber_param)?;
                    self.robber_minigame.gun_batch.clear();
                }
                self.robber_minigame.robber_batch.clear();
                if !self.robber_minigame.robber_dead {
                    let shots_left_dst = graphics::Point2::new(0.0, WINDOW_SIZE.1 - 45.0);
                    graphics::draw(ctx, &self.robber_minigame.shots_left_text, shots_left_dst, 0.0)?;
                    let misses_left_dst = graphics::Point2::new(0.0, WINDOW_SIZE.1 - 90.0);
                    graphics::draw(ctx, &self.robber_minigame.misses_left_text, misses_left_dst, 0.0)?;
                    let action_dst = graphics::Point2::new(WINDOW_SIZE.0 / 2.0 - self.robber_minigame.action_text.get_dimensions().w / 2.0,
                        30.0);
                    graphics::draw(ctx, &self.robber_minigame.action_text, action_dst, 0.0)?;
                    self.robber_minigame.time_bar.draw(ctx);
                    
                }
            } else if self.current_minigame == minigame::Minigame::Shelf {
                self.shelf_minigame.draw();
                let shelf_param = self.shelf_minigame.return_param(dpiscale);
                graphics::draw_ex(ctx, &self.shelf_minigame.shelf_batch, shelf_param)?;
                graphics::draw_ex(ctx, &self.shelf_minigame.trophy_batch, shelf_param)?;
                self.shelf_minigame.shelf_batch.clear();
                self.shelf_minigame.trophy_batch.clear();
                if !self.shelf_minigame.ended {
                    graphics::draw_ex(ctx, &self.shelf_minigame.duster_batch, shelf_param)?;
                    self.shelf_minigame.duster_batch.clear();
                }
                for i in &mut self.shelf_minigame.dust_clouds {
                    let dust_param = i.return_param(dpiscale);
                    graphics::draw_ex(ctx, &i.batch, dust_param)?;
                    i.batch.clear();
                }
                if !self.shelf_minigame.ended {
                    let action_dst = graphics::Point2::new(WINDOW_SIZE.0 / 2.0 - self.robber_minigame.action_text.get_dimensions().w / 2.0,
                        30.0);
                    graphics::draw(ctx, &self.shelf_minigame.action_text, action_dst, 0.0)?;
                    self.shelf_minigame.time_bar.draw(ctx);
                }
            }
        }
        
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
        if keycode == keyboard::Keycode::X {
            self.is_x_pressed = true;
        }
        if keycode == keyboard::Keycode::F {
            self.is_f_pressed = true;
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: Keycode, _keymod: Mod, _release: bool) {
        if keycode == keyboard::Keycode::A {
            self.is_a_pressed = false;
        }
        if keycode == keyboard::Keycode::D {
            self.is_d_pressed = false;
        }
        if keycode == keyboard::Keycode::X {
            self.is_x_pressed = false;
        }
        if keycode == keyboard::Keycode::F {
            self.is_f_pressed = false;
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
    let ctx = &mut ContextBuilder::new("STAYAWAKE", "hunterkepley")
        .window_setup(conf::WindowSetup{
            title: "STAYAWAKE".to_owned(),
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