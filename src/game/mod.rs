mod cell;

use crate::util;

pub enum GameState {
  RUNNING,
  WON,
  LOST
}

pub struct Game {
  field: Vec<Vec<cell::Cell>>,

  game_state: GameState,
  game_mode: util::GameMode,
  quick_clear_enabled: bool,
  is_first_sweep: bool,

  width: u8,
  height: u8,
  num_mines: u8,
  flag_count: u8,

  i: u8,
  j: u8
}

impl Game {
  pub fn startGame(game_mode: util::GameMode, quick_clear_enabled: bool) -> Game {
    let mut width, height, num_mines = match game_mode {
      util::GameMode::BEGINNER => {
        (9, 9, 10)
      }
      util::GameMode::INTERMEDIATE => {
        (16, 16, 40)
      }
      util::GameMode::EXPERT => {
        (16, 30, 99)
      }
      _ => {
        (0, 0, 0)
      }
    };

    let field = vec![vec![cell::Cell::new(); width as usize]; height as usize];

    Game {
      field,

      game_state: GameState::RUNNING,
      game_mode,
      quick_clear_enabled,
      is_first_sweep: true,

      width,
      height,
      num_mines,
      flag_count: 0,

      i: height / 2,
      j: width / 2
    }
  }
}