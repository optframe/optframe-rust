pub use super::optframe::core::base_concepts::{XESolution, XEvaluation, XSolution};
pub use super::optframe::core::mod_move::Move;
pub use super::optframe::core::ns::NS;

use std::marker::PhantomData;

pub struct FNS<XES: XESolution, F>
where
    F: Fn(&XES) -> Box<dyn Move<XES>>,
{
    pub f_random: F,
    pub phantom_xes: PhantomData<XES>,
}

impl<XES: XESolution, F> NS<XES> for FNS<XES, F>
where
    F: Fn(&XES) -> Box<dyn Move<XES>>,
{
    fn random_move(&self, se: &XES) -> Box<dyn Move<XES>> {
        (self.f_random)(se)
    }
}
