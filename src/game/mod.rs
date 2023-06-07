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
  pub fn start_game(game_mode: util::GameMode, quick_clear_enabled: bool) -> Game {
    let (width, height, num_mines) = match game_mode {
      util::GameMode::BEGINNER => {
        (9, 9, 10)
      }
      util::GameMode::INTERMEDIATE => {
        (16, 16, 40)
      }
      util::GameMode::EXPERT => {
        (30, 16, 99)
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

  pub fn get_move(&self) {
    unimplemented!("get_move() not implemented");
    // get key
    // match key, pass if none
  }

  pub fn sweep_cell(&self, sweep_i: u8, sweep_j: u8) {
    unimplemented!("sweep_cell() not implemented");
    if self.is_first_sweep {
      self.do_first_sweep(sweep_i, sweep_j);
      self.is_first_sweep = false;
      return
    }
    // if cell is flagged, pass
  }

  fn do_first_sweep(&self, sweep_i: u8, sweep_j: u8) {
    unimplemented!("do_first_sweep() not implemented");
    // randomly populate field with mines, such that no mines are within the 3x3 block around the first sweep
    // update all surrounding cells w adj mine counts
  }

  pub fn is_game_over(&self) -> bool {
    self.game_state != GameState::RUNNING
  }

  pub fn display_game(&self) {
    unimplemented!("display_game() not implemented");
    // display field
    // display flag count header
    // display flag count sevseg
  }

  pub fn display_game_over_message(&self) {
    match self.game_state {
      GameState::WON => {
        util::display_victory_message();
      }
      GameState::LOST => {
        util::display_defeat_message();
      }
      _ => panic!("display_game_over_message() called when game is still running");
    }
  }
}
