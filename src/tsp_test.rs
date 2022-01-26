use std::{fmt, marker::PhantomData, rc::Rc};

use ndarray::Array2;
use rand::{prelude::SliceRandom, thread_rng};

use crate::{
    core::{
        evaluation::{XESolution, XEvaluation, XSolution},
        mod_move::Move,
        Constructive, Evaluation, Evaluator,
    },
    FConstructive,
    FEvaluator,
    //FxNSIterator,
    FNS,
};

//use crate::optfcore::fxnsseq::FxNSIteratorCoro;
use crate::optfcore::fxnsseq::FxNSIterator;
use crate::optfcore::fxnsseq::FxNSIteratorCoro;
use crate::optfcore::fxnsseq::GeneratorFuncNSIteratorAdapter;
use crate::optfcore::fxnsseq::GeneratorIteratorAdapter;
use crate::optfcore::fxnsseq::GeneratorNSIteratorAdapter;
use crate::optframe::core::ns::NS;
use crate::optframe::core::nsseq::NSIterator;

// -------

use std::{
    ops::{Generator, GeneratorState},
    pin::Pin,
};

// -------

// =======================================

pub struct ESolutionTSP {
    pub first_value: Vec<i32>,
    pub second_value: Evaluation,
}

//impl XRepresentation for Vec<i32> {
// nothing to do! must have something to do with Copy trait!!!
//}

impl XSolution for Vec<i32> {
    // nothing to do! must have something to do with Copy trait!!!
}

impl XESolution for ESolutionTSP {
    // types
    type FirstType = Vec<i32>;
    type SecondType = Evaluation;
    // methods
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
    pub p_tsp: Rc<TSPProblemContext>,
    pub i: usize,
    pub j: usize,
}

// ------------------------

impl Move<ESolutionTSP> for MoveSwap {
    fn apply(&self, se: &mut ESolutionTSP) -> Box<dyn Move<ESolutionTSP>> {
        println!("apply from MoveSwap {} {}", self.i, self.j);
        //println!("problem is:\n {}", self.pTSP);

        let aux: i32 = se.first()[self.j];
        se.first_mut()[self.j] = se.first_mut()[self.i];
        se.first_mut()[self.i] = aux;

        // reverse move

        Box::new(MoveSwap {
            p_tsp: self.p_tsp.clone(),
            i: self.j,
            j: self.i,
        })
    }

    fn can_be_applied(&self, _se: &ESolutionTSP) -> bool {
        (i32::abs((self.i - self.j) as i32) >= 2) && (self.i >= 1) && (self.j >= 1)
    }
    //

    fn apply_update(&self, se: &mut ESolutionTSP) -> Box<dyn Move<ESolutionTSP>> {
        // input cannot be outdated
        assert!(!se.second().is_outdated());
        let s = &mut se.first();
        //
        let i: usize = self.i;
        let j: usize = self.j;
        let p_tsp = &self.p_tsp;
        //
        let mut diff: f64 = (-p_tsp.dist[[s[(i - 1)] as usize, s[i] as usize]]
            - p_tsp.dist[[s[i] as usize, s[(i + 1) % p_tsp.n] as usize]]
            - p_tsp.dist[[s[j - 1] as usize, s[j] as usize]]
            - p_tsp.dist[[s[j] as usize, s[(j + 1) % p_tsp.n] as usize]])
            as f64;
        //
        diff += (p_tsp.dist[[s[(i - 1)] as usize, s[j] as usize]]
            + p_tsp.dist[[s[j] as usize, s[(i + 1) % p_tsp.n] as usize]]
            + p_tsp.dist[[s[j - 1] as usize, s[i] as usize]]
            + p_tsp.dist[[s[i] as usize, s[(j + 1) % p_tsp.n] as usize]]) as f64;
        // solution swap
        let rev = self.apply(se);
        //
        let new_obj_val = se.second().evaluation() + diff;
        se.second_mut().set_obj_val(new_obj_val);
        //se.second().setObjFunction(se.second().evaluation() + diff);
        rev
    }
    //

    //fn cost(const ESolutionTSP& se, bool allowEstimated) -> op<Evaluation<int>>
    fn cost(&self, se: &ESolutionTSP, _allow_estimated: bool) -> Option<Evaluation> {
        assert!(!se.second().is_outdated());
        let s = &se.first();
        //
        let i: usize = self.i;
        let j: usize = self.j;
        let p_tsp = &self.p_tsp;
        //
        let mut diff: f64 = (-p_tsp.dist[[s[(i - 1)] as usize, s[i] as usize]]
            - p_tsp.dist[[s[i] as usize, s[(i + 1) % p_tsp.n] as usize]]
            - p_tsp.dist[[s[j - 1] as usize, s[j] as usize]]
            - p_tsp.dist[[s[j] as usize, s[(j + 1) % p_tsp.n] as usize]])
            as f64;
        //
        diff += (p_tsp.dist[[s[(i - 1)] as usize, s[j] as usize]]
            + p_tsp.dist[[s[j] as usize, s[(i + 1) % p_tsp.n] as usize]]
            + p_tsp.dist[[s[j - 1] as usize, s[i] as usize]]
            + p_tsp.dist[[s[i] as usize, s[(j + 1) % p_tsp.n] as usize]]) as f64;
        //
        Some(Evaluation {
            obj_val: diff,
            outdated: false,
        })
    }

    fn to_string(&self) -> String {
        format!("MoveSwap i={} j={}", self.i, self.j)
    }
}

impl fmt::Display for MoveSwap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", <Self as Move<_>>::to_string(self))
    }
}

// ========================
// helper to 'makeMoveSwap'
// (not necessary, but part of OptFrame Quickstart tutorial)
// ------------------------

//#[allow(dead_code)]
fn make_move_swap(p_tsp: Rc<TSPProblemContext>, i: usize, j: usize) -> Box<dyn Move<ESolutionTSP>> {
    Box::new(MoveSwap { p_tsp, i, j })
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

// ---------------
// https://stackoverflow.com/questions/16421033/lazy-sequence-generation-in-rust/30279122#30279122

struct GeneratorIteratorAdapter1<G>(Pin<Box<G>>);

impl<G> GeneratorIteratorAdapter1<G>
where
    G: Generator<Return = ()>,
{
    fn new(gen: G) -> Self {
        Self(Box::pin(gen))
    }
}

impl<G> Iterator for GeneratorIteratorAdapter1<G>
where
    G: Generator<Return = ()>,
{
    type Item = G::Yield;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0.as_mut().resume(()) {
            GeneratorState::Yielded(x) => Some(x),
            GeneratorState::Complete(_) => None,
        }
    }
}
// ---------------

/*
impl FxNSIteratorCoro<ESolutionTSP, TSPProblemContext> for FxNSIterator<ESolutionTSP> {
    fn compute_generator<G: Generator<Yield = Box<dyn Move<ESolutionTSP>>, Return = ()>>(
        se: &ESolutionTSP,
        problem: Rc<TSPProblemContext>,
    ) -> Box<GeneratorIteratorAdapter<G>> {
        //
        pub fn firstn_moves_se(
            _se: &ESolutionTSP,
            my_p_tsp: Rc<TSPProblemContext>,
        ) -> impl Generator<Yield = Box<dyn Move<ESolutionTSP>>, Return = ()> + '_ {
            move || {
                let mut n = _se.first().len();
                let mut num = 0;
                while num < n {
                    let val: usize = num as usize;
                    let cop: Rc<TSPProblemContext> = my_p_tsp.clone();
                    let mv: Box<dyn Move<ESolutionTSP>> = make_move_swap(cop, val, val);
                    yield mv;
                    num += 1;
                }
            }
        }

        for i in GeneratorIteratorAdapter::new(firstn_moves_se(se, problem)) {
            println!("i={}\n", i.to_string());
        }

        Box::new(GeneratorIteratorAdapter::new(firstn_moves_se(se, problem)))
        //return firstn_moves_se(se, problem);
    }
}
*/

// ===============

struct Foo {
    pub foo: Box<dyn Fn(Rc<TSPProblemContext>) -> usize>,
}

impl Foo {
    fn new(foo: impl Fn(Rc<TSPProblemContext>) -> usize + 'static) -> Self {
        Self { foo: Box::new(foo) }
    }
}

struct FooTest {
    pub pin: Option<Pin<Box<dyn Generator<Yield = u64, Return = ()>>>>,
}

impl FooTest {
    pub fn new() -> Self {
        Self { pin: None }
    }

    pub fn first(&mut self, k: u64) -> () {
        let mut gen_test = || {
            yield 1;
        };

        fn firstn_moves(n: u64) -> impl Generator<Yield = u64, Return = ()> {
            move || {
                let mut m = 0;
                while m < n {
                    yield m;
                    m = m + 1;
                }
            }
        }
        //

        self.pin = Some(Box::pin(firstn_moves(k)));
    }
}

// ==================

impl FxNSIteratorCoro<ESolutionTSP> for FxNSIterator<ESolutionTSP> {
    //
    //pub pin: Option<Pin<Box<dyn Generator<Yield = u64, Return = ()>>>>,
    //
    fn setup_coro(&mut self) -> () {
        //
        fn firstn_moves_x(
            n: u64,
        ) -> impl Generator<Yield = Box<dyn Move<ESolutionTSP>>, Return = ()> {
            move || {
                //
                let rc_tsp: Rc<TSPProblemContext> = Rc::new(TSPProblemContext {
                    n: 5,
                    dist: Array2::<i32>::ones((5, 5)),
                });
                let mut num = 0;
                while num < n {
                    let val: usize = num as usize;
                    let mv: Box<dyn Move<ESolutionTSP>> = make_move_swap(rc_tsp.clone(), val, val);
                    yield mv;
                    num += 1;
                }
            }
        }
        //
        let k = 5;
        self.pin = Some(Box::pin(firstn_moves_x(k)));
    }
    //
    fn get_coro(
        &mut self,
    ) -> &mut Option<Pin<Box<dyn Generator<Yield = Box<dyn Move<ESolutionTSP>>, Return = ()>>>>
    {
        &mut self.pin
    }
    //
    fn set_move(&mut self, mv: Option<Box<dyn Move<ESolutionTSP>>>) {
        self.mv = mv;
    }
    // get move object and move it out
    fn get_move_mv(&mut self) -> Option<Box<dyn Move<ESolutionTSP>>> {
        //
        // https://stackoverflow.com/questions/52031002/how-do-i-move-out-of-a-struct-field-that-is-an-option
        // TODO: check if mem::replace is the only way
        //
        // let mv2 = std::mem::replace(&mut self.mv, None);
        //
        // Using Option::take
        let mv2 = self.mv.take();

        mv2
    }
    //
    fn get_move_ref(&mut self) -> &Option<Box<dyn Move<ESolutionTSP>>> {
        &self.mv
    }
    //}
}

// ==================

#[test]
fn main() {
    println!("Welcome to OptFrame Project (Rust version) - github.com/optframe");

    // =================================================
    // will not use local variable '_pTSP' (only P_TSP),
    // otherwise 'frandom' becomes closure, not lambda
    // =================================================
    let p_tsp = TSPProblemContext {
        n: 5,
        dist: Array2::<i32>::ones((5, 5)), //dist: [[0 as i32; 5].to_vec() ; 5].to_vec()
    };

    let rc_tsp: Rc<TSPProblemContext> = Rc::new(TSPProblemContext {
        n: 5,
        dist: Array2::<i32>::ones((5, 5)), //dist: [[0 as i32; 5].to_vec() ; 5].to_vec()
    });
    // ==================

    let foo = Foo {
        foo: Box::new(|prob| prob.n + 1),
    };
    (foo.foo)(rc_tsp.clone()); // do not move 'rc_tsp'

    (Foo::new(|prob| prob.n + 1).foo)(rc_tsp.clone());

    let mut foo_test = FooTest::new();

    foo_test.first(5);

    // ----------

    let mut gen_test = || {
        yield 1;
    };

    //let bx_gen_test: Box<dyn Fn(Rc<TSPProblemContext>) -> _> = Box::new(gen_test);

    //let foo2 = FooGen {
    //    foo: Box::new(|prob| yield 1),
    //};
    //
    //(foo.foo)(rc_tsp.clone()); // do not move 'rc_tsp'

    // =============================================

    let fc = FConstructive {
        func: || -> Vec<i32> {
            let _n: usize = p_tsp.n;
            //let v : Vec<i32> = Vec::new();
            let mut v: Vec<i32> = vec![0; _n];
            let mut i: usize = 0;
            while i < _n {
                v[i] = i as i32;
                i += 1;
            }
            v.shuffle(&mut thread_rng());
            v
        },
    };

    let sol = fc.generate_solution();

    println!("solution: {:?}", sol);
    println!("distances:\n {:?}", p_tsp.dist);

    // =============================================

    let fev = FEvaluator {
        f_evaluate: |s: &Vec<i32>| -> Evaluation {
            let mut f: f64 = 0.0;
            let mut i: usize = 0;
            while i < (p_tsp.n - 1) {
                f += p_tsp.dist[[s[i] as usize, s[i + 1] as usize]] as f64;
                i += 1;
            }
            f += p_tsp.dist[[s[p_tsp.n - 1] as usize, s[0] as usize]] as f64;
            Evaluation {
                obj_val: f,
                outdated: false,
            }
        },
        phantom_xs: PhantomData,
        phantom_xev: PhantomData,
    };

    let ev: Evaluation = fev.evaluate(&sol);

    println!("evaluation: {:?}", ev.evaluation());

    // =============================================

    let mut esol = ESolutionTSP {
        first_value: sol,
        second_value: ev,
    };

    println!("evaluation: {:?}", esol.second().evaluation());

    // =============================================

    // ======================
    // tests with moves
    // ======================

    let my_p_tsp: Rc<TSPProblemContext> = Rc::new(p_tsp);
    //
    let mv1 = MoveSwap {
        p_tsp: my_p_tsp.clone(),
        i: 0,
        j: 1,
    };

    println!("mv1: {}", mv1);

    let mv2 = mv1.apply(&mut esol);

    println!("mv2: {}", mv2.to_string());

    let _mv3 = mv2.apply(&mut esol);

    // =============================================

    let fns_swap = FNS {
        f_random: |_se: &ESolutionTSP| -> Box<dyn Move<ESolutionTSP>> {
            let my_p_tsp1 = &my_p_tsp;
            let mut i: usize = rand::random::<usize>() % my_p_tsp1.n;
            let mut j = i;
            while j <= i {
                i = rand::random::<usize>() % my_p_tsp1.n;
                j = rand::random::<usize>() % my_p_tsp1.n;
            }
            return make_move_swap(my_p_tsp1.clone(), i, j);
        },
        phantom_xes: PhantomData,
    };

    let mv4 = fns_swap.random_move(&esol);
    println!("mv4: {}", mv4.to_string());

    // =============================================

    fn firstn(n: u64) -> impl Generator<Yield = u64, Return = ()> {
        move || {
            let mut num = 0;
            while num < n {
                yield num;
                num += 1;
            }
        }
    }

    for i in GeneratorIteratorAdapter1::new(firstn(10)) {
        println!("i={}\n", i);
    }

    // =============================================

    fn firstn_moves(
        n: u64,
        my_p_tsp1: Rc<TSPProblemContext>,
    ) -> impl Generator<Yield = Box<dyn Move<ESolutionTSP>>, Return = ()> {
        move || {
            let mut num = 0;
            while num < n {
                let val: usize = num as usize;
                let mv: Box<dyn Move<ESolutionTSP>> = make_move_swap(my_p_tsp1.clone(), val, val);
                yield mv;
                num += 1;
            }
        }
    }

    for i in GeneratorIteratorAdapter1::new(firstn_moves(10, my_p_tsp.clone())) {
        println!("i={}\n", i.to_string());
    }

    let mut g = GeneratorIteratorAdapter1::new(firstn_moves(10, my_p_tsp.clone()));
    let mut x = g.next();
    while !x.is_none() {
        match x {
            Some(y) => println!("Result: {}", y.to_string()),
            None => {}
        }
        x = g.next();
    }

    // =====================================
    println!("testing GeneratorNSIterator");

    let mut g1 = GeneratorNSIteratorAdapter::new(firstn_moves(10, my_p_tsp.clone()), &esol);
    //
    g1.first();
    while !g1.is_done() {
        match g1.current() {
            Some(y) => println!("Result NSIterator: {}", y.to_string()),
            None => {}
        }
        g1.next();
    }

    // =============

    let mut ns_it_coro: FxNSIterator<ESolutionTSP> = FxNSIterator::new();
    println!("TESTING ns_it_coro\n");
    ns_it_coro.first();
    while !ns_it_coro.is_done() {
        let mv = ns_it_coro.current();
        match mv {
            Some(x) => {
                println!("mv = {}\n", x.to_string())
            }
            None => {}
        }
        ns_it_coro.next();
    }

    /*
    fn firstn_moves_prob(
        xes: ESolutionTSP,
        //my_p_tsp1: Rc<TSPProblemContext>,
    ) -> impl Generator<Yield = Box<dyn Move<ESolutionTSP>>, Return = ()> {
        move || {
            let my_p_tsp1: Rc<TSPProblemContext> = Rc::new(TSPProblemContext {
                n: 5,
                dist: Array2::<i32>::ones((5, 5)), //dist: [[0 as i32; 5].to_vec() ; 5].to_vec()
            });
            let mut num = 0;
            let n = xes.first().len();
            while num < n {
                let val: usize = num as usize;
                let mv: Box<dyn Move<ESolutionTSP>> = make_move_swap(my_p_tsp1.clone(), val, val);
                yield mv;
                num += 1;
            }
        }
    }

    let func2 = Box::new(firstn_moves_prob);

    let mut g2 = GeneratorFuncNSIteratorAdapter::new(func2, &esol, my_p_tsp.clone());
    */

    /*
    fn firstn_moves_se(
        _se: &ESolutionTSP,
    ) -> impl Generator<Yield = Box<dyn Move<ESolutionTSP>>, Return = ()> {
        move || {
            let mut n = _se.first().len();
            let mut num = 0;
            while num < n {
                let val: usize = num as usize;
                let mv: Box<dyn Move<ESolutionTSP>> = make_move_swap(my_p_tsp.clone(), val, val);
                yield mv;
                num += 1;
            }
        }
    }
    */

    //type GenTSP = Generator<Yield = Box<dyn Move<ESolutionTSP>>, Return = ()>;
    /*
    fn firstn_closure(
        _se: &ESolutionTSP,
    ) -> impl Generator<Yield = Box<dyn Move<ESolutionTSP>>, Return = ()> {
        move || {
            let my_p_tsp1 = &my_p_tsp;
            let mut n = _se.first().len();
            let mut num = 0;
            while num < n {
                let val: usize = num as usize;
                let mv: Box<dyn Move<ESolutionTSP>> = make_move_swap(my_p_tsp1.clone(), val, val);
                yield mv;
                num += 1;
            }
        }
    }
    */

    /*
    let firstn_closure_2: dyn Fn(
        &ESolutionTSP,
    )
        -> impl Generator<Yield = Box<dyn Move<ESolutionTSP>>, Return = ()> = firstn_closure;

    let fns_it = FxNSIterator {
        fgen: Box::new(
            |_se: &ESolutionTSP| -> impl Generator<Yield = Box<dyn Move<ESolutionTSP>>, Return = ()> {
                move || {
                    let my_p_tsp1 = &my_p_tsp;
                    let mut n = _se.first().len();
                    let mut num = 0;
                    while num < n {
                        let val: usize = num as usize;
                        let mv: Box<dyn Move<ESolutionTSP>> =
                            make_move_swap(my_p_tsp1.clone(), val, val);
                        yield mv;
                        num += 1;
                    }
                }
            },
        ),
        consumed_current: false,
        done: false,
        phantom_xes: PhantomData,
        phantom_gen: PhantomData,
    };
    */

    //let mut nsit = FxNSIterator::new(firstn_moves(10, my_p_tsp.clone()));

    /*
    let fxnsiterator = FxNSIterator {
        f_generator:
            |_se: &ESolutionTSP| -> dyn Generator<Yield = Box<dyn Move<XES>>, Return = ()> {
                let my_p_tsp1 = &my_p_tsp;
                let mut i: usize = rand::random::<usize>() % my_p_tsp1.n;
                let mut j = i;
                while j <= i {
                    i = rand::random::<usize>() % my_p_tsp1.n;
                    j = rand::random::<usize>() % my_p_tsp1.n;
                }
                return make_move_swap(my_p_tsp1.clone(), i, j);
            },
        phantom_xes: PhantomData,
    };
    */

    //let f2 : dyn Fn()->Vec<i32> = frandom;

    // Generate random solution
    //FConstructive<std::vector<int>> crand{
    //frandom
    //};
}
