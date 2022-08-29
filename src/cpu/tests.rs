#![allow(non_snake_case)]

use super::*;

fn panic(e: i32) -> ! {
  panic!("Exited with error code {e}.");
}

fn run_cpu(code: &[u16], init: fn(&mut CPU)) -> CPU { run_cpu_n_times(code, code.len(), init) }

fn run_cpu_n_times(code: &[u16], n: usize, init: fn(&mut CPU)) -> CPU {
  let mut cpu = CPU::new();
  cpu.exit_handler = panic;

  cpu.reset();
  cpu.load(code);
  init(&mut cpu);

  for _ in 0..n {
    cpu.run_single();
  }

  cpu
}

#[test]
#[should_panic]
fn test_EXT() { run_cpu(&[0x0000], |_| ()); }

#[test]
fn test_STR() {
  let cpu = run_cpu(&[0x1101], |cpu| cpu.regs[1] = 10);
  assert_eq!(cpu.mem[PC_START as usize + 2], 10);
}

#[test]
fn test_LDR() {
  let cpu = run_cpu(&[0x2101], |cpu| cpu.regs[1] = 10);
  assert_eq!(cpu.regs[1], 0);
}

#[test]
fn test_INC() {
  let cpu = run_cpu(&[0x3001], |cpu| cpu.regs[1] = 10);
  assert_eq!(cpu.regs[1], 11);
  let cpu = run_cpu(&[0x3801], |_| ());
  assert_eq!(cpu.mem[PC_START as usize + 2], 1);
}

#[test]
fn test_DEC() {
  let cpu = run_cpu(&[0x4001], |cpu| cpu.regs[1] = 10);
  assert_eq!(cpu.regs[1], 9);
  let cpu = run_cpu(&[0x4801], |_| ());
  assert_eq!(cpu.mem[PC_START as usize + 2], u16::MAX);
}

#[test]
fn test_CMP() {
  let cpu = run_cpu(&[0x5024], |cpu| {
    cpu.regs[1] = 10;
    cpu.regs[2] = 10;
  });
  assert_eq!(cpu.regs[Reg::RC as usize], 1);
  let cpu = run_cpu(&[0x5224], |cpu| {
    cpu.regs[1] = 10;
    cpu.regs[2] = 11;
  });
  assert_eq!(cpu.regs[Reg::RC as usize], 1);
  let cpu = run_cpu(&[0x5424], |cpu| {
    cpu.regs[1] = 10;
    cpu.regs[2] = 11;
  });
  assert_eq!(cpu.regs[Reg::RC as usize], 1);
  let cpu = run_cpu(&[0x5624], |cpu| {
    cpu.regs[1] = 10;
    cpu.regs[2] = 10;
  });
  assert_eq!(cpu.regs[Reg::RC as usize], 1);
  let cpu = run_cpu(&[0x5824], |cpu| {
    cpu.regs[1] = 11;
    cpu.regs[2] = 10;
  });
  assert_eq!(cpu.regs[Reg::RC as usize], 1);
  let cpu = run_cpu(&[0x5A24], |cpu| {
    cpu.regs[1] = 10;
    cpu.regs[2] = 10;
  });
  assert_eq!(cpu.regs[Reg::RC as usize], 1);
}

#[test]
fn test_JMC() {
  let cpu = run_cpu_n_times(&[0x6003], 1, |cpu| cpu.regs[Reg::RC as usize] = 1);
  assert_eq!(cpu.regs[Reg::RPC as usize], 0x3);
  let cpu = run_cpu_n_times(&[0x6003], 1, |cpu| cpu.regs[Reg::RC as usize] = 0);
  assert_eq!(cpu.regs[Reg::RPC as usize], PC_START as u32 + 1);
}

#[test]
fn test_JMP() {
  let cpu = run_cpu_n_times(&[0x7003], 1, |_| ());
  assert_eq!(cpu.regs[Reg::RPC as usize], 0x3);
}

#[test]
fn test_MOV() {
  let cpu = run_cpu(&[0x8001], |cpu| cpu.regs[1] = 10);
  assert_eq!(cpu.regs[0], 10);
  let cpu = run_cpu(&[0x808A], |_| ());
  assert_eq!(cpu.regs[0], 10);
}

#[test]
fn test_ADD() {
  let cpu = run_cpu(&[0x9001], |cpu| {
    cpu.regs[0] = 10;
    cpu.regs[1] = 20;
  });
  assert_eq!(cpu.regs[0], 30);
  let cpu = run_cpu(&[0x908A], |cpu| {
    cpu.regs[0] = 10;
  });
  assert_eq!(cpu.regs[0], 20);
}

#[test]
fn test_SUB() {
  let cpu = run_cpu(&[0xA001], |cpu| {
    cpu.regs[0] = 10;
    cpu.regs[1] = 20;
  });
  assert_eq!(cpu.regs[0], -10i32 as u32);
  let cpu = run_cpu(&[0xA08A], |cpu| {
    cpu.regs[0] = 10;
  });
  assert_eq!(cpu.regs[0], 0);
}

#[test]
fn test_MUL() {
  let cpu = run_cpu(&[0xB001], |cpu| {
    cpu.regs[0] = 10;
    cpu.regs[1] = 20;
  });
  assert_eq!(cpu.regs[0], 200);
  let cpu = run_cpu(&[0xB08A], |cpu| {
    cpu.regs[0] = 10;
  });
  assert_eq!(cpu.regs[0], 100);
}

#[test]
fn test_DIV() {
  let cpu = run_cpu(&[0xC001], |cpu| {
    cpu.regs[0] = 20;
    cpu.regs[1] = 10;
  });
  assert_eq!(cpu.regs[0], 2);
  let cpu = run_cpu(&[0xC08A], |cpu| {
    cpu.regs[0] = 10;
  });
  assert_eq!(cpu.regs[0], 1);
}

#[test]
fn test_REM() {
  let cpu = run_cpu(&[0xD001], |cpu| {
    cpu.regs[0] = 20;
    cpu.regs[1] = 10;
  });
  assert_eq!(cpu.regs[0], 0);
  let cpu = run_cpu(&[0xD08A], |cpu| {
    cpu.regs[0] = 25;
  });
  assert_eq!(cpu.regs[0], 5);
}

#[test]
fn test_CLL() {
  let cpu = run_cpu(&[0xE000], |_| ());
  assert_eq!(cpu.mem[CALL_STACK_START as usize], PC_START + 1);
  assert_eq!(cpu.rr(Reg::RSP), CALL_STACK_START as u32 - 1);
}

#[test]
fn test_RET() {
  let cpu = run_cpu_n_times(&[0xE000], 2, |cpu| cpu.mem[0x0000] = 0xF000);
  assert_eq!(cpu.rr(Reg::RSP), CALL_STACK_START as u32);
  assert_eq!(cpu.rr(Reg::RPC), PC_START as u32 + 1);
}
