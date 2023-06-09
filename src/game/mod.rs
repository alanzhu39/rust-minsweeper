mod cell;

use crate::util;
use crate::util::{Buffer, GameMode, Key};
use cell::{Cell, CellState};
use std::collections::{VecDeque, HashSet};
use rand::distributions::{Distribution, Uniform};
use colored::Colorize;

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

  width: i32,
  height: i32,
  num_mines: i32,
  flag_count: i32,

  i: i32,
  j: i32
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
    self.update_game_state();
  }

  fn update_game_state(&mut self) {
    if matches!(self.game_state, GameState::RUNNING) && self.get_num_hidden_cells() == self.num_mines {
      self.game_state = GameState::WON;
    }
  }

  fn get_mut_cell(&mut self, cell_i: i32, cell_j: i32) -> &mut Cell {
    &mut self.field[cell_i as usize][cell_j as usize]
  }

  fn get_cell(&self, cell_i: i32, cell_j: i32) -> &Cell {
    &self.field[cell_i as usize][cell_j as usize]
  }

  fn get_adj_cells(&self, cell_i: i32, cell_j: i32) -> Vec<(i32, i32)> {
    let mut adj_cells = Vec::new();
    for i in cell_i - 1..cell_i + 2 {
      for j in cell_j - 1..cell_j + 2 {
        if i == cell_i && j == cell_j {
          continue;
        }
        if i < 0 || i >= self.height || j < 0 || j >= self.width {
          continue;
        }
        adj_cells.push((i, j));
      }
    }
    adj_cells
  }

  fn get_num_adj_flagged_cells(&self, cell_i: i32, cell_j: i32) -> i32 {
    let mut num_adj_flagged_cells = 0;
    for (adj_i, adj_j) in self.get_adj_cells(cell_i, cell_j) {
      let cell = self.get_cell(adj_i, adj_j);
      if cell.flagged {
        num_adj_flagged_cells += 1;
      }
    }
    num_adj_flagged_cells
  }

  fn get_num_hidden_cells(&self) -> i32 {
    let mut num_hidden_cells = 0;
    for i in 0..self.height {
      for j in 0..self.width {
        let cell = self.get_cell(i, j);
        if cell.hidden {
          num_hidden_cells += 1;
        }
      }
    }
    num_hidden_cells
  }

  // FIXME: testing
  pub fn foo(&mut self) {
    let mut buffer = Buffer::new();
    let mut cell = self.get_mut_cell(0, 0);
    cell.hidden = false;

    for n in 1..9 {
      let mut cell = self.get_mut_cell(0, n);
      cell.hidden = false;
      cell.state = CellState::ADJ_TO_MINE;
      cell.num_adj_mines = n;
    }

    let mut cell = self.get_mut_cell(1, 0);
    cell.hidden = false;
    cell.state = CellState::MINE;
    let mut cell = self.get_mut_cell(1, 1);
    cell.flagged = true;

    self.display_field(&mut buffer);
    buffer.go_to_line(0);
    util::display_mine_count_header(&mut buffer);
    util::display_sevseg(&mut buffer, 5, self.num_mines - self.flag_count);
    buffer.display_buffer();
  }

  fn sweep_cell(&mut self, sweep_i: i32, sweep_j: i32, quick_clear: bool) {
    let cell = self.get_cell(sweep_i, sweep_j);
    if cell.flagged {
      return;
    }
    if self.is_first_sweep {
      self.do_first_sweep(sweep_i, sweep_j);
      self.is_first_sweep = false;
      return;
    }

    let mut cell = self.get_mut_cell(sweep_i, sweep_j);
    match cell.state {
      CellState::EMPTY => {
        if cell.hidden {
          cell.hidden = false;
          self.sweep_empty_cell(sweep_i, sweep_j);
        }
      }
      CellState::MINE => {
        if cell.hidden {
          self.reveal_all_mines();
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

  fn reveal_all_mines(&mut self) {
    for i in 0..self.height {
      for j in 0..self.width {
        let mut cell = self.get_mut_cell(i, j);
        if matches!(cell.state, CellState::MINE) {
          cell.hidden = false;
        }
      }
    }
  }

  fn sweep_empty_cell(&mut self, sweep_i: i32, sweep_j: i32) {
    let mut Q = VecDeque::new();
    let mut explored = HashSet::new();
    explored.insert((sweep_i, sweep_j));
    Q.push_back((sweep_i, sweep_j));
    while !Q.is_empty() {
      let (i, j) = Q.pop_front().unwrap();
      let mut cell = self.get_mut_cell(i, j);
      if matches!(cell.state, CellState::ADJ_TO_MINE) {
        cell.hidden = false;
      } else if matches!(cell.state, CellState::EMPTY) {
        cell.hidden = false;
        for (adj_i, adj_j) in self.get_adj_cells(i, j) {
          if !explored.contains(&(adj_i, adj_j)) {
            explored.insert((adj_i, adj_j));
            Q.push_back((adj_i, adj_j));
          }
        }
      }
    }
  }

  fn sweep_quick_clear(&mut self, sweep_i: i32, sweep_j: i32) {
    let mut cell = self.get_mut_cell(sweep_i, sweep_j);
    if cell.num_adj_mines == self.get_num_adj_flagged_cells(sweep_i, sweep_j) {
      for (adj_i, adj_j) in self.get_adj_cells(sweep_i, sweep_j) {
        self.sweep_cell(adj_i, adj_j, false);
      }
    }
  }

  fn do_first_sweep(&mut self, sweep_i: i32, sweep_j: i32) {
    let mut num_mines_planted = 0;
    let mut rng = rand::thread_rng();
    let mine_dist = Uniform::from(0..self.width * self.height);
    while num_mines_planted < self.num_mines {
      let mine_idx = mine_dist.sample(&mut rng);
      let mine_i = mine_idx / self.width;
      let mine_j = mine_idx % self.width;
      if (mine_i - sweep_i).abs() <= 1 && (mine_j - sweep_j).abs() <= 1 {
        continue;
      }
      let mut cell = self.get_mut_cell(mine_i, mine_j);
      if matches!(cell.state, CellState::EMPTY) {
        cell.state = CellState::MINE;
        num_mines_planted += 1;

        for (adj_i, adj_j) in self.get_adj_cells(mine_i, mine_j) {
          let mut adj_cell = self.get_mut_cell(adj_i, adj_j);
          if matches!(adj_cell.state, CellState::EMPTY) {
            adj_cell.state = CellState::ADJ_TO_MINE;
          }
          adj_cell.num_adj_mines += 1;
        }
      }
    }
    self.sweep_cell(sweep_i, sweep_j, self.quick_clear_enabled);
  }

  fn toggle_flag(&mut self, flag_i: i32, flag_j: i32) {
    if self.flag_count >= self.num_mines {
      return;
    }
    let mut cell = self.get_mut_cell(flag_i, flag_j);
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
    self.display_field(&mut buffer);

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

  fn display_field(&self, buffer: &mut Buffer) {
    for i in 0..self.height {
      for j in 0..self.width {
        let cell = self.get_cell(i, j);
        // draw top
        if i == 0 {
          if j == 0 {
            if cell.hidden {
              buffer.writeln("    ┏━━━".normal());
            } else {
              buffer.writeln("    ┌───".normal());
            }
          } else {
            let left_cell = self.get_cell(i, j - 1);
            if left_cell.hidden && cell.hidden {
              buffer.writeln("┳━━━".normal());
            } else if left_cell.hidden && !cell.hidden {
              buffer.writeln("┱───".normal());
            } else if !left_cell.hidden && cell.hidden {
              buffer.writeln("┲━━━".normal());
            } else {
              buffer.writeln("┬───".normal());
            }
          }
        } else {
          let top_cell = self.get_cell(i - 1, j);
          if j == 0 {
            if top_cell.hidden && cell.hidden {
              buffer.writeln("    ┣━━━".normal());
            } else if top_cell.hidden && !cell.hidden {
              buffer.writeln("    ┡━━━".normal());
            } else if !top_cell.hidden && cell.hidden {
              buffer.writeln("    ┢━━━".normal());
            } else {
              buffer.writeln("    ├───".normal());
            }
          } else {
            let left_cell = self.get_cell(i, j - 1);
            let top_left_cell = self.get_cell(i - 1, j - 1);
            let top_edge = top_left_cell.hidden || top_cell.hidden;
            let left_edge = top_left_cell.hidden || left_cell.hidden;
            let right_edge = top_cell.hidden || cell.hidden;
            let bottom_edge = left_cell.hidden || cell.hidden;
            match (top_edge, right_edge, bottom_edge, left_edge) {
              (true, true, true, true) => {
                buffer.writeln("╋━━━".normal());
              }
              (true, false, false, false) => {
                buffer.writeln("╃───".normal());
              } 
              (true, false, true, true) => {
                buffer.writeln("╉───".normal());
              } 
              (true, true, false, true) => {
                buffer.writeln("╇━━━".normal());
              }
              (true, true, false, false) => {
                buffer.writeln("╄━━━".normal());
              }
              (false, false, true, true) => {
                buffer.writeln("╅───".normal());
              }
              (false, true, true, false) => {
                buffer.writeln("╆━━━".normal());
              }
              (false, true, true, true) => {
                buffer.writeln("╊━━━".normal());
              }
              (false, true, true, true) => {
                buffer.writeln("╈━━━".normal());
              }
              (false, false, false, false) => {
                buffer.writeln("┼───".normal());
              }
              _ => {}
            }
          }
        }
        // draw top right
        if j == self.width - 1 {
          buffer.go_to_line(buffer.get_curr_line() - 1);
          if i == 0 {
            if cell.hidden {
              buffer.writeln("┓".normal());
            } else {
              buffer.writeln("┐".normal());
            }
          } else {
            let top_cell = self.get_cell(i - 1, j);
            if top_cell.hidden && cell.hidden {
              buffer.writeln("┫".normal());
            } else if top_cell.hidden && !cell.hidden {
              buffer.writeln("┩".normal());
            } else if !top_cell.hidden && cell.hidden {
              buffer.writeln("┪".normal());
            } else {
              buffer.writeln("┤".normal());
            }
          }
        }
        // draw left
        if j == 0 {
          if cell.hidden {
            buffer.write("    ┃ ".normal());
          } else {
            buffer.write("    │ ".normal());
          }
        } else {
          let left_cell = self.get_cell(i, j - 1);
          if left_cell.hidden || cell.hidden {
            buffer.write("┃ ".normal());
          } else {
            buffer.write("│ ".normal());
          }
        }
        // draw cell
        let mut cell_str = " ".normal();
        if cell.hidden {
          if cell.flagged {
            cell_str = "▶".bold().green();
          } else {
            cell_str = " ".on_white();
          }
        } else {
          match cell.state {
            CellState::EMPTY => {
              cell_str = " ".normal();
            }
            CellState::MINE => {
              cell_str = "*".bold().red();
            }
            CellState::ADJ_TO_MINE => {
              match cell.num_adj_mines {
                1 => {
                  cell_str = "1".bold().cyan();
                }
                2 => {
                  cell_str = "2".bold().green();
                }
                3 => {
                  cell_str = "3".bold().red();
                }
                4 => {
                  cell_str = "4".bold().blue();
                }
                5 => {
                  cell_str = "5".bold().magenta();
                }
                6 => {
                  cell_str = "6".bold().bright_cyan();
                }
                7 => {
                  cell_str = "7".bold().bright_magenta();
                }
                8 => {
                  cell_str = "8".bold().bright_black();
                }
                _ => {}
              }
            }
          }
        }
        if self.i == i && self.j == j {
          cell_str = cell_str.on_blue();
        }
        buffer.write(cell_str);
        buffer.writeln(" ".normal());
        // draw right
        if j == self.width - 1 {
          buffer.go_to_line(buffer.get_curr_line() - 1);
          if cell.hidden {
            buffer.writeln("┃".normal());
          } else {
            buffer.writeln("│".normal());
          }
        } else if i < self.height - 1 {
          buffer.go_to_line(buffer.get_curr_line() - 2);
        }
        // draw bottom
        if i == self.height - 1 {
          if j == 0 {
            if cell.hidden {
              buffer.writeln("    ┗━━━".normal());
            } else {
              buffer.writeln("    └───".normal());
            }
          } else {
            let left_cell = self.get_cell(i, j - 1);
            if left_cell.hidden && cell.hidden {
              buffer.writeln("┻━━━".normal());
            } else if left_cell.hidden && !cell.hidden {
              buffer.writeln("┹───".normal());
            } else if !left_cell.hidden && cell.hidden {
              buffer.writeln("┺━━━".normal());
            } else {
              buffer.writeln("┴───".normal());
            }
          }
          if j == self.width - 1 {
            buffer.go_to_line(buffer.get_curr_line() - 1);
            if cell.hidden {
              buffer.writeln("┛".normal());
            } else {
              buffer.writeln("┘".normal());
            }
          } else {
            buffer.go_to_line(buffer.get_curr_line() - 3);
          }
        }
      }
    }
  }
}
