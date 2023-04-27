


mod state_machine { // API ina module
    use std::marker::PhantomData;

    // Define the states.  Public
    pub struct A;
    pub struct B;
    pub struct C;

    // The state machine
    pub struct StateMachine<State=A> {  // Default state is A
        _state: PhantomData<State>  //Private. Cant be set outside the module
        // Other variables here
    }

    impl StateMachine {      // Constructor impl block
        pub fn new() -> Self {      // Constructor
            Self { _state:PhantomData::<A> }
        } 
    }

    // State transitions
    impl StateMachine<A> {
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
    use crate::state_machine::StateMachine;

    let sm  = StateMachine::new();
    let sm = sm.b();
    let sm = sm.c();
    let sm = sm.b();

    use num_traits::Float;

    pub struct Foo<T: Float = f32> {
        val: T,
    }

    impl Foo {
        fn new() -> Self {
            Foo { val: 0.0 }
        }
    }
    impl <T: Float> Foo<T> {
        fn foo(&self) -> &T {
            &self.val
        }
    }

    let y = Foo::new();
    let z = Foo::foo();
    
    println!("Foo: {}",y.foo());

}