extern crate piston_meta;

extern crate piston_meta as cfg;
extern crate range;
extern crate rand;

use cfg::*;
use term::*;

mod term;
mod var;
mod cst;
mod qubit;

fn mk_parser() -> Result<Syntax, String> {
    let rules = r#"  
        1 0-tuple = "*"
        2 constant = {"H":"H" "U":"U" "meas":"meas" "new":"new"}
        3 variable = {"a":"a" "b":"b" "c":"c" "x":"x" "y":"y" "z":"z"}
        4 application = ["[" .w? term:"function" .w! term:"argument" .w? "]"]
        5 lambda = 
          ["L" .w? {0-tuple:"null" variable:"binds"} .w? "." .w? term:"body"]
        6 pair = ["<" .w? term:"pair0" .w? "," .w? term:"pair1" .w? ">"]
        7 assignment = 
          ["let" .w! {variable:"var" pair:"pair"} .w? "=" .w? term .w! "in" .w! term]
        8 rec-fn = 
          ["let rec" .w! variable:"funcvar" .w! variable:"argvar" .w? 
          "=" .w? term .w! "in" .w! term]
        9 innerterm = {
          assignment:"assign"
          rec-fn:"rec"
          constant:"const"
          variable:"var"
          application:"app"
          lambda:"lam"
          pair:"pair"
          0-tuple:"null"

        }
       10 term = { ["(" .w? innerterm .w? ")"] innerterm}        
       11 document = term:"term"
	"#;
    match syntax_errstr(rules) {
        Err(err) => return Err(
            format!("could not create parser:\n{}", err)
        ),
        Ok(parser) => Ok(parser),
    }
}
    
fn main() {
    use std::io;
    use std::io::prelude::*;
    let rules = match mk_parser() {
        Err(err) => {
	    println!("{}", err);
	    return;
	}
        Ok(rules) => rules
    };
  let stdin = io::stdin();
  for line in stdin.lock().lines() {
      let input = line.unwrap();
      let mut data = vec![];
      match parse_errstr(&rules, &input[..], &mut data) {
        Err(err) => {
            println!("{}", err);
            return;
        }
        Ok(_) => (),
      };
      let mut nodelist = (&data).into_iter();
      let term = make_term(&mut nodelist);
      println!("Term: {}",term);
      let reduced = betared(&term);
      println!("Reduced term: {}", reduced);
  }
}

