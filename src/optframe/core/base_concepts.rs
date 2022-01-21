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

// keep XEvaluation clean of template arguments (only leave in Evaluation implementation)
// adopting 'objType' (in Camel Case) as in OptFrame C++
pub trait XEvaluation {
    // type (TODO: replace 'Real' with some generic numerics)
    type ObjType: Real;
    // methods
    fn evaluation(&self) -> Self::ObjType;
    fn set_obj_val(&mut self, obj_val: Self::ObjType) -> ();
    // variable 'outdated' (now becomes method)
    fn is_outdated(&self) -> bool;
    fn set_outdated(&mut self, outdated: bool) -> ();
}

// =====================================================
// this is essentially a pair of (XSolution, XEvaluation)
//
pub trait XESolution {
    // types
    type FirstType: XSolution;
    type SecondType: XEvaluation;
    // methods
    fn first(&self) -> &Self::FirstType;
    fn second(&self) -> &Self::SecondType;
    fn first_mut(&mut self) -> &mut Self::FirstType;
    fn second_mut(&mut self) -> &mut Self::SecondType;
}

// ========================================
// NOTE: why not adopting "tuple struct" here?
// EXAMPLE: pub struct ESolutionTEST (Vec<i32>, Evaluation);
//
// The issue here is that:
// (1) community doesn't seem to like anonymous fields named like .0 and .1
// (2) it is not easy to extract the types of these fields (at least not to a starter like me...)
// (3) even if it works, named fields are certainly much easier to read and to manage.
//
// CONCLUSION: we will adopt a similiar syntax to C++,
// with 'first' (but method) and 'first_type' (but Camel Case)
// ========================================
