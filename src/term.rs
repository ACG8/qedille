
use cfg::*;
use var::*;
use cst::*;
use range::Range;

#[derive(Clone)]
pub enum Term {
    Var(Var),
    Const(Const),
    App(Box<Term>,Box<Term>),
    Lam(Var,Box<Term>),
    Par(Box<Term>,Box<Term>),
//    Asg(Box<Term>,Box<Term>,Box<Term>),
//    Rec(Var,Var,Box<Term>,Box<Term>)
}

//"substitute t2 for x in t1"
fn subst(t2:&Term, x:&Var, t1:&Term) -> Term {
    return match *t1 {
        Term::Var( ref v ) => {
            match *v {
                Var::Null => t1.clone(),
                _ => t2.clone(),
            }
        },
        Term::Const( _ ) => t1.clone(),
        Term::App(ref a, ref b) => Term::App(Box::new(subst(t2,x,a)),Box::new(subst(t2,x,b))),
        Term::Lam(ref v, ref a) => Term::Lam(v.clone(),Box::new(subst(t2,x,a))),
        Term::Par(ref a, ref b) => Term::Par(Box::new(subst(t2,x,a)),Box::new(subst(t2,x,b))),
    }
}

pub fn betared(t: &Term) -> Term {
    return match *t {
        Term::Var( ref v ) => Term::Var(v.clone() ),
        Term::Const( ref c ) => Term::Const(c.clone() ),
        Term::App(ref a, ref b) => {
            match **a {
                Term::Lam(ref x,ref t) => subst(b,&x,&*t),
                _ => Term::App(Box::new(betared(a)),Box::new(betared(b))),
            }
        },
        Term::Par(ref a, ref b) => Term::Par(Box::new(betared(a)),Box::new(betared(b))),
        Term::Lam(ref v, ref a) => Term::Lam(v.clone(),Box::new(betared(a))),
    }
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
            Term::Par( ref a, ref b ) => write!(f, "<{},{}>", a, b), //todo: syntactic sugar for tuples greater than 2
            Term::Lam( ref v, ref t ) => write!(f, "λ{}.{}", v, t),
        //    _ => write!(f, "{}", "_"), //=BUG= fill in the rest of them
        }
    }
}

pub fn make_term<'a,T>( nodelist: &mut T ) -> Term
    where T: Iterator<Item=&'a Range<MetaData>> {
    use cfg::MetaData::*;
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
                        "pair" => Term::Par(Box::new(make_term(nodelist)),Box::new(make_term(nodelist))),
                        _ => make_term(nodelist),
                    }
                    
                }
                _ => make_term(nodelist),
            }
        }
        None => return Term::Var(Var::Null),
    }        
}
