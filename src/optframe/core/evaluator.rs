
//mod base_concepts;

#![allow(non_snake_case)]

pub use super::base_concepts::{XSolution, XEvaluation, XESolution};

pub trait Evaluator<XS: XSolution, XEv: XEvaluation>{//, XES: XESolution<XS, XEv>> {
    fn evaluate(&self, s: &XS) -> XEv;
}
