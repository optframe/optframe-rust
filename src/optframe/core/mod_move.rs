//mod base_concepts;

// module 'move' is named 'mod_move'

#![allow(non_snake_case)]

/*
use std::fmt;
use std::fmt::Display;
*/

pub use super::base_concepts::{XESolution, XEvaluation, XSolution};

pub trait Move<XS, XEv, XES: XESolution<XS, XEv>>
where
    XS: XSolution,
    XEv: XEvaluation,
{
    //, XES: XESolution<XS, XEv>> {

    //
    fn apply(&self, se: &mut XES) -> Box<dyn Move<XS, XEv, XES>>;
    //
    fn canBeApplied(&self, _se: &XES) -> bool {
        // default: all can be applied
        true
    }
    //
    fn applyUpdate(&self, se: &mut XES) -> Box<dyn Move<XS, XEv, XES>> {
        let e: &mut XEv = se.second_mut();
        // ====== from OptFrame (C++) ======
        // boolean 'outdated' indicates that Evaluation needs update (after Solution change)
        // note that even if the reverse move is applied, the Evaluation will continue with
        // the outdated status set to true. So more efficient approaches may rewrite this
        // method, or use  efficient re-evaluation by means of the 'cost' method.
        e.setOutdated(true);
        // apply the move to R and ADS, saving the reverse (or undo) move
        // Box<dyn Move<XS, XEv, XES>>
        // update neighborhood local optimum status TODO:deprecated
        //updateNeighStatus(se);

        // return reverse move (or null)
        self.apply(se)
    }
    //
    fn cost(&self, _se: &XES, _allowEstimated: bool) -> Option<XEv> {
        None
    }

    // ======= from Component =======
    fn toString(&self) -> String;
}

/*
//
class MoveSwap : public Move<ESolutionTSP>
{
public:
   int i, j;

   MoveSwap(int _i, int _j)
     : i{ _i }
     , j{ _j }
   {
   }

   bool canBeApplied(const ESolutionTSP& se) override
   {
      return (::abs(i - j) >= 2) && (i >= 1) && (j >= 1);
   }

   uptr<Move<ESolutionTSP>> applyUpdate(ESolutionTSP& se) override
   {
      // input cannot be outdated
      assert(!se.second.outdated);
      auto& s = se.first;
      int diff = -pTSP.dist(s[i - 1], s[i]) - pTSP.dist(s[i], s[(i + 1) % pTSP.n]) - pTSP.dist(s[j - 1], s[j]) - pTSP.dist(s[j], s[(j + 1) % pTSP.n]);
      diff += pTSP.dist(s[i - 1], s[j]) + pTSP.dist(s[j], s[(i + 1) % pTSP.n]) + pTSP.dist(s[j - 1], s[i]) + pTSP.dist(s[i], s[(j + 1) % pTSP.n]);
      // solution swap
      auto rev = this->apply(se);
      se.second.setObjFunction(se.second.evaluation() + diff);
      return rev;
   }

   virtual op<Evaluation<int>> cost(const ESolutionTSP& se, bool allowEstimated) override
   {
      assert(!se.second.outdated);
      auto& s = se.first;
      int diff = -pTSP.dist(s[i - 1], s[i]) - pTSP.dist(s[i], s[(i + 1) % pTSP.n]) - pTSP.dist(s[j - 1], s[j]) - pTSP.dist(s[j], s[(j + 1) % pTSP.n]);
      diff += pTSP.dist(s[i - 1], s[j]) + pTSP.dist(s[j], s[(i + 1) % pTSP.n]) + pTSP.dist(s[j - 1], s[i]) + pTSP.dist(s[i], s[(j + 1) % pTSP.n]);
      return std::make_optional(Evaluation<int>(diff));
   }

   uptr<Move<ESolutionTSP>> apply(ESolutionTSP& se) override
   {
      // perform swap of clients i and j
      int aux = se.first[j];
      se.first[j] = se.first[i];
      se.first[i] = aux;
      return uptr<Move<ESolutionTSP>>(new MoveSwap{ j, i }); // return a reverse move ('undo' move)s
   }

   bool
   operator==(const Move<ESolutionTSP>& other) const override
   {
      auto& fmove = (MoveSwap&)other;
      return (i == fmove.i) && (j == fmove.j);
   }
};
*/

// no need to support "Display" (operator<<), only "toString" is enough
/*
impl<XS, XEv, XES: XESolution<XS, XEv>> fmt::Display for dyn Move<XS, XEv, XES>
    where XS: XSolution, XEv: XEvaluation
 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Move")
    }
}
*/
