#[derive(Debug, Copy, Clone)]
pub enum Colors {
    Red,
    Green,
    Blue,
    Yellow,
    Cyan,
    Magenta,
    Normal,
}

  pub const COLOR_MAP: [Colors; 6] = [
    Colors::Red,
    Colors::Green,
    Colors::Blue,
    Colors::Yellow,
    Colors::Cyan,
    Colors::Magenta,
];
