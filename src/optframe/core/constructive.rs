
//mod base_concepts;

#![allow(non_snake_case)]

pub use super::base_concepts::{XSolution, XEvaluation, XESolution};

pub trait Constructive<XS: XSolution> {
    fn generateSolution(&self) -> XS;
}
