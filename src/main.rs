pub mod util;
mod display;
mod game;

fn main() {
  // util::clearScreen();
  let gameMode = display::getGameMode();
  let quickClearEnabled = display::getQuickClearSettings();

  // start game

  // while (true) {
  //   util::clearScreen();
  //   displayBanner()
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
