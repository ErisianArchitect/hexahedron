use std::ptr::NonNull;

pub struct UnsafeArray<T: Sized> {
    ptr: Option<std::ptr::NonNull<T>>,
    capacity: usize,
}

impl<T: Sized> UnsafeArray<T> {
    pub fn new(size: usize) -> Self {
        unsafe {
            let layout = Self::make_layout(size);
            let ptr = std::alloc::alloc(layout) as *mut T;
            let non_null = std::ptr::NonNull::new(ptr).expect("Allocation failure.");
            Self {
                ptr: Some(non_null),
                capacity: size,
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
            *self = Self::new(size);
            return;
        };
        self.ptr = NonNull::new(std::alloc::realloc(ptr, layout, new_size))
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
}

impl<T: Sized + Copy> UnsafeArray<T> {
    pub fn safe_dealloc_no_drop(&mut self) {
        unsafe { self.unsafe_dealloc_no_drop() }
    }
}