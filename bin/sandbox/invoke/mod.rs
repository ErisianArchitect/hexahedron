// invoke.rs

/* What I'll need

`Context` that stores all values that are accessed by callbacks.
`Scheduler` that schedules events
`Queue` for events that are called every frame.

*/
pub mod context;
pub mod scheduler;
pub mod time;