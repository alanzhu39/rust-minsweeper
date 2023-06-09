use super::Buffer;
use colored::Colorize;

pub fn display_sevseg(buffer: &mut Buffer, num_digits: i32, number: i32) {
  for x in (0..num_digits).rev() {
    let digit = NUMBERS[((number / (10 as i32).pow(x as u32)) % 10) as usize];
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
