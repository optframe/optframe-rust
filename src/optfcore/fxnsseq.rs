//
pub use super::optframe::core::base_concepts::{XESolution, XEvaluation, XSolution};
pub use super::optframe::core::mod_move::Move;
pub use super::optframe::core::nsseq::NSIterator;
pub use super::optframe::core::nsseq::NSSeq;

// -------

use std::{
    ops::{Generator, GeneratorState},
    pin::Pin,
};

// -------

//use std::marker::PhantomData;

use std::{marker::PhantomData, rc::Rc};

// -------

// ---------------
// https://stackoverflow.com/questions/16421033/lazy-sequence-generation-in-rust/30279122#30279122

pub struct GeneratorIteratorAdapter<G>(Pin<Box<G>>);

impl<G> GeneratorIteratorAdapter<G>
where
    G: Generator<Return = ()>,
{
    pub fn new(gen: G) -> Self {
        Self(Box::pin(gen))
    }
}

impl<G> Iterator for GeneratorIteratorAdapter<G>
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

// =================
//     NSIterator
// =================

pub struct GeneratorNSIteratorAdapter<XES: XESolution, G> {
    pin: Pin<Box<G>>,
    mv: Option<Box<dyn Move<XES>>>,
}

impl<XES: XESolution, G> GeneratorNSIteratorAdapter<XES, G>
where
    G: Generator<Return = ()>,
{
    pub fn new(gen: G, _se: &XES) -> Self {
        Self {
            pin: Box::pin(gen),
            mv: None,
        }
    }
}

impl<G, XES: XESolution> NSIterator<XES> for GeneratorNSIteratorAdapter<XES, G>
where
    G: Generator<Yield = Box<dyn Move<XES>>, Return = ()>,
{
    fn first(&mut self) -> () {
        let _mv: Option<<G as Generator>::Yield> = match self.pin.as_mut().resume(()) {
            GeneratorState::Yielded(x) => Some(x),
            GeneratorState::Complete(_) => None,
        };
        self.mv = _mv;
    }
    fn next(&mut self) -> () {
        let _mv = match self.pin.as_mut().resume(()) {
            GeneratorState::Yielded(x) => Some(x),
            GeneratorState::Complete(_) => None,
        };
        self.mv = _mv;
    }
    fn is_done(&mut self) -> bool {
        return self.mv.is_none();
    }
    fn current(&mut self) -> &Option<Box<dyn Move<XES>>> {
        let mv2 = &self.mv;
        mv2
    }
}

// =====================

pub struct FxNSIterator<XES: XESolution> {
    pub pin: Option<Pin<Box<dyn Generator<Yield = Box<dyn Move<XES>>, Return = ()>>>>,
    pub mv: Option<Box<dyn Move<XES>>>,
    //pub phantom_xes: PhantomData<XES>,
}

impl<XES: XESolution> FxNSIterator<XES> {
    pub fn new() -> Self {
        Self {
            pin: None,
            mv: None, //phantom_xes: PhantomData,
        }
    }
}

/*
pub trait FxNSIteratorCoro<XES: XESolution>: NSIterator<XES> {
    //impl<XES: XESolution> NSIterator<XES> for FxNSIterator<XES> {
    //
    fn setup_coro(&mut self) -> ();
    //
    fn get_coro(&mut self) -> Option<Pin<Box<dyn Generator<Yield = u64, Return = ()>>>>;
    //
    fn set_move(&mut self, mv: Option<Box<dyn Move<XES>>>);
    //
    fn get_move(&self) -> Option<Box<dyn Move<XES>>>;
    //
    fn get_move_ref(&mut self) -> &Option<Box<dyn Move<XES>>>;
    //}
    //impl<XES: XESolution> FxNSIteratorCoro<XES> for FxNSIterator<XES> {
    //
    // ======= REAL PUBLIC =======
    //
    fn first(&mut self) -> () {
        //
        self.setup_coro();
        //
        let mut _coro = match self.get_coro() {
            Some(x) => x,
            None => {
                panic!("oi")
            }
        };
        let _mv: Option<u64> = match _coro.as_mut().resume(()) {
            GeneratorState::Yielded(x) => Some(x),
            GeneratorState::Complete(_) => None,
        };
        //self.set_move(_mv);
        //self.mv = _mv;
    }
    fn next(&mut self) -> () {
        let mut _coro = match self.get_coro() {
            Some(x) => x,
            None => {
                panic!("oi")
            }
        };
        let _mv: Option<u64> = match _coro.as_mut().resume(()) {
            GeneratorState::Yielded(x) => Some(x),
            GeneratorState::Complete(_) => None,
        };
    }
    fn is_done(&mut self) -> bool {
        return self.get_move_ref().is_none();
    }
    fn current(&mut self) -> Option<Box<dyn Move<XES>>> {
        self.get_move()
    }
}

*/

pub trait FxNSIteratorCoro<XES: XESolution> {
    //: NSIterator<XES> {
    //impl<XES: XESolution> NSIterator<XES> for FxNSIterator<XES> {
    //
    fn setup_coro(&mut self) -> ();
    //
    fn get_coro(
        &mut self,
    ) -> &mut Option<Pin<Box<dyn Generator<Yield = Box<dyn Move<XES>>, Return = ()>>>>;
    //
    fn set_move(&mut self, mv: Option<Box<dyn Move<XES>>>);
    //
    fn get_move_mv(&mut self) -> Option<Box<dyn Move<XES>>>;
    //
    fn get_move_ref(&mut self) -> &Option<Box<dyn Move<XES>>>;
    //}
    //impl<XES: XESolution> FxNSIteratorCoro<XES> for FxNSIterator<XES> {
    //
    // ======= REAL PUBLIC =======
    //
    fn first(&mut self) -> () {
        //
        self.setup_coro();
        //
        let mut _coro = match self.get_coro() {
            Some(x) => x,
            None => {
                panic!("oi")
            }
        };
        let _mv: Option<Box<dyn Move<XES>>> = match _coro.as_mut().resume(()) {
            GeneratorState::Yielded(x) => Some(x),
            GeneratorState::Complete(_) => None,
        };
        self.set_move(_mv);
        //self.mv = _mv;
    }
    fn next(&mut self) -> () {
        let mut _coro = match self.get_coro() {
            Some(x) => x,
            None => {
                panic!("oi")
            }
        };
        let _mv: Option<Box<dyn Move<XES>>> = match _coro.as_mut().resume(()) {
            GeneratorState::Yielded(x) => Some(x),
            GeneratorState::Complete(_) => None,
        };
        self.set_move(_mv);
    }
    fn is_done(&mut self) -> bool {
        return self.get_move_ref().is_none();
    }
    fn current(&mut self) -> Option<Box<dyn Move<XES>>> {
        self.get_move_mv()
    }
}

/*
impl<XES: XESolution> NSIterator<XES> for dyn FxNSIteratorCoro<XES> {
    //
    fn setup_coro(&mut self) -> ();
    //
    fn get_coro(&mut self) -> Option<Pin<Box<dyn Generator<Yield = u64, Return = ()>>>>;
    //
    fn set_move(&mut self, mv: Option<Box<dyn Move<XES>>>);
    //
    fn get_move(&self) -> Option<Box<dyn Move<XES>>>;
    //
    fn get_move_ref(&mut self) -> &Option<Box<dyn Move<XES>>>;
    //}
    //impl<XES: XESolution> FxNSIteratorCoro<XES> for FxNSIterator<XES> {
    //
    // ======= REAL PUBLIC =======
    //
    fn first(&mut self) -> () {
        //
        self.setup_coro();
        //
        let mut _coro = match self.get_coro() {
            Some(x) => x,
            None => {
                panic!("oi")
            }
        };
        let _mv: Option<u64> = match _coro.as_mut().resume(()) {
            GeneratorState::Yielded(x) => Some(x),
            GeneratorState::Complete(_) => None,
        };
        //self.set_move(_mv);
        //self.mv = _mv;
    }
    fn next(&mut self) -> () {
        let mut _coro = match self.get_coro() {
            Some(x) => x,
            None => {
                panic!("oi")
            }
        };
        let _mv: Option<u64> = match _coro.as_mut().resume(()) {
            GeneratorState::Yielded(x) => Some(x),
            GeneratorState::Complete(_) => None,
        };
    }
    fn is_done(&mut self) -> bool {
        return self.get_move_ref().is_none();
    }
    fn current(&mut self) -> Option<Box<dyn Move<XES>>> {
        self.get_move()
    }
}
*/

// ===============

pub struct GeneratorFuncNSIteratorAdapter<XES: XESolution, Prob, G> {
    // storing a closure that generates a Generator
    //pin_func: Box<dyn Fn(&XES, Rc<Prob>) -> Pin<Box<G>>>,
    pin_func: Box<dyn Fn(&XES) -> Pin<Box<G>>>,
    mv: Option<Box<dyn Move<XES>>>,
    prob: Rc<Prob>,
}

impl<XES: XESolution, Prob, G> GeneratorFuncNSIteratorAdapter<XES, Prob, G>
where
    G: Generator<Return = ()>,
{
    pub fn new(
        //func: Box<dyn Fn(&XES, Rc<Prob>) -> Pin<Box<G>>>,
        func: Box<dyn Fn(&XES) -> Pin<Box<G>>>,
        _se: &XES,
        my_prob: Rc<Prob>,
    ) -> Self {
        Self {
            pin_func: func,
            mv: None,
            prob: my_prob,
        }
    }
}

/*
pub trait FxNSIteratorCoro<XES: XESolution, P> //where
//    G: Generator<Return = ()>,
{
    //fn compute_generator(se: &XES, problem: Rc<P>) -> G;
    fn compute_generator<G: Generator<Return = ()>>(
        se: &XES,
        problem: Rc<P>,
    ) -> Box<GeneratorIteratorAdapter<G>>;

    //GeneratorIteratorAdapter<G>
    //fn get_generator(&self) -> G;
    //fn set_done(&self, b: bool) -> ();

    /*

    fn first(&self)
    {
       done = !gen.next();      // advance and update bool
       consumedCurrent = false; // can allow consuming again
    }

    virtual void next()
    {
       done = !gen.next();      // advance and update bool
       consumedCurrent = false; // can allow consuming again
    }

    virtual bool isDone()
    {
       return done; // verify bool
    }

    virtual uptr<Move<XES>> current()
    {
       // should never repeat pointer (enforce 'unique' semantics!)
       if (consumedCurrent) {
          //std::cerr << "ALREADY CONSUMED!!" << std::endl;
          return nullptr;
       }
       consumedCurrent = true;
       return uptr<Move<XES>>(gen.getValue());
    }
    */
}


pub struct FxNSIterator<
    XES: XESolution,
    GenType: Generator<Yield = Box<dyn Move<XES>>, Return = ()>,
> {
    pub done: bool,
    pub consumed_current: bool,
    //pub gen: dyn Generator<Yield = Box<dyn Move<XES>>, Return = ()>,
    //GeneratorIteratorAdapter::new(firstn(10)) {
    //
    //pub gen_adapted: GeneratorIteratorAdapter<FCoro>,
    //pub fgen: FCoro,
    //fn fgen(&XES) -> Generator<Yield = Box<dyn Move<XES>>, Return = ()>,
    //pub fgen: Box<dyn Fn(&XES) -> GenType>, //Box<dyn FCoro>, //Fn(&XES) -> dyn Generator<Yield = Box<dyn Move<XES>>, Return = ()>,
    pub my_gen: GenType,
    pub phantom_xes: PhantomData<XES>,
    //pub phantom_gen: PhantomData<GenType>,
}
*/

/*
impl<XES: XESolution, GenType: Generator<Yield = Box<dyn Move<XES>>, Return = ()>>
    FxNSIterator<XES, GenType>
//where
//FCoro: Fn(&XES) -> dyn Generator<Yield = Box<dyn Move<XES>>, Return = ()>,
//FCoro: Fn(&XES) -> GenType,
{
    fn new(gen: Box<dyn Fn(&XES) -> GenType>) -> Self {
        Self {
            done: false,
            consumed_current: false,
            //gen_adapted: GeneratorIteratorAdapter(Box::pin(gen)),
            fgen: gen,
            phantom_xes: PhantomData,
            phantom_gen: PhantomData,
        }
    }
}
*/

/*
pub struct FxNSIterator<
    XES: XESolution,
    GenType: Generator<Yield = Box<dyn Move<XES>>, Return = ()>,
> {
    pub done: bool,
    pub consumed_current: bool,
    //pub gen: dyn Generator<Yield = Box<dyn Move<XES>>, Return = ()>,
    //GeneratorIteratorAdapter::new(firstn(10)) {
    //
    //pub gen_adapted: GeneratorIteratorAdapter<FCoro>,
    //pub fgen: FCoro,
    //fn fgen(&XES) -> Generator<Yield = Box<dyn Move<XES>>, Return = ()>,
    pub fgen: Box<dyn Fn(&XES) -> GenType>, //Box<dyn FCoro>, //Fn(&XES) -> dyn Generator<Yield = Box<dyn Move<XES>>, Return = ()>,
    pub phantom_xes: PhantomData<XES>,
    pub phantom_gen: PhantomData<GenType>,
}

impl<XES: XESolution, GenType: Generator<Yield = Box<dyn Move<XES>>, Return = ()>>
    FxNSIterator<XES, GenType>
//where
//FCoro: Fn(&XES) -> dyn Generator<Yield = Box<dyn Move<XES>>, Return = ()>,
//FCoro: Fn(&XES) -> GenType,
{
    fn new(gen: Box<dyn Fn(&XES) -> GenType>) -> Self {
        Self {
            done: false,
            consumed_current: false,
            //gen_adapted: GeneratorIteratorAdapter(Box::pin(gen)),
            fgen: gen,
            phantom_xes: PhantomData,
            phantom_gen: PhantomData,
        }
    }
}
*/

/*
pub struct FxNSIterator<XES: XESolution, FCoro>
where
    FCoro: Fn(&XES) -> dyn Generator<Yield = Box<dyn Move<XES>>, Return = ()>,
{
    pub done: bool,
    pub consumed_current: bool,
    //pub gen: dyn Generator<Yield = Box<dyn Move<XES>>, Return = ()>,
    //GeneratorIteratorAdapter::new(firstn(10)) {
    //
    //pub gen_adapted: GeneratorIteratorAdapter<FCoro>,
    pub fgen: FCoro,
    pub phantom_xes: PhantomData<XES>,
}

impl<XES: XESolution, FCoro> FxNSIterator<XES, FCoro>
where
    FCoro: Fn(&XES) -> dyn Generator<Yield = Box<dyn Move<XES>>, Return = ()>,
{
    fn new(gen: FCoro) -> Self {
        Self {
            done: false,
            consumed_current: false,
            //gen_adapted: GeneratorIteratorAdapter(Box::pin(gen)),
            fgen: gen,
            phantom_xes: PhantomData,
        }
    }
}
*/

// =========
//

/*


pub struct FxNSIterator<XES: XESolution, FCoro>
where
    FCoro: Fn(&XES) -> dyn Generator<Yield = Box<dyn Move<XES>>, Return = ()>,
{
    pub done: bool,
    pub consumedCurrent: bool,
    //pub gen: dyn Generator<Yield = Box<dyn Move<XES>>, Return = ()>,
    //GeneratorIteratorAdapter::new(firstn(10)) {
    pub gen_adapted: GeneratorIteratorAdapter,
}

// ----

impl<XES: XESolution, FCoro> NSIterator<XES> for FxNSIterator<XES, FCoro>
where
    FCoro: Fn(&XES) -> dyn Generator<Yield = Box<dyn Move<XES>>, Return = ()>,
{
    fn first(&self) -> () {
        let first_value = self.gen_adapted.next();
        self.done = !self.gen.next(); // advance and update bool
        self.consumedCurrent = false; // can allow consuming again
    }

    fn next(&self) -> () {
        self.done = !self.gen.next(); // advance and update bool
        self.consumedCurrent = false; // can allow consuming again
    }

    fn is_done(&self) -> bool {
        return self.done; // verify bool
    }

    fn current(&self) -> Option<Box<dyn Move<XES>>> {
        // should never repeat pointer (enforce 'unique' semantics!)
        if self.consumedCurrent {
            return None;
        }
        self.consumedCurrent = true;
        //
        //Box<dyn Move<XES>>
        //
        return self.gen.getValue(); //uptr<Move<XES>>(gen.getValue());
    }
}

*/

// ======================

/*
// =================
//       NSSeq
// =================

pub struct FxNSSeqFancy<XES: XESolution, F, FCoro>
where
    F: Fn(&XES) -> Box<dyn Move<XES>>,
    FCoro: Fn(&XES) -> dyn Generator<Yield = Box<dyn Move<XES>>, Return = ()>,
{
    pub f_random: F,
    pub f_generator: FCoro,
    pub phantom_xes: PhantomData<XES>,
}

impl<XES: XESolution, F, FCoro> NSSeq<XES> for FxNSSeqFancy<XES, F, FCoro>
where
    F: Fn(&XES) -> Box<dyn Move<XES>>,
    FCoro: Fn(&XES) -> dyn Generator<Yield = Box<dyn Move<XES>>, Return = ()>,
{
    // Generator<Move<XES>*> (*fGenerator)(const XES&) // fGenerator: IMPORTANT! must respect 'unique' semantics! never repeat pointer.
    fn random_move(&self, se: &XES) -> Box<dyn Move<XES>> {
        (self.f_random)(se)
    }

    fn get_iterator(&self, se: &XES) -> Box<dyn NSIterator<XES>> {
        //let gen = self.f_generator(se);
        let gen_adapted = GeneratorIteratorAdapter::new((self.f_generator)(se));
        return Box::new(FxNSIterator { gen });
    }
}
*/
