use std::ops::Range;

#[derive(Debug, Clone)]
pub struct TokenT {
  pub kind: Token,
  pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Span {
  pub start: usize,
  pub end: usize,
}

impl From<Range<usize>> for Span {
  fn from(a: Range<usize>) -> Self {
    Self {
      start: a.start,
      end: a.end,
    }
  }
}

impl From<Span> for Range<usize> {
  fn from(a: Span) -> Self {
    Range {
      start: a.start,
      end: a.end,
    }
  }
}

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
macro_rules! token {
  ($a:expr,$b:expr) => {
    TokenT { kind: $a, span: $b }
  };
}

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

  pub fn get_token(&self) -> Vec<TokenT> {
    use Token::*;
    let mut i = 0;
    let mut res = Vec::new();
    loop {
      if i == self.input.len() {
        res.push(token!(T![EOF], Span::from(i..self.input.len())));
        break;
      }

      match self.input[i] {
        'a'..='z' | 'A'..='Z' => {
          let mut aa = String::new();
          let old = i;
          loop {
            if !(i < self.input.len() && self.input[i].is_alphabetic()) {
              break;
            }
            aa.push(self.input[i]);
            i += 1;
          }

          res.push(token!(
            match aa.as_str() {
              "def" => T![def],
              "extern" => Extern,
              "if" => If,
              "then" => Then,
              "else" => Else,
              "for" => For,
              "in" => In,
              "binary" => Binary,
              "unary" => Unary,
              "var" => Var,
              a @ _ => Identifier(a.into()),
            },
            (old..i).into()
          ));
        }
        '0'..='9' => {
          let mut aa = String::new();
          let ol = i;
          loop {
            if !(i < self.input.len() && (self.input[i].is_digit(10) || self.input[i] == '.')) {
              break;
            }
            aa.push(self.input[i]);
            i += 1;
          }
          res.push(token!(Number(aa.parse::<f64>().unwrap()), (ol..i).into()));
        }
        ' ' | '\n' | '\r' => {
          i += 1;
        }
        a @ _ => {
          let old = i;
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
            res.push(token!(Oppar, (old..i).into()));
          } else if a == '}' {
            res.push(token!(Clpar, (old..i).into()));
          } else {
            res.push(token!(Char(a), (old..i).into()));
          }
          i += 1;
        }
      }
    }
    res
  }
}
