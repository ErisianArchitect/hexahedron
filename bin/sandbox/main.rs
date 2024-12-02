#![allow(unused)]

use hexahedron::prelude::Increment;

fn main() {
    
}

// Code Graveyard beyond this point.

/// Code that is completely ignored, as it lay it to rest.
/// But also you wanna keep it around and make it look pretty.
macro_rules! grave {
    ($($_:tt)*) => {};
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