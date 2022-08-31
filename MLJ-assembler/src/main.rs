use std::io::Read;
use std::process;

use byteorder::{
  BigEndian,
  WriteBytesExt,
};
use line_col::LineColLookup;
use logos::Logos;
use token::Token;

mod token;

fn main() {
  let in_file_path = match std::env::args().nth(1) {
    Some(in_file_path) => in_file_path,
    None => {
      eprintln!("Expected file path!");
      process::exit(1);
    },
  };
  let path = std::path::Path::new(&in_file_path).with_extension("obj");
  let out_file_path = path.to_string_lossy();

  let in_file = match std::fs::File::open(&in_file_path) {
    Ok(in_file) => in_file,
    Err(_) => {
      eprintln!("Cannot read file!");
      process::exit(1);
    },
  };
  let mut in_file = std::io::BufReader::new(in_file);
  let mut in_file_buf = String::new();
  let _ = in_file.read_to_string(&mut in_file_buf);
  let mut lexer = Token::lexer(&in_file_buf);
  let lookup = LineColLookup::new(&in_file_buf);

  let out_file =
    match std::fs::File::options().create(true).truncate(true).write(true).open(&*out_file_path) {
      Ok(out_file) => out_file,
      Err(_) => {
        eprintln!("Cannot write out file!");
        process::exit(1);
      },
    };
  let mut out_file = std::io::BufWriter::new(out_file);

  let mut errored = false;
  macro_rules! err_at_pos {
    ($($tt:tt)*) => {{
      let (line, col) = lookup.get(lexer.span().start);
      eprint!("{in_file_path}:{line}:{col}");
      eprintln!($($tt)*);
      errored = true;
      Ok(())
    }};
  }

  while let Some(token) = lexer.next() {
    let write_result = match token {
      Token::Unknown => err_at_pos!("Unknown character `{ch}`!", ch = lexer.slice()),
      Token::EXT => out_file.write_u16::<BigEndian>(0x0000),
    };
    match write_result {
      Ok(()) => (),
      Err(_) => {
        eprintln!("Failed to write to out file!");
        errored = true;
      },
    }
  }

  if errored {
    process::exit(1);
  }
}
