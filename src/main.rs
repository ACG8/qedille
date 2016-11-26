extern crate piston_meta;

extern crate piston_meta as cfg;
extern crate range;

use cfg::*;
use range::Range;

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

fn make_term<'a,T>( nodelist: &mut T ) -> Term
    where T: Iterator<Item=&'a Range<MetaData>> {
    use cfg::MetaData::*;
    use std::sync::Arc;
//    =BUG= currently assuming there is only one term.
    return match nodelist.next() {
        Some(metadata) => {
            match metadata.data {
                StartNode(ref arcstring) => {
                    match &arcstring[..] {
                        "var" => Term::Var(make_var(nodelist)),
                        "lam" => Term::Lam(make_var(nodelist),Box::new(make_term(nodelist))),
                        "app" => Term::App(Box::new(make_term(nodelist)),Box::new(make_term(nodelist))),
                        "const" => Term::Const(make_const(nodelist)),
                        _ => make_term(nodelist),
                    }
                    
                }
                _ => make_term(nodelist),
            }
        }
        None => return Term::Var(Var::Null),
    }        
}

fn make_var<'a,T>( nodelist: &mut T ) -> Var
    where T: Iterator<Item=&'a Range<MetaData>> {
    use cfg::MetaData::*;
    use std::sync::Arc;
    return match nodelist.next() {
        Some(metadata) => {
            match metadata.data {
                Bool(ref arcstring,_) => {
                    match &arcstring[..] {
                        "*" => Var::Null,
                        x => Var::Var(std::string::String::from(x)),
                    }
                }
                _ => make_var(nodelist)
            }
        }
        None => unreachable!(),
    }
}

fn make_const<'a,T>( nodelist: &mut T ) -> Const
    where T: Iterator<Item=&'a Range<MetaData>> {
    use cfg::MetaData::*;
    use std::sync::Arc;
//    =BUG= currently assuming there is only one term.
    return match nodelist.next() {
        Some(metadata) => {
            match metadata.data {
                Bool(ref arcstring,_) => {
                    match &arcstring[..] {
                        "H" => Const::H,
                        "new" => Const::New,
                        "meas" => Const::Meas,
                        _ => unreachable!(),
                    }
                }
                _ => unreachable!(),
            }
        }
        None => unreachable!(),
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

enum Const {
    H,
    New,
    Meas,
}

enum Var {
    Null,
    Var(String),
}

enum Term {
    Var(Var),
    Const(Const),
    App(Box<Term>,Box<Term>),
    Lam(Var,Box<Term>),
    Par(Box<Term>,Box<Term>),
    Asg(Box<Term>,Box<Term>,Box<Term>),
    Rec(Var,Var,Box<Term>,Box<Term>)
}

use std::fmt;
impl fmt::Display for Const {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Const::H => write!(f, "{}", "H"),
            Const::New => write!(f, "{}", "new"),
            Const::Meas => write!(f, "{}", "meas"),
        }
    }
}

impl fmt::Display for Var {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Var::Null => write!(f, "{}", "*"),
            Var::Var( ref c ) => write!(f, "{}", c),
        }
    }
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Term::Const( ref c ) => write!(f, "{}", c),
            Term::Var( ref v ) => write!(f, "{}", v),
            Term::App( ref a, ref b ) =>
                match **a {
                    Term::Lam(_,_) =>
                        match **b {
                            Term::App(_,_) => write!(f, "({}) ({})", a, b),
                            _ => write!(f, "({}) {}", a, b),
                        },
                    _ =>
                        match **b {
                            Term::App(_,_) => write!(f, "{} ({})", a, b),
                            _ => write!(f, "{} {}", a, b),
                        },
                },
            Term::Lam( ref v, ref t ) => write!(f, "λ{}.{}", v, t),
            _ => write!(f, "{}", "_"), //=BUG= fill in the rest of them
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

