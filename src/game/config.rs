pub struct ConfigStruct {
  pub width: i32,
  pub height: i32,
  pub vx: f32,
  pub vy: f32,
  pub fps: f32
}

pub const CONFIG : ConfigStruct = ConfigStruct {
  width:  800,
  height: 600,
  vx:     3.0,
  vy:     3.0,
  fps:   60.0
};

