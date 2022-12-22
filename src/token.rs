use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
  Eof,
  Def,
  Extern,
  Identifier(String),
  Number(f64),
  Char(char),
  If,
  Else,
  Then,
  //> if-token
  For,
  In,
  Eq,
  //> ch-4 for-token
  Binary,
  Unary,
  //> ch-5
  Var,
  Oppar,
  Clpar,
}

#[macro_use]
#[macro_export]
macro_rules! T {
    [+] => {        $crate::token::Token::Char('+')};
    [-] => {        $crate::token::Token::Char('-')};
    [*] => {        $crate::token::Token::Char('*')};
    [/] => {        $crate::token::Token::Char('/')};
    [=] => {        $crate::token::Token::Eq};
    [.] => {        $crate::token::Token::Dot};
    [,] => {        $crate::token::Token::Char(',')};
    [!] => {        $crate::token::Token::Char('!')};
    [|] => {        $crate::token::Token::Char('|')};
    [:] => {        $crate::token::Token::Char(':')};
    [;] => {        $crate::token::Token::Char(';')};
    [<] => {        $crate::token::Token::Char('<')};
    [>] => {        $crate::token::Token::Char('>')};
    ['['] => {      $crate::token::Token::Char('[')};
    [']'] => {      $crate::token::Token::Char(']')};
    ['{'] => {      $crate::token::Token::Oppar};
    ['}'] => {      $crate::token::Token::Clpar};
    ['('] => {      $crate::token::Token::Char('(')};
    [')'] => {      $crate::token::Token::Char(')')};
    [then] => {     $crate::token::Token::Then};
    [def] => {      $crate::token::Token::Def};
    [extern] => {   $crate::token::Token::Extern};
    [if] => {       $crate::token::Token::If};
    [else] => {     $crate::token::Token::Else};
    [for] => {      $crate::token::Token::For};
    [in] => {       $crate::token::Token::In};
    [EOF] => {      $crate::token::Token::Eof};
    [var] => {      $crate::token::Token::Var};

}

#[derive(Debug)]
pub struct Lexer {
  input: Vec<char>,
}

impl Lexer {
  pub fn new(inp: &str) -> Self {
    Self {
      input: inp.chars().collect::<Vec<char>>(),
    }
  }

  pub fn get_token(&self) -> Vec<Token> {
    use Token::*;
    let mut i = 0;
    let mut res = Vec::new();
    loop {
      if i == self.input.len() {
        res.push(T![EOF]);
        break;
      }

      match self.input[i] {
        'a'..='z' | 'A'..='Z' => {
          let mut aa = String::new();
          loop {
            if !(i < self.input.len() && self.input[i].is_alphabetic()) {
              break;
            }
            aa.push(self.input[i]);
            i += 1;
          }

          match aa.as_str() {
            "def" => res.push(T![def]),
            "then" => res.push(T![then]),
            "extern" => res.push(Extern),
            "if" => res.push(If),
            "then" => res.push(Then),
            "else" => res.push(Else),
            "for" => res.push(For),
            "in" => res.push(In),
            "binary" => res.push(Binary),
            "unary" => res.push(Unary),
            "var" => res.push(Var),
            a @ _ => res.push(Identifier(a.into())),
          }
        }
        '0'..='9' => {
          let mut aa = String::new();
          loop {
            if !(i < self.input.len() && (self.input[i].is_digit(10) || self.input[i] == '.')) {
              break;
            }
            aa.push(self.input[i]);
            i += 1;
          }
          res.push(Number(aa.parse::<f64>().unwrap()));
        }
        ' ' | '\n' => {
          i += 1;
        }
        a @ _ => {
          if a == '#' {
            // ign comments
            loop {
              let ch = self.input[i];
              if ch == '\r' || ch == '\n' {
                i += 1;
                break;
              }
              i += 1;
            }
          } else if a == '{' {
            res.push(Oppar);
          } else if a == '}' {
            res.push(Clpar);
          } else {
            res.push(Char(a));
          }
          i += 1;
        }
      }
    }
    res
  }
}
