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
