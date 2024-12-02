// I don't actually need this because there's std::iter::once
#[inline]
pub const fn once<T>(value: T) -> OnceIterator<T> {
    OnceIterator(Some(value))
}

#[inline]
pub const fn forever<T: Clone>(value: T) -> YieldForever<T> {
    YieldForever(value)
}

#[inline]
pub const fn forever_with<T, F: FnMut() -> T>(f: F) -> YieldForeverWith<T, F> {
    YieldForeverWith(f)
}

#[inline]
pub const fn repeat<T: Clone>(count: usize, value: T) -> RepeatIterator<T> {
    RepeatIterator {
        value,
        count,
        index: 0
    }
}

#[inline]
pub const fn repeat_with<T, F: FnMut() -> T>(count: usize, f: F) -> RepeatWithIterator<T, F> {
    RepeatWithIterator {
        f,
        count,
        index: 0,
    }
}

// These actually aren't functional, I should move them into a module called iter.
pub struct OnceIterator<T>(Option<T>);
pub struct YieldForever<T: Clone>(T);
pub struct YieldForeverWith<T, F: FnMut() -> T>(F);
pub struct RepeatIterator<T: Clone> {
    value: T,
    count: usize,
    index: usize,
}
pub struct RepeatWithIterator<T, F: FnMut() -> T> {
    f: F,
    count: usize,
    index: usize,
}

impl<T> Iterator for OnceIterator<T> {
    type Item = T;
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = if self.0.is_some() { 1 } else { 0 };
        (size, Some(size))
    }


    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.take()
    }
}

impl<T, F: FnMut() -> T> Iterator for YieldForeverWith<T, F> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        Some((self.0)())
    }
}

impl<T: Clone> Iterator for YieldForever<T> {
    type Item = T;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.0.clone())
    }
}

impl<T: Clone> Iterator for RepeatIterator<T> {
    type Item = T;
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = self.count - self.index;
        (size, Some(size))
    }

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.count {
            let result = Some(self.value.clone());
            self.index += 1;
            result
        } else {
            None
        }
    }
}

impl<T, F: FnMut() -> T> Iterator for RepeatWithIterator<T, F> {
    type Item = T;

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = self.count - self.index;
        (size, Some(size))
    }

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.count {
            let result = Some((self.f)());
            self.index += 1;
            result
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::Increment;

    use super::*;
    #[test]
    fn once_test() {
        println!("### once ###");
        for value in once("Hello, world!") {
            println!("{value}");
        }
        println!("### repeat ###");
        for (i, value) in repeat(5, "I will not write code to repeatedly write the same line over and over again.").enumerate() {
            println!("{i}: {value}");
        }
        let mut counter = 0usize;
        forever_with(
            || format!("{:>4}|   The quick brown fox jumps over the lazy dog.", counter.increment())
        ).enumerate().find(|(index, text)| {
            println!("{text}");
            *index > 10
        });
    }
}