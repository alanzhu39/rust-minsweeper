use clearscreen;

#[derive(Debug)]
pub enum GameMode {
  BEGINNER,
  INTERMEDIATE,
  EXPERT,
  CUSTOM
}

pub fn clearScreen() {
  clearscreen::clear().expect("failed to clear screen")
}
