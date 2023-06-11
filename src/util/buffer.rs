use colored::{ColoredString, Colorize};

pub struct Buffer {
  buf: Vec<Vec<ColoredString>>,
  line: usize
}

impl Buffer {
  pub fn new() -> Buffer {
    Buffer {
      buf: vec![vec!["".normal()]],
      line: 0
    }
  }

  pub fn write(&mut self, s: ColoredString) {
    self.buf[self.line].push(s)
  }

  pub fn writeln(&mut self, s: ColoredString) {
    self.buf[self.line].push(s);
    if self.line == self.buf.len() - 1 {
      self.buf.push(vec!["".normal()]);
    }
    self.line += 1;
  }

  pub fn get_curr_line(&self) -> usize {
    self.line
  }

  pub fn go_to_line(&mut self, line: usize) {
    self.line = line;
  }

  pub fn display_buffer(&self) {
    for line in &self.buf {
      for s in line {
        print!("{}", s);
      }
      println!();
    }
  }

  pub fn clear_buffer(&mut self) {
    self.buf = vec![vec!["".normal()]];
    self.line = 0;
  }
}
