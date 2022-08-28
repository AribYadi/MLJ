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
  let cpu = run_cpu(&[0x1201], |cpu| cpu.regs[1] = 10);
  assert_eq!(cpu.mem[PC_START as usize + 2], 10);
}

#[test]
fn test_LDR() {
  let cpu = run_cpu(&[0x2201], |cpu| cpu.regs[1] = 10);
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
  let cpu = run_cpu(&[0x5050], |cpu| {
    cpu.regs[1] = 10;
    cpu.regs[2] = 10;
  });
  assert_eq!(cpu.regs[Reg::RC as usize], 1);
  let cpu = run_cpu(&[0x5250], |cpu| {
    cpu.regs[1] = 10;
    cpu.regs[2] = 11;
  });
  assert_eq!(cpu.regs[Reg::RC as usize], 1);
  let cpu = run_cpu(&[0x5450], |cpu| {
    cpu.regs[1] = 10;
    cpu.regs[2] = 11;
  });
  assert_eq!(cpu.regs[Reg::RC as usize], 1);
  let cpu = run_cpu(&[0x5650], |cpu| {
    cpu.regs[1] = 10;
    cpu.regs[2] = 10;
  });
  assert_eq!(cpu.regs[Reg::RC as usize], 1);
  let cpu = run_cpu(&[0x5850], |cpu| {
    cpu.regs[1] = 11;
    cpu.regs[2] = 10;
  });
  assert_eq!(cpu.regs[Reg::RC as usize], 1);
  let cpu = run_cpu(&[0x5A50], |cpu| {
    cpu.regs[1] = 10;
    cpu.regs[2] = 10;
  });
  assert_eq!(cpu.regs[Reg::RC as usize], 1);
}

#[test]
fn test_JMC() {
  let cpu = run_cpu_n_times(&[0x6002, 0x0000, 0x5000], 2, |cpu| cpu.regs[Reg::RC as usize] = 1);
  assert_eq!(cpu.regs[Reg::RPC as usize], 0x3);
  let cpu = run_cpu_n_times(&[0x6002, 0x5000, 0x0000], 2, |cpu| cpu.regs[Reg::RC as usize] = 0);
  assert_eq!(cpu.regs[Reg::RPC as usize], 0x2);
}

#[test]
fn test_JMP() {
  let cpu = run_cpu_n_times(&[0x7002, 0x0000, 0x5000], 2, |_| ());
  assert_eq!(cpu.regs[Reg::RPC as usize], 0x3);
}

#[test]
fn test_MOV() {
  let cpu = run_cpu(&[0x8001], |cpu| cpu.regs[1] = 10);
  assert_eq!(cpu.regs[0], 10);
  let cpu = run_cpu(&[0x810A], |_| ());
  assert_eq!(cpu.regs[0], 10);
}

#[test]
fn test_ADD() {
  let cpu = run_cpu(&[0x9001], |cpu| {
    cpu.regs[0] = 10;
    cpu.regs[1] = 20;
  });
  assert_eq!(cpu.regs[0], 30);
  let cpu = run_cpu(&[0x910A], |cpu| {
    cpu.regs[0] = 10;
  });
  assert_eq!(cpu.regs[0], 20);
}
