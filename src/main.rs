extern crate allegro;
extern crate allegro_font;
extern crate allegro_image;

pub mod game;

use rand::Rng;
use allegro::*;
use crate::game::Game;
use crate::game::sprite::{simple_sprite::SimpleSprite, eight_directions_sprite::EightDirectionsSprite};
use crate::game::scene::Scene;
use crate::game::config::CONFIG;

allegro_main! {
  let game = Game::get_instance();
  let mut rng = rand::thread_rng();
  let mut main_scene : Scene = Scene::new("main");

  // Create 10 random sprites
  for _ in 1..10 {
    let x = rng.gen_range(0..CONFIG.width - 181);
    let y = rng.gen_range(0..CONFIG.height - 226);
    let vx = 1 + rng.gen_range(0..5);
    let vy = 1 + rng.gen_range(0..5);
    let mut sprite = SimpleSprite::new(x as f32, y as f32, "assets/mario.png");
    sprite.vx = vx as f32;
    sprite.vy = vy as f32;
    main_scene.sprites.push(Box::new(sprite));
  }

  // Create a playable sprite
  let mario = EightDirectionsSprite::new(10.0, 10.0, "assets/mario.png");
  main_scene.sprites.push(Box::new(mario));

  // Add the main scene
  game.scenes.insert("main".to_string(), main_scene);

  // Create other scene
  let mut other_scene : Scene = Scene::new("avgn");
  let avgn = EightDirectionsSprite::new(10.0, 10.0, "assets/avgn.png");
  other_scene.sprites.push(Box::new(avgn));
  game.scenes.insert("avgn".to_string(), other_scene);

  // Boot up the game
  game.game_loop();    
}
