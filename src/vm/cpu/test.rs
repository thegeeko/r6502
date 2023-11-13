#[cfg(test)]
mod tests {
  use crate::vm::Mem;

  use super::CPU;

  #[test]
  fn init_cpu() {
    let mut mem = Mem::new();
    mem.load(&[0x01, 0x00], 0xFFFC);
    let cpu = CPU::new(mem);
    assert_eq!(
      cpu.pc, 0x0001,
      "CPU program counter didn't load successfully it has {} instead of 0x0001",
      cpu.pc
    );
  }

  #[test]
  fn load_accumulator() {
    let mut mem = Mem::new();
    // starting address
    mem.load(&[0x00, 0x00], 0xFFFC);

    // program
    mem.load(&[0xA9], 0);
    mem.load(&[0x45], 1);

    let mut cpu = CPU::new(mem);

    // for testing purposes
    cpu.cycles = 0;
    cpu.clock();

    assert_eq!(
      cpu.working_addr, 0x0001,
      "CPU didn't load working address with 0x0001, it has {} instead",
      cpu.working_addr
    );
    assert_eq!( cpu.reg_a, 0x45,
      "CPU didn't load accumulator with 0x45, it has {} instead",
      cpu.reg_a
    );
  }
}
