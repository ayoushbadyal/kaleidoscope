mod ast;
mod driver;
mod parser;
mod token;

use parser::*;

fn main() {
  let mut n = Parser::new(include_str!("stdlib.kl"));
  let _ = n.file().iter().for_each(|x| println!("{}\n", x));
}
