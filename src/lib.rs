#![doc = include_str!("../README.md")]
#![feature(generators, generator_trait)]

mod optfcore;
mod optframe;

pub use optfcore::fconstructive::FConstructive;
pub use optfcore::fevaluator::FEvaluator;
pub use optfcore::fns::FNS;
//pub use optfcore::fxnsseq::FxNSIterator;
//pub use optfcore::fxnsseq::FxNSIterator;

pub use crate::optframe::core;
pub use crate::optframe::heuristics;

#[cfg(test)]
mod tsp_test;
