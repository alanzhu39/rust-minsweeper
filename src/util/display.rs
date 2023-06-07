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

pub fn display_flag_count_header(buffer: &mut Buffer) {
  buffer.writeln(r"   _ __ ___ (_)_ __   ___ ___ ".bold().white());
  buffer.writeln(r"  | '_ ` _ \| | '_ \ / _ / __|".bold().white());
  buffer.writeln(r"  | | | | | | | | | |  __\__ \".bold().white());
  buffer.writeln(r"  |_| |_| |_|_|_| |_|\___|___/".bold().white());
}

pub fn display_victory_message() {
  unimplemented!("display_victory_message() not implemented");
}

pub fn display_defeat_message() {
  unimplemented!("display_defeat_message() not implemented");
}

pub fn display_controls() {
  unimplemented!("display_controls() not implemented");
}
