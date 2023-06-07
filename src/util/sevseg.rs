use super::Buffer;
use colored::Colorize;

pub fn display_sevseg(buffer: &mut Buffer, num_digits: u8, number: u8) {
  for x in (0..num_digits).rev() {
    let digit = NUMBERS[(((number as u32) / (10 as u32).pow(x.into())) % 10) as usize];
    let start_line = buffer.get_curr_line();
    for line in digit {
      buffer.write("  ".normal());
      buffer.writeln(line.red());
    }
    buffer.go_to_line(start_line);
  }
}

const NUMBERS: [[&str; 5]; 10] = [
  [
    "╻══╻",
    "║  ║",
    "║  ║",
    "║  ║",
    "╹══╹"
  ],
  [
    "   ╻",
    "   ║",
    "   ║",
    "   ║",
    "   ╹"
  ],
  [
    " ══╻",
    "   ║",
    "╻══╹",
    "║   ",
    "╹══ "
  ],
  [
    " ══╻",
    "   ║",
    " ══║",
    "   ║",
    " ══╹"
  ], 
  [
    "╻  ╻",     
    "║  ║",     
    "╹══║",     
    "   ║",     
    "   ╹"
  ],
  [
    "╻══ ",
    "║   ",
    "╹══╻",
    "   ║",
    " ══╹"
  ],
  [
    "╻══ ",
    "║   ",
    "║══╻",
    "║  ║",
    "╹══╹"
  ],
  [
    " ══╻",
    "   ║",
    "   ║",
    "   ║",
    "   ╹"
  ],
  [
    "╻══╻",
    "║  ║",
    "║══║",
    "║  ║",
    "╹══╹"
  ],
  [
    "╻══╻",
    "║  ║",
    "╹══║",
    "   ║",
    " ══╹"
  ]
];
