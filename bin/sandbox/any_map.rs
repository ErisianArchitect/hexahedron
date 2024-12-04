use std::{any::*, sync::{Arc, Mutex, RwLock}};

use hashbrown::HashMap;

pub trait Mappable: Any + Send + Sync + 'static {}

impl<T: Any + Send + Sync + 'static> Mappable for T {}

#[derive(Default)]
pub struct AnyMap {
    map: HashMap<TypeId, Arc<dyn Any + Send + Sync + 'static>>,
}

impl AnyMap {
    pub fn insert<T: Mappable>(&mut self, value: T) -> Option<Arc<T>> {
        self.map.insert(TypeId::of::<T>(), Arc::new(value))?.downcast().ok()
    }

    pub fn insert_mutex<T: Mappable>(&mut self, value: T) -> Option<Arc<Mutex<T>>> {
        self.insert(Mutex::new(value))
    }

    pub fn insert_rw_lock<T: Mappable>(&mut self, value: T) -> Option<Arc<RwLock<T>>> {
        self.insert(RwLock::new(value))
    }

    pub fn get<T: Mappable>(&self) -> Option<Arc<T>> {
        let entry = self.map.get(&TypeId::of::<T>())?.clone();
        let arc: Arc<T> = Arc::downcast(entry).ok()?;
        Some(arc.clone())
    }

    pub fn get_mutex<T: Mappable>(&self) -> Option<Arc<Mutex<T>>> {
        self.get::<Mutex<T>>()
    }

    pub fn get_rw_lock<T: Mappable>(&self) -> Option<Arc<RwLock<T>>> {
        self.get::<RwLock<T>>()
    }

    pub fn remove<T: Mappable>(&mut self) -> Option<Arc<T>> {
        self.map.remove(&TypeId::of::<T>())?.downcast().ok()
    }

    pub fn remove_mutex<T: Mappable>(&mut self) -> Option<Arc<Mutex<T>>> {
        self.remove::<Mutex<T>>()
    }

    pub fn remove_rw_lock<T: Mappable>(&mut self) -> Option<Arc<RwLock<T>>> {
        self.remove::<RwLock<T>>()
    }
}

pub fn any_map_test() {
    let mut map = AnyMap::default();
    map.insert_mutex(String::from("Hello, world!"));
    if let Some(text) = map.get_mutex::<String>() {
        let mut text = text.lock().unwrap();
        *text = "This is a test.".into();
        // println!("{text}");
    }
    if let Some(text) = map.get_mutex::<String>() {
        let mut text = text.lock().unwrap();
        // *text = "This is a test.".into();
        println!("{text}");
    }
    map.remove_mutex::<String>();
}