use std::collections::HashSet;
use allegro::{self, Core, Flag, Bitmap, BitmapLike};
use crate::game::sprite::{Sprite, Point};
use crate::game::config::CONFIG;

pub struct SimpleSprite {
  pub x: f32,
  pub y: f32,  
  pub vx: f32,
  pub vy: f32,
  pub image: Bitmap
}

impl SimpleSprite {
  pub fn new(core: &Core, x: f32, y: f32, filename: &str) -> SimpleSprite {
    SimpleSprite {
      x: x,
      y: y,
      vx: CONFIG.vx,
      vy: CONFIG.vy,
      image: Bitmap::load(core, filename).unwrap()
    }
  }

  pub fn translate(&mut self, dx: f32, dy: f32) {
    self.x += dx;
    self.y += dy;
  }

  pub fn draw(&self, core: &Core) {
    core.draw_bitmap(&self.image, self.x, self.y, Flag::zero());    
  }
}  

impl Sprite for SimpleSprite {
  fn get_position(&self) -> Point {
    return Point { x: self.x, y: self.y };
  }

  fn set_position(&mut self, pos : Point) {
    self.x = pos.x;
    self.y = pos.y;
  }

  fn update(&mut self) {
    self.translate(self.vx, self.vy);
    if self.x as i32 > CONFIG.width  - self.image.get_width()  { self.vx = -self.vx }
    if self.y as i32 > CONFIG.height - self.image.get_height() { self.vy = -self.vy }
    if self.x < 0.0 { self.vx = -self.vx }
    if self.y < 0.0 { self.vy = -self.vy }
  }

  fn draw(&self, core: &Core) {
    let pos : Point = self.get_position();
    core.draw_bitmap(&self.image, pos.x, pos.y, Flag::zero());    
  }

  fn process_input(&mut self, _pressed_keys: &HashSet<allegro::KeyCode>) {    
    // do nothing
  }
}
