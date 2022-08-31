use logos::Logos;

#[allow(clippy::upper_case_acronyms)]
#[derive(Logos)]
pub enum Token {
  #[regex("[\n\r\t ]+", logos::skip)]
  #[error]
  Unknown,

  #[regex("[0-9]+")]
  Number,

  #[regex("R0|r0")]
  R0,
  #[regex("R1|r1")]
  R1,
  #[regex("R2|r2")]
  R2,
  #[regex("R3|r3")]
  R3,
  #[regex("R4|r4")]
  R4,
  #[regex("R5|r5")]
  R5,
  #[regex("RC|rc")]
  RC,
  #[regex("RPC|rpc")]
  RPC,
  #[regex("RSP|rsp")]
  RSP,

  #[regex("EXT|ext")]
  EXT,
  #[regex("STR|str")]
  STR,
  #[regex("LDR|ldr")]
  LDR,
}
