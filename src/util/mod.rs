mod sevseg;
mod display;

use clearscreen;

pub use sevseg::display_sevseg;
pub use display::{display_banner};

#[derive(Debug)]
pub enum GameMode {
  BEGINNER,
  INTERMEDIATE,
  EXPERT,
  CUSTOM
}

pub fn clear_screen() {
  clearscreen::clear().expect("failed to clear screen")
}
