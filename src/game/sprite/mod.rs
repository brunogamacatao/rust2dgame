pub mod eight_directions_sprite;
pub mod simple_sprite;

use std::collections::HashSet;

pub struct Point {
  pub x: f32,
  pub y: f32
}

pub trait Sprite {
  fn get_position(&self) -> Point;
  fn set_position(&mut self, pos : Point);
  fn process_input(&mut self, pressed_keys: &HashSet<allegro::KeyCode>);
  fn update(&mut self);
  fn draw(&self);
}