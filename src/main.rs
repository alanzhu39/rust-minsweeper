pub mod util;
mod setup;
mod game;

fn main() {
  // util::clear_screen();
  let game_mode = setup::get_game_mode();
  let quick_clear_enabled = setup::get_quick_clear_settings();

  let mut curr_game = game::Game::start_game(game_mode, quick_clear_enabled);

  loop {
    util::clear_screen();
    util::display_banner();
    curr_game.display_game();
    // displayFlagCount();

    // if (gameIsOver) {
    //   displayGameOverMessage()
    // } else {
    //   displayControls()
    // }

    // if (gameIsRunning) {
    //   getMove()
    // } else {
    //   break;
    // }
  }
}
