use clearscreen;

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
