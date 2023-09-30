pub mod config;
pub mod sprite;
pub mod util;

use allegro::{self, Core, Display, Timer, EventQueue, Color, DisplayClose, TimerTick, KeyDown, KeyUp};
use allegro_font::{self, Font, FontAddon, FontAlign, FontDrawing};
use allegro_image::{self, ImageAddon};

use allegro::keycodes::KeyCode;
use std::collections::HashSet;
use rand::{self, Rng};

use crate::game::util::println;
use crate::game::sprite::{Sprite, SimpleSprite, EightDirectionsSprite};
use crate::game::config::CONFIG;

pub struct Game {
  pub core: Core,
  pub running: bool,
  pub pressed_keys: HashSet<allegro::KeyCode>,
  pub sprites: Vec<Box<dyn Sprite>>
}

impl Game {
  pub fn new() -> Game {
    let core = Core::init().unwrap();

    Game {
      core: core,
      running: true,
      pressed_keys: HashSet::new(),
      sprites: Vec::new()
    }
  }

  fn initialize(&mut self) {
    let mut rng = rand::thread_rng();

    // Create 10 random sprites
    for _ in 1..10 {
      let x = rng.gen_range(0..CONFIG.width - 181);
      let y = rng.gen_range(0..CONFIG.height - 226);
      let vx = 1 + rng.gen_range(0..5);
      let vy = 1 + rng.gen_range(0..5);
      let mut sprite = SimpleSprite::new(&self.core, x as f32, y as f32, "assets/mario.png");
      sprite.vx = vx as f32;
      sprite.vy = vy as f32;
      self.sprites.push(Box::new(sprite));
    }

    // Create a playable sprite
    let mario = EightDirectionsSprite::new(&self.core, 10.0, 10.0, "assets/mario.png");
    self.sprites.push(Box::new(mario));
  }

  pub fn game_loop(&mut self) {
    // Setup game engine
    let font_addon = FontAddon::init(&self.core).unwrap();
    ImageAddon::init(&self.core).unwrap();
  
    let display = Display::new(&self.core, CONFIG.width, CONFIG.height).unwrap();
    let timer   = Timer::new(&self.core, 1.0 / 60.0).unwrap();
    let font    = Font::new_builtin(&font_addon).unwrap();

    // Initialize game entities
    self.initialize();
  
    // Setup events
    let queue = EventQueue::new(&self.core).unwrap();

    self.core.install_keyboard().unwrap();
    self.core.install_mouse().unwrap();

    queue.register_event_source(display.get_event_source());
    queue.register_event_source(timer.get_event_source());
    queue.register_event_source(self.core.get_keyboard_event_source().unwrap());
    queue.register_event_source(self.core.get_mouse_event_source().unwrap());

    let black = Color::from_rgb_f(0.0, 0.0, 0.0);
    let white = Color::from_rgb_f(1.0, 1.0, 1.0);

    let mut redraw = true;

    // Game loop
    timer.start();
    while self.is_running() {
      if redraw && queue.is_empty() {
        self.core.clear_to_color(black);
        self.draw();
        self.core.draw_text(&font, white,
          (display.get_width() / 2) as f32, (display.get_height() / 2) as f32,
          FontAlign::Centre, "Welcome to RustAllegro!");
        self.core.flip_display();
        redraw = false;
      }

      match queue.wait_for_event() {
        DisplayClose{..} => {
          self.stop();
          break;
        },
        TimerTick{..} => redraw = true,
        KeyDown{keycode: k, ..} => self.key_press(k),
        KeyUp{keycode: k, ..} => self.key_release(k),
        _ => (),
      }

      self.process_input();
      self.update();
    }    
  }

  pub fn draw(&self) {
    // draw NPCs
    let slice = &self.sprites[..];
    for s in slice {
      s.draw(&self.core);
    }
  }

  pub fn process_input(&mut self) {
    if self.pressed_keys.contains(&KeyCode::Escape) {
      println("EXIT !");
      self.stop();
      return;
    }
  }

  pub fn update(&mut self) {
    for b in self.sprites.iter_mut() {
      let sprite : &mut dyn Sprite = b.as_mut();
      sprite.update();
      sprite.process_input(&self.pressed_keys);
    }
  }

  pub fn key_press(&mut self, k: KeyCode) {
    self.pressed_keys.insert(k);
  }

  pub fn key_release(&mut self, k: KeyCode) {
    self.pressed_keys.remove(&k);
  }

  pub fn stop(&mut self) {
    self.running = false;
  }

  pub fn is_running(&self) -> bool {
    self.running
  }
}  