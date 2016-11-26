extern crate piston_meta;

extern crate piston_meta as cfg;
extern crate range;

use cfg::*;
use range::Range;
use term::*;

mod term;
mod var;
mod cst;


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
          ["let" .w! {variable:"var" pair:"pair"} .w? "=" .w? term .w! "in" term]
        8 rec-fn = 
          ["let rec" .w! variable:"funcvar" .w! variable:"argvar" .w? 
          "=" .w? term .w! "in" term]
        9 innerterm = {
          constant:"const"
          variable:"var"
          application:"app"
          lambda:"lam"
          pair:"pair"
          0-tuple:"null"
          assignment:"assign"
          rec-fn:"rec"
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

fn regurgitate( ast: & [ Range<MetaData> ] ) {
    use cfg::MetaData::*;
    let mut nodelist = ast.into_iter();
    let testerm = make_term(&mut nodelist);
    println!("test: {}",testerm);
    loop {
        match nodelist.next() {
            Some(metadata) => {
                match metadata.data {
                    StartNode(ref arcstring) => {println!("Start Node: {}",arcstring)}
                    EndNode(ref arcstring) => {println!("End Node: {}",arcstring)}
                    Bool(ref arcstring,_) => {println!("Bool: {}",arcstring)}
                    F64 (ref arcstring,_) => {}
                    String (ref arcstring1,ref arcstring2) => {println!("String: {}={}",arcstring1,arcstring2)}
                }
            }
            None => break
        }
    }
}
    
fn main() {
    let text = r#"[Lx.[x [x x]] meas]"#;

    let rules = match mk_parser() {
        Err(err) => {
	    println!("{}", err);
	    return;
	}
        Ok(rules) => rules
    };
    let mut data = vec![];
    match parse_errstr(&rules, text, &mut data) {
	Err(err) => {
	    println!("{}", err);
	    return;
	}
	Ok(ast) => regurgitate( &data )
    };
    
    //json::print(&data);
}

