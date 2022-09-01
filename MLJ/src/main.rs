use byteorder::{
  BigEndian,
  ReadBytesExt,
};
use cpu::{
  CPU,
  DISPLAY_SIZE,
  DISPLAY_START,
};
use macroquad::prelude::*;

mod cpu;

#[macro_export]
macro_rules! error {
  ($exit_handler:expr, $($tt:tt)+) => {{
    eprintln!($($tt)+);
    $exit_handler(1);
  }};
}

const DISPLAY_MULTIPLIER: usize = 15;

const COLORS: &[Color] = &[
  color_u8!(0x00, 0x00, 0x00, 0xFF), // BLACK
  color_u8!(0xFF, 0x00, 0x44, 0xFF), // RED
  color_u8!(0xF7, 0x76, 0x22, 0xFF), // ORANGE
  color_u8!(0xFE, 0xE7, 0x61, 0xFF), // YELLOW
  color_u8!(0x63, 0xC7, 0x4D, 0xFF), // GREEN
  color_u8!(0x00, 0x99, 0xDB, 0xFF), // BLUE
  color_u8!(0xB5, 0x50, 0x88, 0xFF), // PURPLE
];

fn display_conf() -> Conf {
  Conf {
    window_title: "MLJ".to_string(),
    window_width: (DISPLAY_SIZE * DISPLAY_MULTIPLIER) as i32,
    window_height: (DISPLAY_SIZE * DISPLAY_MULTIPLIER) as i32,
    ..Default::default()
  }
}

#[macroquad::main(display_conf)]
async fn main() {
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

    for (i, pixel) in cpu.mem
      [DISPLAY_START as usize..DISPLAY_START as usize + DISPLAY_SIZE * DISPLAY_SIZE]
      .iter()
      .enumerate()
    {
      let x = i % DISPLAY_SIZE;
      let y = i / DISPLAY_SIZE;
      draw_rectangle(
        (x * DISPLAY_MULTIPLIER) as f32,
        (y * DISPLAY_MULTIPLIER) as f32,
        DISPLAY_MULTIPLIER as f32,
        DISPLAY_MULTIPLIER as f32,
        COLORS[*pixel as usize % COLORS.len()],
      );
    }

    next_frame().await;
  }
}
