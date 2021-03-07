/// Implements stack of dictionaries, where top one is writable, all remaining ones are read-only,
/// and with ability to get value from anywhere in stack transparently.
/// We will call individual dictionaries in stack "frames".

use std::collections::HashMap;

pub struct Stack<K, V> where K: std::cmp::Eq + std::hash::Hash {
    stack: Vec<HashMap<K, V>>
}

impl<K, V> Stack<K, V> where K: std::cmp::Eq + std::hash::Hash {

    /// Creates new Stack with one Frame in it.
    pub fn new() -> Stack<K, V> {
        Stack { stack: vec!(HashMap::new()) }
    }

    /// Adds Value with given Key in top Frame of this Stack.
    pub fn add_to_current_frame(&mut self, key: K, value: V) {
        let frame = self.stack.last_mut();
        if let Some(frame) = frame {
            frame.insert(key, value);
        }
    }

    /// Adds new empty Frame on top of this Stack.
    pub fn add_frame(&mut self) {
        self.stack.push(HashMap::new());
    }

    /// We can't have Stack without at least one Frame in it,
    /// therefore this method consumes itself and optionally
    /// returns Stack in case there are more frames remaining.
    /*
    pub fn pop_frame(mut self) -> Option<Stack<K, V>> {
        self.stack.pop();
        match self.stack.len() {
            0 => None,
            _ => Some(self)
        }
    }
    */

    pub fn pop_frame(&mut self) {
        self.stack.pop();
    }

    /// Iterates Frames from top to bottom and returns first
    /// occurance of value found for given key.
    pub fn get(&self, key: K) -> Option<&V> {
        for frame in self.stack.iter().rev() {
            if let Some(value) = frame.get(&key) {
                return Some(value);
            }
        }

        None
    }
}