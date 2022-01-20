#![allow(non_snake_case)]

use num_traits::real::Real;
use std::fmt;

pub use super::base_concepts::{XSolution, XEvaluation, XESolution};

pub struct Evaluation<R: Real = f64> {
    pub objVal: R,
    pub outdated : bool,
}

impl<R: Real> XEvaluation<R> for Evaluation<R> {
    fn evaluation(&self) -> R
    {
        self.objVal
    }
    fn setObjFunction(&mut self, objVal: R) -> ()
    {
        self.objVal = objVal
    }
    // ======== variable 'outdated' ======== 
    fn isOutdated(&self) -> bool
    {
        return self.outdated;
    }
    //
    fn setOutdated(&mut self, outdated: bool) -> ()
    {
        self.outdated = outdated;
    }
}

impl fmt::Display for Evaluation<f64> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Evaluation value: {:?}", self.evaluation())
    }
}