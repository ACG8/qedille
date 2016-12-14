
/* |x> = A|0> + B|1>
 * where
 *  A = sin(theta*pi/2)
 *  B = e^(i*phi*pi)*cos(theta*pi/2)
 * |0> => (theta -> pi, phi -> 0)
 * |1> => (theta -> 0, phi -> 0) 
 */

use rand;
use std::fmt;

#[derive(Clone)]
pub struct Qubit {
    pub theta: f64,
    pub phi: f64,
}

impl fmt::Display for Qubit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use std::f64::consts::PI;
        let a: f64 = (self.theta * PI * 0.5).sin();
        let b: f64 = (self.theta * PI * 0.5).cos();
        if a == 1.0 { return write!(f, "❘0⟩"); }
        else if b == 1.0 { return write!(f, "❘1⟩"); }
        else { return write!(f, "{}❘0⟩ + {}❘1⟩", a, b); }
    }
}

impl Qubit {
    pub fn new(b: &Bit) -> Qubit {
        return
            if b.value { Qubit { theta : 0.0, phi : 0.0 } }
        else { Qubit { theta : 1.0, phi : 0.0 } }
    }
    pub fn meas(&self) -> Bit {
        use std::f64::consts::PI;
        use rand::Rng;
        let num: f64 = rand::thread_rng().gen_range(0.0,1.0);//task_rng().gen_range(0.0, 1.0);
        return if ((self.theta * PI * 0.5).sin()).powi(2) > num {Bit{value:true}} else {Bit{value:false}} ;
    }
}
          
#[derive(Clone)] 
pub struct Bit {
    pub value: bool,
}

impl fmt::Display for Bit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", (if self.value {"1"} else {"0"}))
    }
}