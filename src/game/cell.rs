#[derive(Clone, Debug)]
pub enum CellState {
  EMPTY,
  MINE,
  ADJ_TO_MINE
}

#[derive(Clone, Debug)]
pub struct Cell {
  pub flagged: bool,
  pub hidden: bool,
  pub state: CellState,
  pub num_adj_mines: i32
}

impl Cell {
  pub fn new() -> Cell {
    Cell {
      flagged: false,
      hidden: true,
      state: CellState::EMPTY,
      num_adj_mines: 0
    }
  }
}
