use std::process;

use crate::error;

const MEM_SIZE: usize = 0xFFFF;
type AddrType = u16;
const REGS_COUNT: usize = 8;
const PC_START: u16 = 0x0000;

#[rustfmt::skip]
enum Reg {
  R0, R1, R2, R3, R4, R5, // General purpose registers
  RC,                     // Condition
  RPC,                    // Program Counter
}

pub struct CPU {
  mem: [u16; MEM_SIZE],
  regs: [u32; REGS_COUNT],
}

impl CPU {
  pub fn new() -> CPU { CPU { mem: [0; MEM_SIZE], regs: [0; REGS_COUNT] } }

  fn mr(&self, addr: AddrType) -> u16 { self.mem[addr as usize] }

  fn mw(&mut self, addr: AddrType, value: u16) { self.mem[addr as usize] = value; }

  fn rr(&self, reg: Reg) -> u32 { self.regs[reg as usize] }

  fn rs(&mut self, reg: Reg, value: u32) { self.regs[reg as usize] = value; }

  pub fn reset(&mut self) { self.rs(Reg::RPC, PC_START as u32); }

  pub fn load(&mut self, code: &[u16]) {
    self.mem[PC_START as usize..PC_START as usize + code.len()].copy_from_slice(code);
  }

  pub fn run_single(&mut self) {
    let instr = self.mr(self.rr(Reg::RPC) as u16);
    let op = instr >> 12;

    match op {
      0x0000 => process::exit(0),
      _ => error!("Unknown opcode `{op:#04x}`!"),
    }
  }
}
