use std::sync::{Arc, atomic::{AtomicBool, Ordering}};

#[derive(Debug, Clone)]
pub struct SharedBool {
    value: Arc<AtomicBool>,
    ordering: Ordering,
}

impl SharedBool {
    pub fn new(value: bool, ordering: Ordering) -> Self {
        Self {
            value: Arc::new(AtomicBool::new(value)),
            ordering,
        }
    }

    pub fn load(&self) -> bool {
        self.value.load(self.ordering)
    }

    pub fn store(&self, value: bool) {
        self.value.store(value, self.ordering);
    }

    pub fn swap(&self, value: bool) -> bool {
        self.value.swap(value, self.ordering)
    }
}