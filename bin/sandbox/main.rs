#![allow(unused)]

use hexahedron::prelude::Increment;

fn main() {
    shared_state_experiment::experiment();
}

mod shared_state_experiment {

    pub fn experiment() {
        let mut injector1 = ArgInjector::new((
                1234i32,
                String::from("Hello, world!")
            ),
            |num: &mut i32, text: &mut String, age: &u8, name: &String| {
                println!("***\nNum: {num}\nText: {text}\nName: {name}\nAge: {age}\n***");
                *num += 1;
            });
        let mut injector2 = ArgInjector::new((
                420i32,
                69i32,
                1337i32
            ),
            |x: &mut i32, y: &mut i32, z: &mut i32, name: &String, occupation: &Occupation| {
                println!("({x}, {y}, {z}): {name}, {occupation:#?}");
                *z += 1;
            }
        );
        fn take_injector<Inject, Args, Injector: ArgInject<Inject, Args>>(injector: &mut Injector) {
            let mut state = State::new("Harold", 84, Occupation::Programmer);
            injector.exec(&state);
        }
        take_injector(&mut injector1);
        take_injector(&mut injector1);
        take_injector(&mut injector2);
        take_injector(&mut injector2);
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum Occupation {
        Programmer,
        Artist,
        Musician,
        SoundDesigner,
        GameDesigner,
        Writer,
    }

    pub struct State {
        name: String,
        age: u8,
        occupation: Occupation,
    }

    impl State {
        pub fn new<Name: Into<String>>(name: Name, age: u8, occupation: Occupation) -> Self {
            Self {
                name: name.into(),
                age,
                occupation,
            }
        }
    }

    pub trait StateFilter {
        fn get_value(state: &State) -> &Self;
    }

    struct StateName<'a>(&'a String);
    struct StateAge;
    struct StateOccupation;

    impl StateFilter for String {
        fn get_value(state: &State) -> &Self {
            &state.name
        }
    }

    impl StateFilter for u8 {
        fn get_value(state: &State) -> &Self {
            &state.age
        }
    }

    impl StateFilter for Occupation {
        fn get_value(state: &State) -> &Self {
            &state.occupation
        }
    }

    pub struct ArgInjector<Args, F> {
        args: Args,
        function: F,
    }

    impl<Args, F> ArgInjector<Args, F> {
        pub fn new(args: Args, function: F) -> Self {
            Self {
                args,
                function,
            }
        }
    }

    pub trait ArgInject<Inject, Args> {
        fn exec(&mut self, state: &State);
    }

    impl<I0, I1, T0: StateFilter, T1: StateFilter, F: Fn(&mut I0, &mut I1, &T0, &T1)> ArgInject<(I0, I1), (T0, T1)> for ArgInjector<(I0, I1), F> {
        fn exec(&mut self, state: &State) {
            let (
                arg0,
                arg1,
            ) = &mut self.args;
            let (
                arg2,
                arg3,
            ) = (
                T0::get_value(state),
                T1::get_value(state),
            );
            (self.function)(arg0, arg1, arg2, arg3);
        }
    }

    impl<I0, I1, I2, T0: StateFilter, T1: StateFilter, F: Fn(&mut I0, &mut I1, &mut I2, &T0, &T1)> ArgInject<(I0, I1, I2), (T0, T1)> for ArgInjector<(I0, I1, I2), F> {
        fn exec(&mut self, state: &State) {
            let (
                arg0,
                arg1,
                arg2,
            ) = &mut self.args;
            let (
                arg3,
                arg4,
            ) = (
                T0::get_value(state),
                T1::get_value(state),
            );
            (self.function)(arg0, arg1, arg2, arg3, arg4);
        }
    }
}

// Code Graveyard beyond this point.

/// Code that is completely ignored, as it lay it to rest.
/// But also you wanna keep it around and make it look pretty.
macro_rules! grave {
    ($($_:tt)*) => {};
}

grave!{
    mod any_experiment {
        use std::sync::{Arc, Mutex};
        pub fn any_experiment() {
            use std::any::*;
            fn take_type<T: Any, R, F: FnMut(&T) -> R>(value: &Arc<Mutex<dyn Any>>, mut access: F) -> Option<R> {
                let lock = value.lock().unwrap();
                if let Some(vref) = lock.downcast_ref() {
                    Some(access(vref))
                } else {
                    None
                }
            }
            struct Somebody(&'static str);
            let somebody: Arc<Mutex<dyn Any>> = Arc::new(Mutex::new(Somebody("Nesya")));
            take_type(&somebody, |body: &Somebody| {
                println!("{}", body.0);
            });
        }
    }
}


grave! {
    fn drop_experiment() {
        // The goal is to find out what happens to an object that is returned
        // from a function/method that is unused. I want to see if it's dropped
        // right away or if it's dropped at the end of the frame.
    
        // Conclusion: It's dropped right away.
    
        struct DropMe(&'static str);
    
        impl Drop for DropMe {
            fn drop(&mut self) {
                println!("Dropped: {}", self.0);
            }
        }
    
        fn returns_dropme() -> DropMe {
            DropMe("returns_dropme()")
        }
    
        {
            let first = DropMe("first");
            returns_dropme();
            println!("After returns_dropme()");
        }
        println!("Out of dropme scope.");
    }
}

grave! {
    fn cancel_worker_test() {
        use cancelable_test::*;
        let mut counter = 0u32;
        cancelable_work(
            || format!("{:>4}|   Hello, world!", counter.increment()),
            |cancel: Cancel, text| {
                println!("{text}");
                cancel.cancel();
            }
        );
    }
    
    mod cancelable_test {
        use std::{cell::RefCell, rc::Rc, sync::atomic::AtomicBool};
    
        #[derive(Debug, Clone)]
        pub struct Cancel(Rc<RefCell<bool>>);
        
        impl Cancel {
            pub fn cancel(&self) {
                self.0.replace(true);
            }
    
            pub fn is_canceled(&self) -> bool {
                *self.0.borrow()
            }
        }
        
        pub fn cancelable_work<T, Args: CancelableWorkerArgs, D: FnMut() -> T, W: CancelableWorker<T, Args>>(mut data_extractor: D, mut worker: W) {
            let cancel = Cancel(Rc::new(RefCell::new(false)));
            while !cancel.is_canceled() {
                let data = data_extractor();
                worker.run(cancel.clone(), data);
            }
        }
    
        pub trait CancelableWorkerArgs {}
    
        impl<T> CancelableWorkerArgs for (T,) {}
        impl<T> CancelableWorkerArgs for (Cancel, T) {}
    
        pub trait CancelableWorker<T, Args: CancelableWorkerArgs> {
            fn run(&mut self, cancel: Cancel, data: T);
        }
    
        impl<T, F> CancelableWorker<T, (Cancel, T)> for F
        where
            F: FnMut(Cancel, T) -> () {
                fn run(&mut self, cancel: Cancel, data: T) {
                    self(cancel, data);
                }
            }
        
        impl<T, F> CancelableWorker<T, (T,)> for F
        where
            F: FnMut(T) -> () {
                fn run(&mut self, cancel: Cancel, data: T) {
                    self(data);
                }
            }
    }
}

grave!{
    fn time_large_collection_clone() {
        struct NoCopy(u32);
        let mut updates: Vec<(NoCopy, IVec3)> = (0..1024*1024).map(|_| {
            (
                NoCopy(rand::random()),
                ivec3(rand::random(), rand::random(), rand::random()),
            )
        }).collect();
        let start_time = std::time::Instant::now();
        
        let update_clone = updates.iter().map(|(_, c)| *c).collect::<Vec<_>>();
        
        let elapsed = start_time.elapsed();
        
        let mut fin = 0i32;
        for c in update_clone.into_iter() {
            fin = fin.wrapping_add(c.x);
            fin = fin.wrapping_add(c.y);
            fin = fin.wrapping_add(c.z);
        }
        println!("Fin: {fin}");
        println!("Elapsed: {}", elapsed.as_secs_f64())
    }
}

grave!{
    fn gen_bsn_table() -> std::fmt::Result {
        use std::fmt::Write;
        let mut table = String::new();
        writeln!(table, "// Column: Multiplier")?;
        writeln!(table, "// Row: 2.pow(Exponent)")?;
        write!(table, "//     ")?;
        for mult in 32.iter() {
            write!(table, "  {mult:2} ")?;
        }
        writeln!(table)?;
        for exp in 8.iter() {
            write!(table, "/* {exp} */ ")?;
            for mult in 32.iter() {
                let size = block_size_notation::<5>(mult, exp);
                write!(table, "{size:04},")?;
            }
            writeln!(table)?;
        }
        println!("{table}");
        Ok(())
    }
}