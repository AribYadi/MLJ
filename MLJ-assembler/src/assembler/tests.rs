#![allow(non_snake_case)]

use std::io::Cursor;

use byteorder::ReadBytesExt;

use super::*;

fn test_assemble(input: &str, expect: &[u16]) {
  let mut out = vec![];
  assert!(!assemble("<test>", input, &mut out));

  let mut out = Cursor::new(out);
  let mut buf = vec![];
  while let Ok(word) = out.read_u16::<BE>() {
    buf.push(word);
  }
  assert_eq!(buf, expect);
}

#[test]
fn test_assemble_EXT() { test_assemble("ext", &[0x0000]) }

#[test]
fn test_assemble_STR() { test_assemble("str rpc 123", &[0x0BFB]) }

#[test]
fn test_assemble_LDR() { test_assemble("ldr rpc 0x7B", &[0x13FB]) }

#[test]
fn test_assemble_INC() {
  test_assemble("inc 0xFF", &[0x18FF]);
  test_assemble("inc rsp", &[0x1C08]);
}

#[test]
fn test_assemble_DEC() {
  test_assemble("dec 0xFF", &[0x20FF]);
  test_assemble("dec rsp", &[0x2408]);
}

#[test]
fn test_assemble_CMP() {
  test_assemble("cmp r0 r1 eq", &[0x2801]);
  test_assemble("cmp r0 r1 ne", &[0x2901]);
  test_assemble("cmp r0 r1 lt", &[0x2A01]);
  test_assemble("cmp r0 r1 le", &[0x2B01]);
  test_assemble("cmp r0 r1 gt", &[0x2C01]);
  test_assemble("cmp r0 r1 ge", &[0x2D01]);
}

#[test]
fn test_assemble_JMC() { test_assemble("jmc 0x0201", &[0x3201]) }
