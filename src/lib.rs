#![doc = include_str!("../README.md")]

mod optfcore;
mod optframe;

pub use optfcore::fconstructive::FConstructive;
pub use optfcore::fevaluator::FEvaluator;

pub use crate::optframe::core;
pub use crate::optframe::heuristics;

#[cfg(test)]
mod tsp_test;
