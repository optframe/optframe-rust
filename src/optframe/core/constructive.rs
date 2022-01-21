//mod base_concepts;

pub use super::base_concepts::{XESolution, XEvaluation, XSolution};

pub trait Constructive<XS: XSolution> {
    fn generate_solution(&self) -> XS;
}
