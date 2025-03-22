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

pub struct MeasureTimeResult<T> {
    pub start_time: std::time::Instant,
    pub result: T,
    pub duration: std::time::Duration,
}

#[macro_export]
macro_rules! measure_time {
    ($($token:tt)*) => {
        {
            let start_time = std::time::Instant::now();
            let result = {
                $($token)*
            };
            let duration = start_time.elapsed();
            $crate::macros::MeasureTimeResult {
                start_time,
                result,
                duration,
            }
        }
    };
}

pub use crate::measure_time;