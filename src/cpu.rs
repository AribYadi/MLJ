#![allow(dead_code)]

#[cfg(test)]
mod tests;

use crate::error;

const MEM_SIZE: usize = 0xFFFF;
type AddrType = u16;
const REGS_COUNT: usize = 9;
const CALL_STACK_START: u16 = 0x200;
const PC_START: u16 = CALL_STACK_START + 1;

macro_rules! unwrap_reg {
  ($self:ident, $reg:expr) => {{
    if $reg as usize >= REGS_COUNT {
      error!($self.exit_handler, "Unknown register id `{reg}`!", reg = $reg);
    }
    $reg
  }};
}

#[rustfmt::skip]
#[allow(clippy::upper_case_acronyms)]
enum Reg {
  R0, R1, R2, R3, R4, R5, // General purpose registers
  RC,                     // Condition
  RPC,                    // Program Counter
  RSP,                    // Stack Pointer
}

#[allow(clippy::upper_case_acronyms)]
pub struct CPU {
  pub mem: [u16; MEM_SIZE],
  pub regs: [u32; REGS_COUNT],
  exit_handler: fn(i32) -> !,
}

impl CPU {
  pub fn new() -> CPU {
    CPU { mem: [0; MEM_SIZE], regs: [0; REGS_COUNT], exit_handler: std::process::exit }
  }

  fn mr(&self, addr: AddrType) -> u16 { self.mem[addr as usize] }

  fn mw(&mut self, addr: AddrType, value: u16) { self.mem[addr as usize] = value; }

  fn rr(&self, reg: Reg) -> u32 { self.regs[reg as usize] }

  fn rw(&mut self, reg: Reg, value: u32) { self.regs[reg as usize] = value; }

  fn push_rpc(&mut self) {
    let rpc = self.rr(Reg::RPC) as u16;
    let rsp = &mut self.regs[Reg::RSP as usize];
    self.mem[*rsp as usize] = rpc;
    *rsp -= 1;
  }

  fn pop_rpc(&mut self) {
    let rsp = &mut self.regs[Reg::RSP as usize];
    *rsp += 1;
    let rpc = self.mem[*rsp as usize];
    self.rw(Reg::RPC, rpc as u32);
  }

  pub fn reset(&mut self) {
    self.regs.copy_from_slice(&[0; REGS_COUNT]);
    self.rw(Reg::RPC, PC_START as u32);
    self.rw(Reg::RSP, CALL_STACK_START as u32);
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
      0x0 => (self.exit_handler)(0),
      0x1 => self.STR(instr),
      0x2 => self.LDR(instr),
      0x3 => self.INC(instr),
      0x4 => self.DEC(instr),
      0x5 => self.CMP(instr),
      0x6 => self.JMC(instr),
      0x7 => self.JMP(instr),
      0x8 => self.MOV(instr),
      0x9 => self.ADD(instr),
      0xA => self.SUB(instr),
      0xB => self.MUL(instr),
      0xC => self.DIV(instr),
      0xD => self.REM(instr),
      0xE => self.CLL(instr),
      0xF => self.RET(instr),

      _ => error!(self.exit_handler, "Unknown opcode `{op:#02x}`!"),
    }
  }
}

#[allow(non_snake_case)]
impl CPU {
  fn STR(&mut self, instr: u16) {
    let sr = unwrap_reg!(self, (instr >> 9) & 0x7);
    let off = sext(instr & 0x1F, 9);

    let addr = off + self.rr(Reg::RPC) as u16;
    self.mw(addr, self.regs[sr as usize] as u16);
  }

  fn LDR(&mut self, instr: u16) {
    let dr = unwrap_reg!(self, (instr >> 9) & 0x07);
    let off = sext(instr & 0x1FF, 9);

    let addr = off + self.rr(Reg::RPC) as u16;
    self.regs[dr as usize] = self.mr(addr) as u32;
  }

  fn INC(&mut self, instr: u16) {
    let mode = (instr >> 11) & 1;
    match mode {
      0 => {
        let reg = unwrap_reg!(self, instr & 0x7);
        let reg = &mut self.regs[reg as usize];
        *reg = reg.wrapping_add(1);
      },
      1 => {
        let off = sext(instr & 0x7FF, 11);
        let addr = off + self.rr(Reg::RPC) as u16;
        let mem = &mut self.mem[addr as usize];
        *mem = mem.wrapping_add(1);
      },
      _ => unreachable!(),
    }
  }

  fn DEC(&mut self, instr: u16) {
    let mode = (instr >> 11) & 0x1;
    match mode {
      0 => {
        let reg = unwrap_reg!(self, instr & 0x7);
        let reg = &mut self.regs[reg as usize];
        *reg = reg.wrapping_sub(1);
      },
      1 => {
        let off = sext(instr & 0x7FF, 11);
        let addr = off + self.rr(Reg::RPC) as u16;
        let mem = &mut self.mem[addr as usize];
        *mem = mem.wrapping_sub(1);
      },
      _ => unreachable!(),
    }
  }

  fn CMP(&mut self, instr: u16) {
    let sr1 = unwrap_reg!(self, (instr >> 6) & 0x7);
    let sr1 = self.regs[sr1 as usize];
    let sr2 = unwrap_reg!(self, (instr >> 3) & 0x7);
    let sr2 = self.regs[sr2 as usize];

    let mode = (instr >> 9) & 0x7;
    let result = match mode {
      0 => sr1 == sr2,
      1 => sr1 != sr2,
      2 => sr1 < sr2,
      3 => sr1 <= sr2,
      4 => sr1 > sr2,
      5 => sr1 >= sr2,
      _ => error!(self.exit_handler, "Unknown mode `{mode}` for `CMP`!"),
    };

    self.rw(Reg::RC, result as u32);
  }

  fn JMC(&mut self, instr: u16) {
    if self.rr(Reg::RC) == 1 {
      let addr = instr & 0xFFF;
      self.rw(Reg::RPC, addr as u32);
    }
  }

  fn JMP(&mut self, instr: u16) {
    let addr = instr & 0xFFF;
    self.rw(Reg::RPC, addr as u32);
  }

  fn MOV(&mut self, instr: u16) {
    let dr = unwrap_reg!(self, (instr >> 9) & 0x7);

    let mode = (instr >> 8) & 0x1;
    match mode {
      0 => {
        let sr = unwrap_reg!(self, instr & 0xFF);
        self.regs[dr as usize] = self.regs[sr as usize];
      },
      1 => {
        let imm = sext(instr & 0xFF, 8);
        self.regs[dr as usize] = imm as u32;
      },
      _ => unreachable!(),
    }
  }

  fn ADD(&mut self, instr: u16) {
    let sr1 = unwrap_reg!(self, (instr >> 9) & 0x7);

    let mode = (instr >> 8) & 0x1;
    match mode {
      0 => {
        let sr2 = unwrap_reg!(self, instr & 0xFF);
        let sr2 = self.regs[sr2 as usize];
        let sr1 = &mut self.regs[sr1 as usize];
        *sr1 = sr1.wrapping_add(sr2);
      },
      1 => {
        let imm = sext(instr & 0xFF, 8);
        let sr1 = &mut self.regs[sr1 as usize];
        *sr1 = sr1.wrapping_add(imm as u32);
      },
      _ => unreachable!(),
    }
  }

  fn SUB(&mut self, instr: u16) {
    let sr1 = unwrap_reg!(self, (instr >> 9) & 0x7);

    let mode = (instr >> 8) & 0x1;
    match mode {
      0 => {
        let sr2 = unwrap_reg!(self, instr & 0xFF);
        let sr2 = self.regs[sr2 as usize];
        let sr1 = &mut self.regs[sr1 as usize];
        *sr1 = sr1.wrapping_sub(sr2);
      },
      1 => {
        let imm = sext(instr & 0xFF, 8);
        let sr1 = &mut self.regs[sr1 as usize];
        *sr1 = sr1.wrapping_sub(imm as u32);
      },
      _ => unreachable!(),
    }
  }

  fn MUL(&mut self, instr: u16) {
    let sr1 = unwrap_reg!(self, (instr >> 9) & 0x7);

    let mode = (instr >> 8) & 0x1;
    match mode {
      0 => {
        let sr2 = unwrap_reg!(self, instr & 0xFF);
        let sr2 = self.regs[sr2 as usize];
        let sr1 = &mut self.regs[sr1 as usize];
        *sr1 = sr1.wrapping_mul(sr2);
      },
      1 => {
        let imm = sext(instr & 0xFF, 8);
        let sr1 = &mut self.regs[sr1 as usize];
        *sr1 = sr1.wrapping_mul(imm as u32);
      },
      _ => unreachable!(),
    }
  }

  fn DIV(&mut self, instr: u16) {
    let sr1 = unwrap_reg!(self, (instr >> 9) & 0x7);

    let mode = (instr >> 8) & 0x1;
    match mode {
      0 => {
        let sr2 = unwrap_reg!(self, instr & 0xFF);
        let sr2 = self.regs[sr2 as usize];
        let sr1 = &mut self.regs[sr1 as usize];
        *sr1 = sr1.wrapping_div(sr2);
      },
      1 => {
        let imm = sext(instr & 0xFF, 8);
        let sr1 = &mut self.regs[sr1 as usize];
        *sr1 = sr1.wrapping_div(imm as u32);
      },
      _ => unreachable!(),
    }
  }

  fn REM(&mut self, instr: u16) {
    let sr1 = unwrap_reg!(self, (instr >> 9) & 0x7);

    let mode = (instr >> 8) & 0x1;
    match mode {
      0 => {
        let sr2 = unwrap_reg!(self, instr & 0xFF);
        let sr2 = self.regs[sr2 as usize];
        let sr1 = &mut self.regs[sr1 as usize];
        *sr1 = sr1.wrapping_rem(sr2);
      },
      1 => {
        let imm = sext(instr & 0xFF, 8);
        let sr1 = &mut self.regs[sr1 as usize];
        *sr1 = sr1.wrapping_rem(imm as u32);
      },
      _ => unreachable!(),
    }
  }

  fn CLL(&mut self, instr: u16) {
    self.push_rpc();
    let addr = instr & 0xFFF;
    self.rw(Reg::RPC, addr as u32);
  }

  fn RET(&mut self, _instr: u16) { self.pop_rpc(); }
}

fn sext(n: u16, bits: u16) -> u16 {
  if ((n >> (bits - 1)) & 1) == 1 {
    n | (u16::MAX << bits)
  } else {
    n
  }
}
