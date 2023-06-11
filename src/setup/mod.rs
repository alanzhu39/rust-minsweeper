use crate::util;
use crate::util::GameMode;
use colored::Colorize;
use std::io::{self, Write};

const INDENT: u8 = 20;

/* Get gamemode, returns enum variant */
pub fn get_game_mode() -> GameMode {
  let mut line = String::new();
  loop {
    util::clear_screen();
    util::display_banner();

    // print game mode prompt
    println!("{:INDENT$}{}", "Maximize terminal or press F11 for the optimal experience.".bold().yellow());
    println!("");
    println!("{:INDENT$}{}", "Choose Game Mode".bold().white());
    println!("{:INDENT$}{}", "1. Beginner".bold().green());
    println!("{:INDENT$}{}", "2. Intermediate".bold().yellow());
    println!("{:INDENT$}{}", "3. Expert".bold().red());
    println!("{:INDENT$}{}", "4. Custom".bold().blue());
    println!("");
    print!("{:INDENT$}", "");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut line).unwrap();

    // validate input choice
    match line.trim_end() {
      "1" => return GameMode::BEGINNER,
      "2" => return GameMode::INTERMEDIATE,
      "3" => return GameMode::EXPERT,
      "4" => return GameMode::CUSTOM,
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
{:INDENT$}Quick Clear : If a cell with a number on it is already revealed,
{:INDENT$}              and it's neighbours have been flagged with the same number,
{:INDENT$}              then all tiles adjacent to this cell are also sweeped.

{:INDENT$}Enable Quick Clear?").bold().yellow(),
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
