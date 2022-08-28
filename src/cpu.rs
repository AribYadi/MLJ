use std::process;

use crate::error;

const MEM_SIZE: usize = 0xFFFF;
type AddrType = u16;
const REGS_COUNT: usize = 8;
const PC_START: u16 = 0x0000;

#[rustfmt::skip]
#[allow(clippy::upper_case_acronyms)]
enum Reg {
  R0, R1, R2, R3, R4, R5, // General purpose registers
  RC,                     // Condition
  RPC,                    // Program Counter
}

#[allow(clippy::upper_case_acronyms)]
pub struct CPU {
  pub mem: [u16; MEM_SIZE],
  pub regs: [u32; REGS_COUNT],
}

impl CPU {
  pub fn new() -> CPU { CPU { mem: [0; MEM_SIZE], regs: [0; REGS_COUNT] } }

  fn mr(&self, addr: AddrType) -> u16 { self.mem[addr as usize] }

  fn mw(&mut self, addr: AddrType, value: u16) { self.mem[addr as usize] = value; }

  fn rr(&self, reg: Reg) -> u32 { self.regs[reg as usize] }

  fn rw(&mut self, reg: Reg, value: u32) { self.regs[reg as usize] = value; }

  pub fn reset(&mut self) {
    self.regs.copy_from_slice(&[0; REGS_COUNT]);
    self.rw(Reg::RPC, PC_START as u32);
  }

  pub fn load(&mut self, code: &[u16]) {
    self.mem[PC_START as usize..PC_START as usize + code.len()].copy_from_slice(code);
  }

  pub fn run_single(&mut self) {
    let pc = self.rr(Reg::RPC);
    let instr = self.mr(pc as u16);
    self.rw(Reg::RPC, pc + 1);

    let op = instr >> 12;

    match op {
      0x00 => process::exit(0),
      0x01 => self.STR(instr),
      0x02 => self.LDR(instr),

      _ => error!("Unknown opcode `{op:#04x}`!"),
    }
  }
}

#[allow(non_snake_case)]
impl CPU {
  fn STR(&mut self, instr: u16) {
    let sr = self.regs[((instr >> 9) & 0x07) as usize];
    let off = sext(instr & 0x1F, 9);

    let addr = off + self.rr(Reg::RPC) as u16;
    self.mw(addr, sr as u16);
  }

  fn LDR(&mut self, instr: u16) {
    let dr = (instr >> 9) & 0x07;
    let off = sext(instr & 0x1F, 9);

    let addr = off + self.rr(Reg::RPC) as u16;
    self.regs[dr as usize] = self.mr(addr) as u32;
  }
}

fn sext(n: u16, bits: u16) -> u16 {
  if ((n >> (bits - 1)) & 1) == 1 {
    n | (u16::MAX << bits)
  } else {
    n
  }
}
