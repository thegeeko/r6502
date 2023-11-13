mod cpu;
mod defs;
mod mem;

pub use cpu::CPU;
pub use mem::Mem;

use self::defs::Word;

struct Vm {
  cpu: CPU,
}

impl Vm {
  fn new() -> Self {
    let mem = Mem::new();
    Self { cpu: CPU::new(mem) }
  }

  fn reset(&mut self) {
    self.cpu.reset();
  }

  fn load(&mut self, data: &[u8], offset: Word) {
    self.cpu.mem.load(data, offset);
  }

  fn step(&mut self) {
    self.cpu.clock();
  }

}
