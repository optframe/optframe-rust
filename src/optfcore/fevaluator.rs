pub use super::optframe::core::base_concepts::{XESolution, XEvaluation, XSolution};
pub use super::optframe::core::evaluator::Evaluator;

use std::marker::PhantomData;

/*
enum MinOrMax
{
   MINIMIZE,
   MAXIMIZE
}
*/

pub struct FEvaluator<XS: XSolution, XEv: XEvaluation, F>
//, XES: XESolution<XS, XEv>, F>
where
    F: Fn(&XS) -> XEv,
{
    pub fEvaluate: F,
    pub phantomXS: PhantomData<XS>,
    pub phantomXEv: PhantomData<XEv>,
    //phantomXES: PhantomData<XES>, // TODO: remove?
}

impl<XS: XSolution, XEv: XEvaluation, F> Evaluator<XS, XEv> for FEvaluator<XS, XEv, F>
where
    F: Fn(&XS) -> XEv,
{
    fn evaluate(&self, s: &XS) -> XEv {
        return (self.fEvaluate)(s);
    }
}

impl<XS: XSolution, XEv: XEvaluation, F> FEvaluator<XS, XEv, F>
//FEvaluator<XS, XEv, XES, F>
where
    F: Fn(&XS) -> XEv
{
}
