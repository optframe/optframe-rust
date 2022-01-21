// module 'ns'

pub use super::base_concepts::{XESolution, XEvaluation, XSolution};
pub use super::mod_move::Move;

pub trait NS<XES: XESolution> {
    //
    fn random_move(&self, se: &XES) -> Box<dyn Move<XES>>;

    // ======= from Component =======
    //fn to_string(&self) -> String;
}
