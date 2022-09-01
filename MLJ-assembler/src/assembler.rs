#[cfg(test)]
mod tests;

use byteorder::{
  WriteBytesExt,
  BE,
};
use line_col::LineColLookup;
use logos::Logos;

use crate::token::Token;

pub fn assemble(
  input_file_path: &str,
  input_file: &str,
  out_file: &mut impl WriteBytesExt,
) -> bool {
  let mut lexer = Token::lexer(input_file);
  let lookup = LineColLookup::new(input_file);

  let mut errored = false;
  macro_rules! err_at_pos {
    ($($tt:tt)*) => {{
      let (line, col) = lookup.get(lexer.span().start);
      eprint!("{input_file_path}:{line}:{col}: ");
      eprintln!($($tt)*);
      errored = true;
      continue;
    }};
  }

  macro_rules! get_reg {
    () => {
      match lexer.clone().next() {
        Some(Token::R0) => {
          lexer.next();
          Some(0)
        },
        Some(Token::R1) => {
          lexer.next();
          Some(1)
        },
        Some(Token::R2) => {
          lexer.next();
          Some(2)
        },
        Some(Token::R3) => {
          lexer.next();
          Some(3)
        },
        Some(Token::R4) => {
          lexer.next();
          Some(4)
        },
        Some(Token::R5) => {
          lexer.next();
          Some(5)
        },
        Some(Token::RC) => {
          lexer.next();
          Some(6)
        },
        Some(Token::RPC) => {
          lexer.next();
          Some(7)
        },
        Some(Token::RSP) => {
          lexer.next();
          Some(8)
        },
        _ => None,
      }
    };
  }

  macro_rules! get_num {
    ($max:literal) => {
      match lexer.clone().next() {
        Some(Token::Number) => {
          lexer.next();
          let slice = lexer.slice();
          let number = i64::from_str_radix(
            slice.strip_prefix("0x").unwrap_or(slice),
            if slice.starts_with("0x") { 16 } else { 10 },
          )
          .unwrap();
          Some(((number % $max) & $max) as u16)
        },
        _ => None,
      }
    };
  }

  while let Some(token) = lexer.next() {
    let write_result = match token {
      Token::Unknown => err_at_pos!("Unknown character `{ch}`!", ch = lexer.slice()),
      Token::EXT => out_file.write_u16::<BE>(0x0000),
      Token::STR => {
        let sr = match get_reg!() {
          Some(sr) => sr,
          None => err_at_pos!("Expected register!"),
        };
        let off = match get_num!(0x7F) {
          Some(off) => off,
          None => err_at_pos!("Expected a number!"),
        };
        out_file.write_u16::<BE>(0x0800 | (sr << 7) | off)
      },
      Token::LDR => {
        let sr = match get_reg!() {
          Some(sr) => sr,
          None => err_at_pos!("Expected register!"),
        };
        let off = match get_num!(0x7F) {
          Some(off) => off,
          None => err_at_pos!("Expected a number!"),
        };
        out_file.write_u16::<BE>(0x1000 | (sr << 7) | off)
      },
      Token::INC => match get_num!(0x3FF) {
        Some(off) => out_file.write_u16::<BE>(0x1800 | off),
        None => match get_reg!() {
          Some(reg) => out_file.write_u16::<BE>(0x1C00 | reg),
          None => err_at_pos!("Expected a memory offset or a register!"),
        },
      },
      Token::DEC => match get_num!(0x3FF) {
        Some(off) => out_file.write_u16::<BE>(0x2000 | off),
        None => match get_reg!() {
          Some(reg) => out_file.write_u16::<BE>(0x2400 | reg),
          None => err_at_pos!("Expected a memory offset or a register!"),
        },
      },
      Token::CMP => {
        let sr1 = match get_reg!() {
          Some(sr1) => sr1,
          None => err_at_pos!("Expected register!"),
        };
        let sr2 = match get_reg!() {
          Some(sr2) => sr2,
          None => err_at_pos!("Expected register!"),
        };
        let mode = match lexer.next() {
          Some(Token::ModeEq) => 0,
          Some(Token::ModeNe) => 1,
          Some(Token::ModeLt) => 2,
          Some(Token::ModeLe) => 3,
          Some(Token::ModeGt) => 4,
          Some(Token::ModeGe) => 5,
          _ => err_at_pos!("Expected `CMP` mode!"),
        };
        out_file.write_u16::<BE>(0x2800 | (mode << 8) | (sr1 << 4) | sr2)
      },
      Token::JMC => {
        let addr = match get_num!(0x7FF) {
          Some(addr) => addr,
          None => err_at_pos!("Expected address!"),
        };
        out_file.write_u16::<BE>(0x3000 | addr)
      },

      _ => err_at_pos!("Expected instruction!"),
    };
    match write_result {
      Ok(()) => (),
      Err(_) => {
        eprintln!("Failed to write to out file!");
        errored = true;
      },
    }
  }

  errored
}
