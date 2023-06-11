use colored::Colorize;
use super::Buffer;

/* Display Minesweeper banner */
pub fn display_banner() {
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

pub fn display_mine_count_header(buffer: &mut Buffer) {
  buffer.writeln(r"   _ __ ___ (_)_ __   ___ ___ ".bold().white());
  buffer.writeln(r"  | '_ ` _ \| | '_ \ / _ / __|".bold().white());
  buffer.writeln(r"  | | | | | | | | | |  __\__ \".bold().white());
  buffer.writeln(r"  |_| |_| |_|_|_| |_|\___|___/".bold().white());
  buffer.writeln("".normal());
}

pub fn display_game_over_message(buffer: &mut Buffer, is_victory: bool) {
  buffer.writeln(r"  __   _____  _   _ ".bold().white());
  buffer.writeln(r"  \ \ / / _ \| | | |".bold().white());
  buffer.writeln(r"   \ V / (_) | |_| |".bold().white());
  buffer.writeln(r"    |_| \___/ \___/ ".bold().white());
  buffer.writeln("".normal());

  if is_victory {
    buffer.writeln(r"  __      _____ _  _ _ ".bold().white());
    buffer.writeln(r"  \ \    / /_ _| \| | |".bold().white());
    buffer.writeln(r"   \ \/\/ / | || .` |_|".bold().white());
    buffer.writeln(r"    \_/\_/ |___|_|\_(_)".bold().white());
  } else {
    buffer.writeln(r"   _    ___  ___ ___ _ ".bold().white());
    buffer.writeln(r"  | |  / _ \/ __| __| |".bold().white());
    buffer.writeln(r"  | |_| (_) \__ \ _||_|".bold().white());
    buffer.writeln(r"  |____\___/|___/___(_)".bold().white());
  }
}

pub fn display_controls(buffer: &mut Buffer) {
  buffer.writeln(r"  ←, ↓, ↑, →           ".bold().white());
  buffer.writeln(r"      OR     : Movement".bold().white());
  buffer.writeln(r"  H, J, K, L           ".bold().white());
  buffer.writeln(r"  S : Sweep".bold().white());
  buffer.writeln(r"  F : Toggle Flag".bold().white());
}
