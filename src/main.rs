pub mod util;
mod display;

fn main() {
  // util::clearScreen();
  let gameMode = display::getGameMode();
  let quickClearEnabled = display::getQuickClearSettings();
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
