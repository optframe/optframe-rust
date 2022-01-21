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

//pub struct FEvaluator<F, ObjType = f64>
//<XES: XESolution, F>
pub struct FEvaluator<XS: XSolution, XEv: XEvaluation, F>
//, XES: XESolution<XS, XEv>, F>
where
    F: Fn(&XS) -> XEv,
    //F: Fn(&dyn XSolution) -> dyn XEvaluation<ObjType = ObjType>,
{
    pub f_evaluate: F,
    //pub phantom_xes: PhantomData<XES>,
    pub phantom_xs: PhantomData<XS>,
    pub phantom_xev: PhantomData<XEv>,
    //phantomXES: PhantomData<XES>, // TODO: remove?
}

impl<XS: XSolution, XEv: XEvaluation, F> Evaluator<XS, XEv> for FEvaluator<XS, XEv, F>
//impl<XS: XSolution, XEv: XEvaluation, F> Evaluator<XS, XEv> for FEvaluator<XS, XEv, F>
where
    F: Fn(&XS) -> XEv,
{
    fn evaluate(&self, s: &XS) -> XEv {
        (self.f_evaluate)(s)
    }
}

/*
impl<XS: XSolution, XEv: XEvaluation, F> FEvaluator<XS, XEv, F>
//FEvaluator<XS, XEv, XES, F>
where
    F: Fn(&XS) -> XEv
{
}
*/
