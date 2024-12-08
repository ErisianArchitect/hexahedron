// invoke.rs

/* What I'll need

`Context` that stores all values that are accessed by callbacks.
`Scheduler` that schedules events
`Queue` for events that are called every frame.

*/
pub mod context;
pub mod scheduler;
pub mod time;
pub mod time_key;
pub mod task_context;
pub mod task_response;
pub mod callback;
pub mod variadic_callback;
pub mod scheduler_context;
pub mod optional;
pub mod tuple_combine;
pub mod tuple_flatten;