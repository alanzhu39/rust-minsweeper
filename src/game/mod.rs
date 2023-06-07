mod cell;

use crate::util;
use crate::util::{GameMode, Key};
use cell::Cell;

pub enum GameState {
  RUNNING,
  WON,
  LOST
}

pub struct Game {
  field: Vec<Vec<Cell>>,

  game_state: GameState,
  game_mode: GameMode,
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
  pub fn start_game(game_mode: GameMode, quick_clear_enabled: bool) -> Game {
    let (width, height, num_mines) = match game_mode {
      GameMode::BEGINNER => {
        (9, 9, 10)
      }
      GameMode::INTERMEDIATE => {
        (16, 16, 40)
      }
      GameMode::EXPERT => {
        (30, 16, 99)
      }
      _ => {
        (0, 0, 0)
      }
    };

    let field = vec![vec![Cell::new(); width as usize]; height as usize];

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

  pub fn get_move(&mut self) {
    let key = util::get_key();
    match key {
      Some(Key::K_UP) => {
        if self.i > 0 {
          self.i -= 1;
        }
      }
      Some(Key::K_DOWN) => {
        if self.i < self.height - 1 {
          self.i += 1;
        }
      }
      Some(Key::K_LEFT) => {
        if self.j > 0 {
          self.j -= 1;
        }
      }
      Some(Key::K_RIGHT) => {
        if self.j < self.width - 1 {
          self.j += 1;
        }
      }
      Some(Key::K_S) => {
        self.sweep_cell(self.i, self.j);
      }
      Some(Key::K_F) => {
        self.flag_cell(self.i, self.j);
      }
      _ => {}
    }
  }

  fn get_cell(&mut self, i: u8, j: u8) -> &mut Cell {
    &mut self.field[i as usize][j as usize]
  }

  fn sweep_cell(&mut self, sweep_i: u8, sweep_j: u8) {
    unimplemented!("sweep_cell() not implemented");
    if self.is_first_sweep {
      self.do_first_sweep(sweep_i, sweep_j);
      self.is_first_sweep = false;
      return
    }
    let mut cell = self.get_cell(sweep_i, sweep_j);
    if cell.hidden && !cell.flagged {
      cell.hidden = false;
      cell.flagged = true;
      if self.flag_count > 0 {
        self.flag_count -= 1;
      }
    }
  }

  fn flag_cell(&mut self, flag_i: u8, flag_j: u8) {
    unimplemented!("flag_cell() not implemented");
    // if cell is hidden, flag it
    // else, pass
  }

  fn do_first_sweep(&mut self, sweep_i: u8, sweep_j: u8) {
    unimplemented!("do_first_sweep() not implemented");
    // randomly populate field with mines, such that no mines are within the 3x3 block around the first sweep
    // update all surrounding cells w adj mine counts
  }

  pub fn is_running(&self) -> bool {
    matches!(self.game_state, GameState::RUNNING)
  }

  pub fn display_game(&self) {
    unimplemented!("display_game() not implemented");
    // display field
    // display flag count header
    // display flag count sevseg
    match self.game_state {
      GameState::WON => {
        util::display_victory_message();
      }
      GameState::LOST => {
        util::display_defeat_message();
      }
      GameState::RUNNING => {
        util::display_controls();
      }
    }
  }

  fn diplay_field(&self) {
    unimplemented!("display_field() not implemented");
  }
}
