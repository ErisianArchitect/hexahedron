use std::time::{Duration, Instant};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TaskResponse {
    /// Finish task, do nothing.
    #[default]
    Continue,
    Finish,
    /// Reschedule task immediately.
    Immediate,
    /// Reschedule task after [Duration] relative to the instant before the task 
    AfterTaskBegan(Duration),
    /// Reschedule task after [Duration] relative to the instant after the task returns.
    AfterTaskEnds(Duration),
    /// Reschedule task after [Duration] relative to the task's scheduled time.
    AfterScheduled(Duration),
    /// Reschedule task at [Instant].
    At(Instant),
}

impl TaskResponse {
    /// Returns `default` if `self` == `TaskResponse::Finish`.
    #[inline]
    pub fn unfinished_or<D: Into<TaskResponse>>(self, default: D) -> Self {
        match self {
            TaskResponse::Finish => default.into(),
            unfinished => unfinished,
        }
    }

    /// Returns the result `default` if `self` == `TaskResponse::Finish`.
    #[inline]
    pub fn unfinished_or_else<R: Into<TaskResponse>, F: FnOnce() -> R>(self, default: F) -> Self {
        match self {
            TaskResponse::Finish => default().into(),
            unfinished => unfinished,
        }
    }

    #[inline]
    pub const fn finished(self) -> bool {
        matches!(self, Self::Finish)
    }
}

impl From<()> for TaskResponse {
    #[inline]
    fn from(value: ()) -> Self {
        TaskResponse::Finish
    }
}

impl From<Duration> for TaskResponse {
    #[inline]
    fn from(value: Duration) -> Self {
        TaskResponse::AfterTaskEnds(value)
    }
}

impl From<Instant> for TaskResponse {
    #[inline]
    fn from(value: Instant) -> Self {
        TaskResponse::At(value)
    }
}

impl<T: Into<TaskResponse>> From<Option<T>> for TaskResponse {
    #[inline]
    fn from(value: Option<T>) -> Self {
        if let Some(value) = value {
            value.into()
        } else {
            TaskResponse::Finish
        }
    }
}