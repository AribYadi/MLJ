use logos::Logos;

#[allow(clippy::upper_case_acronyms)]
#[derive(Logos)]
pub enum Token {
  #[regex("[\n\r\t ]+", logos::skip)]
  #[error]
  Unknown,

  #[regex("EXT|ext")]
  EXT,
}
