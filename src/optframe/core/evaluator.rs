//mod base_concepts;

pub use super::base_concepts::{XESolution, XEvaluation, XSolution};

pub trait Evaluator<XS: XSolution, XEv: XEvaluation> {
    // TODO: XES is not required here (TODO: check GeneralEvaluator)
    fn evaluate(&self, s: &XS) -> XEv;
}
