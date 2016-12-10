
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
    Asg(Box<Term>,Box<Term>,Box<Term>),
    Rec(Var,Var,Box<Term>,Box<Term>),
}

//"substitute t2 for x in t1"
fn subst(t2:&Term, x:&Var, t1:&Term) -> Term {
    return match *t1 {
        Term::Var( ref v ) => {
            match *v {
                Var::Var(ref a) => {
                    match *x {
                        Var::Var(ref b) => if a==b {t2.clone()} else {t1.clone()},
                        _ => t1.clone(),
                    }
                },
                _ => t1.clone(),
            }
        },
        Term::Const( _ ) => t1.clone(),
        Term::App(ref a, ref b) => Term::App(Box::new(subst(t2,x,a)),Box::new(subst(t2,x,b))),
        Term::Lam(ref v, ref a) => Term::Lam(v.clone(),Box::new(subst(t2,x,a))),
        Term::Par(ref a, ref b) => Term::Par(Box::new(subst(t2,x,a)),Box::new(subst(t2,x,b))),
        Term::Asg(ref a, ref b, ref c) => Term::Asg(a.clone(),Box::new(subst(t2,x,b)),Box::new(subst(t2,x,c))),
        Term::Rec(ref f, ref x, ref M, ref N) => Term::Rec(f.clone(),x.clone(),Box::new(subst(t2,x,M)),Box::new(subst(t2,x,N))),
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
        Term::Asg(ref a, ref b, ref c) => {
            match **a {
                Term::Var( ref v ) => {
                    subst(b,v,c)
                },
                Term::Par( ref x, ref y ) =>
                    match (*x.clone(),*y.clone()) {
                        (Term::Var( ref x0 ), Term::Var( ref y0 )) =>
                            match **b {
                                Term::Par(ref t1, ref t2) => {
                                    subst(t1,x0,&subst(t2,y0,c))
                                },   
                                _ => t.clone(),
                            },
                        _ => t.clone(),
                    },
                _ => t.clone(),
            }
        },
        Term::Rec(ref f, ref x, ref M, ref N) => {
            subst(&Term::Lam(x.clone(),Box::new(Term::Rec(f.clone(),x.clone(),M.clone(),M.clone()))),f,N)
        },
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
            Term::Asg( ref v, ref t2, ref t1 ) => write!(f, "let {} = {} in {}", v, t2, t1),
            Term::Rec( ref F, ref x, ref M, ref N) => write!(f, "let rec {} {} = {} in {}", F, x, M, N),
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
                        "assign" => Term::Asg(Box::new(make_term(nodelist)),Box::new(make_term(nodelist)),Box::new(make_term(nodelist))),
                        "rec" => Term::Rec(make_var(nodelist),make_var(nodelist),Box::new(make_term(nodelist)),Box::new(make_term(nodelist))),
                        _ => make_term(nodelist),
                    }
                    
                }
                _ => make_term(nodelist),
            }
        }
        None => return Term::Var(Var::Null),
    }        
}
