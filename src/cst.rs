use cfg::*;
use range::Range;

#[derive(Clone)]
pub enum Const {
    H,
    New,
    Meas,
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


pub fn make_const<'a,T>( nodelist: &mut T ) -> Const
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
