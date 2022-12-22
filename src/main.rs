#![allow(unused)]

mod ast;
mod driver;
mod parser;
mod token;

use parser::*;
use token::*;

fn main() {
  let mut n = Parser::new(
    "
def starline() (for i = 0, i < 80 in putstar()) : newline();

def start() starline() :
    puthash() : (for i = 0, i < 23 in putspace()) :
    putspace() : putchard(73) : putchard(114) :
    putchard(111) : putchard(110) : putspace() :
    putchard(75) : putchard(97) : putchard(108) : putchard(101) : putchard(105) :
    putchard(100) : putchard(111) : putchard(115) : putchard(99) :
    putchard(111) : putchard(112) : putchard(101) : putspace() :
    putchard(115) : putchard(116) : putchard(100) : putchard(108) :
    putchard(105) : putchard(98) : putspace() :
    putchard(108) : putchard(111) : putchard(97) : putchard(100) : putchard(101) : putchard(100) :
    (for i = 0, i < 23 in putspace()) :
    puthash() : newline() :
    starline();

start();    ",
  );
  let _ = n.file().iter().for_each(|x| println!("{}", x));
}
