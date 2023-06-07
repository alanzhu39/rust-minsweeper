use crate::util;
use crate::GameMode;
use colored::Colorize;
use std::io::{self, Write};

/* Get gamemode, returns enum variant */
pub fn get_game_mode() -> GameMode {
  let mut line = String::new();
  loop {
    util::clear_screen();
    util::display_banner();

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
      "1" => return GameMode::BEGINNER,
      "2" => return GameMode::INTERMEDIATE,
      "3" => return GameMode::EXPERT,
      // TODO: implement custom gamemode
      // "4" => return GameMode::CUSTOM,
      _ => {
        println!("{}", "Invalid input, please try again".red());
        line.clear();
        continue;
      }
    }
  }
}

/* Get quick clear settings */
pub fn get_quick_clear_settings() -> bool {
  let mut line = String::new();
  loop {
    util::clear_screen();
    util::display_banner();

    // print quick clear prompt
    print!(
      "{} {} {} {} {} {}",
      ("
      Quick Clear : If a cell with a number on it is already revealed,
      and it's neighbours have been flagged with the same number,
      then all tiles adjacent to this cell are also sweeped.

      Enable Quick Clear?").bold().yellow(),
      "[".bold().white(),
      "Y".bold().green(),
      "/".bold().white(),
      "N".bold().red(),
      "] : ".bold().white(),
    );
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut line).unwrap();

    // validate input choice
    match line.trim_end() {
      "y" | "Y" => return true,
      "n" | "N" => return false,
      _ => {
        println!("{}", "Invalid input, please try again".red());
        line.clear();
        continue;
      }
    }
  }
}
