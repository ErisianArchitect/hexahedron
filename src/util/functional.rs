pub fn eval<R, F: FnOnce() -> R>(f: F) -> R {
    f()
}

pub fn catch<T, E, F: FnOnce() -> Result<T, E>>(f: F) -> Result<T, E> {
    f()
}