extern crate allegro;
extern crate allegro_font;
extern crate allegro_image;

pub mod game;

use rand::Rng;
use allegro::*;
use crate::game::Game;
use crate::game::sprite::{simple_sprite::SimpleSprite, eight_directions_sprite::EightDirectionsSprite};
use crate::game::config::CONFIG;

allegro_main! {
  let game = Game::get_instance();
  let mut rng = rand::thread_rng();

  // Create 10 random sprites
  for _ in 1..10 {
    let x = rng.gen_range(0..CONFIG.width - 181);
    let y = rng.gen_range(0..CONFIG.height - 226);
    let vx = 1 + rng.gen_range(0..5);
    let vy = 1 + rng.gen_range(0..5);
    let mut sprite = SimpleSprite::new(x as f32, y as f32, "assets/mario.png");
    sprite.vx = vx as f32;
    sprite.vy = vy as f32;
    game.sprites.push(Box::new(sprite));
  }

  // Create a playable sprite
  let mario = EightDirectionsSprite::new(10.0, 10.0, "assets/mario.png");
  game.sprites.push(Box::new(mario));

  game.game_loop();    
}
