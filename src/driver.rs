use crate::{parser::*, token::*};
use std::{
  env,
  io::{self, BufRead, Write},
};

//===================================================**/
const USAGE: &'static str = "
Usage: iron_kaleidoscope [(-l | -p | -i)]

Options:
    -l  Run only lexer and show its output.
    -p  Run only parser and show its output.
    -i  Run only IR builder and show its output.
";
//===================================================**/
#[derive(Debug)]
pub struct Args {
  pub flag_l: bool,
  pub flag_p: bool,
  pub flag_i: bool,
}

pub fn main_loop() {
  println!("{}", USAGE);
  let ard = env::args().skip(1).collect::<Vec<String>>();
  print!(">");
  io::stdout().flush();
  for x in io::stdin().lock().lines() {
    let src = x.unwrap();
    if ard[0] == "-l".to_string() {
      for x in Lexer::new(src.clone().as_str()).get_token().iter() {
        println!("{:?}", x);
      }
    } else {
      println!("{:?}", Parser::new(src.clone().as_str()).file());
    }
    // println!("{}", x.unwrap());
    print!(">");
    io::stdout().flush();
  }
}
