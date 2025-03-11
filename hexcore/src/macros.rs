pub use hexmacros::*;

#[macro_export]
macro_rules! pipeline {
    ($input:expr => $($pipe:expr) =>+) => {
        (|piped| {
            $(
                let piped = ($pipe)(piped);
            )*
            piped
        })($input)
    };
}


pub use crate::pipeline;