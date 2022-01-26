// module 'ns'

pub use super::base_concepts::{XESolution, XEvaluation, XSolution};
pub use super::mod_move::Move;

pub trait NSIterator<XES: XESolution> {
    //
    //type MoveType;
    //
    fn first(&mut self) -> ();
    fn next(&mut self) -> ();
    fn is_done(&mut self) -> bool;
    fn current(&mut self) -> &Option<Box<dyn Move<XES>>>;
}

pub trait NSSeq<XES: XESolution> {
    //
    fn random_move(&self, se: &XES) -> Box<dyn Move<XES>>;
    fn get_iterator(&self, se: &XES) -> Box<dyn NSIterator<XES>>;

    // ======= from Component =======
    //fn to_string(&self) -> String;
}
