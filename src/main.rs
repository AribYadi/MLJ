mod cpu;

#[macro_export]
macro_rules! error {
  ($exit_handler:expr, $($tt:tt)+) => {{
    eprintln!($($tt)+);
    $exit_handler(1);
  }};
}

fn main() {
  todo!();
}
