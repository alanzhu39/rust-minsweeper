mod cell;

use crate::util;
use crate::util::{Buffer, GameMode, Key};
use cell::{Cell, CellState};

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
        self.sweep_cell(self.i, self.j, self.quick_clear_enabled);
      }
      Some(Key::K_F) => {
        self.toggle_flag(self.i, self.j);
      }
      _ => {}
    }
  }

  fn get_cell(&mut self, cell_i: u8, cell_j: u8) -> &mut Cell {
    &mut self.field[cell_i as usize][cell_j as usize]
  }

  // FIXME: testing
  pub fn foo(&mut self) {
    let mut buffer = Buffer::new();
    util::display_mine_count_header(&mut buffer);
    util::display_sevseg(&mut buffer, 5, self.num_mines - self.flag_count);
    buffer.display_buffer();
  }

  fn sweep_cell(&mut self, sweep_i: u8, sweep_j: u8, quick_clear: bool) {
    unimplemented!("sweep_cell() not implemented");
    let cell = self.get_cell(sweep_i, sweep_j);
    if cell.flagged {
      return;
    }
    if self.is_first_sweep {
      self.do_first_sweep(sweep_i, sweep_j);
      self.is_first_sweep = false;
      return;
    }

    let mut cell = self.get_cell(sweep_i, sweep_j);
    match cell.state {
      CellState::EMPTY => {
        if cell.hidden {
          cell.hidden = false;
          self.sweep_empty_cell(sweep_i, sweep_j);
        }
      }
      CellState::MINE => {
        if cell.hidden {
          // reveal cell
          // reveal all mines?
          self.game_state = GameState::LOST;
        }
      }
      CellState::ADJ_TO_MINE => {
        if cell.hidden {
          cell.hidden = false;
        } else if quick_clear {
          self.sweep_quick_clear(sweep_i, sweep_j);
        }
      }
    }
  }

  fn sweep_empty_cell(&mut self, sweep_i: u8, sweep_j: u8) {
    unimplemented!("sweep empty cell not implemented");
    // bfs through neighboring cells
  }

  fn sweep_quick_clear(&mut self, sweep_i: u8, sweep_j: u8) {
    unimplemented!("sweep_quick_clear() not implemented");
    // if cell is revealed and has adj mine count
    // if cell has adj mine count == num of flagged adj cells
    // reveal all adj cells
  }

  fn do_first_sweep(&mut self, sweep_i: u8, sweep_j: u8) {
    unimplemented!("do_first_sweep() not implemented");
    // randomly populate field with mines, such that no mines are within the 3x3 block around the first sweep
    // update all surrounding cells w adj mine counts
  }

  fn toggle_flag(&mut self, flag_i: u8, flag_j: u8) {
    if self.flag_count >= self.num_mines {
      return;
    }
    let mut cell = self.get_cell(flag_i, flag_j);
    if !cell.hidden {
      return;
    }
    if cell.flagged {
      cell.flagged = false;
      self.flag_count -= 1;
    } else {
      cell.flagged = true;
      self.flag_count += 1;
    }
  }

  pub fn is_running(&self) -> bool {
    matches!(self.game_state, GameState::RUNNING)
  }

  pub fn display_game(&self) {
    let mut buffer = Buffer::new();
    // display field
    self.display_field();

    // display mine count
    buffer.go_to_line(0);
    util::display_mine_count_header(&mut buffer);
    util::display_sevseg(&mut buffer, 5, self.num_mines - self.flag_count);

    match self.game_state {
      GameState::WON => {
        util::display_game_over_message(&mut buffer, true);
      }
      GameState::LOST => {
        util::display_game_over_message(&mut buffer, false);
      }
      GameState::RUNNING => {
        util::display_controls(&mut buffer);
      }
    }
    buffer.display_buffer();
  }

  fn display_field(&self) {
    unimplemented!("display_field() not implemented");
  }
}
