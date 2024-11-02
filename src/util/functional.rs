pub fn eval<R, F: FnOnce() -> R>(f: F) -> R {
    f()
}

pub fn catch<T, E, F: FnOnce() -> Result<T, E>>(f: F) -> Result<T, E> {
    f()
}

pub fn once<T>(value: T) -> OnceIterator<T> {
    OnceIterator(Some(value))
}

pub fn forever<T: Clone>(value: T) -> YieldForever<T> {
    YieldForever(value)
}

pub fn repeat<T: Clone>(value: T, count: usize) -> RepeatIterator<T> {
    RepeatIterator {
        value,
        count,
        index: 0
    }
}

pub struct OnceIterator<T>(Option<T>);
pub struct YieldForever<T: Clone>(T);
pub struct RepeatIterator<T: Clone> {
    value: T,
    count: usize,
    index: usize,
}

impl<T> Iterator for OnceIterator<T> {
    type Item = T;
    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = if self.0.is_some() { 1 } else { 0 };
        (size, Some(size))
    }

    fn next(&mut self) -> Option<Self::Item> {
        self.0.take()
    }
}

impl<T: Clone> Iterator for YieldForever<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.0.clone())
    }
}

impl<T: Clone> Iterator for RepeatIterator<T> {
    type Item = T;
    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = self.count - self.index;
        (size, Some(size))
    }

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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn once_test() {
        println!("### once ###");
        for value in once("Hello, world!") {
            println!("{value}");
        }
        println!("### repeat ###");
        for (i, value) in repeat("I will not write code to repeatedly write the same line over and over again.", 5).enumerate() {
            println!("{i}: {value}");
        }
    }
}