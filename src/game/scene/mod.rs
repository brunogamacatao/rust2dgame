use crate::game::sprite::Sprite;

pub struct Scene {
  pub name : String,
  pub sprites: Vec<Box<dyn Sprite>>
}

impl Scene {
  pub fn new(name : &str) -> Scene {
    Scene {
      name: name.to_string(),
      sprites: Vec::new()
    }
  }
}