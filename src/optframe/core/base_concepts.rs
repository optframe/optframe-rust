// mod ?

// TODO: require some more general total_order trait, or numeric trait
// TODO: experiment impls with OR logic
// https://docs.rs/impls/1.0.3/impls/

use num_traits::real::Real;

//#[derive(Copy, Clone)]
// TODO: REQUIRES Copy!
pub trait XRepresentation {}

// TODO: REQUIRES XRepresentation
pub trait XSolution {}

pub trait XEvaluation<R: Real = f64> {
    fn evaluation(&self) -> R;
    fn set_obj_function(&mut self, obj_val: R);
    // variable 'outdated'
    fn is_outdated(&self) -> bool;
    fn set_outdated(&mut self, outdated: bool);
}

pub trait XESolution<XS: XSolution, XEv: XEvaluation> {
    fn first(&self) -> &XS;
    fn second(&self) -> &XEv;
    fn first_mut(&mut self) -> &mut XS;
    fn second_mut(&mut self) -> &mut XEv;
}
