use num_traits::real::Real;
use std::fmt;

pub use super::base_concepts::{XESolution, XEvaluation, XSolution};

pub struct Evaluation<R: Real = f64> {
    pub obj_val: R,
    pub outdated: bool,
}

impl<R: Real> XEvaluation<R> for Evaluation<R> {
    fn evaluation(&self) -> R {
        self.obj_val
    }
    fn set_obj_function(&mut self, obj_val: R) {
        self.obj_val = obj_val
    }
    // ======== variable 'outdated' ========
    fn is_outdated(&self) -> bool {
        self.outdated
    }
    //
    fn set_outdated(&mut self, outdated: bool) {
        self.outdated = outdated;
    }
}

impl fmt::Display for Evaluation<f64> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Evaluation value: {:?}", self.evaluation())
    }
}
