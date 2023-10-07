use std::collections::HashSet;
use allegro::keycodes::KeyCode;
use allegro::{self, Flag, Bitmap};
use crate::game::Game;
use crate::game::sprite::{Sprite, Point};
use crate::game::config::CONFIG;

pub struct EightDirectionsSprite {
  pub x: f32,
  pub y: f32,  
  pub vx: f32,
  pub vy: f32,
  pub image: Bitmap
}

impl EightDirectionsSprite {
  pub fn new(x: f32, y: f32, filename: &str) -> EightDirectionsSprite {
    EightDirectionsSprite {
      x: x,
      y: y,
      vx: CONFIG.vx,
      vy: CONFIG.vy,
      image: Bitmap::load(&Game::get_instance().core, filename).unwrap()
    }
  }

  pub fn translate(&mut self, dx: f32, dy: f32) {
    self.x += dx;
    self.y += dy;
  }

  pub fn draw(&self) {
    Game::get_instance().core.draw_bitmap(&self.image, self.x, self.y, Flag::zero());    
  }
}  

impl Sprite for EightDirectionsSprite {
  fn get_position(&self) -> Point {
    return Point { x: self.x, y: self.y };
  }

  fn set_position(&mut self, pos : Point) {
    self.x = pos.x;
    self.y = pos.y;
  }

  fn update(&mut self) {
  }

  fn draw(&self) {
    let pos : Point = self.get_position();
    Game::get_instance().core.draw_bitmap(&self.image, pos.x, pos.y, Flag::zero());    
  }

  fn process_input(&mut self, pressed_keys: &HashSet<allegro::KeyCode>) {    
    if pressed_keys.contains(&KeyCode::Right) {
      self.translate(self.vx, 0.0);
    }
    if pressed_keys.contains(&KeyCode::Left) {
      self.translate(-self.vx, 0.0);
    }
    if pressed_keys.contains(&KeyCode::Up) {
      self.translate(0.0, -self.vy);
    }
    if pressed_keys.contains(&KeyCode::Down) {
      self.translate(0.0, self.vy);
    }
  }
}