#![allow(non_snake_case)]

mod optfcore;
mod optframe;

use optfcore::fconstructive::FConstructive;
use optfcore::fevaluator::FEvaluator;
use optframe::core::base_concepts::{XESolution, XSolution};
use optframe::core::evaluation::Evaluation;

use crate::optframe::core::constructive::Constructive;
use crate::optframe::core::evaluator::Evaluator;
use crate::optframe::core::mod_move::Move;

use crate::optframe::core::base_concepts::XEvaluation;

// reference counted (for Problem Context)
use std::rc::Rc;

// formatter (operator<< on Display)
use std::fmt;

//use rand::Rng;
use rand::seq::SliceRandom;
use rand::thread_rng;

use std::marker::PhantomData;

// ------------------------
// begin real code
// ------------------------

pub struct ESolutionTSP {
    pub first_value: Vec<i32>,
    pub second_value: Evaluation,
}

impl XSolution for Vec<i32> {
    // nothing to do! must have something to do with Copy trait!!!
}

impl XESolution<Vec<i32>, Evaluation> for ESolutionTSP {
    fn first(&self) -> &Vec<i32> {
        &self.first_value
    }
    fn second(&self) -> &Evaluation {
        &self.second_value
    }
    fn first_mut(&mut self) -> &mut Vec<i32> {
        &mut self.first_value
    }
    fn second_mut(&mut self) -> &mut Evaluation {
        &mut self.second_value
    }
}

// ------------------------

use ndarray::Array2;

// TSP problem context and data reads
pub struct TSPProblemContext {
    pub n: usize,
    pub dist: Array2<i32>,
    /*
    public:
       int n;            // number of clients
       Matrix<int> dist; // distance matrix (Euclidean)
       // load data from Scanner
       void load(Scanner& scanner)
       {
          n = *scanner.nextInt();   // reads number of clients
          dist = Matrix<int>(n, n); // initializes n x n matrix
          //
          vector<double> xvalues(n);
          vector<double> yvalues(n);
          //
          for (int i = 0; i < n; i++) {
             scanner.next();
             xvalues[i] = *scanner.nextDouble(); // reads x
             yvalues[i] = *scanner.nextDouble(); // reads y
          }
          // calculate distance values, for every client pair (i,j)
          for (int i = 0; i < n; i++)
             for (int j = 0; j < n; j++)
                dist(i, j) = ::round(distance(xvalues.at(i), yvalues.at(i), xvalues.at(j), yvalues.at(j)));
       }
       // euclidean distance (double as return)
       double distance(double x1, double y1, double x2, double y2)
       {
          return sqrt((x1 - x2) * (x1 - x2) + (y1 - y2) * (y1 - y2));
       }
       */
}

impl fmt::Display for TSPProblemContext {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "n={} dist:\n{}\n", self.n, self.dist)
    }
}

// ------------------------

pub struct MoveSwap {
    pub pTSP: Rc<TSPProblemContext>,
    pub i: usize,
    pub j: usize,
}

// ------------------------

impl Move<Vec<i32>, Evaluation, ESolutionTSP> for MoveSwap {
    fn apply(&self, se: &mut ESolutionTSP) -> Box<dyn Move<Vec<i32>, Evaluation, ESolutionTSP>> {
        println!("apply from MoveSwap {} {}", self.i, self.j);
        //println!("problem is:\n {}", self.pTSP);

        let aux: i32 = se.first()[self.j];
        se.first_mut()[self.j] = se.first_mut()[self.i];
        se.first_mut()[self.i] = aux;

        // reverse move
        let mv = Box::new(MoveSwap {
            pTSP: self.pTSP.clone(),
            i: self.j,
            j: self.i,
        });
        return mv;
    }

    fn canBeApplied(&self, _se: &ESolutionTSP) -> bool {
        return (i32::abs((self.i - self.j) as i32) >= 2) && (self.i >= 1) && (self.j >= 1);
    }
    //

    fn applyUpdate(
        &self,
        se: &mut ESolutionTSP,
    ) -> Box<dyn Move<Vec<i32>, Evaluation, ESolutionTSP>> {
        // input cannot be outdated
        assert!(!se.second().isOutdated());
        let s = &mut se.first();
        //
        let i: usize = self.i;
        let j: usize = self.j;
        let pTSP = &self.pTSP;
        //
        let mut diff: f64 = (-pTSP.dist[[s[(i - 1)] as usize, s[i] as usize]]
            - pTSP.dist[[s[i] as usize, s[(i + 1) % pTSP.n] as usize]]
            - pTSP.dist[[s[j - 1] as usize, s[j] as usize]]
            - pTSP.dist[[s[j] as usize, s[(j + 1) % pTSP.n] as usize]])
            as f64;
        //
        diff += (pTSP.dist[[s[(i - 1)] as usize, s[j] as usize]]
            + pTSP.dist[[s[j] as usize, s[(i + 1) % pTSP.n] as usize]]
            + pTSP.dist[[s[j - 1] as usize, s[i] as usize]]
            + pTSP.dist[[s[i] as usize, s[(j + 1) % pTSP.n] as usize]]) as f64;
        // solution swap
        let rev = self.apply(se);
        //
        let newObjVal = se.second().evaluation() + diff;
        se.second_mut().setObjFunction(newObjVal);
        //se.second().setObjFunction(se.second().evaluation() + diff);
        return rev;
    }
    //

    //fn cost(const ESolutionTSP& se, bool allowEstimated) -> op<Evaluation<int>>
    fn cost(&self, se: &ESolutionTSP, _allowEstimated: bool) -> Option<Evaluation> {
        assert!(!se.second().isOutdated());
        let s = &se.first();
        //
        let i: usize = self.i;
        let j: usize = self.j;
        let pTSP = &self.pTSP;
        //
        let mut diff: f64 = (-pTSP.dist[[s[(i - 1)] as usize, s[i] as usize]]
            - pTSP.dist[[s[i] as usize, s[(i + 1) % pTSP.n] as usize]]
            - pTSP.dist[[s[j - 1] as usize, s[j] as usize]]
            - pTSP.dist[[s[j] as usize, s[(j + 1) % pTSP.n] as usize]])
            as f64;
        //
        diff += (pTSP.dist[[s[(i - 1)] as usize, s[j] as usize]]
            + pTSP.dist[[s[j] as usize, s[(i + 1) % pTSP.n] as usize]]
            + pTSP.dist[[s[j - 1] as usize, s[i] as usize]]
            + pTSP.dist[[s[i] as usize, s[(j + 1) % pTSP.n] as usize]]) as f64;
        //
        return Some(Evaluation {
            objVal: diff,
            outdated: false,
        });
    }

    fn toString(&self) -> String {
        let str = format!("MoveSwap i={} j={}", self.i, self.j);
        return str;
    }
}

impl fmt::Display for MoveSwap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.toString())
    }
}

// ========================
// helper to 'makeMoveSwap'
// (not necessary, but part of OptFrame Quickstart tutorial)
// ------------------------

fn makeMoveSwap(
    pTSP: Rc<TSPProblemContext>,
    i: usize,
    j: usize,
) -> Box<dyn Move<Vec<i32>, Evaluation, ESolutionTSP>> {
    return Box::new(MoveSwap {
        pTSP: pTSP.clone(),
        i,
        j,
    });
}

// ------------------------

/*

uptr<Move<ESolutionTSP>>
fRandomSwap(const ESolutionTSP& se)
{
   int i = rand() % pTSP.n;
   int j = i;
   while (j <= i) {
      i = rand() % pTSP.n;
      j = rand() % pTSP.n;
   }
   //return uptr<Move<ESolutionTSP>>(new MoveSwap{ make_pair(i, j), fApplySwap, fDefaultCanBeApplied<std::pair<int, int>, ESolutionTSP>, fCompare });
   return uptr<Move<ESolutionTSP>>(makeMoveSwap(i, j));
}

// Swap move (NS)
FNS<ESolutionTSP> nsswap{
   fRandomSwap
};
*/

fn main() {
    println!("Welcome to OptFrame Project (Rust version) - github.com/optframe");

    // =================================================
    // will not use local variable '_pTSP' (only P_TSP),
    // otherwise 'frandom' becomes closure, not lambda
    // =================================================
    let pTSP = TSPProblemContext {
        n: 5,
        dist: Array2::<i32>::ones((5, 5)), //dist: [[0 as i32; 5].to_vec() ; 5].to_vec()
    };

    //let mut v1 : Vec<i32> = Vec::new();

    /*
    let frandom : fn()->Vec<i32> = || -> Vec<i32> {
    //let frandom : dyn Fn()->Vec<i32> = || -> Vec<i32> {
        let _n : usize = _pTSP.n;
        //let v : Vec<i32> = Vec::new();
        let v : Vec<i32> = vec![0; _n];
        return v
    };
    */

    //let fc = FConstructive{func : frandom};

    let fc = FConstructive {
        func: || -> Vec<i32> {
            let _n: usize = pTSP.n;
            //let v : Vec<i32> = Vec::new();
            let mut v: Vec<i32> = vec![0; _n];
            let mut i: usize = 0;
            while i < _n {
                v[i] = i as i32;
                i = i + 1;
            }
            v.shuffle(&mut thread_rng());
            return v;
        },
    };

    let sol = fc.generateSolution();

    print!("solution: {:?}\n", sol);
    print!("distances:\n {:?}\n", pTSP.dist);

    let fev = FEvaluator {
        fEvaluate: |s: &Vec<i32>| -> Evaluation {
            let mut f: f64 = 0.0;
            let mut i: usize = 0;
            while i < (pTSP.n - 1) {
                f += pTSP.dist[[s[i] as usize, s[i + 1] as usize]] as f64;
                i = i + 1;
            }
            f += pTSP.dist[[s[pTSP.n - 1] as usize, s[0] as usize]] as f64;
            return Evaluation {
                objVal: f,
                outdated: false,
            };
        },
        phantomXS: PhantomData,
        phantomXEv: PhantomData,
    };

    let ev = fev.evaluate(&sol);

    print!("evaluation: {:?}\n", ev.evaluation());

    // ======================
    // tests with moves
    // ======================

    let my_pTSP: Rc<TSPProblemContext> = Rc::new(pTSP);
    //
    let mv1 = MoveSwap {
        pTSP: my_pTSP,
        i: 0,
        j: 1,
    };

    print!("mv1: {}\n", mv1);

    let mut esol = ESolutionTSP {
        first_value: sol,
        second_value: ev,
    };

    let mv2 = mv1.apply(&mut esol);

    print!("mv2: {}\n", mv2.toString());

    let _mv3 = mv2.apply(&mut esol);

    //print!("mv2: {}\n", *mv2);

    //let f2 : dyn Fn()->Vec<i32> = frandom;

    // Generate random solution
    //FConstructive<std::vector<int>> crand{
    //frandom
    //};
}
