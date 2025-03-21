#![allow(unused)]
use std::{hash::Hash, rc::Rc};

use hashbrown::HashMap;
use hexmacros::*;

// mod any_map;
// mod arg_injection;
mod invoke;
// mod scheduler;
// mod shared_state;
mod graph;
mod rng;

use hex::prelude::Increment;
use hexahedron::{self as hex, util::extensions::*};
use rand::Rng;

/*
Next experiment:
    A sparse struct using manual memory management
    and unsafe code.
*/

// #[tokio::main]
// async fn main() {
// }
prototype! {
    (% for #i in 0..32 %)
        pub fn [< test_i #i >]() {
            println!("{}", #i);
        }
    (% continue %)
}

table!(
    /// A test table to see how I like the syntax.
    macro test_table {
        ["Hello, world"]
        ["This is a test. {}", 1]
    }
);

trait ForFn<F> {
    fn call(self);
}

impl<F> ForFn<F> for F
where F: FnOnce() {
    fn call(self) {
        self();
    }
}

fn foo() {
    println!("foo()");
}

fn main() {
    foo.call();
    return;
    fn takes_slice(slice: &[i32]) {
        for val in slice {
            println!("{}", val);
        }
    }
    let val = 1;
    takes_slice(val.as_slice_of_one());

    prototype!(impl<T> Deterministic<Marker<T>> where T: std::hash::Hash {
        T;
        &[T] where T: Seq;
    });
    // becomes
    prototype!(
        impl<T> Deterministic<Marker<T>> for T where T: std::hash::Hash {}
        impl<T> for &[T] {}
    );

    prototype!(crate; token_input());
    prototype!(token_input());
    prototype!(as Deterministic;);

    table!(
        macro colors {}
    );

    test_table!(foreach(println));

    prototype_macro!(invoke_table);

    invoke_table!(test_table; ($first:expr $(, $args:expr)*) => {
        print!("{}", $first);
        $(
            print!(" {}", $args);
        )*
    });
    /* Maybe invoke_table could expand into something like:
    macro_rules! ___invoke_table_artifact {
        ($( { $first:expr $(, $args:expr)* } )*) => {
            $(
                print!("{}", $first);
                $(
                    print!(" {}", $args);
                )*
            )*
        }
    }
    test_table!(___invoke_table_artifact);

    Then you'll have ___invoke_table_artifact as an artifact in your scope,
    but that's not really a problem because you're not forced to use it.
     */

    #[rustfmt::skip]
    foreach!(println!(
        ("Test {}", 0),
        ("hello, world"),
        ("test")
    ));

    define!(
        /// Test print macro.
        macro qprint {
            println!("Hello, world!");
        }
    );

    prototype!(
        define!(macro get_text "This was from the get_text macro.")
        let text = get_text!();
    );

    qprint!();

    return;
    struct WithKey<K, V>
    where
        K: Hash,
    {
        key: K,
        value: V,
    }
    impl<K: Hash + Clone, V> WithKey<K, V> {
        fn new<IK: Into<K>>(key: IK, value: V) -> Self {
            Self {
                key: key.into(),
                value,
            }
        }

        fn key(&self) -> K {
            self.key.clone()
        }
    }
    let mut map = HashMap::<Rc<str>, WithKey<Rc<str>, &'static str>>::new();
    let value1 = WithKey::new("key1", "hello, world");
    let value2 = WithKey::new("key2", "this is a test");
    let value3 = WithKey::new("key3", "I hope this work.");
    map.insert(value1.key(), value1);
    map.insert(value2.key(), value2);
    map.insert(value3.key(), value3);
    if let Some(value) = map.get("key2") {
        println!("  key: {}\nvalue: {}", value.key(), &value.value)
    }
}

mod crypto_experiment {
    use aes::{Aes256, Aes256Dec, Aes256Enc};
    pub fn run() {
        let argon = argon2::Argon2::default();
        // let salt: [u8; 32] = rand::random();
        let salt: &[u8] = blake3::hash(b"salt").as_bytes();
        let salt: &[u8] = &[
            218, 60, 5, 2, 28, 249, 43, 203, 248, 31, 76, 118, 199, 198, 103, 129, 176, 135, 210,
            67, 161, 52, 17, 76, 111, 129, 206, 232, 48, 104, 32, 48,
        ];
        let mut output: [u8; 32] = [0; 32];
        argon
            .hash_password_into(b"password", salt, &mut output)
            .unwrap();
        println!("{:?}", output);
    }
}

pub mod rng_experiment {
    // use std::io::Write;
    use crate::rng::*;

    use hashbrown::HashSet;
    use hexahedron::blockstate;
    // use std::io::Write;
    use rand::prelude::*;

    pub fn run() {
        // let mut rng = (1, 2, 3).make_rng();
        // let mut rng = make_rng((1, 2, 3));
        // let byte = rng.gen::<u8>();
        // println!("{}", byte);
        // println!("{}", rng.gen::<u16>());
        let state = blockstate!(random[test = "hello, world"]);
        let mut rng = make_rng_from_hash(state);
        println!("{}", rng.gen::<u16>());
    }
}

// Code Graveyard beyond this point.
prototype_macro!(
    /// Code that is completely ignored, as it lay it to rest.
    /// But also you wanna keep it around and make it look pretty.
    grave
);

grave! {
    #[experiment(on)]
    mod experiment {
        pub fn run() {

        }
    }
}

grave! {
    mod sched_experiment {
        use hashbrown::HashMap;
        use parking_lot::Mutex;
        use std::{
            env::Args, io::Write, marker::PhantomData, sync::{
                atomic::{AtomicBool, AtomicU32},
                Arc, // Mutex,
            }, time::{Duration, Instant}
        };

        use chrono::Timelike;
        use hexahedron::{math::minmax, prelude::Increment};

        use crate::invoke::{
            callback::*, context::SharedState, modifiers::{
                conditional::*, every::{every, EveryTimeAnchor},
            }, optional::Optional, scheduler::Scheduler, scheduler_context::{*}, scheduler_context::{*}, task_context::{self, TaskContext}, task_response::TaskResponse
        };
        use TaskResponse::*;

        pub fn run() {
            let mut context = SharedState::new();
            context.insert(Mutex::new(vec![
                String::from("Hello, world!"),
                String::from("The quick brown fox jumps over the lazy dog."),
                String::from("This is a test."),
            ]));
            context.insert(AtomicU32::new(0));
            let mut scheduler = Scheduler::<SharedState>::new();
            // scheduler.now();
            // scheduler.after_secs(10, Clear);
            let mut counter = 0u32;
            let trigger = Trigger::new(false);
            context.insert_arc(trigger.clone_inner());
            scheduler.now(every(
                Duration::from_secs(1),
                EveryTimeAnchor::After,
                conditional(trigger.clone(), || {
                    println!("The condition is active!");
                }),
            ));
            let trig2 = trigger.clone();
            // scheduler.after_secs(3, move || {
            //     trig2.activate();
            // });
            scheduler.after_secs(
                3,
                |opt: Optional<Arc<String>>, trigger: Arc<AtomicBool>, mut task_context: TaskContext<'_, SharedState>| {
                    trigger.store(true, std::sync::atomic::Ordering::Relaxed);
                }
            );
            scheduler.after_days(1, || {
                println!("Why were you running this program for a whole day?");
            });
            scheduler.after_secs(10, move || {
                trigger.deactivate();
            });
            scheduler.after_secs(13, Clear);
            scheduler.process_blocking(&mut context);
            // scheduler.now(every(Duration::from_secs(1) / 60, EveryTimeAnchor::Schedule, |num: Optional<Arc<i32>>, mut context: TaskContext<'_, SharedState>| {
            //     println!("Hello, world!");
            // }));

            // (|mut context: Arc<i32>| {}).into_callback();
            // let mut task_ctx: TaskContext<'static, SharedState>;
            // (|mut context: TaskContext<'static, SharedState>| {}).call_mutable((task_ctx,));
            // scheduler.now({
            //     #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
            //     struct Timer(Instant);
            //     impl Timer {
            //         fn time(&mut self) -> Duration {
            //             let duration = self.0.elapsed();
            //             self.0 = Instant::now();
            //             duration
            //         }

            //         fn framerate(self) -> f64 {
            //             1.0 / self.0.elapsed().as_secs_f64()
            //         }
            //     }
            //     let mut timer = Timer(Instant::now());
            //     let mut times = Some(vec![]);
            //     move |atomic: Arc<AtomicU32>, Skip(skip): Skip<Arc<bool>>, mut context: TaskContext<'_, SharedState>| {
            //         let old_time = timer;
            //         let time = timer.time();
            //         if atomic.fetch_add(1, std::sync::atomic::Ordering::Relaxed) > 600 {
            //             let times = times.take().unwrap();
            //             // context.now(move || {
            //             //     for (fps, frametime_diff, fps_diff) in times.iter() {
            //             //         println!("{fps:2.8}  |  {frametime_diff:.8}  |  {fps_diff:.8}");
            //             //     }
            //             // });
            //             return None;
            //         }
            //         let Some(times) = &mut times else {
            //             return None;
            //         };
            //         let (min, max) = minmax(old_time.0.elapsed().as_secs_f64(), 1.0 / 60.0);
            //         let frametime_diff = max - min;
            //         let (min, max) = minmax(old_time.framerate(), 60.0);
            //         let fps_diff = max - min;
            //         times.push((old_time.framerate(), frametime_diff, fps_diff));
            //         // println!("FPS: {:2.8}  |  {frametime_diff:.8}  |  {fps_diff:.8}", old_time.framerate());
            //         Some(AfterTaskBegan(Duration::from_secs_f64(1.0 / 59.0)))
            //     }
            // });
            // scheduler.now(|mut context: TaskContext<'_>| {
            //     let start_time = Instant::now();
            //     let end_time = start_time + Duration::from_secs(20);
            //     let final_time = end_time + Duration::from_secs(5);
            //     context.after_secs(2, move || {
            //         println!("Every 2 Seconds...");
            //         if Instant::now() < end_time {
            //             After(Duration::from_secs(2))
            //         } else if Instant::now() < final_time {
            //             At(final_time)
            //         } else {
            //             Finish
            //         }
            //     });
            //     println!("Starting...");
            //     context.after_secs(3, with((0i32,), |num: &mut i32, string: Arc<Mutex<Vec<String>>>, not_here: Option<Arc<i32>>, mut context: TaskContext<'_>| {
            //         let chron = chrono::Local::now();
            //         // assert!(not_here.is_none());
            //         println!("Frame {:>2} {:>2} {:>16}", num.increment(), chron.second(), chron.timestamp_micros());
            //         if *num < 61 {
            //             Some(Duration::from_secs(1) / 60)
            //         } else {
            //             let mut lock = string.lock();
            //             for s in lock.iter_mut() {
            //                 println!("{s}");
            //                 *s = format!("Frames: {num}");
            //             }
            //             println!("Sleeping for a second.");
            //             spin_sleep::sleep(Duration::from_secs(1));
            //             context.after_secs(3, |strings: Arc<Mutex<Vec<String>>>, mut context: TaskContext<'_>| {
            //                 let chron = chrono::Local::now();
            //                 println!("*** {:>16}", chron.timestamp_micros());
            //                 let before = Instant::now();
            //                 let lock = strings.lock();
            //                 let elapsed = before.elapsed();
            //                 println!("Locked in: {} ns", elapsed.as_nanos());
            //                 for s in lock.iter() {
            //                     println!("{s}");
            //                 }
            //                 // println!("Finished! {}", chron.timestamp_millis());
            //                 for i in 0..10 {
            //                     context.after_millis(i * 100 + 100, move || {
            //                         let chron = chrono::Local::now();
            //                         println!("Hello, world! {i:>2} {:>13}", chron.timestamp_micros());
            //                     });
            //                 }
            //             });
            //             None
            //         }
            //     }));
            // });
        }
    }
}

grave! {
    mod extract_ref {
        use std::{
            borrow::BorrowMut,
            sync::{Arc, Mutex, MutexGuard},
        };

        fn experiment() {
            let arc = Arc::new(Mutex::new(String::from("Hello, world!")));
            let arc_clone = arc.clone();
            let mut lock = lock_mutex(&arc_clone);
            let reft = extract_ref(&mut lock);
            println!("");
        }

        fn lock_mutex<'a, T>(arc: &'a Arc<Mutex<T>>) -> MutexGuard<'a, T> {
            arc.lock().unwrap()
        }

        fn extract_ref<'a, 'b: 'a, T>(guard: &'a mut MutexGuard<'b, T>) -> &'a mut T {
            guard.borrow_mut()
        }
    }
}

grave! {
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

grave! {
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

grave! {
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