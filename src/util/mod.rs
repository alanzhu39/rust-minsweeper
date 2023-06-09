mod sevseg;
mod display;
mod buffer;
mod getch;

use clearscreen;
use getch::Getch;

pub use sevseg::display_sevseg;
pub use display::{
  display_banner,
  display_controls,
  display_mine_count_header,
  display_game_over_message
};
pub use buffer::Buffer;

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
  match ch {
    b'k' | 37 => return Some(Key::K_UP),
    b'j' | 38 => return Some(Key::K_DOWN),
    b'h' | 40 => return Some(Key::K_LEFT),
    b'l' | 39 => return Some(Key::K_RIGHT),
    b's' => return Some(Key::K_S),
    b'f' => return Some(Key::K_F),
    _ => None
  }
}
