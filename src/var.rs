
use cfg::*;
use range::Range;

pub enum Var {
    Null,
    Var(String),
}

use std::fmt;
impl fmt::Display for Var {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Var::Null => write!(f, "{}", "*"),
            Var::Var( ref c ) => write!(f, "{}", c),
        }
    }
}


pub fn make_var<'a,T>( nodelist: &mut T ) -> Var
    where T: Iterator<Item=&'a Range<MetaData>> {
    use cfg::MetaData::*;
    use std::sync::Arc;
    use std::string;
    return match nodelist.next() {
        Some(metadata) => {
            match metadata.data {
                Bool(ref arcstring,_) => {
                    match &arcstring[..] {
                        "*" => Var::Null,
                        x => Var::Var(string::String::from(x)),
                    }
                }
                _ => make_var(nodelist)
            }
        }
        None => unreachable!(),
    }
}
