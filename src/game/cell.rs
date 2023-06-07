#[derive(Clone)]
pub enum CellState {
  EMPTY,
  MINE,
  ADJ_TO_MINE
}

#[derive(Clone)]
pub struct Cell {
  flagged: bool,
  hidden: bool,
  state: CellState,
  num_adj_mines: u8
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
