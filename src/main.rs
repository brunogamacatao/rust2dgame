extern crate allegro;
extern crate allegro_font;
extern crate allegro_image;

pub mod game;

use allegro::*;
use crate::game::Game;

allegro_main! {
  let mut game = Game::new();
  game.game_loop();
}
