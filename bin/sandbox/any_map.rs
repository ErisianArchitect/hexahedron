use std::{any::*, sync::{Arc, Mutex, RwLock}};

use hashbrown::HashMap;

pub trait Mappable: Sized {
    fn get(&self) -> Self;
}

impl<T: Any + Clone> Mappable for T {
    fn get(&self) -> Self {
        self.clone()
    }
}

#[derive(Default)]
pub struct AnyMap {
    map: HashMap<TypeId, Box<dyn Any>>,
}

impl AnyMap {
    pub fn insert<T: Any + Clone>(&mut self, value: T) {
        self.map.insert(TypeId::of::<T>(), Box::new(value));
    }

    pub fn insert_arc_mutex<T: 'static>(&mut self, value: T) {
        self.insert(Arc::new(Mutex::new(value)));
    }

    pub fn insert_arc_rw_lock<T: 'static>(&mut self, value: T) {
        self.insert(Arc::new(RwLock::new(value)));
    }

    pub fn get<T: Any + Clone>(&self) -> Option<T> {
        let entry = self.map.get(&TypeId::of::<T>())?;
        let arc: &T = entry.downcast_ref()?;
        Some(arc.clone())
    }

    pub fn get_mutex<T: 'static>(&self) -> Option<Arc<Mutex<T>>> {
        self.get::<Arc<Mutex<T>>>()
    }

    pub fn get_rw_lock<T: 'static>(&self) -> Option<Arc<RwLock<T>>> {
        self.get::<Arc<RwLock<T>>>()
    }
}

pub fn any_map_test() {
    let mut map = AnyMap::default();
    map.insert_arc_mutex(String::from("Hello, world!"));
    if let Some(text) = map.get_mutex::<String>() {
        let mut text = text.lock().unwrap();
        *text = "This is a test.".into();
        println!("{text}");
    }
}