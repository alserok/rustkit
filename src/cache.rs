use std::collections::HashMap;

pub trait Cache<T> {
    fn set(&mut self, key: String, val: T) where T: Clone;
    fn get(&mut self,key: String) -> Option<T> where T: Clone;
}

pub struct Lru<T> {
    values: HashMap<String, T>,
}

impl<T> Lru<T> {
    pub fn new() -> Self {
        Lru{
            values: HashMap::new(),
        }
    }
}

impl<V: Clone> Cache<V> for Lru<V> {
    fn get(&mut self,key: String) -> Option<V> {
        self.values.get(&key).cloned()
    }

    fn set(&mut self, key: String, val: V) {
        self.values.insert(key.clone(), val);
    }
}