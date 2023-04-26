mod state_machine { // API ina module
    use std::marker::PhantomData;

    // Define the states.  Public
    pub struct A;
    pub struct B;
    pub struct C;

    // The state machine
    pub struct StateMachine<State = A> {  // Default state is A
        _state: PhantomData<State>  //Private. Cant be set outside the module
        // Other variables here
    }

    impl StateMachine<A> {
        pub fn new() -> Self {      // Constructor
            Self { _state:PhantomData::<A> }
        } 
        pub fn b(self) -> StateMachine<B> {
            StateMachine {_state: PhantomData::<B>,}
        }
    }
    impl StateMachine<B> {
        pub fn c(self) -> StateMachine<C> {
            StateMachine { _state: PhantomData::<C> }
        }
    }
    impl StateMachine<C> {
        pub fn b(self) -> StateMachine<B> {
            StateMachine { _state: PhantomData::<B>}
        }
    }
}

fn main(){
  let sm  = crate::state_machine::StateMachine::new();
  let sm = sm.b();
  let sm = sm.c();
  let sm = sm.b();
}