#![allow(non_snake_case)]

use super::*;

fn panic(e: i32) -> ! {
  panic!("Exited with error code {e}.");
}

fn run_cpu(code: &[u16], init: fn(&mut CPU)) -> CPU {
  let mut cpu = CPU::new();
  cpu.exit_handler = panic;

  cpu.reset();
  cpu.load(code);
  init(&mut cpu);

  for _ in 0..code.len() {
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
