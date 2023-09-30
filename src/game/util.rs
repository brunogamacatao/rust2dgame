use std::io::{self, Write};

pub fn println(s: &str) {
  print!("{}\n", s);
  io::stdout().flush().unwrap();
}
