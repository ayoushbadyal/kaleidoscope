use crate::ast;
use crate::ast::*;
use crate::token::Token::*;
use crate::token::*;
use crate::T as t;
use std::collections;
use std::{iter::Peekable, vec::IntoIter};

#[derive(Debug, Clone)]
pub struct TokenIter {
  pub tok: IntoIter<Token>,
}

impl Iterator for TokenIter {
  type Item = Token;
  fn next(&mut self) -> Option<Self::Item> {
    self.tok.next()
  }
}

pub struct Parser<'a> {
  pub src: &'a str,
  pub t: Peekable<TokenIter>,
}

trait Operator {
  fn prefix_binding_power(&self) -> ((), u8);
  fn infix_binding_power(&self) -> Option<(u8, u8)>;
  fn postfix_binding_power(&self) -> Option<(u8, ())>;
}

impl Operator for Token {
  fn prefix_binding_power(&self) -> ((), u8) {
    match self {
      t![+] | t![-] => ((), 51),
      _ => unreachable!("Not a prefix operator: {:?}", self),
    }
  }

  fn infix_binding_power(&self) -> Option<(u8, u8)> {
    let result = match self {
      t![+] | t![-] => (9, 10),
      t![*] | t![/] => (11, 12),
      t![<] | t![>] => (7, 8),
      t![|] => (6, 7),
      Char('^') => (22, 21), // <- This binds stronger to the lefTokChar
      _ => return None,
    };
    Some(result)
  }

  fn postfix_binding_power(&self) -> Option<(u8, ())> {
    use Token::*;
    let result = match self {
      Char('!') => (101, ()),
      _ => return None,
    };
    Some(result)
  }
}

impl<'a> Parser<'a> {
  pub fn new(src: &'a str) -> Self {
    Self {
      src,
      t: TokenIter {
        tok: Lexer::new(src).get_token().into_iter(),
      }
      .peekable(),
    }
  }
  /// # Examples
  ///
  /// ```
  /// let mut n = Parser::new(
  ///     "
  /// def print(a)
  ///     lame(1,3,3,4)
  ///      if a>b then
  ///       y=x+1
  ///     else
  ///       x=m+lame(12);
  /// def main(fuck)
  ///     print(1,2,aloha);
  /// ",
  /// );
  /// let src = n.file();
  /// dbg!(&src);
  /// println!("{:?}", src);
  /// ```

  #[inline]
  pub fn file(&mut self) -> Vec<ExprAst> {
    let mut b = Vec::new();
    while !self.at(t![EOF]) {
      let ec = self._parse_exp(0);
      self.consume(t![;]);
      b.push(ec);
    }
    b
  }

  pub fn _block(&mut self) -> Vec<ExprAst> {
    let mut expr = Vec::new();
    loop {
      // as Char(')') also also a constraint
      if self.at(t![')']) | self.at(t![;]) {
        break;
      }
      let va = self._parse_exp(0);
      if self.at(t![:]) {
        self.consume(t![:])
      }
      expr.push(va);
    }
    expr
  }

  pub fn _scope(&mut self) -> ExprAst {
    self.consume(t![')']);
    let va = self._parse_exp(0);
    self.consume(t![')']); // since every entity ends by `;`
    va
  }

  pub fn _args(&mut self) -> Vec<ExprAst> {
    self.consume(t!['(']);
    let mut args = Vec::new();
    while !self.at(t![')']) {
      let ar = self._parse_exp(0);
      args.push(ar);
      if self.peek() == t![,] {
        self.next();
      }
    }
    self.consume(t![')']);
    args
  }

  #[inline]
  pub fn _stmt(&mut self) -> ExprAst {
    let res = match self.peek() {
      /*
      # Iterative fib.
      def fibi(x)
        var a = 1, b = 1, c in
        (for i = 3, i < x in
           c = a + b :
           a = b :
           b = c) :
        b;
          */
      a @ t![var] => {
        let mut variables = Vec::new();
        while !self.at(t![in]) {
          variables.push(self._parse_exp(0));
          if self.at(t![,]) {
            self.next();
          }
        }
        self.consume(t![in]);
        let body = self._block();
        unimplemented!("finding what var does ?")
      }
      a @ t![for] => {
        self.consume(a);
        // for a=12,
        let init = self._parse_exp(0);

        self.consume(t![,]);
        let cond = self._parse_exp(0);
        let mut inc_val = 0f64;
        if self.at(t![,]) {
          self.consume(t![,]);
          match self._parse_exp(0) {
            ExprAst::NumberExpr(a) => inc_val += a.val,
            _ => {}
          }
          // since increment is optional
        } else {
          inc_val = 1.0;
        }
        self.consume(t![in]);
        let block = self._block();
        ExprAst::ForLoopBlock(ForIn {
          init: Box::new(init),
          cond: Box::new(cond),
          inc_val,
          body: block,
        })
      }
      a @ t![if] => {
        self.consume(a);
        let condi = self._parse_exp(0); //cong
        self.consume(t![then]);
        let mut body = self._block();
        self.consume(t![else]);
        let mut el_body = self._block();
        ExprAst::Stmt(Stmt::Ifelse(ConditionalBl {
          condi: Box::new(condi),
          body,
          else_co: el_body,
        }))
      }
      t![extern] => {
        self.consume(t![extern]);
        let extern_func = self._parse_exp(0);
        ExprAst::ExternNode(ExternNodeAst {
          body: Box::new(extern_func),
        })
      }
      t![def] => {
        self.consume(t![def]);
        let name = self.next().unwrap().into();
        let args = self._args();
        let block = self._block();
        ExprAst::Func(FunctionAst {
          proto: PrototypeAst { name, args },
          body: Box::new(block),
        })
      }
      kind => panic!("unexpected {:?}", (kind, &self.t)),
    };
    res
  }

  #[inline]
  pub fn _parse_exp(&mut self, b_po: usize) -> ExprAst {
    let mut lhs = match self.peek() {
      a @ t!['('] => self._scope(),
      Number(a) => {
        self.next();
        ExprAst::NumberExpr(NumberExprAst { val: a })
      }
      Identifier(a) => {
        self.next();
        if self.peek() != t!['('] && self.peek() != Char('=') {
          ExprAst::VariableExpr(VariableExprAst { name: a })
        } else if self.peek() == Char('=') {
          self.consume(Char('='));
          let val = self._parse_exp(0);
          ExprAst::Stmt(Stmt::Assignment(Identifier(a), Box::new(val)))
        } else {
          ExprAst::CallExpr(CallExprAst {
            callee: a,
            args: self._args(),
          })
        }
      }
      (t![def] | t![extern] | t![if] | t![for]) => self._stmt(),
      kind => panic!("unexpected {:?}", (kind, &self.t)),
    };
    loop {
      let a = self.peek();
      if let Some((left_binding_power, right_binding_power)) = a.infix_binding_power() {
        if (left_binding_power as usize) < b_po {
          break;
        }
        self.consume(a.clone());
        let rhs = self._parse_exp(right_binding_power.into());
        lhs = ast::ExprAst::BinaryExpr(BinaryExprAst {
          op: match a {
            t![+] => '+',
            t![-] => '-',
            t![*] => '*',
            t![/] => '/',
            t![|] => '|',
            t![>] => '>',
            t![<] => '<',
            kind => panic!("this is unexpected {:?}", kind),
          },
          lhs: Box::new(lhs),
          rhs: Box::new(rhs.clone()),
        });
        continue;
      }
      break;
    }
    lhs
  }
  fn at(&mut self, tok: Token) -> bool {
    self.t.peek().unwrap_or(&t![EOF]).to_owned() == tok
  }
  fn next(&mut self) -> Option<Token> {
    self.t.next()
  }
  fn peek(&mut self) -> Token {
    self.t.to_owned().peek().cloned().unwrap_or(t![EOF])
  }
  fn consume(&mut self, tok: Token) {
    self
      .next()
      .expect(format!("expected tok {:?}", tok).as_str());
  }
}
