pub mod util;
mod display;
mod game;

fn main() {
  // util::clearScreen();
  let game_mode = display::get_game_mode();
  let quick_clear_enabled = display::get_quick_clear_settings();

  // start game

  // while (true) {
  //   util::clearScreen();
  //   display_banner()
  //   displayField()
  //   displayFlagCount()

  //   if (gameIsOver) {
  //     displayGameOverMessage()
  //   } else {
  //     displayControls()
  //   }

  //   if (gameIsRunning) {
  //     getMove()
  //   } else {
  //     break;
  //   }
  // }
}
