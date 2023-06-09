pub mod util;
mod setup;
mod game;

fn main() {
  // FIXME: debugging
  // let mut curr_game = game::Game::start_game(util::GameMode::BEGINNER, false);
  // curr_game.foo();
  // return;

  util::clear_screen();
  let game_mode = setup::get_game_mode();
  let quick_clear_enabled = setup::get_quick_clear_settings();

  let mut curr_game = game::Game::start_game(game_mode, quick_clear_enabled);

  loop {
    util::clear_screen();
    util::display_banner();
    curr_game.display_game();

    if curr_game.is_running() {
      curr_game.get_move();
    } else {
      break;
    }
  }
}
