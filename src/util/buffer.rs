pub struct Buffer {
  buf: Vec<String>,
  line: usize
}

impl Buffer {
  pub fn new() -> Buffer {
    Buffer {
      buf: vec![String::new()],
      line: 0
    }
  }

  pub fn write(&mut self, s: &str) {
    self.buf[self.line].push_str(s);
  }

  pub fn writeln(&mut self, s: &str) {
    self.buf[self.line].push_str(s);
    if self.line == self.buf.len() - 1 {
      self.buf.push(String::new());
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
      println!("{}", line);
    }
  }

  pub fn clear_buffer(&mut self) {
    self.buf = vec![String::new()];
    self.line = 0;
  }
}
