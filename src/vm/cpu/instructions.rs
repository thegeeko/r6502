use super::{CpuStatus, CPU};

#[derive(PartialEq, Eq, Clone)]
pub enum ADDR_MODE {
  IMMEDIATE,
  ZERO_PAGE,
  ZERO_PAGE_X,
  ZERO_PAGE_Y,
  ABSOLUTE,
  ABSOLUTE_X,
  ABSOLUTE_Y,
  INDIRECT,
  INDIRECT_X,
  INDIRECT_Y,
  RELATIVE,
  IMPLIED,

  NONE,
}

#[rustfmt::skip]
#[derive(Clone)]
pub enum OPS {
  ADC, AND, ASL, BCC,
  BCS, BEQ, BIT, BMI,
  BNE, BPL, BRK, BVC, 
  BVS, CLC, CLD, CLI, 
  CLV, CMP, CPX, CPY,
  DEC, DEX, DEY, EOR,
  INC, INX, INY, JMP,
  JSR, LDA, LDX, LDY,
  LSR, NOP, ORA, PHA,
  PHP, PLA, PLP, ROL,
  ROR, RTI, RTS, SBC,
  SEC, SED, SEI, STA,
  STX, STY, TAX, TAY,
  TSX, TXA, TXS, TYA,
  
  XXX
}

#[derive(Clone)]
pub struct Instruction {
  pub name: String,
  pub opcode: u8,
  pub opr: OPS,
  pub addr_mode: ADDR_MODE,
  pub cycles: u8,
}

impl Default for Instruction {
  fn default() -> Self {
    Self {
      name: String::from("NO NAME"),
      opcode: 0x00,
      opr: OPS::XXX,
      addr_mode: ADDR_MODE::NONE,
      cycles: 0,
    }
  }
}

impl Instruction {
  // write a function that returns an instruction based on the op code
  pub fn from_op_code(op_code: u8) -> Self {
    match op_code {
      0x69 => Self {
        name: String::from("ADC"),
        opcode: 0x69,
        opr: OPS::ADC,
        addr_mode: ADDR_MODE::IMMEDIATE,
        cycles: 2,
      },
      0x65 => Self {
        name: String::from("ADC"),
        opcode: 0x65,
        opr: OPS::ADC,
        addr_mode: ADDR_MODE::ZERO_PAGE,
        cycles: 3,
      },
      0x75 => Self {
        name: String::from("ADC"),
        opcode: 0x75,
        opr: OPS::ADC,
        addr_mode: ADDR_MODE::ZERO_PAGE_X,
        cycles: 4,
      },
      0x6D => Self {
        name: String::from("ADC"),
        opcode: 0x6D,
        opr: OPS::ADC,
        addr_mode: ADDR_MODE::ABSOLUTE,
        cycles: 4,
      },
      0x7D => Self {
        name: String::from("ADC"),
        opcode: 0x7D,
        opr: OPS::ADC,
        addr_mode: ADDR_MODE::ABSOLUTE_X,
        cycles: 4,
      },
      0x79 => Self {
        name: String::from("ADC"),
        opcode: 0x79,
        opr: OPS::ADC,
        addr_mode: ADDR_MODE::ABSOLUTE_Y,
        cycles: 4,
      },
      0x61 => Self {
        name: String::from("ADC"),
        opcode: 0x61,
        opr: OPS::ADC,
        addr_mode: ADDR_MODE::INDIRECT_X,
        cycles: 6,
      },
      0x71 => Self {
        name: String::from("ADC"),
        opcode: 0x71,
        opr: OPS::ADC,
        addr_mode: ADDR_MODE::INDIRECT_Y,
        cycles: 5,
      },
      0x29 => Self {
        name: String::from("AND"),
        opcode: 0x29,
        opr: OPS::AND,
        addr_mode: ADDR_MODE::IMMEDIATE,
        cycles: 2,
      },
      0x25 => Self {
        name: String::from("AND"),
        opcode: 0x25,
        opr: OPS::AND,
        addr_mode: ADDR_MODE::ZERO_PAGE,
        cycles: 3,
      },
      0x35 => Self {
        name: String::from("AND"),
        opcode: 0x35,
        opr: OPS::AND,
        addr_mode: ADDR_MODE::ZERO_PAGE_X,
        cycles: 4,
      },
      0x2D => Self {
        name: String::from("AND"),
        opcode: 0x2D,
        opr: OPS::AND,
        addr_mode: ADDR_MODE::ABSOLUTE,
        cycles: 4,
      },
      0x3D => Self {
        name: String::from("AND"),
        opcode: 0x3D,
        opr: OPS::AND,
        addr_mode: ADDR_MODE::ABSOLUTE_X,
        cycles: 4,
      },
      0x39 => Self {
        name: String::from("AND"),
        opcode: 0x39,
        opr: OPS::AND,
        addr_mode: ADDR_MODE::ABSOLUTE_Y,
        cycles: 4,
      },
      0x21 => Self {
        name: String::from("AND"),
        opcode: 0x21,
        opr: OPS::AND,
        addr_mode: ADDR_MODE::INDIRECT_X,
        cycles: 6,
      },
      0x31 => Self {
        name: String::from("AND"),
        opcode: 0x31,
        opr: OPS::AND,
        addr_mode: ADDR_MODE::INDIRECT_Y,
        cycles: 5,
      },
      0x0A => Self {
        name: String::from("ASL"),
        opcode: 0x0A,
        opr: OPS::ASL,
        addr_mode: ADDR_MODE::IMPLIED,
        cycles: 2,
      },
      0x06 => Self {
        name: String::from("ASL"),
        opcode: 0x06,
        opr: OPS::ASL,
        addr_mode: ADDR_MODE::ZERO_PAGE,
        cycles: 5,
      },
      0x16 => Self {
        name: String::from("ASL"),
        opcode: 0x16,
        opr: OPS::ASL,
        addr_mode: ADDR_MODE::ZERO_PAGE_X,
        cycles: 6,
      },
      0x0E => Self {
        name: String::from("ASL"),
        opcode: 0x0E,
        opr: OPS::ASL,
        addr_mode: ADDR_MODE::ABSOLUTE,
        cycles: 6,
      },
      0x1E => Self {
        name: String::from("ASL"),
        opcode: 0x1E,
        opr: OPS::ASL,
        addr_mode: ADDR_MODE::ABSOLUTE_X,
        cycles: 7,
      },
      0x90 => Self {
        name: String::from("BCC"),
        opcode: 0x90,
        opr: OPS::BCC,
        addr_mode: ADDR_MODE::RELATIVE,
        cycles: 2,
      },
      0xB0 => Self {
        name: String::from("BCS"),
        opcode: 0xB0,
        opr: OPS::BCS,
        addr_mode: ADDR_MODE::RELATIVE,
        cycles: 2,
      },
      0xF0 => Self {
        name: String::from("BEQ"),
        opcode: 0xF0,
        opr: OPS::BEQ,
        addr_mode: ADDR_MODE::RELATIVE,
        cycles: 2,
      },
      0x24 => Self {
        name: String::from("BIT"),
        opcode: 0x24,
        opr: OPS::BIT,
        addr_mode: ADDR_MODE::ZERO_PAGE,
        cycles: 3,
      },
      0x2C => Self {
        name: String::from("BIT"),
        opcode: 0x2C,
        opr: OPS::BIT,
        addr_mode: ADDR_MODE::ABSOLUTE,
        cycles: 4,
      },
      0x30 => Self {
        name: String::from("BMI"),
        opcode: 0x30,
        opr: OPS::BMI,
        addr_mode: ADDR_MODE::RELATIVE,
        cycles: 2,
      },
      0xD0 => Self {
        name: String::from("BNE"),
        opcode: 0xD0,
        opr: OPS::BNE,
        addr_mode: ADDR_MODE::RELATIVE,
        cycles: 2,
      },
      0x10 => Self {
        name: String::from("BPL"),
        opcode: 0x10,
        opr: OPS::BPL,
        addr_mode: ADDR_MODE::RELATIVE,
        cycles: 2,
      },
      0x00 => Self {
        name: String::from("BRK"),
        opcode: 0x00,
        opr: OPS::BRK,
        addr_mode: ADDR_MODE::IMPLIED,
        cycles: 7,
      },
      0x50 => Self {
        name: String::from("BVC"),
        opcode: 0x50,
        opr: OPS::BVC,
        addr_mode: ADDR_MODE::RELATIVE,
        cycles: 2,
      },
      0x70 => Self {
        name: String::from("BVS"),
        opcode: 0x70,
        opr: OPS::BVS,
        addr_mode: ADDR_MODE::RELATIVE,
        cycles: 2,
      },
      0x18 => Self {
        name: String::from("CLC"),
        opcode: 0x18,
        opr: OPS::CLC,
        addr_mode: ADDR_MODE::IMPLIED,
        cycles: 2,
      },
      0xD8 => Self {
        name: String::from("CLD"),
        opcode: 0xD8,
        opr: OPS::CLD,
        addr_mode: ADDR_MODE::IMPLIED,
        cycles: 2,
      },
      0x58 => Self {
        name: String::from("CLI"),
        opcode: 0x58,
        opr: OPS::CLI,
        addr_mode: ADDR_MODE::IMPLIED,
        cycles: 2,
      },
      0xB8 => Self {
        name: String::from("CLV"),
        opcode: 0xB8,
        opr: OPS::CLV,
        addr_mode: ADDR_MODE::IMPLIED,
        cycles: 2,
      },
      0xC9 => Self {
        name: String::from("CMP"),
        opcode: 0xC9,
        opr: OPS::CMP,
        addr_mode: ADDR_MODE::IMMEDIATE,
        cycles: 2,
      },
      0xC5 => Self {
        name: String::from("CMP"),
        opcode: 0xC5,
        opr: OPS::CMP,
        addr_mode: ADDR_MODE::ZERO_PAGE,
        cycles: 3,
      },
      0xD5 => Self {
        name: String::from("CMP"),
        opcode: 0xD5,
        opr: OPS::CMP,
        addr_mode: ADDR_MODE::ZERO_PAGE_X,
        cycles: 4,
      },
      0xCD => Self {
        name: String::from("CMP"),
        opcode: 0xCD,
        opr: OPS::CMP,
        addr_mode: ADDR_MODE::ABSOLUTE,
        cycles: 4,
      },
      0xDD => Self {
        name: String::from("CMP"),
        opcode: 0xDD,
        opr: OPS::CMP,
        addr_mode: ADDR_MODE::ABSOLUTE_X,
        cycles: 4,
      },
      0xD9 => Self {
        name: String::from("CMP"),
        opcode: 0xD9,
        opr: OPS::CMP,
        addr_mode: ADDR_MODE::ABSOLUTE_Y,
        cycles: 4,
      },
      0xC1 => Self {
        name: String::from("CMP"),
        opcode: 0xC1,
        opr: OPS::CMP,
        addr_mode: ADDR_MODE::INDIRECT_X,
        cycles: 6,
      },
      0xD1 => Self {
        name: String::from("CMP"),
        opcode: 0xD1,
        opr: OPS::CMP,
        addr_mode: ADDR_MODE::INDIRECT_Y,
        cycles: 5,
      },
      0xE0 => Self {
        name: String::from("CPX"),
        opcode: 0xE0,
        opr: OPS::CPX,
        addr_mode: ADDR_MODE::IMMEDIATE,
        cycles: 2,
      },
      0xE4 => Self {
        name: String::from("CPX"),
        opcode: 0xE4,
        opr: OPS::CPX,
        addr_mode: ADDR_MODE::ZERO_PAGE,
        cycles: 3,
      },
      0xEC => Self {
        name: String::from("CPX"),
        opcode: 0xEC,
        opr: OPS::CPX,
        addr_mode: ADDR_MODE::ABSOLUTE,
        cycles: 4,
      },
      0xC0 => Self {
        name: String::from("CPY"),
        opcode: 0xC0,
        opr: OPS::CPY,
        addr_mode: ADDR_MODE::IMMEDIATE,
        cycles: 2,
      },
      0xC4 => Self {
        name: String::from("CPY"),
        opcode: 0xC4,
        opr: OPS::CPY,
        addr_mode: ADDR_MODE::ZERO_PAGE,
        cycles: 3,
      },
      0xCC => Self {
        name: String::from("CPY"),
        opcode: 0xCC,
        opr: OPS::CPY,
        addr_mode: ADDR_MODE::ABSOLUTE,
        cycles: 4,
      },
      0xC6 => Self {
        name: String::from("DEC"),
        opcode: 0xC6,
        opr: OPS::DEC,
        addr_mode: ADDR_MODE::ZERO_PAGE,
        cycles: 5,
      },
      0xD6 => Self {
        name: String::from("DEC"),
        opcode: 0xD6,
        opr: OPS::DEC,
        addr_mode: ADDR_MODE::ZERO_PAGE_X,
        cycles: 6,
      },
      0xCE => Self {
        name: String::from("DEC"),
        opcode: 0xCE,
        opr: OPS::DEC,
        addr_mode: ADDR_MODE::ABSOLUTE,
        cycles: 6,
      },
      0xDE => Self {
        name: String::from("DEC"),
        opcode: 0xDE,
        opr: OPS::DEC,
        addr_mode: ADDR_MODE::ABSOLUTE_X,
        cycles: 7,
      },
      0xCA => Self {
        name: String::from("DEX"),
        opcode: 0xCA,
        opr: OPS::DEX,
        addr_mode: ADDR_MODE::IMPLIED,
        cycles: 2,
      },
      0x88 => Self {
        name: String::from("DEY"),
        opcode: 0x88,
        opr: OPS::DEY,
        addr_mode: ADDR_MODE::IMPLIED,
        cycles: 2,
      },
      0x49 => Self {
        name: String::from("EOR"),
        opcode: 0x49,
        opr: OPS::EOR,
        addr_mode: ADDR_MODE::IMMEDIATE,
        cycles: 2,
      },
      0x45 => Self {
        name: String::from("EOR"),
        opcode: 0x45,
        opr: OPS::EOR,
        addr_mode: ADDR_MODE::ZERO_PAGE,
        cycles: 3,
      },
      0x55 => Self {
        name: String::from("EOR"),
        opcode: 0x55,
        opr: OPS::EOR,
        addr_mode: ADDR_MODE::ZERO_PAGE_X,
        cycles: 4,
      },
      0x4D => Self {
        name: String::from("EOR"),
        opcode: 0x4D,
        opr: OPS::EOR,
        addr_mode: ADDR_MODE::ABSOLUTE,
        cycles: 4,
      },
      0x5D => Self {
        name: String::from("EOR"),
        opcode: 0x5D,
        opr: OPS::EOR,
        addr_mode: ADDR_MODE::ABSOLUTE_X,
        cycles: 4,
      },
      0x59 => Self {
        name: String::from("EOR"),
        opcode: 0x59,
        opr: OPS::EOR,
        addr_mode: ADDR_MODE::ABSOLUTE_Y,
        cycles: 4,
      },
      0x41 => Self {
        name: String::from("EOR"),
        opcode: 0x41,
        opr: OPS::EOR,
        addr_mode: ADDR_MODE::INDIRECT_X,
        cycles: 6,
      },
      0x51 => Self {
        name: String::from("EOR"),
        opcode: 0x51,
        opr: OPS::EOR,
        addr_mode: ADDR_MODE::INDIRECT_Y,
        cycles: 5,
      },
      0xE6 => Self {
        name: String::from("INC"),
        opcode: 0xE6,
        opr: OPS::INC,
        addr_mode: ADDR_MODE::ZERO_PAGE,
        cycles: 5,
      },
      0xF6 => Self {
        name: String::from("INC"),
        opcode: 0xF6,
        opr: OPS::INC,
        addr_mode: ADDR_MODE::ZERO_PAGE_X,
        cycles: 6,
      },
      0xEE => Self {
        name: String::from("INC"),
        opcode: 0xEE,
        opr: OPS::INC,
        addr_mode: ADDR_MODE::ABSOLUTE,
        cycles: 6,
      },
      0xFE => Self {
        name: String::from("INC"),
        opcode: 0xFE,
        opr: OPS::INC,
        addr_mode: ADDR_MODE::ABSOLUTE_X,
        cycles: 7,
      },
      0xE8 => Self {
        name: String::from("INX"),
        opcode: 0xE8,
        opr: OPS::INX,
        addr_mode: ADDR_MODE::IMPLIED,
        cycles: 2,
      },
      0xC8 => Self {
        name: String::from("INY"),
        opcode: 0xC8,
        opr: OPS::INY,
        addr_mode: ADDR_MODE::IMPLIED,
        cycles: 2,
      },
      0x4C => Self {
        name: String::from("JMP"),
        opcode: 0x4C,
        opr: OPS::JMP,
        addr_mode: ADDR_MODE::ABSOLUTE,
        cycles: 3,
      },
      0x6C => Self {
        name: String::from("JMP"),
        opcode: 0x6C,
        opr: OPS::JMP,
        addr_mode: ADDR_MODE::INDIRECT,
        cycles: 5,
      },
      0x20 => Self {
        name: String::from("JSR"),
        opcode: 0x20,
        opr: OPS::JSR,
        addr_mode: ADDR_MODE::ABSOLUTE,
        cycles: 6,
      },
      0xA9 => Self {
        name: String::from("LDA"),
        opcode: 0xA9,
        opr: OPS::LDA,
        addr_mode: ADDR_MODE::IMMEDIATE,
        cycles: 2,
      },
      0xA5 => Self {
        name: String::from("LDA"),
        opcode: 0xA5,
        opr: OPS::LDA,
        addr_mode: ADDR_MODE::ZERO_PAGE,
        cycles: 3,
      },
      0xB5 => Self {
        name: String::from("LDA"),
        opcode: 0xB5,
        opr: OPS::LDA,
        addr_mode: ADDR_MODE::ZERO_PAGE_X,
        cycles: 4,
      },
      0xAD => Self {
        name: String::from("LDA"),
        opcode: 0xAD,
        opr: OPS::LDA,
        addr_mode: ADDR_MODE::ABSOLUTE,
        cycles: 4,
      },
      0xBD => Self {
        name: String::from("LDA"),
        opcode: 0xBD,
        opr: OPS::LDA,
        addr_mode: ADDR_MODE::ABSOLUTE_X,
        cycles: 4,
      },
      0xB9 => Self {
        name: String::from("LDA"),
        opcode: 0xB9,
        opr: OPS::LDA,
        addr_mode: ADDR_MODE::ABSOLUTE_Y,
        cycles: 4,
      },
      0xA1 => Self {
        name: String::from("LDA"),
        opcode: 0xA1,
        opr: OPS::LDA,
        addr_mode: ADDR_MODE::INDIRECT_X,
        cycles: 6,
      },
      0xB1 => Self {
        name: String::from("LDA"),
        opcode: 0xB1,
        opr: OPS::LDA,
        addr_mode: ADDR_MODE::INDIRECT_Y,
        cycles: 5,
      },
      0xA2 => Self {
        name: String::from("LDX"),
        opcode: 0xA2,
        opr: OPS::LDX,
        addr_mode: ADDR_MODE::IMMEDIATE,
        cycles: 2,
      },
      0xA6 => Self {
        name: String::from("LDX"),
        opcode: 0xA6,
        opr: OPS::LDX,
        addr_mode: ADDR_MODE::ZERO_PAGE,
        cycles: 3,
      },
      0xB6 => Self {
        name: String::from("LDX"),
        opcode: 0xB6,
        opr: OPS::LDX,
        addr_mode: ADDR_MODE::ZERO_PAGE_Y,
        cycles: 4,
      },
      0xAE => Self {
        name: String::from("LDX"),
        opcode: 0xAE,
        opr: OPS::LDX,
        addr_mode: ADDR_MODE::ABSOLUTE,
        cycles: 4,
      },
      0xBE => Self {
        name: String::from("LDX"),
        opcode: 0xBE,
        opr: OPS::LDX,
        addr_mode: ADDR_MODE::ABSOLUTE_Y,
        cycles: 4,
      },
      0xA0 => Self {
        name: String::from("LDY"),
        opcode: 0xA0,
        opr: OPS::LDY,
        addr_mode: ADDR_MODE::IMMEDIATE,
        cycles: 2,
      },
      0xA4 => Self {
        name: String::from("LDY"),
        opcode: 0xA4,
        opr: OPS::LDY,
        addr_mode: ADDR_MODE::ZERO_PAGE,
        cycles: 3,
      },
      0xB4 => Self {
        name: String::from("LDY"),
        opcode: 0xB4,
        opr: OPS::LDY,
        addr_mode: ADDR_MODE::ZERO_PAGE_X,
        cycles: 4,
      },
      0xAC => Self {
        name: String::from("LDY"),
        opcode: 0xAC,
        opr: OPS::LDY,
        addr_mode: ADDR_MODE::ABSOLUTE,
        cycles: 4,
      },
      0xBC => Self {
        name: String::from("LDY"),
        opcode: 0xBC,
        opr: OPS::LDY,
        addr_mode: ADDR_MODE::ABSOLUTE_X,
        cycles: 4,
      },
      0x4A => Self {
        name: String::from("LSR"),
        opcode: 0x4A,
        opr: OPS::LSR,
        addr_mode: ADDR_MODE::IMPLIED,
        cycles: 2,
      },
      0x46 => Self {
        name: String::from("LSR"),
        opcode: 0x46,
        opr: OPS::LSR,
        addr_mode: ADDR_MODE::ZERO_PAGE,
        cycles: 5,
      },
      0x56 => Self {
        name: String::from("LSR"),
        opcode: 0x56,
        opr: OPS::LSR,
        addr_mode: ADDR_MODE::ZERO_PAGE_X,
        cycles: 6,
      },
      0x4E => Self {
        name: String::from("LSR"),
        opcode: 0x4E,
        opr: OPS::LSR,
        addr_mode: ADDR_MODE::ABSOLUTE,
        cycles: 6,
      },
      0x5E => Self {
        name: String::from("LSR"),
        opcode: 0x5E,
        opr: OPS::LSR,
        addr_mode: ADDR_MODE::ABSOLUTE_X,
        cycles: 7,
      },
      0xEA => Self {
        name: String::from("NOP"),
        opcode: 0xEA,
        opr: OPS::NOP,
        addr_mode: ADDR_MODE::IMPLIED,
        cycles: 2,
      },
      0x09 => Self {
        name: String::from("ORA"),
        opcode: 0x09,
        opr: OPS::ORA,
        addr_mode: ADDR_MODE::IMMEDIATE,
        cycles: 2,
      },
      0x05 => Self {
        name: String::from("ORA"),
        opcode: 0x05,
        opr: OPS::ORA,
        addr_mode: ADDR_MODE::ZERO_PAGE,
        cycles: 3,
      },
      0x15 => Self {
        name: String::from("ORA"),
        opcode: 0x15,
        opr: OPS::ORA,
        addr_mode: ADDR_MODE::ZERO_PAGE_X,
        cycles: 4,
      },
      0x0D => Self {
        name: String::from("ORA"),
        opcode: 0x0D,
        opr: OPS::ORA,
        addr_mode: ADDR_MODE::ABSOLUTE,
        cycles: 4,
      },
      0x1D => Self {
        name: String::from("ORA"),
        opcode: 0x1D,
        opr: OPS::ORA,
        addr_mode: ADDR_MODE::ABSOLUTE_X,
        cycles: 4,
      },
      0x19 => Self {
        name: String::from("ORA"),
        opcode: 0x19,
        opr: OPS::ORA,
        addr_mode: ADDR_MODE::ABSOLUTE_Y,
        cycles: 4,
      },
      0x01 => Self {
        name: String::from("ORA"),
        opcode: 0x01,
        opr: OPS::ORA,
        addr_mode: ADDR_MODE::INDIRECT_X,
        cycles: 6,
      },
      0x11 => Self {
        name: String::from("ORA"),
        opcode: 0x11,
        opr: OPS::ORA,
        addr_mode: ADDR_MODE::INDIRECT_Y,
        cycles: 5,
      },
      0x48 => Self {
        name: String::from("PHA"),
        opcode: 0x48,
        opr: OPS::PHA,
        addr_mode: ADDR_MODE::IMPLIED,
        cycles: 3,
      },
      0x08 => Self {
        name: String::from("PHP"),
        opcode: 0x08,
        opr: OPS::PHP,
        addr_mode: ADDR_MODE::IMPLIED,
        cycles: 3,
      },
      0x68 => Self {
        name: String::from("PLA"),
        opcode: 0x68,
        opr: OPS::PLA,
        addr_mode: ADDR_MODE::IMPLIED,
        cycles: 4,
      },
      0x28 => Self {
        name: String::from("PLP"),
        opcode: 0x28,
        opr: OPS::PLP,
        addr_mode: ADDR_MODE::IMPLIED,
        cycles: 4,
      },
      0x2A => Self {
        name: String::from("ROL"),
        opcode: 0x2A,
        opr: OPS::ROL,
        addr_mode: ADDR_MODE::IMPLIED,
        cycles: 2,
      },
      0x26 => Self {
        name: String::from("ROL"),
        opcode: 0x26,
        opr: OPS::ROL,
        addr_mode: ADDR_MODE::ZERO_PAGE,
        cycles: 5,
      },
      0x36 => Self {
        name: String::from("ROL"),
        opcode: 0x36,
        opr: OPS::ROL,
        addr_mode: ADDR_MODE::ZERO_PAGE_X,
        cycles: 6,
      },
      0x2E => Self {
        name: String::from("ROL"),
        opcode: 0x2E,
        opr: OPS::ROL,
        addr_mode: ADDR_MODE::ABSOLUTE,
        cycles: 6,
      },
      0x3E => Self {
        name: String::from("ROL"),
        opcode: 0x3E,
        opr: OPS::ROL,
        addr_mode: ADDR_MODE::ABSOLUTE_X,
        cycles: 7,
      },
      0x6A => Self {
        name: String::from("ROR"),
        opcode: 0x6A,
        opr: OPS::ROR,
        addr_mode: ADDR_MODE::IMPLIED,
        cycles: 2,
      },
      0x66 => Self {
        name: String::from("ROR"),
        opcode: 0x66,
        opr: OPS::ROR,
        addr_mode: ADDR_MODE::ZERO_PAGE,
        cycles: 5,
      },
      0x76 => Self {
        name: String::from("ROR"),
        opcode: 0x76,
        opr: OPS::ROR,
        addr_mode: ADDR_MODE::ZERO_PAGE_X,
        cycles: 6,
      },
      0x6E => Self {
        name: String::from("ROR"),
        opcode: 0x6E,
        opr: OPS::ROR,
        addr_mode: ADDR_MODE::ABSOLUTE,
        cycles: 6,
      },
      0x7E => Self {
        name: String::from("ROR"),
        opcode: 0x7E,
        opr: OPS::ROR,
        addr_mode: ADDR_MODE::ABSOLUTE_X,
        cycles: 7,
      },
      0x40 => Self {
        name: String::from("RTI"),
        opcode: 0x40,
        opr: OPS::RTI,
        addr_mode: ADDR_MODE::IMPLIED,
        cycles: 6,
      },
      0x60 => Self {
        name: String::from("RTS"),
        opcode: 0x60,
        opr: OPS::RTS,
        addr_mode: ADDR_MODE::IMPLIED,
        cycles: 6,
      },
      0xE9 => Self {
        name: String::from("SBC"),
        opcode: 0xE9,
        opr: OPS::SBC,
        addr_mode: ADDR_MODE::IMMEDIATE,
        cycles: 2,
      },
      0xE5 => Self {
        name: String::from("SBC"),
        opcode: 0xE5,
        opr: OPS::SBC,
        addr_mode: ADDR_MODE::ZERO_PAGE,
        cycles: 3,
      },
      0xF5 => Self {
        name: String::from("SBC"),
        opcode: 0xF5,
        opr: OPS::SBC,
        addr_mode: ADDR_MODE::ZERO_PAGE_X,
        cycles: 4,
      },
      0xED => Self {
        name: String::from("SBC"),
        opcode: 0xED,
        opr: OPS::SBC,
        addr_mode: ADDR_MODE::ABSOLUTE,
        cycles: 4,
      },
      0xFD => Self {
        name: String::from("SBC"),
        opcode: 0xFD,
        opr: OPS::SBC,
        addr_mode: ADDR_MODE::ABSOLUTE_X,
        cycles: 4,
      },
      0xF9 => Self {
        name: String::from("SBC"),
        opcode: 0xF9,
        opr: OPS::SBC,
        addr_mode: ADDR_MODE::ABSOLUTE_Y,
        cycles: 4,
      },
      0xE1 => Self {
        name: String::from("SBC"),
        opcode: 0xE1,
        opr: OPS::SBC,
        addr_mode: ADDR_MODE::INDIRECT_X,
        cycles: 6,
      },
      0xF1 => Self {
        name: String::from("SBC"),
        opcode: 0xF1,
        opr: OPS::SBC,
        addr_mode: ADDR_MODE::INDIRECT_Y,
        cycles: 5,
      },
      0x38 => Self {
        name: String::from("SEC"),
        opcode: 0x38,
        opr: OPS::SEC,
        addr_mode: ADDR_MODE::IMPLIED,
        cycles: 2,
      },
      0xF8 => Self {
        name: String::from("SED"),
        opcode: 0xF8,
        opr: OPS::SED,
        addr_mode: ADDR_MODE::IMPLIED,
        cycles: 2,
      },
      0x78 => Self {
        name: String::from("SEI"),
        opcode: 0x78,
        opr: OPS::SEI,
        addr_mode: ADDR_MODE::IMPLIED,
        cycles: 2,
      },
      0x85 => Self {
        name: String::from("STA"),
        opcode: 0x85,
        opr: OPS::STA,
        addr_mode: ADDR_MODE::ZERO_PAGE,
        cycles: 3,
      },
      0x95 => Self {
        name: String::from("STA"),
        opcode: 0x95,
        opr: OPS::STA,
        addr_mode: ADDR_MODE::ZERO_PAGE_X,
        cycles: 4,
      },
      0x8D => Self {
        name: String::from("STA"),
        opcode: 0x8D,
        opr: OPS::STA,
        addr_mode: ADDR_MODE::ABSOLUTE,
        cycles: 4,
      },
      0x9D => Self {
        name: String::from("STA"),
        opcode: 0x9D,
        opr: OPS::STA,
        addr_mode: ADDR_MODE::ABSOLUTE_X,
        cycles: 5,
      },
      0x99 => Self {
        name: String::from("STA"),
        opcode: 0x99,
        opr: OPS::STA,
        addr_mode: ADDR_MODE::ABSOLUTE_Y,
        cycles: 5,
      },
      0x81 => Self {
        name: String::from("STA"),
        opcode: 0x81,
        opr: OPS::STA,
        addr_mode: ADDR_MODE::INDIRECT_X,
        cycles: 6,
      },
      0x91 => Self {
        name: String::from("STA"),
        opcode: 0x91,
        opr: OPS::STA,
        addr_mode: ADDR_MODE::INDIRECT_Y,
        cycles: 6,
      },
      0x86 => Self {
        name: String::from("STX"),
        opcode: 0x86,
        opr: OPS::STX,
        addr_mode: ADDR_MODE::ZERO_PAGE,
        cycles: 3,
      },
      0x96 => Self {
        name: String::from("STX"),
        opcode: 0x96,
        opr: OPS::STX,
        addr_mode: ADDR_MODE::ZERO_PAGE_Y,
        cycles: 4,
      },
      0x8E => Self {
        name: String::from("STX"),
        opcode: 0x8E,
        opr: OPS::STX,
        addr_mode: ADDR_MODE::ABSOLUTE,
        cycles: 4,
      },
      0x84 => Self {
        name: String::from("STY"),
        opcode: 0x84,
        opr: OPS::STY,
        addr_mode: ADDR_MODE::ZERO_PAGE,
        cycles: 3,
      },
      0x94 => Self {
        name: String::from("STY"),
        opcode: 0x94,
        opr: OPS::STY,
        addr_mode: ADDR_MODE::ZERO_PAGE_X,
        cycles: 4,
      },
      0x8C => Self {
        name: String::from("STY"),
        opcode: 0x8C,
        opr: OPS::STY,
        addr_mode: ADDR_MODE::ABSOLUTE,
        cycles: 4,
      },
      0xAA => Self {
        name: String::from("TAX"),
        opcode: 0xAA,
        opr: OPS::TAX,
        addr_mode: ADDR_MODE::IMPLIED,
        cycles: 2,
      },
      0xA8 => Self {
        name: String::from("TAY"),
        opcode: 0xA8,
        opr: OPS::TAY,
        addr_mode: ADDR_MODE::IMPLIED,
        cycles: 2,
      },
      0xBA => Self {
        name: String::from("TSX"),
        opcode: 0xBA,
        opr: OPS::TSX,
        addr_mode: ADDR_MODE::IMPLIED,
        cycles: 2,
      },
      0x8A => Self {
        name: String::from("TXA"),
        opcode: 0x8A,
        opr: OPS::TXA,
        addr_mode: ADDR_MODE::IMPLIED,
        cycles: 2,
      },
      0x9A => Self {
        name: String::from("TXS"),
        opcode: 0x9A,
        opr: OPS::TXS,
        addr_mode: ADDR_MODE::IMPLIED,
        cycles: 2,
      },
      0x98 => Self {
        name: String::from("TYA"),
        opcode: 0x98,
        opr: OPS::TYA,
        addr_mode: ADDR_MODE::IMPLIED,
        cycles: 2,
      },
      _ => Self {
        name: String::from("???"),
        opcode: 0x00,
        opr: OPS::NOP,
        addr_mode: ADDR_MODE::IMPLIED,
        cycles: 0,
      },
    }
  }
}

pub fn execute(cpu: &mut CPU, instruction: Instruction) -> u8 {
  let mut addi_cycles: u8 = 0;
  addi_cycles += match instruction.addr_mode {
    ADDR_MODE::ABSOLUTE => abs(cpu),
    ADDR_MODE::ABSOLUTE_X => abx(cpu),
    ADDR_MODE::ABSOLUTE_Y => aby(cpu),
    ADDR_MODE::IMMEDIATE => imm(cpu),
    ADDR_MODE::IMPLIED => imp(cpu),
    ADDR_MODE::INDIRECT => ind(cpu),
    ADDR_MODE::INDIRECT_X => inx(cpu),
    ADDR_MODE::INDIRECT_Y => iny(cpu),
    ADDR_MODE::RELATIVE => rel(cpu),
    ADDR_MODE::ZERO_PAGE => zp0(cpu),
    ADDR_MODE::ZERO_PAGE_X => zpx(cpu),
    ADDR_MODE::ZERO_PAGE_Y => zpy(cpu),
    _ => 0x00,
  };

  addi_cycles += match instruction.opr {
    OPS::ADC => adc(cpu),
    OPS::AND => and(cpu),
    OPS::ASL => asl(cpu),
    OPS::BCC => bcc(cpu),
    OPS::BCS => bcs(cpu),
    OPS::BEQ => beq(cpu),
    OPS::BIT => bit(cpu),
    OPS::BMI => bmi(cpu),
    OPS::BNE => bne(cpu),
    OPS::BPL => bpl(cpu),
    OPS::BRK => brk(cpu),
    OPS::BVC => bvc(cpu),
    OPS::BVS => bvs(cpu),
    OPS::CLC => clc(cpu),
    OPS::CLD => cld(cpu),
    OPS::CLI => cli(cpu),
    OPS::CLV => clv(cpu),
    OPS::CMP => cmp(cpu),
    OPS::CPX => cpx(cpu),
    OPS::CPY => cpy(cpu),
    OPS::DEC => dec(cpu),
    OPS::DEX => dex(cpu),
    OPS::DEY => dey(cpu),
    OPS::EOR => eor(cpu),
    OPS::INC => inc(cpu),
    OPS::INX => inx(cpu),
    OPS::INY => iny(cpu),
    OPS::JMP => jmp(cpu),
    OPS::JSR => jsr(cpu),
    OPS::LDA => lda(cpu),
    OPS::LDX => ldx(cpu),
    OPS::LDY => ldy(cpu),
    OPS::LSR => lsr(cpu),
    OPS::NOP => nop(cpu),
    OPS::ORA => ora(cpu),
    OPS::PHA => pha(cpu),
    OPS::PHP => php(cpu),
    OPS::PLA => pla(cpu),
    OPS::PLP => plp(cpu),
    OPS::ROL => rol(cpu),
    OPS::ROR => ror(cpu),
    OPS::RTI => rti(cpu),
    OPS::RTS => rts(cpu),
    OPS::SBC => sbc(cpu),
    OPS::SEC => sec(cpu),
    OPS::SED => sed(cpu),
    OPS::SEI => sei(cpu),
    OPS::STA => sta(cpu),
    OPS::STX => stx(cpu),
    OPS::STY => sty(cpu),
    OPS::TAX => tax(cpu),
    OPS::TAY => tay(cpu),
    OPS::TSX => tsx(cpu),
    OPS::TXA => txa(cpu),
    OPS::TXS => txs(cpu),
    OPS::TYA => tya(cpu),
    _ => 0x00,
  };

  addi_cycles
}

/* ------- addressing modes -------- */
// immediate mode
fn imm(cpu: &mut CPU) -> u8 {
  cpu.working_addr = cpu.pc;

  cpu.pc += 1;
  0x00
}

// implied mode
fn imp(cpu: &mut CPU) -> u8 {
  cpu.working_data = cpu.reg_a;
  0x00
}

// zero page mode
fn zp0(cpu: &mut CPU) -> u8 {
  cpu.working_addr = cpu.fetch() as u16;

  0x00
}

// zero page x mode
fn zpx(cpu: &mut CPU) -> u8 {
  cpu.working_addr = (cpu.fetch() as u16 + cpu.reg_x as u16) & 0x00FF;

  0x00
}

// zero page y mode
fn zpy(cpu: &mut CPU) -> u8 {
  cpu.working_addr = (cpu.fetch() as u16 + cpu.reg_y as u16) & 0x00FF;

  0x00
}

// absolute mode
fn abs(cpu: &mut CPU) -> u8 {
  cpu.working_addr = cpu.fetch_word();

  0x00
}

// absolute x mode
fn abx(cpu: &mut CPU) -> u8 {
  let addr = cpu.fetch_word();
  cpu.working_addr = addr + cpu.reg_x as u16;

  // page boundary crossing
  if (addr & 0xFF00) != (0xFF00 & cpu.working_addr) {
    0x01
  } else {
    0x00
  }
}

// absolute y mode
fn aby(cpu: &mut CPU) -> u8 {
  let addr = cpu.fetch_word();
  cpu.working_addr = addr + cpu.reg_y as u16;

  // page boundary crossing
  if (addr & 0xFF00) != (0xFF00 & cpu.working_addr) {
    0x01
  } else {
    0x00
  }
}

// indirect mode
fn ind(cpu: &mut CPU) -> u8 {
  let addr = cpu.fetch_word();
  // simulate page boundary bug
  if addr & 0x00FF == 0x00FF {
    cpu.working_addr = cpu.mem.read(addr) as u16 | (cpu.mem.read(addr & 0xFF00) as u16) << 8;
  } else {
    cpu.working_addr = cpu.mem.read(addr) as u16 | (cpu.mem.read(addr + 1) as u16) << 8;
  }

  0x00
}

// indirect zero page x mode
fn izx(cpu: &mut CPU) -> u8 {
  let addr = cpu.fetch() as u16;
  let lo = cpu.mem.read((addr + cpu.reg_x as u16) & 0x00FF) as u16;
  let hi = cpu.mem.read((addr + cpu.reg_x as u16 + 1) & 0x00FF) as u16;
  cpu.working_addr = lo | (hi << 8);

  0x00
}

// indirect zero page y mode
fn izy(cpu: &mut CPU) -> u8 {
  let addr = cpu.fetch() as u16;
  let lo = cpu.mem.read(addr & 0x00FF) as u16;
  let hi = cpu.mem.read((addr + 1) & 0x00FF) as u16;
  cpu.working_addr = (lo | (hi << 8)) + cpu.reg_y as u16;

  // page boundary crossing
  if (addr & 0xFF00) != (0xFF00 & cpu.working_addr) {
    0x01
  } else {
    0x00
  }
}

// relative mode
fn rel(cpu: &mut CPU) -> u8 {
  cpu.working_addr = cpu.fetch() as u16;
  // page boundary crossing
  if (cpu.working_addr & 0x80) == 0x80 {
    cpu.working_addr |= 0xFF00;
  }

  0x00
}

/* ------- addressing modes -------- */

/* ------- OPs -------- */
// AND
fn and(cpu: &mut CPU) -> u8 {
  cpu.fill_working_data();
  cpu.reg_a &= cpu.working_data;

  if cpu.reg_a == 0x00 {
    cpu.status.set_flag(CpuStatus::Z);
  } else {
    cpu.status.clear_flag(CpuStatus::Z);
  }

  if cpu.reg_a & 0x80 == 0x80 {
    cpu.status.set_flag(CpuStatus::N);
  } else {
    cpu.status.clear_flag(CpuStatus::N);
  }

  0x01
}

// ASL
fn asl(cpu: &mut CPU) -> u8 {
  cpu.fill_working_data();
  let mut data = cpu.working_data;

  if data & 0x80 == 0x80 {
    cpu.status.set_flag(CpuStatus::C);
  } else {
    cpu.status.clear_flag(CpuStatus::C);
  }

  data <<= 1;

  if data == 0x00 {
    cpu.status.set_flag(CpuStatus::Z);
  } else {
    cpu.status.clear_flag(CpuStatus::Z);
  }

  if data & 0x80 == 0x80 {
    cpu.status.set_flag(CpuStatus::N);
  } else {
    cpu.status.clear_flag(CpuStatus::N);
  }

  if cpu.curr_instruction.addr_mode == ADDR_MODE::IMPLIED {
    cpu.reg_a = data;
  } else {
    cpu.mem.write(cpu.working_addr, data);
  }

  0x00
}

// BCS
fn bcs(cpu: &mut CPU) -> u8 {
  if cpu.status.is_flag_set(CpuStatus::C) {
    let mut cycles: u8 = 1;
    let addr = cpu.pc + cpu.rel_working_addr;

    // page boundary crossing
    if (addr & 0xFF00) != (cpu.pc & 0xFF00) {
      cycles += 1;
    }

    cpu.pc = addr;
    return cycles;
  }

  0x00
}

// BCC
fn bcc(cpu: &mut CPU) -> u8 {
  if !cpu.status.is_flag_set(CpuStatus::C) {
    let mut cycles: u8 = 1;
    let addr = cpu.pc + cpu.rel_working_addr;

    // page boundary crossing
    if (addr & 0xFF00) != (cpu.pc & 0xFF00) {
      cycles += 1;
    }

    cpu.pc = addr;
    return cycles;
  }

  0x00
}

// BEQ
fn beq(cpu: &mut CPU) -> u8 {
  if cpu.status.is_flag_set(CpuStatus::Z) {
    let mut cycles: u8 = 1;
    let addr = cpu.pc + cpu.rel_working_addr;

    // page boundary crossing
    if (addr & 0xFF00) != (cpu.pc & 0xFF00) {
      cycles += 1;
    }

    cpu.pc = addr;
    return cycles;
  }

  0x00
}

// BMI
fn bmi(cpu: &mut CPU) -> u8 {
  if cpu.status.is_flag_set(CpuStatus::N) {
    let mut cycles: u8 = 1;
    let addr = cpu.pc + cpu.rel_working_addr;

    // page boundary crossing
    if (addr & 0xFF00) != (cpu.pc & 0xFF00) {
      cycles += 1;
    }

    cpu.pc = addr;
    return cycles;
  }

  0x00
}

// BNE
fn bne(cpu: &mut CPU) -> u8 {
  if !cpu.status.is_flag_set(CpuStatus::Z) {
    let mut cycles: u8 = 1;
    let addr = cpu.pc + cpu.rel_working_addr;

    // page boundary crossing
    if (addr & 0xFF00) != (cpu.pc & 0xFF00) {
      cycles += 1;
    }

    cpu.pc = addr;
    return cycles;
  }

  0x00
}

// BPL
fn bpl(cpu: &mut CPU) -> u8 {
  if !cpu.status.is_flag_set(CpuStatus::N) {
    let mut cycles: u8 = 1;
    let addr = cpu.pc + cpu.rel_working_addr;

    // page boundary crossing
    if (addr & 0xFF00) != (cpu.pc & 0xFF00) {
      cycles += 1;
    }

    cpu.pc = addr;
    return cycles;
  }

  0x00
}

//BVC
fn bvc(cpu: &mut CPU) -> u8 {
  if !cpu.status.is_flag_set(CpuStatus::V) {
    let mut cycles: u8 = 1;
    let addr = cpu.pc + cpu.rel_working_addr;

    // page boundary crossing
    if (addr & 0xFF00) != (cpu.pc & 0xFF00) {
      cycles += 1;
    }

    cpu.pc = addr;
    return cycles;
  }

  0x00
}

// BVS
fn bvs(cpu: &mut CPU) -> u8 {
  if cpu.status.is_flag_set(CpuStatus::V) {
    let mut cycles: u8 = 1;
    let addr = cpu.pc + cpu.rel_working_addr;

    // page boundary crossing
    if (addr & 0xFF00) != (cpu.pc & 0xFF00) {
      cycles += 1;
    }

    cpu.pc = addr;
    return cycles;
  }

  0x00
}

// BIT
fn bit(cpu: &mut CPU) -> u8 {
  cpu.fill_working_data();
  let data = cpu.working_data;

  if data & 0x80 == 0x80 {
    cpu.status.set_flag(CpuStatus::N);
  } else {
    cpu.status.clear_flag(CpuStatus::N);
  }

  if data & 0x40 == 0x40 {
    cpu.status.set_flag(CpuStatus::V);
  } else {
    cpu.status.clear_flag(CpuStatus::V);
  }

  if data & cpu.reg_a == 0x00 {
    cpu.status.set_flag(CpuStatus::Z);
  } else {
    cpu.status.clear_flag(CpuStatus::Z);
  }

  0x00
}

// BRK
fn brk(cpu: &mut CPU) -> u8 {
  cpu.pc += 1;

  cpu.status.set_flag(CpuStatus::B);
  cpu.status.set_flag(CpuStatus::I);

  cpu.mem.write(cpu.sp as u16 + 0x0100, (cpu.pc >> 8) as u8);
  cpu.sp -= 1;

  cpu.mem.write(cpu.sp as u16 + 0x0100, cpu.pc as u8);
  cpu.sp -= 1;

  cpu.mem.write(cpu.sp as u16 + 0x0100, cpu.status.bits());
  cpu.sp -= 1;

  let lo = cpu.mem.read(0xFFFE) as u16;
  let hi = cpu.mem.read(0xFFFF) as u16;

  cpu.pc = (hi << 8) | lo;

  0x00
}

// CLC
fn clc(cpu: &mut CPU) -> u8 {
  cpu.status.clear_flag(CpuStatus::C);
  0x00
}

// CLD
fn cld(cpu: &mut CPU) -> u8 {
  cpu.status.clear_flag(CpuStatus::D);
  0x00
}

// CLI
fn cli(cpu: &mut CPU) -> u8 {
  cpu.status.clear_flag(CpuStatus::I);
  0x00
}

// CLV
fn clv(cpu: &mut CPU) -> u8 {
  cpu.status.clear_flag(CpuStatus::V);
  0x00
}

// CMP
fn cmp(cpu: &mut CPU) -> u8 {
  cpu.fill_working_data();
  let data = cpu.working_data;

  if cpu.reg_a >= data {
    cpu.status.set_flag(CpuStatus::C);
  } else {
    cpu.status.clear_flag(CpuStatus::C);
  }

  let temp = cpu.reg_a.wrapping_sub(data);

  if temp & 0x80 == 0x80 {
    cpu.status.set_flag(CpuStatus::N);
  } else {
    cpu.status.clear_flag(CpuStatus::N);
  }

  if temp == 0x00 {
    cpu.status.set_flag(CpuStatus::Z);
  } else {
    cpu.status.clear_flag(CpuStatus::Z);
  }

  0x01
}

// CPX
fn cpx(cpu: &mut CPU) -> u8 {
  cpu.fill_working_data();
  let data = cpu.working_data;

  if cpu.reg_x >= data {
    cpu.status.set_flag(CpuStatus::C);
  } else {
    cpu.status.clear_flag(CpuStatus::C);
  }

  let temp = cpu.reg_x.wrapping_sub(data);

  if temp & 0x80 == 0x80 {
    cpu.status.set_flag(CpuStatus::N);
  } else {
    cpu.status.clear_flag(CpuStatus::N);
  }

  if temp == 0x00 {
    cpu.status.set_flag(CpuStatus::Z);
  } else {
    cpu.status.clear_flag(CpuStatus::Z);
  }

  0x00
}

// CPY
fn cpy(cpu: &mut CPU) -> u8 {
  cpu.fill_working_data();
  let data = cpu.working_data;

  if cpu.reg_y >= data {
    cpu.status.set_flag(CpuStatus::C);
  } else {
    cpu.status.clear_flag(CpuStatus::C);
  }

  let temp = cpu.reg_y.wrapping_sub(data);

  if temp & 0x80 == 0x80 {
    cpu.status.set_flag(CpuStatus::N);
  } else {
    cpu.status.clear_flag(CpuStatus::N);
  }

  if temp == 0x00 {
    cpu.status.set_flag(CpuStatus::Z);
  } else {
    cpu.status.clear_flag(CpuStatus::Z);
  }

  0x00
}

// DEC
fn dec(cpu: &mut CPU) -> u8 {
  cpu.fill_working_data();
  let data = cpu.working_data;

  let temp = data.wrapping_sub(1);

  if temp & 0x80 == 0x80 {
    cpu.status.set_flag(CpuStatus::N);
  } else {
    cpu.status.clear_flag(CpuStatus::N);
  }

  if temp == 0x00 {
    cpu.status.set_flag(CpuStatus::Z);
  } else {
    cpu.status.clear_flag(CpuStatus::Z);
  }

  cpu.mem.write(cpu.working_addr, temp);

  0x00
}

// DEX
fn dex(cpu: &mut CPU) -> u8 {
  cpu.reg_x = cpu.reg_x.wrapping_sub(1);

  if cpu.reg_x & 0x80 == 0x80 {
    cpu.status.set_flag(CpuStatus::N);
  } else {
    cpu.status.clear_flag(CpuStatus::N);
  }

  if cpu.reg_x == 0x00 {
    cpu.status.set_flag(CpuStatus::Z);
  } else {
    cpu.status.clear_flag(CpuStatus::Z);
  }

  0x00
}

// DEY
fn dey(cpu: &mut CPU) -> u8 {
  cpu.reg_y = cpu.reg_y.wrapping_sub(1);

  if cpu.reg_y & 0x80 == 0x80 {
    cpu.status.set_flag(CpuStatus::N);
  } else {
    cpu.status.clear_flag(CpuStatus::N);
  }

  if cpu.reg_y == 0x00 {
    cpu.status.set_flag(CpuStatus::Z);
  } else {
    cpu.status.clear_flag(CpuStatus::Z);
  }

  0x00
}

// EOR
fn eor(cpu: &mut CPU) -> u8 {
  cpu.fill_working_data();
  let data = cpu.working_data;

  cpu.reg_a ^= data;

  if cpu.reg_a & 0x80 == 0x80 {
    cpu.status.set_flag(CpuStatus::N);
  } else {
    cpu.status.clear_flag(CpuStatus::N);
  }

  if cpu.reg_a == 0x00 {
    cpu.status.set_flag(CpuStatus::Z);
  } else {
    cpu.status.clear_flag(CpuStatus::Z);
  }

  0x01
}

// INC
fn inc(cpu: &mut CPU) -> u8 {
  cpu.fill_working_data();
  let data = cpu.working_data;

  let temp = data.wrapping_add(1);

  if temp & 0x80 == 0x80 {
    cpu.status.set_flag(CpuStatus::N);
  } else {
    cpu.status.clear_flag(CpuStatus::N);
  }

  if temp == 0x00 {
    cpu.status.set_flag(CpuStatus::Z);
  } else {
    cpu.status.clear_flag(CpuStatus::Z);
  }

  cpu.mem.write(cpu.working_addr, temp);

  0x00
}

// INX
fn inx(cpu: &mut CPU) -> u8 {
  cpu.reg_x = cpu.reg_x.wrapping_add(1);

  if cpu.reg_x & 0x80 == 0x80 {
    cpu.status.set_flag(CpuStatus::N);
  } else {
    cpu.status.clear_flag(CpuStatus::N);
  }

  if cpu.reg_x == 0x00 {
    cpu.status.set_flag(CpuStatus::Z);
  } else {
    cpu.status.clear_flag(CpuStatus::Z);
  }

  0x00
}

// INY
fn iny(cpu: &mut CPU) -> u8 {
  cpu.reg_y = cpu.reg_y.wrapping_add(1);

  if cpu.reg_y & 0x80 == 0x80 {
    cpu.status.set_flag(CpuStatus::N);
  } else {
    cpu.status.clear_flag(CpuStatus::N);
  }

  if cpu.reg_y == 0x00 {
    cpu.status.set_flag(CpuStatus::Z);
  } else {
    cpu.status.clear_flag(CpuStatus::Z);
  }

  0x00
}

// JMP
fn jmp(cpu: &mut CPU) -> u8 {
  cpu.pc = cpu.working_addr;

  0x00
}

// JSR
fn jsr(cpu: &mut CPU) -> u8 {
  let temp = cpu.pc - 1;

  cpu.mem.write(cpu.sp as u16 + 0x0100, (temp >> 8) as u8);
  cpu.sp = cpu.sp.wrapping_sub(1);

  cpu.mem.write(cpu.sp as u16 + 0x0100, temp as u8);
  cpu.sp = cpu.sp.wrapping_sub(1);

  cpu.pc = cpu.working_addr;

  0x00
}

// LDA
fn lda(cpu: &mut CPU) -> u8 {
  cpu.fill_working_data();
  let data = cpu.working_data;

  cpu.reg_a = data;

  if cpu.reg_a & 0x80 == 0x80 {
    cpu.status.set_flag(CpuStatus::N);
  } else {
    cpu.status.clear_flag(CpuStatus::N);
  }

  if cpu.reg_a == 0x00 {
    cpu.status.set_flag(CpuStatus::Z);
  } else {
    cpu.status.clear_flag(CpuStatus::Z);
  }

  0x01
}

// LDX
fn ldx(cpu: &mut CPU) -> u8 {
  cpu.fill_working_data();
  let data = cpu.working_data;

  cpu.reg_x = data;

  if cpu.reg_x & 0x80 == 0x80 {
    cpu.status.set_flag(CpuStatus::N);
  } else {
    cpu.status.clear_flag(CpuStatus::N);
  }

  if cpu.reg_x == 0x00 {
    cpu.status.set_flag(CpuStatus::Z);
  } else {
    cpu.status.clear_flag(CpuStatus::Z);
  }

  0x01
}

// LDY
fn ldy(cpu: &mut CPU) -> u8 {
  cpu.fill_working_data();
  let data = cpu.working_data;

  cpu.reg_y = data;

  if cpu.reg_y & 0x80 == 0x80 {
    cpu.status.set_flag(CpuStatus::N);
  } else {
    cpu.status.clear_flag(CpuStatus::N);
  }

  if cpu.reg_y == 0x00 {
    cpu.status.set_flag(CpuStatus::Z);
  } else {
    cpu.status.clear_flag(CpuStatus::Z);
  }

  0x01
}

// LSR
fn lsr(cpu: &mut CPU) -> u8 {
  cpu.fill_working_data();
  let data = cpu.working_data;

  if data & 0x01 == 0x01 {
    cpu.status.set_flag(CpuStatus::C);
  } else {
    cpu.status.clear_flag(CpuStatus::C);
  }

  let temp = data >> 1;

  if temp & 0x80 == 0x80 {
    cpu.status.set_flag(CpuStatus::N);
  } else {
    cpu.status.clear_flag(CpuStatus::N);
  }

  if temp == 0x00 {
    cpu.status.set_flag(CpuStatus::Z);
  } else {
    cpu.status.clear_flag(CpuStatus::Z);
  }

  if cpu.curr_instruction.addr_mode == ADDR_MODE::IMPLIED {
    cpu.reg_a = temp;
  } else {
    cpu.mem.write(cpu.working_addr, temp);
  }

  0x00
}

// NOP
fn nop(_: &mut CPU) -> u8 {
  0x00
}

// ORA
fn ora(cpu: &mut CPU) -> u8 {
  cpu.fill_working_data();
  let data = cpu.working_data;

  cpu.reg_a = cpu.reg_a | data;

  if cpu.reg_a & 0x80 == 0x80 {
    cpu.status.set_flag(CpuStatus::N);
  } else {
    cpu.status.clear_flag(CpuStatus::N);
  }

  if cpu.reg_a == 0x00 {
    cpu.status.set_flag(CpuStatus::Z);
  } else {
    cpu.status.clear_flag(CpuStatus::Z);
  }

  0x01
}

// PHA
fn pha(cpu: &mut CPU) -> u8 {
  cpu.mem.write(cpu.sp as u16 + 0x0100, cpu.reg_a);
  cpu.sp = cpu.sp.wrapping_sub(1);

  0x00
}

// PHP
fn php(cpu: &mut CPU) -> u8 {
  cpu.mem.write(cpu.sp as u16 + 0x0100, cpu.status.bits());
  cpu.sp = cpu.sp.wrapping_sub(1);

  0x00
}

// PLA
fn pla(cpu: &mut CPU) -> u8 {
  cpu.sp = cpu.sp.wrapping_add(1);
  cpu.reg_a = cpu.mem.read(cpu.sp as u16 + 0x0100);

  if cpu.reg_a & 0x80 == 0x80 {
    cpu.status.set_flag(CpuStatus::N);
  } else {
    cpu.status.clear_flag(CpuStatus::N);
  }

  if cpu.reg_a == 0x00 {
    cpu.status.set_flag(CpuStatus::Z);
  } else {
    cpu.status.clear_flag(CpuStatus::Z);
  }

  0x00
}

// PLP
fn plp(cpu: &mut CPU) -> u8 {
  cpu.sp = cpu.sp.wrapping_add(1);
  cpu.status = CpuStatus::from_bits_truncate(cpu.mem.read(cpu.sp as u16 + 0x0100));

  0x00
}

// ROL
fn rol(cpu: &mut CPU) -> u8 {
  cpu.fill_working_data();
  let data = cpu.working_data;

  let mut temp = (data as u16) << 1;

  if cpu.status.is_flag_set(CpuStatus::C) {
    temp |= 0x0001;
  }

  if temp & 0x0080 == 0x0080 {
    cpu.status.set_flag(CpuStatus::N);
  } else {
    cpu.status.clear_flag(CpuStatus::N);
  }

  if (temp as u8) == 0x00 {
    cpu.status.set_flag(CpuStatus::Z);
  } else {
    cpu.status.clear_flag(CpuStatus::Z);
  }

  if (temp & 0xFF00) > 0 {
    cpu.status.set_flag(CpuStatus::C);
  } else {
    cpu.status.clear_flag(CpuStatus::C);
  }

  if cpu.curr_instruction.addr_mode == ADDR_MODE::IMPLIED {
    cpu.reg_a = temp as u8;
  } else {
    cpu.mem.write(cpu.working_addr, temp as u8);
  }

  0x00
}

// ROR
fn ror(cpu: &mut CPU) -> u8 {
  cpu.fill_working_data();
  let data = cpu.working_data;
  let mut temp = (data as u16) << 1;

  if cpu.status.is_flag_set(CpuStatus::C) {
    temp |= 0x0001;
  }

  if temp & 0x0080 == 0x0080 {
    cpu.status.set_flag(CpuStatus::N);
  } else {
    cpu.status.clear_flag(CpuStatus::N);
  }

  if (temp as u8) == 0x00 {
    cpu.status.set_flag(CpuStatus::Z);
  } else {
    cpu.status.clear_flag(CpuStatus::Z);
  }

  if (temp & 0xFF00) > 0 {
    cpu.status.set_flag(CpuStatus::C);
  } else {
    cpu.status.clear_flag(CpuStatus::C);
  }

  if cpu.curr_instruction.addr_mode == ADDR_MODE::IMPLIED {
    cpu.reg_a = temp as u8;
  } else {
    cpu.mem.write(cpu.working_addr, temp as u8);
  }

  0x00
}

// RTI
fn rti(cpu: &mut CPU) -> u8 {
  cpu.sp = cpu.sp.wrapping_add(1);
  cpu.status = CpuStatus::from_bits_truncate(cpu.mem.read(cpu.sp as u16 + 0x0100));

  cpu.sp = cpu.sp.wrapping_add(1);
  cpu.pc = cpu.mem.read(cpu.sp as u16 + 0x0100) as u16;

  cpu.sp = cpu.sp.wrapping_add(1);
  cpu.pc |= (cpu.mem.read(cpu.sp as u16 + 0x0100) as u16) << 8;

  0x00
}

// RTS
fn rts(cpu: &mut CPU) -> u8 {
  cpu.sp = cpu.sp.wrapping_add(1);
  cpu.pc = cpu.mem.read(cpu.sp as u16 + 0x0100) as u16;

  cpu.sp = cpu.sp.wrapping_add(1);
  cpu.pc |= (cpu.mem.read(cpu.sp as u16 + 0x0100) as u16) << 8;

  cpu.pc = cpu.pc.wrapping_add(1);

  0x00
}

// SEC
fn sec(cpu: &mut CPU) -> u8 {
  cpu.status.set_flag(CpuStatus::C);

  0x00
}

// SED
fn sed(cpu: &mut CPU) -> u8 {
  cpu.status.set_flag(CpuStatus::D);

  0x00
}

// SEI
fn sei(cpu: &mut CPU) -> u8 {
  cpu.status.set_flag(CpuStatus::I);

  0x00
}

// STA
fn sta(cpu: &mut CPU) -> u8 {
  cpu.mem.write(cpu.working_addr, cpu.reg_a);

  0x00
}

// STX
fn stx(cpu: &mut CPU) -> u8 {
  cpu.mem.write(cpu.working_addr, cpu.reg_x);

  0x00
}

// STY
fn sty(cpu: &mut CPU) -> u8 {
  cpu.mem.write(cpu.working_addr, cpu.reg_y);

  0x00
}

// TAX
fn tax(cpu: &mut CPU) -> u8 {
  cpu.reg_x = cpu.reg_a;

  if cpu.reg_x & 0x80 == 0x80 {
    cpu.status.set_flag(CpuStatus::N);
  } else {
    cpu.status.clear_flag(CpuStatus::N);
  }

  if cpu.reg_x == 0x00 {
    cpu.status.set_flag(CpuStatus::Z);
  } else {
    cpu.status.clear_flag(CpuStatus::Z);
  }

  0x00
}

// TAY
fn tay(cpu: &mut CPU) -> u8 {
  cpu.reg_y = cpu.reg_a;

  if cpu.reg_y & 0x80 == 0x80 {
    cpu.status.set_flag(CpuStatus::N);
  } else {
    cpu.status.clear_flag(CpuStatus::N);
  }

  if cpu.reg_y == 0x00 {
    cpu.status.set_flag(CpuStatus::Z);
  } else {
    cpu.status.clear_flag(CpuStatus::Z);
  }

  0x00
}

// TSX
fn tsx(cpu: &mut CPU) -> u8 {
  cpu.reg_x = cpu.sp as u8;

  if cpu.reg_x & 0x80 == 0x80 {
    cpu.status.set_flag(CpuStatus::N);
  } else {
    cpu.status.clear_flag(CpuStatus::N);
  }

  if cpu.reg_x == 0x00 {
    cpu.status.set_flag(CpuStatus::Z);
  } else {
    cpu.status.clear_flag(CpuStatus::Z);
  }

  0x00
}

// TXA
fn txa(cpu: &mut CPU) -> u8 {
  cpu.reg_a = cpu.reg_x;

  if cpu.reg_a & 0x80 == 0x80 {
    cpu.status.set_flag(CpuStatus::N);
  } else {
    cpu.status.clear_flag(CpuStatus::N);
  }

  if cpu.reg_a == 0x00 {
    cpu.status.set_flag(CpuStatus::Z);
  } else {
    cpu.status.clear_flag(CpuStatus::Z);
  }

  0x00
}

// TXS
fn txs(cpu: &mut CPU) -> u8 {
  cpu.sp = cpu.reg_x as u16;

  0x00
}

// TYA
fn tya(cpu: &mut CPU) -> u8 {
  cpu.reg_a = cpu.reg_y;

  if cpu.reg_a & 0x80 == 0x80 {
    cpu.status.set_flag(CpuStatus::N);
  } else {
    cpu.status.clear_flag(CpuStatus::N);
  }

  if cpu.reg_a == 0x00 {
    cpu.status.set_flag(CpuStatus::Z);
  } else {
    cpu.status.clear_flag(CpuStatus::Z);
  }

  0x00
}

// XXX
fn xxx(cpu: &mut CPU) -> u8 {
  panic!("Invalid opcode: {:02X}", cpu.curr_instruction.opcode);
}

// ADC
fn adc(cpu: &mut CPU) -> u8 {
  let data = cpu.working_data;

  let mut temp = cpu.reg_a as u16 + data as u16;

  if cpu.status.is_flag_set(CpuStatus::C) {
    temp += 1;
  }

  if temp & 0x80 == 0x80 {
    cpu.status.set_flag(CpuStatus::N);
  } else {
    cpu.status.clear_flag(CpuStatus::N);
  }

  if temp & 0xFF == 0x00 {
    cpu.status.set_flag(CpuStatus::Z);
  } else {
    cpu.status.clear_flag(CpuStatus::Z);
  }

  if (!(cpu.reg_a as u16 ^ data as u16) & (cpu.reg_a as u16 ^ temp) & 0x0080) == 0x0080 {
    cpu.status.set_flag(CpuStatus::V);
  } else {
    cpu.status.clear_flag(CpuStatus::V);
  }

  if temp > 0xFF {
    cpu.status.set_flag(CpuStatus::C);
  } else {
    cpu.status.clear_flag(CpuStatus::C);
  }

  if cpu.curr_instruction.addr_mode == ADDR_MODE::IMPLIED {
    cpu.reg_a = temp as u8;
  }

  0x01
}

// SBC
fn sbc(cpu: &mut CPU) -> u8 {
  let data = cpu.working_data;

  let mut addi = cpu.reg_a as u16 + data as u16;
  if cpu.status.is_flag_set(CpuStatus::C) {
    addi += 1;
  }

  let mut temp = data as u16 ^ 0x00FF;

  if !cpu.status.is_flag_set(CpuStatus::C) {
    temp -= 1;
  }

  if temp & 0x0080 == 0x0080 {
    cpu.status.set_flag(CpuStatus::N);
  } else {
    cpu.status.clear_flag(CpuStatus::N);
  }

  if temp & 0x00FF == 0x0000 {
    cpu.status.set_flag(CpuStatus::Z);
  } else {
    cpu.status.clear_flag(CpuStatus::Z);
  }

  if (addi ^ cpu.reg_a as u16) & (addi ^ temp) & 0x0080 == 0x0080 {
    cpu.status.set_flag(CpuStatus::V);
  } else {
    cpu.status.clear_flag(CpuStatus::V);
  }

  if temp & 0xFF00 != 0x0000 {
    cpu.status.set_flag(CpuStatus::C);
  } else {
    cpu.status.clear_flag(CpuStatus::C);
  }

  if cpu.curr_instruction.addr_mode == ADDR_MODE::IMPLIED {
    cpu.reg_a = temp as u8;
  }

  0x01
}
