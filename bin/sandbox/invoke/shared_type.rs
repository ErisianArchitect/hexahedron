

pub trait ResolveMaybeOptional<T> {
    fn resolve(input: Option<T>) -> Self;
}

impl<T> ResolveMaybeOptional<T> for T {
    fn resolve(input: Option<T>) -> Self {
        input.expect(format!("Failed to resolve type \"{}\".", std::any::type_name::<T>()).as_str())
    }
}

impl<T> ResolveMaybeOptional<T> for Option<T> {
    fn resolve(input: Option<T>) -> Self {
        input
    }
}

pub trait SharedType {
    fn get<T>(&self) -> Option<T>;
}