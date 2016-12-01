
/* |x> = A|0> + B|1>
 * where
 *  A = sin(theta*pi/2)
 *  B = e^(i*phi*pi)*cos(theta*pi/2)
 * |0> => (theta -> pi, phi -> 0)
 * |1> => (theta -> 0, phi -> 0) 
 */

use rand;
use std::fmt;

pub struct Qubit {
    pub theta: f64,
    pub phi: f64,
}


impl fmt::Display for Qubit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use std::f64::consts::PI;
        let A: f64 = (self.theta * PI * 0.5).sin();
        let B: f64 = (self.theta * PI * 0.5).cos();
        if A == 1.0 { return write!(f, "❘0⟩"); }
        else if B == 1.0 { return write!(f, "❘1⟩"); }
        else { return write!(f, "{}❘0⟩ + {}❘1⟩", A, B); }
    }
}

impl Qubit {
    pub fn new(b: bool) -> Qubit {
        return
            if b { Qubit { theta : 0.0, phi : 0.0 } }
        else { Qubit { theta : 1.0, phi : 0.0 } }
    }
    pub fn meas(&self) -> bool {
        use std::f64::consts::PI;
        use rand::Rng;
        let num: f64 = rand::thread_rng().gen_range(0.0,1.0);//task_rng().gen_range(0.0, 1.0);
        return ((self.theta * PI * 0.5).sin()).powi(2) > num;
    }
}
           

