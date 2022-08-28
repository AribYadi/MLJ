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
  cpu.load(&[0x0000]);
  cpu.run_single();
  cpu.run_single();
}
