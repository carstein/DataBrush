use serde::{Serialize, Deserialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum Colors {
    Red,
    Green,
    Blue,
    Yellow,
    Cyan,
    Magenta,
    Normal,
}

impl Default for Colors {
  fn default() -> Self {
    Colors::Normal
  }
}

  pub const COLOR_MAP: [Colors; 6] = [
    Colors::Red,
    Colors::Green,
    Colors::Blue,
    Colors::Yellow,
    Colors::Cyan,
    Colors::Magenta,
];
