use super::defs::{Byte, Word, MEM_SIZE};

pub struct Mem {
  data: Vec<Byte>,
}

impl Mem {
  pub fn new() -> Self {
    Self {
      data: vec![0; MEM_SIZE],
    }
  }

  pub fn reset(&mut self) {
    for byte in &mut self.data {
      *byte = 0;
    }
  }

  pub fn read(&self, addr: Word) -> Byte {
    self.data[addr as usize]
  }

  pub fn write(&mut self, addr: Word, data: Byte) {
    self.data[addr as usize] = data;
  }

  pub fn load(&mut self, data: &[u8], offset: Word) {
    for (i, byte) in data.iter().enumerate() {
      self.data[i + offset as usize] = *byte;
    }
  }
}
