use super::Buffer;
use colored::Colorize;

pub fn display_sevseg(buffer: &mut Buffer, num_digits: i32, number: i32) {
  let start_line = buffer.get_curr_line();
  for x in (0..num_digits).rev() {
    buffer.go_to_line(start_line);
    let digit = NUMBERS[((number / (10 as i32).pow(x as u32)) % 10) as usize];
    for line in digit {
      buffer.write("  ".normal());
      buffer.writeln(line.red());
    }
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
