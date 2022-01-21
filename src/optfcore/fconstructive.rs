//mod super::optframe::core::base_concepts;
//mod super::optframe::core::constructive;
//
pub use super::optframe::core::base_concepts::{XESolution, XEvaluation, XSolution};
pub use super::optframe::core::constructive::Constructive;

pub struct FConstructive<XS: XSolution, F>
where
    F: Fn() -> XS,
{
    //pub func : fn()->XS // pointer
    pub func: F,
}

impl<XS: XSolution, F> Constructive<XS> for FConstructive<XS, F>
where
    F: Fn() -> XS,
{
    fn generateSolution(&self) -> XS {
        return (self.func)();
    }
}
