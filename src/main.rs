use cpu::CPU;

mod cpu;

#[macro_export]
macro_rules! error {
  ($($tt:tt)+) => {{
    eprintln!($($tt)+);
    process::exit(1);
  }};
}

fn main() {
  let mut cpu = CPU::new();
  cpu.reset();

  cpu.regs[2] = 10;

  cpu.load(&[0x4002]);
  cpu.run_single();
  cpu.run_single();

  assert_eq!(cpu.regs[2], 9);
}
