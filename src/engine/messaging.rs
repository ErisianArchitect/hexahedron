use std::any::Any;
use crate::util::traits::AnyTuple;
pub use crate::args;


pub struct Arg {
    value: Box<dyn Any>,
}

impl Arg {
    pub fn new<T: Any>(value: T) -> Self {
        Self {
            value: Box::new(value),
        }
    }

    pub fn downcast<T: Any>(self) -> Result<T, Self> {
        self.value.downcast::<T>()
            .map(|value| *value)
            .map_err(|value| Self { value })
    }

    pub fn downcast_ref<T: Any>(&self) -> Option<&T> {
        self.value.downcast_ref()
    }

    pub fn downcast_mut<T: Any>(&mut self) -> Option<&mut T> {
        self.value.downcast_mut()
    }
}

#[derive(Debug)]
pub struct Args {
    args: Box<dyn Any>,
}

impl Args {
    pub fn new<T: AnyTuple>(args: T) -> Self {
        Self {
            args: Box::new(args),
        }
    }

    pub fn single<T: Any>(arg: T) -> Self {
        Self::new((arg,))
    }

    pub fn downcast<T: AnyTuple>(self) -> Result<T, Self> {
        self
            .args
            .downcast::<T>()
            .map(|args| { *args })
            .map_err(|args| Self { args })
    }

    pub fn downcast_ref<T: AnyTuple>(&self) -> Option<&T> {
        self
            .args
            .downcast_ref::<T>()
    }

    pub fn downcast_mut<T: AnyTuple>(&mut self) -> Option<&mut T> {
        self
            .args
            .downcast_mut()
    }
}

pub fn arg<T: Any>(arg: T) -> Args {
    Args::single(arg)
}

pub fn args<T: AnyTuple>(args: T) -> Args {
    Args::new(args)
}

#[macro_export]
macro_rules! args {
    ($($arg:expr),*$(,)?) => {
        Args::new(($(($arg),)*))
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn args_test() {
        let args = args!(1, 2, 3);
        let (a, b, c) = args.downcast::<(i32, i32, i32)>().unwrap();
        assert_eq!((a, b, c), (1, 2, 3));
        let args = Args::new((4, 5, 6));
        let (a, b, c) = args.downcast::<(i32, i32, i32)>().unwrap();
        assert_eq!((a, b, c), (4, 5, 6));
        // let Ok((a, b, c)) = args.downcast::<(i32, i32, i32)>() else {
        //     panic!("Failed to downcast.");
        // };
        // extract_args!((a: i32, b: i32, c: i32) in { args } else {
        //     panic!("Failed to extract args.");
        // });
        hexmacros::prototype!((a: i32, b: i32, c: i32) in args else {
            panic!("Failed to downcast.");
        });
    }
}