use std::{any::{Any, TypeId}, sync::{Arc, Mutex, RwLock}};

use hashbrown::HashMap;

use super::scheduler_context::*;

pub struct SharedState {
    map: HashMap<TypeId, Arc<dyn Any + Send + Sync + 'static>>,
}

impl SharedState {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            map: HashMap::with_capacity(capacity),
        }
    }

    pub fn insert_arc<T>(&mut self, value: Arc<T>) -> Option<Arc<T>>
    where T: Send + Sync + 'static {
        self.map.insert(TypeId::of::<T>(), value)?.downcast().ok()
    }

    /// Do not use this method to insert `Arc<T>`, use `insert_arc` instead.
    pub fn insert<T>(&mut self, value: T) -> Option<Arc<T>>
    where T: Any + Send + Sync + 'static {
        self.insert_arc(Arc::new(value))
    }

    pub fn insert_mutex<T>(&mut self, value: T) -> Option<Arc<Mutex<T>>>
    where T: Send + Sync + 'static {
        self.insert(Mutex::new(value))
    }

    pub fn insert_rw_lock<T>(&mut self, value: T) -> Option<Arc<RwLock<T>>>
    where T: Send + Sync + 'static {
        self.insert(RwLock::new(value))
    }

    pub fn get<T>(&self) -> Option<Arc<T>>
    where T: Any + Send + Sync + 'static {
        let entry = self.map.get(&TypeId::of::<T>())?.clone();
        let arc: Arc<T> = Arc::downcast(entry).ok()?;
        Some(arc.clone())
    }

    pub fn get_mutex<T>(&self) -> Option<Arc<Mutex<T>>>
    where T: Send + Sync + 'static {
        self.get::<Mutex<T>>()
    }

    pub fn get_rw_lock<T>(&self) -> Option<Arc<RwLock<T>>>
    where T: Send + Sync + 'static {
        self.get::<RwLock<T>>()
    }

    pub fn remove<T>(&mut self) -> Option<Arc<T>>
    where T: Any + Send + Sync + 'static {
        self.map.remove(&TypeId::of::<T>())?.downcast().ok()
    }

    pub fn remove_mutex<T>(&mut self) -> Option<Arc<Mutex<T>>>
    where T: Send + Sync + 'static {
        self.remove::<Mutex<T>>()
    }

    pub fn remove_rw_lock<T>(&mut self) -> Option<Arc<RwLock<T>>>
    where T: Send + Sync + 'static {
        self.remove::<RwLock<T>>()
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    pub fn contains<T>(&self) -> bool
    where T: Any + Send + Sync + 'static {
        self.map.contains_key(&TypeId::of::<T>())
    }

    pub fn contains_mutex<T>(&self) -> bool
    where T: Send + Sync + 'static {
        self.contains::<Mutex<T>>()
    }

    pub fn contains_rw_lock<T>(&self) -> bool
    where T: Send + Sync + 'static {
        self.contains::<RwLock<T>>()
    }

    pub fn clear(&mut self) {
        self.map.clear();
    }
}

impl SchedulerContext for SharedState {}

impl<T> ContextResolvable<SharedState> for Arc<T>
where T: Send + Sync + 'static {
    fn resolve(context: &mut SharedState) -> super::scheduler_context::ResolveResult<Self> {
        context.get::<T>().ok_or_else(|| ResolveError::NotFound(std::any::type_name::<Arc<T>>()))
    }
}