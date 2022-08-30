use byteorder::{
  BigEndian,
  ReadBytesExt,
};
use cpu::CPU;

mod cpu;

#[macro_export]
macro_rules! error {
  ($exit_handler:expr, $($tt:tt)+) => {{
    eprintln!($($tt)+);
    $exit_handler(1);
  }};
}

fn main() {
  let file_path = match std::env::args().nth(1) {
    Some(file_path) => file_path,
    None => error!(std::process::exit, "Expected object file!"),
  };
  let f = match std::fs::File::open(file_path) {
    Ok(f) => f,
    Err(_) => error!(std::process::exit, "Cannot read file!"),
  };
  let mut rdr = std::io::BufReader::new(f);

  let mut program = vec![];
  while let Ok(word) = rdr.read_u16::<BigEndian>() {
    program.push(word);
  }

  let mut cpu = CPU::new();
  cpu.reset();
  cpu.load(&program);
  loop {
    cpu.run_single();
  }
}
