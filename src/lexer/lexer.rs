use crate::lexer::token::Token;
use logos::{Lexer, Logos};

pub fn generate(input: &str) -> Vec<Token> {
  Token::lexer(input).collect()
}

pub fn to_string(lex: &mut Lexer<Token>) -> Option<String> {
  let mut string: String = lex.slice().to_string();

  if string.starts_with("$") {
    string.remove(0);
  }

  if string.starts_with("\"") | string.starts_with("'") {
    string.remove(0);
  }

  if string.ends_with('"') | string.ends_with('\'') {
    string.remove(string.len() - 1);
  }

  Some(string)
}

pub fn to_char(lex: &mut Lexer<Token>) -> Option<char> {
  let char: char = lex.slice().to_string().chars().nth(1).unwrap();

  Some(char)
}

pub fn to_int(lex: &mut Lexer<Token>) -> Option<i64> {
  let slice = lex.slice();
  let i: i64 = slice.parse().ok()?;
  Some(i)
}
pub fn to_float(lex: &mut Lexer<Token>) -> Option<f64> {
  let slice = lex.slice();
  let i: f64 = slice.parse().ok()?;
  Some(i)
}
