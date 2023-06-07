mod sevseg;
mod display;

use clearscreen;
use getch::Getch;

pub use sevseg::display_sevseg;
pub use display::{display_banner};

#[derive(Debug)]
pub enum GameMode {
  BEGINNER,
  INTERMEDIATE,
  EXPERT,
  CUSTOM
}

#[derive(Debug)]
pub enum Key {
  K_UP,
  K_DOWN,
  K_LEFT,
  K_RIGHT,
  K_S,
  K_F
}

pub fn clear_screen() {
  clearscreen::clear().expect("failed to clear screen")
}

pub fn get_key() -> Option<Key> {
  let ch = Getch::new().getch().unwrap();
  // println!("{}", ch);
  match ch {
    b'k' => return Some(Key::K_UP),
    b'j' => return Some(Key::K_DOWN),
    b'h' => return Some(Key::K_LEFT),
    b'l' => return Some(Key::K_RIGHT),
    b's' => return Some(Key::K_S),
    b'f' => return Some(Key::K_F),
    _ => None
  }
}
