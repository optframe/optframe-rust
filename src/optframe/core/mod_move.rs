// module 'move' is named 'mod_move'

pub use super::base_concepts::{XESolution, XEvaluation, XSolution};

pub trait Move<XES: XESolution> {
    // cannot do some type shortcut here...
    //type XS; // XES::FirstType;
    //type XEv; // XES::SecondType
    //
    fn apply(&self, se: &mut XES) -> Box<dyn Move<XES>>;
    //
    fn can_be_applied(&self, _se: &XES) -> bool {
        // default: all can be applied
        true
    }
    //
    fn apply_update(&self, se: &mut XES) -> Box<dyn Move<XES>> {
        let e: &mut XES::SecondType = se.second_mut();
        // ====== from OptFrame (C++) ======
        // boolean 'outdated' indicates that Evaluation needs update (after Solution change)
        // note that even if the reverse move is applied, the Evaluation will continue with
        // the outdated status set to true. So more efficient approaches may rewrite this
        // method, or use  efficient re-evaluation by means of the 'cost' method.
        e.set_outdated(true);
        // apply the move to R and ADS, saving the reverse (or undo) move
        // Box<dyn Move<XS, XEv, XES>>
        // update neighborhood local optimum status TODO:deprecated
        //updateNeighStatus(se);

        // return reverse move (or null)
        self.apply(se)
    }
    //
    fn cost(&self, _se: &XES, _allow_estimated: bool) -> Option<XES::SecondType> {
        None
    }

    // ======= from Component =======
    fn to_string(&self) -> String;
}
