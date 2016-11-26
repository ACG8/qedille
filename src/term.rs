
use cfg::*;
use var::*;
use cst::*;
use range::Range;

pub enum Term {
    Var(Var),
    Const(Const),
    App(Box<Term>,Box<Term>),
    Lam(Var,Box<Term>),
    Par(Box<Term>,Box<Term>),
    Asg(Box<Term>,Box<Term>,Box<Term>),
    Rec(Var,Var,Box<Term>,Box<Term>)
}

use std::fmt;
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

pub fn make_term<'a,T>( nodelist: &mut T ) -> Term
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
