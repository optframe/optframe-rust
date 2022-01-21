//mod base_concepts;

#![allow(non_snake_case)]

pub use super::base_concepts::{XESolution, XEvaluation, XSolution};

pub trait Constructive<XS: XSolution> {
    fn generateSolution(&self) -> XS;
}
