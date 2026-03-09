use std::sync::{Arc, atomic::{AtomicBool, Ordering}};

#[derive(Debug, Clone)]
pub struct SharedBool {
    shared: Arc<(AtomicBool, Ordering)>,
}

impl SharedBool {
    pub fn new(value: bool, ordering: Ordering) -> Self {
        Self {
            shared: Arc::new((AtomicBool::new(value), ordering)),
        }
    }

    pub fn load(&self) -> bool {
        self.shared.0.load(self.shared.1)
    }

    pub fn store(&self, value: bool) {
        self.shared.0.store(value, self.shared.1);
    }

    pub fn swap(&self, value: bool) -> bool {
        self.shared.0.swap(value, self.shared.1)
    }
}