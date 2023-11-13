use self::instructions::{Instruction, ADDR_MODE};

use super::{
  defs::{Byte, Word},
  mem::Mem,
};
use bitflags::bitflags;

mod instructions;

bitflags! {
  pub struct CpuStatus: u8 {
    const C = 0b00000001; // Carry Flag
    const Z = 0b00000010; // Zero Flag
    const I = 0b00000100; // Interrupt Disable
    const D = 0b00001000; // Decimal Mode
    const B = 0b00010000; // Break Command
    const U = 0b00100000; // Unused
    const V = 0b01000000; // Overflow Flag
    const N = 0b10000000; // Negative Flag
  }
}

impl Default for CpuStatus {
  fn default() -> Self {
    Self { bits: 0 }
  }
}

impl CpuStatus {
  pub fn reset(&mut self) {
    self.bits = 0xFF | CpuStatus::U.bits;
  }

  pub fn set_flag(&mut self, flag: CpuStatus) {
    self.bits |= flag.bits;
  }

  pub fn clear_flag(&mut self, flag: CpuStatus) {
    self.bits &= !flag.bits;
  }

  pub fn is_flag_set(&self, flag: CpuStatus) -> bool {
    self.bits & flag.bits != 0
  }
}

pub struct CPU {
  pub pc: Word,
  pub sp: Word,

  pub reg_a: Byte,
  pub reg_x: Byte,
  pub reg_y: Byte,

  pub status: CpuStatus,
  pub cycles: u8,

  // for convenience
  pub mem: Mem,
  pub working_addr: Word,
  pub rel_working_addr: Word,
  pub working_data: Byte,
  pub curr_instruction: Instruction,
}

impl CPU {
  pub fn new(mem: Mem) -> Self {
    let mut new = Self {
      pc: 0,
      sp: 0,

      reg_a: 0,
      reg_x: 0,
      reg_y: 0,

      status: CpuStatus::default(),
      cycles: 0,
      mem,

      rel_working_addr: 0x0000,
      working_addr: 0x0000,
      working_data: 0x00,
      curr_instruction: Instruction::default(),
    };
    new.reset();

    new
  }

  pub fn reset(&mut self) {
    self.working_addr = 0xFFFC;
    let lo = self.mem.read(self.working_addr) as Word;
    let hi = self.mem.read(self.working_addr + 1) as Word;
    self.pc = (hi << 8) | lo;

    self.sp = 0x0100;
    self.status.reset();

    self.cycles = 8;
  }

  pub fn clock(&mut self) {
    if self.cycles == 0 {
      let op_code = self.fetch();
      let ins = Instruction::from_op_code(op_code);
      self.curr_instruction = ins.clone();
      self.cycles = instructions::execute(self, ins);
    }

    self.cycles -= 1;
  }

  pub fn irq(&mut self) {
    if self.status.is_flag_set(CpuStatus::I) {
      return;
    }

    self.mem.write(self.sp, (self.pc >> 8) as Byte);
    self.sp -= 1;
    self.mem.write(self.sp, (self.pc & 0x00FF) as Byte);
    self.sp -= 1;

    self.status.set_flag(CpuStatus::B);
    self.status.set_flag(CpuStatus::U);
    self.status.set_flag(CpuStatus::I);
    self.mem.write(self.sp, self.status.bits);
    self.sp -= 1;

    self.working_addr = 0xFFFE;
    let lo = self.mem.read(self.working_addr) as Word;
    let hi = self.mem.read(self.working_addr + 1) as Word;
    self.pc = (hi << 8) | lo;

    self.cycles = 7;
  }

  pub fn nmi(&mut self) {
    self.mem.write(self.sp, (self.pc >> 8) as Byte);
    self.sp -= 1;
    self.mem.write(self.sp, (self.pc & 0x00FF) as Byte);
    self.sp -= 1;

    self.status.set_flag(CpuStatus::B);
    self.status.set_flag(CpuStatus::U);
    self.status.set_flag(CpuStatus::I);
    self.mem.write(self.sp, self.status.bits);
    self.sp -= 1;

    self.working_addr = 0xFFFA;
    let lo = self.mem.read(self.working_addr) as Word;
    let hi = self.mem.read(self.working_addr + 1) as Word;
    self.pc = (hi << 8) | lo;

    self.cycles = 8;
  }

  pub fn rti(&mut self) {
    // return from interrupt
    self.sp += 1;
    self.status.bits = self.mem.read(self.sp);
    self.status.clear_flag(CpuStatus::B);
    self.status.clear_flag(CpuStatus::U);

    self.sp += 1;
    let lo = self.mem.read(self.sp) as Word;
    self.sp += 1;
    let hi = self.mem.read(self.sp) as Word;
    self.pc = (hi << 8) | lo;

    self.cycles = 6;
  }

  pub fn fill_working_data(&mut self) {
    if !(self.curr_instruction.addr_mode == ADDR_MODE::IMPLIED) {
      self.working_data = self.mem.read(self.working_addr);
    }
  }

  pub fn fetch(&mut self) -> Byte {
    let data = self.mem.read(self.pc);
    self.pc += 1;
    data
  }

  pub fn fetch_word(&mut self) -> Word {
    let lo = self.fetch() as Word;
    let hi = self.fetch() as Word;
    self.pc += 2;

    (hi << 8) | lo
  }
}
