use std::ptr::NonNull;

pub struct UnsafeArray<T: Sized> {
    ptr: Option<std::ptr::NonNull<T>>,
    capacity: usize,
}

impl<T: Sized> UnsafeArray<T> {
    pub const fn new() -> Self {
        Self {
            ptr: None,
            capacity: 0,
        }
    }

    pub unsafe fn with_capacity(capacity: usize) -> Self {
        unsafe {
            let layout = Self::make_layout(capacity);
            let ptr = std::alloc::alloc(layout) as *mut T;
            let non_null = std::ptr::NonNull::new(ptr).expect("Allocation failure.");
            Self {
                ptr: Some(non_null),
                capacity,
            }
        }

    }

    fn layout(&self) -> std::alloc::Layout {
        Self::make_layout(self.capacity)
    }

    fn make_layout(size: usize) -> std::alloc::Layout {
        std::alloc::Layout::array::<T>(size).expect("Couldn't create layout.")
    }

    pub unsafe fn resize(&mut self, size: usize) {
        let Some(ptr) = self.ptr else {
            *self = Self::with_capacity(size);
            return;
        };
        let layout = self.layout();
        self.ptr = Some(NonNull::new(std::alloc::realloc(ptr.as_ptr() as *mut u8, layout, size) as *mut T).expect("Allocation failure."));
        self.capacity = size;
    }

    /// Only call this method when you know that all elements
    /// need to be dropped.
    pub unsafe fn dealloc_with_drop(&mut self) {
        let Some(ptr) = self.ptr else {
            return;
        };
        unsafe {
            if std::mem::needs_drop::<T>() {
                (0..self.capacity).for_each(|i| {
                    let item_ptr = ptr.add(i);
                    std::ptr::drop_in_place(item_ptr.as_ptr());
                });
            }
            self.unsafe_dealloc_no_drop();
        }
    }

    pub unsafe fn unsafe_dealloc_no_drop(&mut self) {
        let Some(ptr) = self.ptr.take() else {
            return;
        };
        let layout = self.layout();
        unsafe {
            std::alloc::dealloc(ptr.as_ptr() as *mut u8, layout);
        }
    }

    pub unsafe fn get<'a>(&'a self, index: usize) -> Option<&'a T> {
        if index >= self.capacity {
            return None;
        }
        let Some(ptr) = self.ptr else {
            return None;
        };
        unsafe {
            let item_ptr = ptr.add(index);
            Some(item_ptr.as_ref())
        }
    }

    pub unsafe fn get_mut<'a>(&'a mut self, index: usize) -> Option<&'a mut T> {
        if index >= self.capacity {
            return None;
        }
        let Some(ptr) = self.ptr else {
            return None;
        };
        unsafe {
            let mut item_ptr = ptr.add(index);
            Some(item_ptr.as_mut())
        }
    }
}

impl<T: Sized + Copy> UnsafeArray<T> {
    pub fn safe_dealloc_no_drop(&mut self) {
        unsafe { self.unsafe_dealloc_no_drop() }
    }
}

impl<T: Sized> std::ops::Deref for UnsafeArray<T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        let Some(ptr) = self.ptr else {
            panic!("Not allocated.");
        };
        unsafe {
            std::slice::from_raw_parts(ptr.as_ptr(), self.capacity)
        }
    }
}

impl<T: Sized> std::ops::DerefMut for UnsafeArray<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let Some(ptr) = self.ptr else {
            panic!("Not allocated.");
        };
        unsafe {
            std::slice::from_raw_parts_mut(ptr.as_ptr(), self.capacity)
        }
    }
}

// TODO
pub struct Immut<T>(pub T);

impl<T: Clone> Clone for Immut<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }

    fn clone_from(&mut self, source: &Self) {
        self.0.clone_from(&source.0);
    }
}

impl<T: Copy> Copy for Immut<T> {}

impl<T: std::fmt::Debug> std::fmt::Debug for Immut<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl<T: std::fmt::Display> std::fmt::Display for Immut<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// impl<T: std::cmp::PartialEq> std::cmp::PartialEq<Immut<T>> for Immut<T> {
//     fn eq(&self, other: &Immut<T>) -> bool {
//         self.0 == other.0
//     }
// }

impl<Rhs, T: std::cmp::PartialEq<Rhs>> std::cmp::PartialEq<Rhs> for Immut<T> {
    fn eq(&self, other: &Rhs) -> bool {
        self.0 == *other
    }
}

// impl<T: std::cmp::Eq> std::cmp::Eq for Immut<T> {}

// impl<T: std::cmp::PartialOrd> std::cmp::PartialOrd<Immut<T>> for Immut<T> {
//     fn partial_cmp(&self, other: &Immut<T>) -> Option<std::cmp::Ordering> {
//         self.0.partial_cmp(&other.0)
//     }
// }

impl<Rhs, T: std::cmp::PartialOrd<Rhs>> std::cmp::PartialOrd<Rhs> for Immut<T> {
    fn partial_cmp(&self, other: &Rhs) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(other)
    }
}

// impl<T: std::cmp::Ord> std::cmp::Ord for Immut<T> {
//     fn cmp(&self, other: &Self) -> std::cmp::Ordering {
//         self.0.cmp(&other.0)
//     }
// }

impl<T> std::ops::Deref for Immut<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn unsafe_array_test() {
        unsafe {
            let mut array = UnsafeArray::<i32>::with_capacity(4);
            array[0] = 0;
            array[1] = 1;
            array[2] = 2;
            array.resize(1024);
            println!("{}", array[2]);
            array.safe_dealloc_no_drop();
        }
    }

    #[test]
    fn immut_tes() {
        let val = Immut(String::from("hello, world"));
        println!("{}", val);
        println!("{}", &val == &String::from("hello, world"));
    }
}