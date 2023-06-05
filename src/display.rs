use crate::util;
use indoc::indoc;
use colored::Colorize;
use std::io::{self, Write};

/* Display Minesweeper banner */
pub fn displayBanner() {
  println!(
    "{}",
    ("
    ##     ##  ##  ##    ## ########  ######  ##      ## ######## ######## ########  ######## ########
    ###   ###  ##  ###   ## ##       ##    ## ##  ##  ## ##       ##       ##     ## ##       ##     ##
    #### ####  ##  ####  ## ##       ##       ##  ##  ## ##       ##       ##     ## ##       ##     ##
    ## ### ##  ##  ## ## ## ######    ######  ##  ##  ## ######   ######   ########  ######   ########
    ##     ##  ##  ##  #### ##             ## ##  ##  ## ##       ##       ##        ##       ##   ##
    ##     ##  ##  ##   ### ##       ##    ## ##  ##  ## ##       ##       ##        ##       ##    ##
    ##     ##  ##  ##    ## ########  ######   ###  ###  ######## ######## ##        ######## ##     ##
    ").bold().blue()
  );
}

/* Get gamemode, returns enum variant */
pub fn getGameMode() -> util::GameMode {
  let mut line = String::new();
  loop {
    util::clearScreen();
    displayBanner();

    // print game mode prompt
    let indent = 20;
    println!("{:indent$}{}", "", "Maximize terminal or press F11 for the optimal experience.".bold().yellow());
    println!("");
    println!("{:indent$}{}", "", "Choose Game Mode".bold().white());
    println!("{:indent$}{}", "", "1. Beginner".bold().green());
    println!("{:indent$}{}", "", "2. Intermediate".bold().yellow());
    println!("{:indent$}{}", "", "3. Expert".bold().red());
    println!("{:indent$}{}", "", "4. Custom".bold().blue());
    println!("");
    print!("{:indent$}", "");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut line).unwrap();

    // validate input choice
    match line.trim_end() {
      "1" => return util::GameMode::BEGINNER,
      "2" => return util::GameMode::INTERMEDIATE,
      "3" => return util::GameMode::EXPERT,
      "4" => return util::GameMode::CUSTOM,
      _ => {
        println!("{}", "Invalid input, please try again".red());
        line.clear();
        continue;
      }
    }
    break;
  }
  return util::GameMode::BEGINNER;
}